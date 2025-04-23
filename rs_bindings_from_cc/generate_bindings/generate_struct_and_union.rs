// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

use arc_anyhow::{Context, Result};
use code_gen_utils::{expect_format_cc_type_name, make_rs_ident};
use cpp_type_name::{cpp_tagless_type_name_for_record, cpp_type_name_for_record};
use database::code_snippet::ApiSnippets;
use database::rs_snippet::{should_derive_clone, should_derive_copy, RsTypeKind};
use database::BindingsGenerator;
use error_report::{bail, ensure};
use generate_comment::generate_doc_comment;
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;
use std::collections::{BTreeSet, HashMap};
use std::iter;
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
    let ident = make_rs_ident(incomplete_record.rs_name.identifier.as_ref());
    let cc_type = expect_format_cc_type_name(incomplete_record.cc_name.identifier.as_ref());
    let namespace_qualifier = db.ir().namespace_qualifier(&incomplete_record).format_for_cc()?;
    let symbol = quote! {#namespace_qualifier #cc_type}.to_string();
    Ok(quote! {
        forward_declare::forward_declare!(
            pub #ident __SPACE__ = __SPACE__ forward_declare::symbol!(#symbol)
        );
    }
    .into())
}

fn make_rs_field_ident(field: &Field, field_index: usize) -> Ident {
    match field.identifier.as_ref() {
        None => make_rs_ident(&format!("__unnamed_field{}", field_index)),
        Some(Identifier { identifier }) => make_rs_ident(identifier),
    }
}

/// Gets the type of `field` for layout purposes.
///
/// Note that `get_field_rs_type_kind_for_layout` may return Err even if
/// `rs_type_kind` returns Ok.
///
/// In particular, this happens if the field has an attribute which is not
/// supported (with the current Crubit features). For example,
/// `[[no_unique_address]]`, or an unrecognized attribute.
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

/// Returns the type of a type-less, unaligned block of memory that can hold a
/// specified number of bits, rounded up to the next multiple of 8.
fn bit_padding(padding_size_in_bits: usize) -> TokenStream {
    let padding_size = Literal::usize_unsuffixed((padding_size_in_bits + 7) / 8);
    quote! { [::core::mem::MaybeUninit<u8>; #padding_size] }
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

fn field_definition(
    db: &dyn BindingsGenerator,
    record: &Record,
    field: Option<&ir::Field>,
    field_index: usize,
    prev_end: usize,
    offset: usize,
    end: usize,
    desc: &[String],
    override_alignment: &mut bool,
    field_copy_trait_assertions: &mut Vec<TokenStream>,
) -> Result<TokenStream> {
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
        || (field.is_some()
            && get_field_rs_type_kind_for_layout(db, record, field.unwrap()).is_ok())
    {
        0
    } else {
        let padding_start = (prev_end + 7) / 8 * 8; // round up to byte boundary
        offset - padding_start
    };

    let padding = if padding_size_in_bits == 0 {
        quote! {}
    } else {
        let padding_name = make_rs_ident(&format!("__padding{}", field_index));
        let padding_type = bit_padding(padding_size_in_bits);
        quote! { #padding_name: #padding_type, }
    };

    // Bitfields get represented by private padding to ensure overall
    // struct layout is compatible.
    if field.is_none() {
        let name = make_rs_ident(&format!("__bitfields{}", field_index));
        let bitfield_padding = bit_padding(end - offset);
        *override_alignment = true;
        return Ok(quote! {
            __NEWLINE__ #(  __COMMENT__ #desc )*
            #padding #name: #bitfield_padding
        });
    }
    let field = field.unwrap();

    let ident = make_rs_field_ident(field, field_index);
    let field_rs_type_kind = get_field_rs_type_kind_for_layout(db, record, field);
    let doc_comment = match &field_rs_type_kind {
        Ok(_) => generate_doc_comment(field.doc_comment.as_deref(), None, db.environment()),
        Err(msg) => {
            *override_alignment = true;
            let supplemental_text =
                format!("Reason for representing this field as a blob of bytes:\n{:#}", msg);
            let new_text = match &field.doc_comment {
                None => supplemental_text,
                Some(old_text) => format!("{}\n\n{}", old_text.as_ref(), supplemental_text),
            };
            generate_doc_comment(Some(new_text.as_str()), None, db.environment())
        }
    };
    let access = if field.access == AccessSpecifier::Public && field_rs_type_kind.is_ok() {
        quote! { pub }
    } else {
        quote! { pub(crate) }
    };

    let field_type = match field_rs_type_kind {
        Err(_) => bit_padding(end - field.offset),
        Ok(type_kind) => {
            let mut formatted = type_kind.to_token_stream(db);
            if record.should_implement_drop() || record.is_union() {
                if needs_manually_drop(&type_kind) {
                    // TODO(b/212690698): Avoid (somewhat unergonomic) ManuallyDrop
                    // if we can ask Rust to preserve field destruction order if the
                    // destructor is the SpecialMemberFunc::NontrivialMembers
                    // case.
                    formatted = quote! { ::core::mem::ManuallyDrop<#formatted> }
                } else {
                    field_copy_trait_assertions.push(quote! {
                        static_assertions::assert_impl_all!(#formatted: Copy);
                    });
                }
            };
            formatted
        }
    };

    Ok(quote! { #padding #doc_comment #access #ident: #field_type })
}

/// Implementation of `BindingsGenerator::generate_record`.
pub fn generate_record(db: &dyn BindingsGenerator, record: Rc<Record>) -> Result<ApiSnippets> {
    let record_rs_type_kind = db.rs_type_kind(record.as_ref().into())?;
    if let RsTypeKind::Record { known_generic_monomorphization: Some(_), .. } = &record_rs_type_kind
    {
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
    let doc_comment = generate_doc_comment(
        record.doc_comment.as_deref(),
        Some(&record.source_loc),
        db.environment(),
    );
    let mut field_copy_trait_assertions: Vec<TokenStream> = vec![];

    struct FieldWithLayout<'a> {
        /// The IR field. Note that bitfields are represented as `None`.
        ir: Option<&'a ir::Field>,
        /// The offset of the field in the struct.
        offset: usize,
        /// The offset of the end of the field or `None` for opaque fields.
        end: Option<usize>,
        description: Vec<String>,
    }

    let fields_with_bounds: Vec<FieldWithLayout> = (record.fields.iter())
        .filter(|field| field.size != 0)
        .map(|field| {
            FieldWithLayout {
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
                description: vec![format!(
                    "{} : {} bits",
                    field.identifier.as_ref().map(|i| i.identifier.clone()).unwrap_or("".into()),
                    field.size
                )],
            }
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
                    record.cc_name.as_ref()
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
                &mut field_copy_trait_assertions,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    let field_offset_assertions: Vec<TokenStream> = fields_with_bounds
        .iter()
        .enumerate()
        .map(|(field_index, field_with_layout)| {
            let Some(field) = field_with_layout.ir else {
                return quote! {};
            };
            let field_ident = make_rs_field_ident(field, field_index);

            // The assertion below reinforces that the division by 8 on the next line is
            // justified (because the bitfields have been coallesced / filtered out
            // earlier).
            assert_eq!(field.offset % 8, 0);
            let expected_offset = Literal::usize_unsuffixed(field.offset / 8);

            let actual_offset_expr = quote! {
                ::core::mem::offset_of!(#qualified_ident, #field_ident)
            };
            quote! {
                assert!(#actual_offset_expr == #expected_offset);
            }
        })
        .collect_vec();
    let mut features = BTreeSet::new();

    let derives = generate_derives(&record);
    let derives = if derives.is_empty() {
        quote! {}
    } else {
        quote! {#[derive( #(#derives),* )]}
    };
    let record_kind = if record.is_union() {
        quote! { union }
    } else {
        quote! { struct }
    };

    let recursively_pinned_attribute = if record.is_unpin() {
        quote! {}
    } else {
        // negative_impls are necessary for universal initialization due to Rust's
        // coherence rules: PhantomPinned isn't enough to prove to Rust that a
        // blanket impl that requires Unpin doesn't apply. See http://<internal link>=h.f6jp8ifzgt3n
        features.insert(make_rs_ident("negative_impls"));
        if record.should_implement_drop() {
            quote! {#[::ctor::recursively_pinned(PinnedDrop)]}
        } else {
            quote! {#[::ctor::recursively_pinned]}
        }
    };

    let mut repr_attributes = vec![quote! {C}];
    if override_alignment && record.size_align.alignment > 1 {
        let alignment = Literal::usize_unsuffixed(record.size_align.alignment);
        repr_attributes.push(quote! {align(#alignment)});
    }

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
    let head_padding = if head_padding > 0 || !allow_direct_init {
        let n = proc_macro2::Literal::usize_unsuffixed(head_padding);
        quote! {
            __non_field_data: [::core::mem::MaybeUninit<u8>; #n],
        }
    } else {
        quote! {}
    };

    let fully_qualified_cc_name = cpp_tagless_type_name_for_record(&record, ir)?.to_string();

    let mut record_generated_items = record
        .child_item_ids
        .iter()
        .map(|id| {
            let item: &Item = ir.find_decl(*id).with_context(|| {
                format!("Failed to look up `record.child_item_ids` for {:?}", record)
            })?;
            db.generate_item(item.clone())
        })
        .collect::<Result<Vec<ApiSnippets>>>()?;

    let generated_inherited_functions: Vec<ApiSnippets> = filter_out_ambiguous_member_functions(
        db,
        record.clone(),
        collect_unqualified_member_functions_from_all_bases(db, &record),
    )
    .iter()
    .filter_map(|unambiguous_base_class_member_function| -> Option<ApiSnippets> {
        let item = ir.find_untyped_decl(unambiguous_base_class_member_function.id);
        let Item::Func(ir_func) = item else { panic!("Unexpected item type: {:?}", item) };
        let Ok(Some(generated_func)) = db.generate_function(ir_func.clone(), Some(record.clone()))
        else {
            return None;
        };
        Some((*generated_func.snippets).clone())
    })
    .collect();
    record_generated_items.extend(generated_inherited_functions);

    // Both the template definition and its instantiation should enable experimental
    // features.
    let mut crubit_features = ir.target_crubit_features(&record.owning_target);
    if let Some(defining_target) = &record.defining_target {
        crubit_features |= ir.target_crubit_features(defining_target);
    }
    if crubit_features.contains(crubit_feature::CrubitFeature::Experimental) {
        record_generated_items.push(cc_struct_upcast_impl(db, &record, ir)?);
    }
    let no_unique_address_accessors =
        if crubit_features.contains(crubit_feature::CrubitFeature::Experimental) {
            cc_struct_no_unique_address_impl(db, &record)?
        } else {
            quote! {}
        };
    let incomplete_definition = if crubit_features
        .contains(crubit_feature::CrubitFeature::Experimental)
    {
        quote! {
            forward_declare::unsafe_define!(forward_declare::symbol!(#fully_qualified_cc_name), #qualified_ident);
        }
    } else {
        quote! {}
    };

    let mut items = vec![];
    let mut thunks_from_record_items = vec![];
    let mut thunk_impls_from_record_items = vec![cc_struct_layout_assertion(db, &record)?];
    let mut assertions_from_record_items = vec![];

    for generated in record_generated_items {
        items.push(generated.main_api);
        if !generated.thunks.is_empty() {
            thunks_from_record_items.push(generated.thunks);
        }
        if !generated.assertions.is_empty() {
            assertions_from_record_items.push(generated.assertions);
        }
        if !generated.cc_details.is_empty() {
            thunk_impls_from_record_items.push(generated.cc_details);
        }
        features.extend(generated.features.clone());
    }

    let send_impl = if record.trait_derives.send {
        quote! {unsafe impl Send for #ident {}}
    } else {
        quote! {impl !Send for #ident {}}
    };

    let sync_impl = if record.trait_derives.sync {
        quote! {unsafe impl Sync for #ident {}}
    } else {
        quote! {impl !Sync for #ident {}}
    };

    let must_use = match &record.nodiscard {
        None => quote! {},
        Some(message) => {
            if message.is_empty() {
                quote! { #[must_use] }
            } else {
                quote! { #[must_use = #message] }
            }
        }
    };

    let crubit_annotation = format!("CRUBIT_ANNOTATE: cpp_type={fully_qualified_cc_name}");
    let record_tokens = quote! {
        #doc_comment
        #derives
        #recursively_pinned_attribute
        #must_use
        #[repr(#( #repr_attributes ),*)]
        #[doc=#crubit_annotation]
        pub #record_kind #ident {
            #head_padding
            #( #field_definitions, )*
        }

        #send_impl
        #sync_impl

        #incomplete_definition

        #no_unique_address_accessors

        __NEWLINE__ __NEWLINE__
        #( #items __NEWLINE__ __NEWLINE__)*
    };
    features.insert(make_rs_ident("negative_impls"));
    let record_trait_assertions = {
        let record_type_name = record_rs_type_kind.to_token_stream(db);
        let mut assertions: Vec<TokenStream> = vec![];
        let mut add_assertion = |assert_impl_macro: TokenStream, trait_name: TokenStream| {
            assertions.push(quote! {
                static_assertions::#assert_impl_macro (#record_type_name: #trait_name);
            });
        };
        if should_derive_clone(&record) {
            add_assertion(quote! { assert_impl_all! }, quote! { Clone });
        } else {
            // Can't `assert_not_impl_any!` here, because `Clone` may be
            // implemented rather than derived.
        }
        let mut add_conditional_assertion = |should_impl_trait: bool, trait_name: TokenStream| {
            let assert_impl_macro = if should_impl_trait {
                quote! { assert_impl_all! }
            } else {
                quote! { assert_not_impl_any! }
            };
            add_assertion(assert_impl_macro, trait_name);
        };
        add_conditional_assertion(should_derive_copy(&record), quote! { Copy });
        add_conditional_assertion(record.should_implement_drop(), quote! { Drop });
        assertions
    };
    let size_align_assertions = rs_size_align_assertions(qualified_ident, &record.size_align);
    let assertion_tokens = quote! {
        #size_align_assertions
        #( #record_trait_assertions )*
        #( #field_offset_assertions )*
        #( #field_copy_trait_assertions )*
        #( #assertions_from_record_items )*
    };

    let thunk_tokens = quote! {
        #( #thunks_from_record_items )*
    };

    Ok(ApiSnippets {
        main_api: record_tokens,
        features,
        assertions: assertion_tokens,
        thunks: thunk_tokens,
        cc_details: quote! {#(#thunk_impls_from_record_items __NEWLINE__ __NEWLINE__)*},
        ..Default::default()
    })
}

pub fn rs_size_align_assertions(type_name: TokenStream, size_align: &ir::SizeAlign) -> TokenStream {
    let size = Literal::usize_unsuffixed(size_align.size);
    let alignment = Literal::usize_unsuffixed(size_align.alignment);
    quote! {
        assert!(::core::mem::size_of::<#type_name>() == #size);
        assert!(::core::mem::align_of::<#type_name>() == #alignment);
    }
}

pub fn generate_derives(record: &Record) -> Vec<Ident> {
    let mut derives = vec![];
    if should_derive_clone(record) {
        derives.push(make_rs_ident("Clone"));
    }
    if should_derive_copy(record) {
        derives.push(make_rs_ident("Copy"));
    }
    if record.trait_derives.debug == TraitImplPolarity::Positive {
        derives.push(make_rs_ident("Debug"));
    }
    for custom_trait in &record.trait_derives.custom {
        // Breaks for paths right now...
        derives.push(make_rs_ident(custom_trait));
    }
    derives
}

fn cc_struct_layout_assertion(db: &dyn BindingsGenerator, record: &Record) -> Result<TokenStream> {
    let record_ident = expect_format_cc_type_name(record.cc_name.identifier.as_ref());
    let namespace_qualifier = db.ir().namespace_qualifier(record).format_for_cc()?;
    let tag_kind = record.cc_tag_kind();
    let field_assertions = record
        .fields
        .iter()
        .filter(|f| f.access == AccessSpecifier::Public && f.identifier.is_some())
        // https://en.cppreference.com/w/cpp/types/offsetof points out that "if member is [...]
        // a bit-field [...] the behavior [of `offsetof` macro] is undefined.".  In such
        // scenario clang reports an error: cannot compute offset of bit-field 'field_name'.
        .filter(|f| !f.is_bitfield)
        .map(|field| {
            // The IR contains the offset in bits, while `CRUBIT_OFFSET_OF` returns the
            // offset in bytes, so we need to convert.  We can assert that
            // `field.offset` is always at field boundaries, because the
            // bitfields have been filtered out earlier.
            assert_eq!(field.offset % 8, 0);
            let expected_offset = Literal::usize_unsuffixed(field.offset / 8);

            let field_ident =
                expect_format_cc_type_name(&field.identifier.as_ref().unwrap().identifier);
            let actual_offset = quote! {
                CRUBIT_OFFSET_OF(#field_ident, #tag_kind #namespace_qualifier #record_ident)
            };

            quote! { static_assert( #actual_offset == #expected_offset); }
        });
    // only use CRUBIT_SIZEOF for alignment > 1, so as to simplify the generated
    // code.
    let size = Literal::usize_unsuffixed(record.size_align.size);
    let alignment = Literal::usize_unsuffixed(record.size_align.alignment);
    let sizeof = if record.size_align.alignment == 1 {
        quote! {sizeof}
    } else {
        quote! {CRUBIT_SIZEOF}
    };
    Ok(quote! {
        static_assert(#sizeof(#tag_kind #namespace_qualifier #record_ident) == #size);
        static_assert(alignof(#tag_kind #namespace_qualifier #record_ident) == #alignment);
        #( #field_assertions )*
    })
}

/// Returns the accessor functions for no_unique_address member variables.
fn cc_struct_no_unique_address_impl(
    db: &dyn BindingsGenerator,
    record: &Record,
) -> Result<TokenStream> {
    let mut fields = vec![];
    let mut types = vec![];
    let mut field_offsets = vec![];
    let mut doc_comments = vec![];
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
        if let Ok(cpp_type) = field.type_.as_ref() {
            fields.push(make_rs_ident(
                &field
                    .identifier
                    .as_ref()
                    .expect("Unnamed fields can't be annotated with [[no_unique_address]]")
                    .identifier,
            ));
            let type_ident = db.rs_type_kind(cpp_type.clone()).with_context(|| {
                format!("Failed to format type for field {:?} on record {:?}", field, record)
            })?;
            types.push(type_ident.to_token_stream(db));
            field_offsets.push(Literal::usize_unsuffixed(field.offset / 8));
            if field.size == 0 {
                // These fields are not generated at all, so they need to be documented here.
                doc_comments.push(generate_doc_comment(
                    field.doc_comment.as_deref(),
                    None,
                    db.environment(),
                ));
            } else {
                // all other fields already have a doc-comment at the point they were defined.
                doc_comments.push(quote! {});
            }
        }
    }
    if fields.is_empty() {
        return Ok(quote! {});
    }
    let ident = make_rs_ident(record.rs_name.identifier.as_ref());
    // SAFETY: even if there is a named field in Rust for this subobject, it is not
    // safe to just cast the pointer. A `struct S {[[no_unique_address]] A a;
    // char b};` will be represented in Rust using a too-short field `a` (e.g.
    // with `[MaybeUninit<u8>; 3]`, where the trailing fourth byte is actually
    // `b`). We cannot cast this to something wider, which includes `b`, even
    // though the `a` object does in fact include `b` in C++. This is Rust, and
    // these are distinct object allocations. We don't have provenance.
    //
    // However, we can start from the pointer to **S** and perform pointer
    // arithmetic on it to get a correctly-sized `A` reference. This is
    // equivalent to transmuting the type to one where the potentially-overlapping
    // subobject exists, but the fields next to it, which it overlaps, do not.
    // As if it were `struct S {A a;};`. However, we do not use transmutes, and
    // instead reimplement field access using pointer arithmetic.
    //
    // The resulting pointer is valid and correctly aligned, and does not violate
    // provenance. It also does not result in mutable aliasing, because this
    // borrows `self`, not just `a`.
    Ok(quote! {
        impl #ident {
            #(
                #doc_comments
                pub fn #fields(&self) -> &#types {
                    unsafe {
                        let ptr = (self as *const Self as *const u8).offset(#field_offsets);
                        &*(ptr as *const #types)
                    }
                }
            )*
        }
    })
}

/// Returns the implementation of base class conversions, for converting a type
/// to its unambiguous public base classes.
fn cc_struct_upcast_impl(
    db: &dyn BindingsGenerator,
    record: &Rc<Record>,
    ir: &IR,
) -> Result<ApiSnippets> {
    let mut impls = Vec::with_capacity(record.unambiguous_public_bases.len());
    let mut thunks = vec![];
    let mut cc_impls = vec![];
    for base in &record.unambiguous_public_bases {
        let base_record: &Rc<Record> = ir
            .find_decl(base.base_record_id)
            .with_context(|| format!("Can't find a base record of {:?}", record))?;
        let Ok(base_type) = db.rs_type_kind(base_record.as_ref().into()) else {
            // The base type is unknown to Crubit, so don't generate upcast code for it.
            let base_name = &base_record.cc_name;
            let derived_name = &record.cc_name;
            let comment = format!("'{derived_name}' cannot be upcasted to '{base_name}' because the base type doesn't have Crubit bindings.");
            impls.push(quote! { __NEWLINE__ __COMMENT__ #comment __NEWLINE__ });
            continue;
        };
        if base_type.is_bridge_type() {
            // The base class isn't directly represented in Rust, so we can't upcast to it.
            continue;
        }
        let base_name = base_type.to_token_stream(db);
        let derived_name = db.rs_type_kind(record.as_ref().into())?.to_token_stream(db);
        let body = if let Some(offset) = base.offset {
            let offset = Literal::i64_unsuffixed(offset);
            quote! { (derived as *const _ as *const u8).offset(#offset) as *const #base_name }
        } else {
            let cast_fn_name = make_rs_ident(&format!(
                "__crubit_dynamic_upcast__{derived}__to__{base}_{odr_suffix}",
                derived = record.mangled_cc_name,
                base = base_record.mangled_cc_name,
                odr_suffix = record.owning_target.convert_to_cc_identifier(),
            ));
            let base_cc_name = cpp_type_name_for_record(base_record.as_ref(), ir)?;
            let derived_cc_name = cpp_type_name_for_record(record.as_ref(), ir)?;
            cc_impls.push(quote! {
                extern "C" const #base_cc_name& #cast_fn_name(const #derived_cc_name& from) {
                    return from;
                }
            });
            thunks.push(quote! {
                pub fn #cast_fn_name (from: *const #derived_name) -> *const #base_name;
            });
            let crate_root_path = ir.crate_root_path_tokens();
            quote! {
                #crate_root_path::detail::#cast_fn_name(derived)
            }
        };
        impls.push(quote! {
            unsafe impl oops::Inherits<#base_name> for #derived_name {
                unsafe fn upcast_ptr(derived: *const Self) -> *const #base_name {
                    #body
                }
            }
        });
    }

    Ok(ApiSnippets {
        main_api: quote! {#(#impls)*},
        thunks: quote! {#(#thunks)*},
        cc_details: quote! {#(#cc_impls)*},
        ..Default::default()
    })
}
