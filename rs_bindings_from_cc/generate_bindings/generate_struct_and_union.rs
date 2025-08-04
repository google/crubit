// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

use arc_anyhow::{Context, Result};
use code_gen_utils::{expect_format_cc_type_name, make_rs_ident};
use cpp_type_name::{cpp_tagless_type_name_for_record, cpp_type_name_for_record};
use database::code_snippet::{
    ApiSnippets, AssertableTrait, Assertion, BitPadding, BitfieldComment, DeriveAttr,
    DocCommentAttr, Feature, FieldDefinition, FieldType, GeneratedItem, MustUseAttr,
    NoUniqueAddressAccessor, RecursivelyPinnedAttr, SizeofImpl, StructOrUnion, Thunk, ThunkImpl,
    UpcastImpl, UpcastImplBody, Visibility,
};
use database::db;
use database::rs_snippet::{should_derive_clone, should_derive_copy, RsTypeKind};
use database::BindingsGenerator;
use error_report::{bail, ensure};
use flagset::FlagSet;
use generate_comment::generate_doc_comment;
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use quote::ToTokens;
use std::collections::HashMap;
use std::iter;
use std::num::NonZeroUsize;
use std::rc::Rc;

/// Returns whether fields of type `ty` need to be wrapped in `ManuallyDrop<T>`
/// to prevent the fields from being destructed twice (once by the C++
/// destructor calkled from the `impl Drop` of the struct and once by `drop` on
/// the Rust side).
///
/// A type is safe to destroy twice if it implements `Copy`. Fields of such
/// don't need to be wrapped in `ManuallyDrop<T>` even if the struct
/// containing the fields provides an `impl Drop` that calles into a C++
/// destructor (in addition to dropping the fields on the Rust side).
///
/// Note that it is not enough to just be `!needs_drop<T>()`: Rust only
/// guarantees that it is safe to use-after-destroy for `Copy` types. See
/// e.g. the documentation for
/// [`drop_in_place`](https://doc.rust-lang.org/std/ptr/fn.drop_in_place.html):
///
/// > if `T` is not `Copy`, using the pointed-to value after calling
/// > `drop_in_place` can cause undefined behavior
///
/// For non-Copy union fields, failing to use `ManuallyDrop<T>` would
/// additionally cause a compile-time error until https://github.com/rust-lang/rust/issues/55149 is stabilized.
fn needs_manually_drop(ty: &RsTypeKind) -> bool {
    !ty.implements_copy()
}

/// Generates Rust source code for a given incomplete record declaration.
pub fn generate_incomplete_record(
    db: &dyn BindingsGenerator,
    incomplete_record: Rc<IncompleteRecord>,
) -> Result<ApiSnippets> {
    // If the record won't have bindings, we default to `public` to keep going anyway.
    let visibility = db
        .has_bindings(ir::Item::IncompleteRecord(incomplete_record.clone()))
        .unwrap_or_default()
        .visibility;
    let cc_type = expect_format_cc_type_name(incomplete_record.cc_name.identifier.as_ref());
    let namespace_qualifier = db.ir().namespace_qualifier(&incomplete_record).format_for_cc()?;
    Ok(ApiSnippets {
        generated_items: HashMap::from([(
            incomplete_record.id,
            GeneratedItem::ForwardDeclare {
                visibility,
                ident: make_rs_ident(incomplete_record.rs_name.identifier.as_ref()),
                symbol: quote! {#namespace_qualifier #cc_type}.to_string(),
            },
        )]),
        ..Default::default()
    })
}

fn make_rs_field_ident(field: &Field, field_index: usize) -> Ident {
    match field.rust_identifier.as_ref() {
        None => make_rs_ident(&format!("__unnamed_field{}", field_index)),
        Some(Identifier { identifier }) => make_rs_ident(identifier),
    }
}

/// Gets the type of `field` for layout purposes.
///
/// Note that `get_field_rs_type_kind_for_layout` may return Err even if
/// `rs_type_kind` returns Ok.
///
/// In particular, this happens if the field has an unknown size. For example,
/// if it is an error type, or uses an attribute which is not
/// supported (with the current Crubit features), such as
/// `[[no_unique_address]]`.
///
/// Such unsupported fields should be replaced with a typeless, unaligned block
/// of memory, of a size that can fill up space to the next field.
///
/// See docs/struct_layout
fn get_field_rs_type_kind_for_layout(
    db: &dyn BindingsGenerator,
    record: &Record,
    field: &Field,
) -> Result<RsTypeKind> {
    if field.is_no_unique_address {
        bail!("`[[no_unique_address]]` attribute was present.");
    }
    if let Some(unknown_attr) = &field.unknown_attr {
        // Both the template definition and its instantiation should enable experimental
        // features.
        for target in record.defining_target.iter().chain([&record.owning_target]) {
            let enabled_features = db.ir().target_crubit_features(target);
            ensure!(
                enabled_features.contains(crubit_feature::CrubitFeature::Experimental),
                "unknown field attributes are only supported with experimental features \
                enabled on {target}\nUnknown attribute: {unknown_attr}`"
            );
        }
    }
    let type_kind = match &field.type_ {
        Ok(t) => db.rs_type_kind(t.clone())?,
        Err(e) => bail!("{e}"),
    };

    if let RsTypeKind::Error { error, .. } = type_kind {
        return Err(error.clone());
    }

    if type_kind.is_bridge_type() {
        bail!("Bridge-by-value types are not supported in struct fields.")
    }

    for target in record.defining_target.iter().chain([&record.owning_target]) {
        let enabled_features = db.ir().target_crubit_features(target);
        let (missing_features, reason) = type_kind.required_crubit_features(db, enabled_features);
        ensure!(
            missing_features.is_empty(),
            "missing features: [{missing_features}]: {reason}",
            missing_features = missing_features.into_iter().map(|f| f.aspect_hint()).join(", ")
        );
    }

    // In supported, we replace nontrivial fields with opaque blobs.
    // This is because we likely don't want the `ManuallyDrop<T>` solution to be the
    // one users get.
    //
    // Users can still work around this with accessor functions.
    if record.should_implement_drop() && !record.is_union() && needs_manually_drop(&type_kind) {
        for target in record.defining_target.iter().chain([&record.owning_target]) {
            let enabled_features = db.ir().target_crubit_features(target);
            ensure!(
                enabled_features.contains(crubit_feature::CrubitFeature::Experimental),
                "nontrivial fields would be destroyed in the wrong order"
            );
        }
    }
    Ok(type_kind)
}

fn collect_unqualified_member_functions_from_all_bases(
    db: &dyn BindingsGenerator,
    record: &Record,
) -> Rc<[Rc<Func>]> {
    let ir = db.ir();
    record
        .unambiguous_public_bases
        .iter()
        .flat_map(|base_class| {
            let Ok(item) = ir.find_decl::<Item>(base_class.base_record_id) else {
                return vec![];
            };

            match item {
                Item::Record(base_record) => {
                    db.collect_unqualified_member_functions(base_record.clone()).to_vec()
                }
                _ => vec![],
            }
        })
        .collect()
}

/// Implementation of `BindingsGenerator::collect_unqualified_member_functions`.
pub fn collect_unqualified_member_functions(
    db: &dyn BindingsGenerator,
    record: Rc<Record>,
) -> Rc<[Rc<Func>]> {
    let ir = db.ir();
    record
        .child_item_ids
        .iter()
        .filter_map(|id| {
            let Ok(child_item) = ir.find_decl::<Item>(*id) else {
                return None;
            };

            if let Item::Func(member_function) = child_item {
                if let UnqualifiedIdentifier::Identifier(_) = &member_function.rs_name {
                    return Some(member_function.clone());
                }
            }

            None
        })
        .collect()
}

/// Removes functions that are ambiguous from the list of inherited functions.
///
/// Ambiguous functions are functions that have the same name as a function in
/// the base class.
fn filter_out_ambiguous_member_functions(
    db: &dyn BindingsGenerator,
    derived_record: Rc<Record>,
    inherited_functions: Rc<[Rc<Func>]>,
) -> Rc<[Rc<Func>]> {
    let derived_member_functions = db
        .collect_unqualified_member_functions(derived_record.clone())
        .iter()
        .map(|func| (func.rs_name.clone(), func.clone()))
        .collect::<HashMap<_, _>>();
    let mut func_counter = HashMap::<_, (&Rc<Func>, u32)>::new();
    for func in inherited_functions.iter() {
        let Ok(Some(_)) = db.generate_function(func.clone(), None) else {
            continue;
        };
        let unqualified_name = &func.rs_name;
        if derived_member_functions.contains_key(unqualified_name) {
            continue;
        }
        func_counter
            .entry(unqualified_name.clone())
            .and_modify(|pair| pair.1 += 1)
            .or_insert((func, 1));
    }
    func_counter
        .values()
        .filter_map(|(func, count)| if *count == 1 { Some((*func).clone()) } else { None })
        // Sort by name to make the output deterministic.
        .sorted_by_key(|func| func.rs_name.identifier_as_str().unwrap().to_string())
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn field_definition(
    db: &dyn BindingsGenerator,
    record: &Record,
    field: Option<&ir::Field>,
    field_index: usize,
    prev_end: usize,
    offset: usize,
    end: usize,
    desc: &[BitfieldComment],
    override_alignment: &mut bool,
    fields_that_must_be_copy: &mut Vec<TokenStream>,
) -> Result<FieldDefinition> {
    // opaque blob representations are always unaligned, even though the actual C++
    // field might be aligned. To put the current field at the right offset, we
    // might need to insert some extra padding.
    //
    // No padding should be needed if the type of the current field is
    // known (i.e. if the current field is correctly aligned based on
    // its original type).
    //
    // We also don't need padding if we're in a union.
    let padding_size_in_bits = if record.is_union()
        || field.map(|f| get_field_rs_type_kind_for_layout(db, record, f).is_ok()).unwrap_or(false)
    {
        0
    } else {
        let padding_start = (prev_end + 7) / 8 * 8; // round up to byte boundary
        offset - padding_start
    };

    let padding = NonZeroUsize::new(padding_size_in_bits).map(BitPadding);

    // Bitfields get represented by private padding to ensure overall
    // struct layout is compatible.
    let Some(field) = field else {
        *override_alignment = true;
        return Ok(FieldDefinition::Bitfield {
            field_index,
            desc: desc.to_vec(),
            padding,
            bits: BitPadding(
                NonZeroUsize::new(end - offset)
                    .expect("Bit padding should always be greater than 0"),
            ),
        });
    };

    let ident = make_rs_field_ident(field, field_index);
    let field_rs_type_kind = get_field_rs_type_kind_for_layout(db, record, field);
    let doc_comment = match &field_rs_type_kind {
        Ok(_) => generate_doc_comment(field.doc_comment.as_deref(), None, db.environment()),
        Err(msg) => {
            use std::fmt::Write;

            let mut new_text = field
                .doc_comment
                .as_deref()
                .map(|doc_comment| format!("{doc_comment}\n\n"))
                .unwrap_or_default();
            let _ = write!(
                &mut new_text,
                "Reason for representing this field as a blob of bytes:\n{msg:#}"
            );
            generate_doc_comment(Some(new_text.as_str()), None, db.environment())
        }
    };
    let visibility = if field.access == AccessSpecifier::Public && field_rs_type_kind.is_ok() {
        db::type_visibility(db, &record.owning_target, field_rs_type_kind.clone().unwrap())
            .unwrap_or_default()
    } else {
        Visibility::PubCrate
    };

    let field_type = match field_rs_type_kind {
        Err(_) => {
            *override_alignment = true;
            FieldType::Erased(BitPadding(
                NonZeroUsize::new(end - field.offset)
                    .expect("Bit padding should always be greater than 0"),
            ))
        }
        Ok(type_kind) => {
            let ty = type_kind.to_token_stream(db);
            let mut wrap_in_manually_drop = false;
            if record.should_implement_drop() || record.is_union() {
                if needs_manually_drop(&type_kind) {
                    // TODO(b/212690698): Avoid (somewhat unergonomic) ManuallyDrop
                    // if we can ask Rust to preserve field destruction order if the
                    // destructor is the SpecialMemberFunc::NontrivialMembers
                    // case.
                    wrap_in_manually_drop = true;
                } else {
                    fields_that_must_be_copy.push(ty.clone());
                }
            };
            FieldType::Type { needs_manually_drop: wrap_in_manually_drop, ty }
        }
    };

    Ok(FieldDefinition::Field { field_index, padding, doc_comment, visibility, ident, field_type })
}

/// Implementation of `BindingsGenerator::generate_record`.
pub fn generate_record(db: &dyn BindingsGenerator, record: Rc<Record>) -> Result<ApiSnippets> {
    let record_rs_type_kind = db.rs_type_kind(record.as_ref().into())?;
    if matches!(
        &record_rs_type_kind,
        RsTypeKind::Record { uniform_repr_template_type: Some(_), .. }
    ) {
        return Ok(ApiSnippets::default());
    }
    if record_rs_type_kind.is_bridge_type() {
        return Ok(ApiSnippets::default());
    }
    let ir = db.ir();
    let crate_root_path = ir.crate_root_path_tokens();
    let ident = make_rs_ident(record.rs_name.identifier.as_ref());
    let namespace_qualifier = ir.namespace_qualifier(&record).format_for_rs();
    let qualified_ident = {
        quote! { #crate_root_path:: #namespace_qualifier #ident }
    };

    struct FieldWithLayout<'a> {
        /// The IR field. Note that bitfields are represented as `None`.
        ir: Option<&'a ir::Field>,
        /// The offset of the field in the struct.
        offset: usize,
        /// The offset of the end of the field or `None` for opaque fields.
        end: Option<usize>,
        description: Vec<BitfieldComment>,
    }

    let fields_with_bounds: Vec<FieldWithLayout> = record
        .fields
        .iter()
        .filter_map(|field| {
            let size = NonZeroUsize::new(field.size)?;

            Some(FieldWithLayout {
                // We don't represent bitfields directly in Rust. We drop the field itself here
                // and only retain the offset information. Adjacent bitfields then get merged in
                // the next step.
                ir: if field.is_bitfield { None } else { Some(field) },
                offset: field.offset,
                // We retain the end offset of fields only if we have a matching Rust type
                // to represent them. Otherwise we'll fill up all the space to the next field.
                // See: docs/struct_layout
                end: match get_field_rs_type_kind_for_layout(db, &record, field) {
                    // Regular field
                    Ok(_rs_type) => Some(field.offset + field.size),
                    // Opaque field
                    Err(_error) => {
                        if record.is_union() {
                            Some(field.size)
                        } else {
                            None
                        }
                    }
                },
                description: vec![BitfieldComment {
                    field_name: field.rust_identifier.as_ref().map(|i| i.identifier.clone()),
                    bits: size,
                }],
            })
        })
        // Merge consecutive bitfields. This is necessary because they may share storage in the
        // same byte.
        .coalesce(|first, second| {
            if first.ir.is_none() && second.ir.is_none() {
                Ok(FieldWithLayout {
                    ir: None,
                    offset: first.offset,
                    end: second.end,
                    description: [first.description, second.description].concat(),
                })
            } else {
                Err((first, second))
            }
        })
        .collect();

    let mut override_alignment = record.override_alignment;
    let mut fields_that_must_be_copy = vec![];

    // Pair up fields with the preceeding and following fields (if any):
    // - the end offset of the previous field determines if we need to insert
    //   padding.
    // - the start offset of the next field may be need to grow the current field to
    //   there.
    // This uses two separate `map` invocations on purpose to limit available state.
    let field_definitions = iter::once(None)
        .chain(fields_with_bounds.iter().map(Some))
        .chain(iter::once(None))
        .tuple_windows()
        .enumerate()
        .map(|(index, (prev, cur, next))| {
            let cur = cur.unwrap();
            let prev_end = prev.and_then(|p| p.end).unwrap_or(cur.offset);
            let next_offset = next.map(|n| n.offset);
            let end = cur.end.or(next_offset).unwrap_or(record.size_align.size * 8);

            if let Some(&FieldWithLayout { ir: Some(prev_ir), end: Some(prev_end), .. }) = prev {
                assert!(
                    record.is_union() || prev_end <= cur.offset,
                    "Unexpected offset+size for field {:?} in record {}",
                    prev_ir,
                    record.cc_name
                );
            }
            field_definition(
                db,
                &record,
                cur.ir,
                index,
                prev_end,
                cur.offset,
                end,
                &cur.description,
                &mut override_alignment,
                &mut fields_that_must_be_copy,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    let field_offset_assertions = Assertion::FieldOffsets {
        qualified_ident: qualified_ident.clone(),
        fields_and_expected_offsets: fields_with_bounds
            .iter()
            .enumerate()
            .filter_map(|(field_index, field_with_layout)| {
                let field = field_with_layout.ir?;
                let field_ident = make_rs_field_ident(field, field_index);

                // The assertion below reinforces that the division by 8 on the next line is
                // justified (because the bitfields have been coallesced / filtered out
                // earlier).
                assert_eq!(field.offset % 8, 0);
                let expected_offset = field.offset / 8;

                Some((field_ident, expected_offset))
            })
            .collect(),
    };
    let mut api_snippets = ApiSnippets::default();
    let recursively_pinned_attr = if record.is_unpin() {
        None
    } else {
        // negative_impls are necessary for universal initialization due to Rust's
        // coherence rules: PhantomPinned isn't enough to prove to Rust that a
        // blanket impl that requires Unpin doesn't apply. See http://<internal link>=h.f6jp8ifzgt3n
        api_snippets.features |= Feature::negative_impls;
        Some(RecursivelyPinnedAttr { pinned_drop: record.should_implement_drop() })
    };

    // Adjust the struct to also include base class subobjects, vtables, etc.
    let head_padding = if let Some(first_field) = record.fields.first() {
        first_field.offset / 8
    } else {
        record.size_align.size
    };
    // Prevent direct initialization for non-aggregate structs.
    //
    // Technically, any implicit-lifetime type is going to be fine to initialize
    // using direct initialization of the fields, even if it is not an aggregate,
    // because this is "just" setting memory to the appropriate values, and
    // implicit-lifetime types can automatically begin their lifetime without
    // running a constructor at all.
    //
    // However, not all types used in interop are implicit-lifetime. For example,
    // while any `Unpin` C++ value is, some `!Unpin` structs (e.g. `std::list`)
    // will not be. So for consistency, we apply the same rule for both
    // implicit-lifetime and non-implicit-lifetime types: the C++ rule, that the
    // type must be an *aggregate* type.
    //
    // TODO(b/232969667): Protect unions from direct initialization, too.
    let allow_direct_init = record.is_aggregate || record.is_union();
    let head_padding =
        if head_padding > 0 || !allow_direct_init { Some(head_padding) } else { None };

    api_snippets.cc_details.push(cc_struct_layout_assertion(db, &record)?);

    let fully_qualified_cc_name = cpp_tagless_type_name_for_record(&record, ir)?.to_string();

    let mut items = vec![];
    let mut nested_items = vec![];
    for &child_item_id in &record.child_item_ids {
        let item = ir.find_untyped_decl(child_item_id);
        api_snippets.append(db.generate_item(item.clone())?);
        if item.place_in_nested_module_if_nested_in_record()
            && db.has_bindings(item.clone()).is_ok()
        {
            nested_items.push(child_item_id);
        } else {
            items.push(child_item_id);
        }
    }

    let mut indirect_functions = vec![];
    filter_out_ambiguous_member_functions(
        db,
        record.clone(),
        collect_unqualified_member_functions_from_all_bases(db, &record),
    )
    .iter()
    .filter_map(|unambiguous_base_class_member_function| -> Option<ApiSnippets> {
        let item = ir.find_untyped_decl(unambiguous_base_class_member_function.id);
        let Item::Func(ir_func) = item else { panic!("Unexpected item type: {:?}", item) };
        let generated_func =
            db.generate_function(ir_func.clone(), Some(record.clone())).ok().flatten()?;
        Some((*generated_func.snippets).clone())
    })
    .for_each(|mut func_snippets| {
        // After generating the functions pertaining to our record, we pull them out of the
        // generated_items list and add them to the indirect_functions list.
        assert_eq!(
            func_snippets.generated_items.len(),
            1,
            "Expected exactly one generated item per function"
        );
        for (_itemid, generated_item) in func_snippets.generated_items.drain() {
            let GeneratedItem::Func(generated_func) = generated_item else {
                unreachable!("generate_function only creates GeneratedItem::Func");
            };
            indirect_functions.push(generated_func);
        }
        api_snippets.append(func_snippets);
    });

    // Both the template definition and its instantiation should enable experimental
    // features.
    let mut crubit_features = ir.target_crubit_features(&record.owning_target);
    if let Some(defining_target) = &record.defining_target {
        crubit_features |= ir.target_crubit_features(defining_target);
    }
    let mut upcast_impls = vec![];
    if crubit_features.contains(crubit_feature::CrubitFeature::Experimental) {
        let (new_upcast_impls, thunks, thunk_impls) = cc_struct_upcast_impl(db, &record, ir)?;
        upcast_impls = new_upcast_impls;
        api_snippets.thunks.extend(thunks);
        api_snippets.cc_details.extend(thunk_impls);
    }
    let no_unique_address_accessors =
        if crubit_features.contains(crubit_feature::CrubitFeature::Experimental) {
            cc_struct_no_unique_address_impl(db, &record)?
        } else {
            vec![]
        };
    let incomplete_definition = if crubit_features.contains(crubit_feature::CrubitFeature::Wrapper)
    {
        Some(quote! {
            forward_declare::unsafe_define!(forward_declare::symbol!(#fully_qualified_cc_name), #qualified_ident);
        })
    } else {
        None
    };

    let cxx_impl = if fully_qualified_cc_name.contains('<') {
        // cxx can't parse templated type names.
        // In particular, it can only parse ::-delimited idents.
        None
    } else {
        Some(database::code_snippet::CxxExternTypeImpl {
            id: Rc::from(fully_qualified_cc_name.as_ref()),
            kind: if record.is_unpin() {
                database::code_snippet::CxxKind::Trivial
            } else {
                database::code_snippet::CxxKind::Opaque
            },
        })
    };

    let record_tokens = database::code_snippet::Record {
        doc_comment_attr: generate_doc_comment(
            record.doc_comment.as_deref(),
            Some(&record.source_loc),
            db.environment(),
        ),
        derive_attr: generate_derives(&record),
        recursively_pinned_attr,
        must_use_attr: record.nodiscard.clone().map(MustUseAttr),
        align: if override_alignment && record.size_align.alignment > 1 {
            Some(record.size_align.alignment)
        } else {
            None
        },
        crubit_annotation: DocCommentAttr(
            format!("CRUBIT_ANNOTATE: cpp_type={fully_qualified_cc_name}").into(),
        ),
        visibility: db
            .has_bindings(ir::Item::Record(record.clone()))
            .unwrap_or_default()
            .visibility,
        struct_or_union: if record.is_union() {
            StructOrUnion::Union
        } else {
            StructOrUnion::Struct
        },
        ident,
        head_padding,
        field_definitions,
        implements_send: record.trait_derives.send,
        implements_sync: record.trait_derives.sync,
        cxx_impl,
        incomplete_definition,
        upcast_impls,
        no_unique_address_accessors,
        items,
        nested_items,
        indirect_functions,
    };

    api_snippets.features |= Feature::negative_impls;
    let record_trait_assertions = {
        let mut assert_impls = FlagSet::empty();
        let mut assert_not_impls = FlagSet::empty();
        if should_derive_clone(&record) {
            assert_impls |= AssertableTrait::Clone;
        } else {
            // Can't `assert_not_impl_any!` here, because `Clone` may be
            // implemented rather than derived.
        }
        if should_derive_copy(&record) {
            assert_impls |= AssertableTrait::Copy;
        } else {
            assert_not_impls |= AssertableTrait::Copy;
        }
        if record.should_implement_drop() {
            assert_impls |= AssertableTrait::Drop;
        } else {
            assert_not_impls |= AssertableTrait::Drop;
        }
        Assertion::Impls {
            type_name: record_rs_type_kind.to_token_stream(db),
            all_of: assert_impls,
            none_of: assert_not_impls,
        }
    };

    api_snippets.assertions.push(rs_size_align_assertions(qualified_ident, &record.size_align));
    api_snippets.assertions.push(record_trait_assertions);
    api_snippets.assertions.push(field_offset_assertions);
    api_snippets.assertions.extend(fields_that_must_be_copy.into_iter().map(
        |formatted_field_type| Assertion::Impls {
            type_name: formatted_field_type,
            all_of: AssertableTrait::Copy.into(),
            none_of: FlagSet::empty(),
        },
    ));

    api_snippets.generated_items.insert(record.id, GeneratedItem::Record(Box::new(record_tokens)));
    Ok(api_snippets)
}

pub fn rs_size_align_assertions(type_name: TokenStream, size_align: &ir::SizeAlign) -> Assertion {
    Assertion::SizeAlign { type_name, size: size_align.size, alignment: size_align.alignment }
}

pub fn generate_derives(record: &Record) -> DeriveAttr {
    let mut derives = vec![];
    if should_derive_clone(record) {
        derives.push(quote! { Clone });
    }
    if should_derive_copy(record) {
        derives.push(quote! { Copy });
        derives.push(quote! { ::ctor::MoveAndAssignViaCopy });
    }
    if record.trait_derives.debug == TraitImplPolarity::Positive {
        derives.push(quote! { Debug });
    }
    for custom_trait in &record.trait_derives.custom {
        // Breaks for paths right now...
        derives.push(make_rs_ident(custom_trait).to_token_stream());
    }
    DeriveAttr(derives)
}

fn cc_struct_layout_assertion(db: &dyn BindingsGenerator, record: &Record) -> Result<ThunkImpl> {
    let namespace_qualifier = db.ir().namespace_qualifier(record).format_for_cc()?;
    let fields_and_expected_offsets: Vec<(TokenStream, usize)> = record
        .fields
        .iter()
        .filter_map(|field| {
            if field.access != AccessSpecifier::Public {
                return None;
            }

            // https://en.cppreference.com/w/cpp/types/offsetof points out that "if member is [...]
            // a bit-field [...] the behavior [of `offsetof` macro] is undefined.".  In such
            // scenario clang reports an error: cannot compute offset of bit-field 'field_name'.
            if field.is_bitfield {
                return None;
            }

            // The IR contains the offset in bits, while `CRUBIT_OFFSET_OF` returns the
            // offset in bytes, so we need to convert.  We can assert that
            // `field.offset` is always at field boundaries, because the
            // bitfields have been filtered out earlier.
            assert_eq!(field.offset % 8, 0);
            let expected_offset = field.offset / 8;
            let field_ident =
                expect_format_cc_type_name(&field.cpp_identifier.as_ref()?.identifier);
            Some((field_ident, expected_offset))
        })
        .collect();

    // only use CRUBIT_SIZEOF for alignment > 1, so as to simplify the generated
    // code.
    let sizeof_impl = if record.size_align.alignment > 1 {
        SizeofImpl::RoundUpToAlignment
    } else {
        SizeofImpl::Builtin
    };

    Ok(ThunkImpl::LayoutAssertion {
        tag_kind: if record.is_anon_record_with_typedef { None } else { Some(record.record_type) },
        namespace_qualifier,
        record_ident: record.cc_name.identifier.clone(),
        sizeof_impl,
        size: record.size_align.size,
        alignment: record.size_align.alignment,
        fields_and_expected_offsets,
    })
}

/// Returns the accessor functions for no_unique_address member variables.
fn cc_struct_no_unique_address_impl(
    db: &dyn BindingsGenerator,
    record: &Record,
) -> Result<Vec<NoUniqueAddressAccessor>> {
    let mut no_unique_address_accessors = vec![];
    for field in &record.fields {
        if field.access != AccessSpecifier::Public || !field.is_no_unique_address {
            continue;
        }
        // `[[no_unique_address]]` cannot be applied to a bitfield.
        // See e.g. https://en.cppreference.com/w/cpp/language/attributes/no_unique_address
        // Indeed, this is a compilation error in Clang.
        assert_eq!(field.offset % 8, 0, "invalid subobject: [[no_unique_address]] on a bitfield");

        // Can't use `get_field_rs_type_kind_for_layout` here, because we want to dig
        // into no_unique_address fields, despite laying them out as opaque
        // blobs of bytes.
        let Ok(cpp_type) = field.type_.as_ref() else {
            continue;
        };

        let type_ident = db.rs_type_kind(cpp_type.clone()).with_context(|| {
            format!("Failed to format type for field {field:?} on record {record:?}")
        })?;
        no_unique_address_accessors.push(NoUniqueAddressAccessor {
            doc_comment: if field.size == 0 {
                // These fields are not generated at all, so they need to be documented here.
                generate_doc_comment(field.doc_comment.as_deref(), None, db.environment())
            } else {
                // all other fields already have a doc-comment at the point they were defined.
                None
            },
            field: make_rs_ident(
                &field
                    .rust_identifier
                    .as_ref()
                    .expect("Unnamed fields can't be annotated with [[no_unique_address]]")
                    .identifier,
            ),
            type_: type_ident.to_token_stream(db),
            byte_offset: field.offset / 8,
        });
    }
    Ok(no_unique_address_accessors)
}

type UpcastImplResult = Result<UpcastImpl, String>;

/// Returns the implementation of base class conversions, for converting a type
/// to its unambiguous public base classes.
fn cc_struct_upcast_impl(
    db: &dyn BindingsGenerator,
    record: &Rc<Record>,
    ir: &IR,
) -> Result<(Vec<UpcastImplResult>, Vec<Thunk>, Vec<ThunkImpl>)> {
    let mut thunks = vec![];
    let mut thunk_impls = vec![];
    let mut upcast_impls = vec![];
    let derived_name = db.rs_type_kind(record.as_ref().into())?.to_token_stream(db);
    for base in &record.unambiguous_public_bases {
        let base_record: &Rc<Record> = ir
            .find_decl(base.base_record_id)
            .with_context(|| format!("Can't find a base record of {:?}", record))?;
        let Ok(base_type) = db.rs_type_kind(base_record.as_ref().into()) else {
            // The base type is unknown to Crubit, so don't generate upcast code for it.
            upcast_impls.push(Err(format!(
                "'{}' cannot be upcasted to '{}' because the base type doesn't have Crubit bindings.",
                &record.cc_name,
                &base_record.cc_name,
            )));
            continue;
        };
        if let RsTypeKind::Error { .. } = base_type {
            continue;
        }
        if base_type.is_bridge_type() {
            // The base class isn't directly represented in Rust, so we can't upcast to it.
            continue;
        }
        let base_name = base_type.to_token_stream(db);
        let body = if let Some(offset) = base.offset {
            UpcastImplBody::PointerOffset { offset }
        } else {
            let cast_fn_name = make_rs_ident(&format!(
                "__crubit_dynamic_upcast__{derived}__to__{base}_{odr_suffix}",
                derived = record.mangled_cc_name,
                base = base_record.mangled_cc_name,
                odr_suffix = record.owning_target.convert_to_cc_identifier(),
            ));
            let base_cc_name = cpp_type_name_for_record(base_record.as_ref(), ir)?;
            let derived_cc_name = cpp_type_name_for_record(record.as_ref(), ir)?;

            thunks.push(Thunk::Upcast {
                cast_fn_name: cast_fn_name.clone(),
                derived_name: derived_name.clone(),
                base_name: base_name.clone(),
            });
            thunk_impls.push(ThunkImpl::Upcast {
                base_cc_name: base_cc_name.clone(),
                cast_fn_name: cast_fn_name.clone(),
                derived_cc_name: derived_cc_name.clone(),
            });

            UpcastImplBody::CastThunk {
                crate_root_path: ir.crate_root_path().as_deref().map(make_rs_ident),
                cast_fn_name,
            }
        };

        upcast_impls.push(Ok(UpcastImpl {
            base_name: base_name.clone(),
            derived_name: derived_name.clone(),
            body,
        }));
    }

    Ok((upcast_impls, thunks, thunk_impls))
}
