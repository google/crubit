// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

use crate::{BindingsGenerator, Database, GeneratedItem};

use crate::rs_snippet::{should_derive_clone, should_derive_copy, RsTypeKind};
use arc_anyhow::{Context, Result};
use code_gen_utils::make_rs_ident;
use error_report::{bail, ensure};
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{quote, ToTokens};
use std::collections::{BTreeSet, HashMap};
use std::iter;
use std::rc::Rc;

// TODO(jeanpierreda): Make this a method on RsTypeKind, or on Record?
pub(crate) fn should_implement_drop(record: &Record) -> bool {
    match record.destructor {
        // TODO(b/202258760): Only omit destructor if `Copy` is specified.
        SpecialMemberFunc::Trivial => false,

        // TODO(b/212690698): Avoid calling into the C++ destructor (e.g. let
        // Rust drive `drop`-ing) to avoid (somewhat unergonomic) ManuallyDrop
        // if we can ask Rust to preserve C++ field destruction order in
        // NontrivialMembers case.
        SpecialMemberFunc::NontrivialMembers => true,

        // The `impl Drop` for NontrivialUserDefined needs to call into the
        // user-defined destructor on C++ side.
        SpecialMemberFunc::NontrivialUserDefined => true,

        // TODO(b/213516512): Today the IR doesn't contain Func entries for
        // deleted functions/destructors/etc. But, maybe we should generate
        // `impl Drop` in this case? With `unreachable!`? With
        // `std::mem::forget`?
        SpecialMemberFunc::Unavailable => false,
    }
}

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
    db: &Database,
    incomplete_record: &IncompleteRecord,
) -> Result<GeneratedItem> {
    let ident = make_rs_ident(incomplete_record.rs_name.as_ref());
    let namespace_qualifier = db.ir().namespace_qualifier(incomplete_record)?.format_for_cc()?;
    let symbol = quote! {#namespace_qualifier #ident}.to_string();
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
    db: &Database,
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
        Ok(t) => db.rs_type_kind(t.rs_type.clone())?,
        Err(e) => bail!("{e}"),
    };

    for target in record.defining_target.iter().chain([&record.owning_target]) {
        let enabled_features = db.ir().target_crubit_features(target);
        let (missing_features, reason) = type_kind.required_crubit_features(enabled_features);
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
    if should_implement_drop(record) && !record.is_union() && needs_manually_drop(&type_kind) {
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
                if let UnqualifiedIdentifier::Identifier(_) = &member_function.name {
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
        .map(|func| (func.name.clone(), func.clone()))
        .collect::<HashMap<_, _>>();
    let mut func_counter = HashMap::<_, (&Rc<Func>, u32)>::new();
    for func in inherited_functions.iter() {
        let Ok(Some(_)) = db.generate_func(func.clone(), None) else {
            continue;
        };
        let unqualified_name = &func.name;
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
        .sorted_by_key(|func| func.name.identifier_as_str().unwrap().to_string())
        .collect()
}

/// Generates Rust source code for a given `Record` and associated assertions as
/// a tuple.
pub fn generate_record(db: &Database, record: &Rc<Record>) -> Result<GeneratedItem> {
    // If the record has a bridge type, we don't need to generate any bindings.
    if record.bridge_type_info.is_some() {
        return Ok(GeneratedItem::default());
    }
    let ir = db.ir();
    let crate_root_path = crate::crate_root_path_tokens(&ir);
    let ident = make_rs_ident(record.rs_name.as_ref());
    let namespace_qualifier = ir.namespace_qualifier(record)?.format_for_rs();
    let qualified_ident = {
        quote! { #crate_root_path:: #namespace_qualifier #ident }
    };
    let doc_comment = crate::generate_doc_comment(
        record.doc_comment.as_deref(),
        Some(&record.source_loc),
        db.generate_source_loc_doc_comment(),
    );
    let mut field_copy_trait_assertions: Vec<TokenStream> = vec![];

    let fields_with_bounds = (record.fields.iter())
        .filter(|field| field.size != 0)
        .map(|field| {
            (
                // We don't represent bitfields directly in Rust. We drop the field itself here
                // and only retain the offset information. Adjacent bitfields then get merged in
                // the next step.
                if field.is_bitfield { None } else { Some(field) },
                field.offset,
                // We retain the end offset of fields only if we have a matching Rust type
                // to represent them. Otherwise we'll fill up all the space to the next field.
                // See: docs/struct_layout
                match get_field_rs_type_kind_for_layout(db, record, field) {
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
                vec![format!(
                    "{} : {} bits",
                    field.identifier.as_ref().map(|i| i.identifier.clone()).unwrap_or("".into()),
                    field.size
                )],
            )
        })
        // Merge consecutive bitfields. This is necessary, because they may share storage in the
        // same byte.
        .coalesce(|first, second| match (first, second) {
            ((None, offset, _, desc1), (None, _, end, desc2)) => {
                Ok((None, offset, end, [desc1, desc2].concat()))
            }
            pair => Err(pair),
        });

    let mut override_alignment = record.override_alignment;

    // Pair up fields with the preceeding and following fields (if any):
    // - the end offset of the previous field determines if we need to insert
    //   padding.
    // - the start offset of the next field may be need to grow the current field to
    //   there.
    // This uses two separate `map` invocations on purpose to limit available state.
    let field_definitions = iter::once(None)
        .chain(fields_with_bounds.clone().map(Some))
        .chain(iter::once(None))
        .tuple_windows()
        .map(|(prev, cur, next)| {
            let (field, offset, end, desc) = cur.unwrap();
            let prev_end = prev.as_ref().and_then(|(_, _, e, _)| *e).unwrap_or(offset);
            let next_offset = next.map(|(_, o, _, _)| o);
            let end = end.or(next_offset).unwrap_or(record.size_align.size * 8);

            if let Some((Some(prev_field), _, Some(prev_end), _)) = prev {
                assert!(
                    record.is_union() || prev_end <= offset,
                    "Unexpected offset+size for field {:?} in record {}",
                    prev_field,
                    record.cc_name.as_ref()
                );
            }

            (field, prev_end, offset, end, desc)
        })
        .enumerate()
        .map(|(field_index, (field, prev_end, offset, end, desc))| {
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
                override_alignment = true;
                return Ok(quote! {
                    __NEWLINE__ #(  __COMMENT__ #desc )*
                    #padding #name: #bitfield_padding
                });
            }
            let field = field.unwrap();

            let ident = make_rs_field_ident(field, field_index);
            let field_rs_type_kind = get_field_rs_type_kind_for_layout(db, record, field);
            let doc_comment = match &field_rs_type_kind {
                Ok(_) => crate::generate_doc_comment(
                    field.doc_comment.as_deref(),
                    None,
                    db.generate_source_loc_doc_comment(),
                ),
                Err(msg) => {
                    override_alignment = true;
                    let supplemental_text = format!(
                        "Reason for representing this field as a blob of bytes:\n{:#}",
                        msg
                    );
                    let new_text = match &field.doc_comment {
                        None => supplemental_text,
                        Some(old_text) => format!("{}\n\n{}", old_text.as_ref(), supplemental_text),
                    };
                    crate::generate_doc_comment(
                        Some(new_text.as_str()),
                        None,
                        db.generate_source_loc_doc_comment(),
                    )
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
                    let mut formatted = quote! {#type_kind};
                    if should_implement_drop(record) || record.is_union() {
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
        })
        .collect::<Result<Vec<_>>>()?;

    let field_offset_assertions = fields_with_bounds
        .enumerate()
        .map(|(field_index, (field, _, _, _))| {
            if let Some(field) = field {
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
            } else {
                quote! {}
            }
        })
        .collect_vec();
    let mut features = BTreeSet::new();

    let derives = generate_derives(record);
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
        if should_implement_drop(record) {
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

    let fully_qualified_cc_name = crate::cc_tagless_type_name_for_record(record, &ir)?.to_string();

    let mut record_generated_items = record
        .child_item_ids
        .iter()
        .map(|id| {
            let item: &Item = ir.find_decl(*id).with_context(|| {
                format!("Failed to look up `record.child_item_ids` for {:?}", record)
            })?;
            crate::generate_item(db, item)
        })
        .collect::<Result<Vec<_>>>()?;

    let generated_inherited_functions = filter_out_ambiguous_member_functions(
        db,
        record.clone(),
        collect_unqualified_member_functions_from_all_bases(db, record),
    )
    .iter()
    .map(|unambiguous_base_class_member_function| {
        let item: Result<&Item> = ir.find_decl(unambiguous_base_class_member_function.id);
        if item.is_err() {
            return Ok(GeneratedItem::default());
        }

        match item.clone().unwrap() {
            Item::Func(func) => match db.generate_func(func.clone(), Some(record.clone()))? {
                None => Ok(GeneratedItem::default()),
                Some((item, _)) => Ok((*item).clone()),
            },
            _ => panic!("Unexpected item type: {:?}", item),
        }
    })
    .collect::<Result<Vec<_>>>()?;
    record_generated_items.extend(generated_inherited_functions);

    // Both the template definition and its instantiation should enable experimental
    // features.
    let mut crubit_features = ir.target_crubit_features(&record.owning_target);
    if let Some(defining_target) = &record.defining_target {
        crubit_features |= ir.target_crubit_features(defining_target);
    }
    if crubit_features.contains(crubit_feature::CrubitFeature::Experimental) {
        record_generated_items.push(cc_struct_upcast_impl(record, &ir)?);
    }
    let no_unique_address_accessors =
        if crubit_features.contains(crubit_feature::CrubitFeature::Experimental) {
            cc_struct_no_unique_address_impl(db, record)?
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
    let mut thunk_impls_from_record_items = vec![cc_struct_layout_assertion(db, record)?];
    let mut assertions_from_record_items = vec![];

    for generated in record_generated_items {
        items.push(generated.item);
        if !generated.thunks.is_empty() {
            thunks_from_record_items.push(generated.thunks);
        }
        if !generated.assertions.is_empty() {
            assertions_from_record_items.push(generated.assertions);
        }
        if !generated.thunk_impls.is_empty() {
            thunk_impls_from_record_items.push(generated.thunk_impls);
        }
        features.extend(generated.features.clone());
    }

    let record_tokens = quote! {
        #doc_comment
        #derives
        #recursively_pinned_attribute
        #[repr(#( #repr_attributes ),*)]
        #[__crubit::annotate(cpp_type=#fully_qualified_cc_name)]
        pub #record_kind #ident {
            #head_padding
            #( #field_definitions, )*
        }

        impl !Send for #ident {}
        impl !Sync for #ident {}

        #incomplete_definition

        #no_unique_address_accessors

        __NEWLINE__ __NEWLINE__
        #( #items __NEWLINE__ __NEWLINE__)*
    };
    features.insert(make_rs_ident("negative_impls"));
    // For #![register_tool(__crubit)] / #![__crubit::...]
    features.insert(make_rs_ident("register_tool"));

    let record_trait_assertions = {
        let record_type_name = RsTypeKind::new_record(record.clone(), &ir)?.to_token_stream();
        let mut assertions: Vec<TokenStream> = vec![];
        let mut add_assertion = |assert_impl_macro: TokenStream, trait_name: TokenStream| {
            assertions.push(quote! {
                static_assertions::#assert_impl_macro (#record_type_name: #trait_name);
            });
        };
        if should_derive_clone(record) {
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
        add_conditional_assertion(should_derive_copy(record), quote! { Copy });
        add_conditional_assertion(should_implement_drop(record), quote! { Drop });
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

    Ok(GeneratedItem {
        item: record_tokens,
        features,
        assertions: assertion_tokens,
        thunks: thunk_tokens,
        thunk_impls: quote! {#(#thunk_impls_from_record_items __NEWLINE__ __NEWLINE__)*},
        ..Default::default()
    })
}

pub fn rs_size_align_assertions(
    type_name: impl ToTokens,
    size_align: &ir::SizeAlign,
) -> TokenStream {
    let type_name = type_name.into_token_stream();
    let size = Literal::usize_unsuffixed(size_align.size);
    let alignment = Literal::usize_unsuffixed(size_align.alignment);
    quote! {
        assert!(::core::mem::size_of::<#type_name>() == #size);
        assert!(::core::mem::align_of::<#type_name>() == #alignment);
    }
}

fn generate_derives(record: &Record) -> Vec<Ident> {
    let mut derives = vec![];
    if should_derive_clone(record) {
        derives.push(make_rs_ident("Clone"));
    }
    if should_derive_copy(record) {
        derives.push(make_rs_ident("Copy"));
    }
    derives
}

fn cc_struct_layout_assertion(db: &Database, record: &Record) -> Result<TokenStream> {
    let record_ident = crate::format_cc_ident(record.cc_name.as_ref());
    let namespace_qualifier = db.ir().namespace_qualifier(record)?.format_for_cc()?;
    let tag_kind = crate::cc_tag_kind(record);
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
                crate::format_cc_ident(&field.identifier.as_ref().unwrap().identifier);
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
fn cc_struct_no_unique_address_impl(db: &Database, record: &Record) -> Result<TokenStream> {
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
        if let Ok(rs_type) = field.type_.as_ref().map(|t| t.rs_type.clone()) {
            fields.push(make_rs_ident(
                &field
                    .identifier
                    .as_ref()
                    .expect("Unnamed fields can't be annotated with [[no_unique_address]]")
                    .identifier,
            ));
            let type_ident = db.rs_type_kind(rs_type).with_context(|| {
                format!("Failed to format type for field {:?} on record {:?}", field, record)
            })?;
            types.push(type_ident);
            field_offsets.push(Literal::usize_unsuffixed(field.offset / 8));
            if field.size == 0 {
                // These fields are not generated at all, so they need to be documented here.
                doc_comments.push(crate::generate_doc_comment(
                    field.doc_comment.as_deref(),
                    None,
                    db.generate_source_loc_doc_comment(),
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
    let ident = make_rs_ident(record.rs_name.as_ref());
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
fn cc_struct_upcast_impl(record: &Rc<Record>, ir: &IR) -> Result<GeneratedItem> {
    let mut impls = Vec::with_capacity(record.unambiguous_public_bases.len());
    let mut thunks = vec![];
    let mut cc_impls = vec![];
    for base in &record.unambiguous_public_bases {
        let base_record: &Rc<Record> = ir
            .find_decl(base.base_record_id)
            .with_context(|| format!("Can't find a base record of {:?}", record))?;
        let base_name = RsTypeKind::new_record(base_record.clone(), ir)?.into_token_stream();
        let derived_name = RsTypeKind::new_record(record.clone(), ir)?.into_token_stream();
        let body;
        if let Some(offset) = base.offset {
            let offset = Literal::i64_unsuffixed(offset);
            body = quote! {(derived as *const _ as *const u8).offset(#offset) as *const #base_name};
        } else {
            let cast_fn_name = make_rs_ident(&format!(
                "__crubit_dynamic_upcast__{derived}__to__{base}_{odr_suffix}",
                derived = record.mangled_cc_name,
                base = base_record.mangled_cc_name,
                odr_suffix = record.owning_target.convert_to_cc_identifier(),
            ));
            let base_cc_name = crate::cpp_type_name_for_record(base_record.as_ref(), ir)?;
            let derived_cc_name = crate::cpp_type_name_for_record(record.as_ref(), ir)?;
            cc_impls.push(quote! {
                extern "C" const #base_cc_name& #cast_fn_name(const #derived_cc_name& from) {
                    return from;
                }
            });
            thunks.push(quote! {
                pub fn #cast_fn_name (from: *const #derived_name) -> *const #base_name;
            });
            let crate_root_path = crate::crate_root_path_tokens(ir);
            body = quote! {
                #crate_root_path::detail::#cast_fn_name(derived)
            };
        }
        impls.push(quote! {
            unsafe impl oops::Inherits<#base_name> for #derived_name {
                unsafe fn upcast_ptr(derived: *const Self) -> *const #base_name {
                    #body
                }
            }
        });
    }

    Ok(GeneratedItem {
        item: quote! {#(#impls)*},
        thunks: quote! {#(#thunks)*},
        thunk_impls: quote! {#(#cc_impls)*},
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use crate::BindingsTokens;
    use arc_anyhow::Result;
    use googletest::prelude::*;
    use ir_testing::with_lifetime_macros;
    use token_stream_matchers::{assert_cc_matches, assert_rs_matches, assert_rs_not_matches};

    #[gtest]
    fn test_template_in_dependency_and_alias_in_current_target() -> Result<()> {
        // See also the test with the same name in `ir_from_cc_test.rs`.
        let ir = {
            let dependency_src = r#" #pragma clang lifetime_elision
                    template <typename T>
                    struct MyTemplate {
                        ~MyTemplate();
                        T GetValue() { return field; }
                        T field;
                    }; "#;
            let current_target_src = r#" #pragma clang lifetime_elision
                    using MyAliasOfTemplate = MyTemplate<int>; "#;
            ir_from_cc_dependency(current_target_src, dependency_src)?
        };

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                #[__crubit::annotate(cpp_type="MyTemplate < int >")]
                pub struct __CcTemplateInst10MyTemplateIiE {
                    pub field: ::core::ffi::c_int,
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                impl __CcTemplateInst10MyTemplateIiE {
                    #[doc = " Generated from: google3/test/dependency_header.h;l=5"]
                    #[inline(always)]
                    pub fn GetValue<'a>(self: ... Pin<&'a mut Self>) -> ::core::ffi::c_int { unsafe {
                        crate::detail::__rust_thunk___ZN10MyTemplateIiE8GetValueEv__2f_2ftest_3atesting_5ftarget(
                            self)
                    }}
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                pub type MyAliasOfTemplate = crate::__CcTemplateInst10MyTemplateIiE;
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail { ...  extern "C" {
                    ...
                    pub(crate) fn
                    __rust_thunk___ZN10MyTemplateIiE8GetValueEv__2f_2ftest_3atesting_5ftarget<'a>(
                        __this: ... Pin<&'a mut crate::__CcTemplateInst10MyTemplateIiE>
                    ) -> ::core::ffi::c_int;
                    ...
                } }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C"
                int __rust_thunk___ZN10MyTemplateIiE8GetValueEv__2f_2ftest_3atesting_5ftarget(
                        struct MyTemplate<int>* __this) {
                    return __this->GetValue();
                }
            }
        );

        Ok(())
    }

    #[gtest]
    fn test_template_with_out_of_line_definition() -> Result<()> {
        // See also an end-to-end test in the `test/templates/out_of_line_definition`
        // directory.
        let ir = ir_from_cc(
            r#" #pragma clang lifetime_elision
                template <typename T>
                class MyTemplate final {
                 public:
                  static MyTemplate Create(T value);
                  const T& value() const;

                 private:
                  T value_;
                };

                using MyTypeAlias = MyTemplate<int>; "#,
        )?;

        let BindingsTokens { rs_api_impl, .. } = generate_bindings_tokens(ir)?;

        // Even though the member functions above are *not* defined inline (e.g.
        // IR::Func::is_inline is false), they still need to have thunks generated for
        // them (to force/guarantee that the class template and its members get
        // instantiated).  This is also covered in the following end-to-end
        // tests:
        // - test/templates/out_of_line_definition/ - without a thunk, the template
        //   won't be instantiated and Rust bindings won't be able to call the member
        //   function (there will be no instantiation of the member function in the C++
        //   object files)
        // - test/templates/definition_in_cc/ - the instantiation happens in the .cc
        //   file and therefore the thunk is not *required* (but it doesn't hurt to have
        //   the thunk)
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void
                __rust_thunk___ZN10MyTemplateIiE6CreateEi__2f_2ftest_3atesting_5ftarget(
                    class MyTemplate<int>* __return, int value) {
                  new (__return) auto(MyTemplate<int>::Create(value));
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" int const*
                __rust_thunk___ZNK10MyTemplateIiE5valueEv__2f_2ftest_3atesting_5ftarget(
                        const class MyTemplate<int>*__this) {
                    return &__this->value();
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_simple_struct() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct final {
                ~SomeStruct() {}
                int public_int;
              protected:
                int protected_int;
              private:
               int private_int;
            };
        "#,
        )?;

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[::ctor::recursively_pinned(PinnedDrop)]
                #[repr(C, align(4))]
                #[__crubit::annotate(cpp_type="SomeStruct")]
                pub struct SomeStruct {
                    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
                    pub public_int: ::core::ffi::c_int,
                    #[doc = " Reason for representing this field as a blob of bytes:\n Types of non-public C++ fields can be elided away"]
                    pub(crate) protected_int: [::core::mem::MaybeUninit<u8>; 4],
                    #[doc = " Reason for representing this field as a blob of bytes:\n Types of non-public C++ fields can be elided away"]
                    pub(crate) private_int: [::core::mem::MaybeUninit<u8>; 4],
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                const _ : () = {
                    ...
                    assert!(::core::mem::size_of::<crate::SomeStruct>() == 12);
                    assert!(::core::mem::align_of::<crate::SomeStruct>() == 4);
                    static_assertions::assert_not_impl_any!(crate::SomeStruct: Copy);
                    static_assertions::assert_impl_all!(crate::SomeStruct: Drop);
                    assert!(::core::mem::offset_of!(crate::SomeStruct, public_int) == 0);
                    assert!(::core::mem::offset_of!(crate::SomeStruct, protected_int) == 4);
                    assert!(::core::mem::offset_of!(crate::SomeStruct, private_int) == 8);
                    ...
                };
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN10SomeStructD1Ev(struct SomeStruct * __this) {
                    std::destroy_at(__this);
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                static_assert(CRUBIT_SIZEOF(struct SomeStruct) == 12);
                static_assert(alignof(struct SomeStruct) == 4);
                static_assert(CRUBIT_OFFSET_OF(public_int, struct SomeStruct) == 0);
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_struct_vs_class() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct final {
                SomeStruct() {}
                int field;
            };
            class SomeClass final {
              public:
                SomeClass() {}
                int field;
            };
        "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;

        // A Rust `struct` is generated for both `SomeStruct` and `SomeClass`.
        assert_rs_matches!(rs_api, quote! { pub struct SomeStruct },);
        assert_rs_matches!(rs_api, quote! { pub struct SomeClass },);

        // But in C++ we still should refer to `struct SomeStruct` and `class
        // SomeClass`. See also b/238212337.
        assert_cc_matches!(rs_api_impl, quote! { struct SomeStruct * __this });
        assert_cc_matches!(rs_api_impl, quote! { class SomeClass * __this });
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(CRUBIT_SIZEOF(struct SomeStruct) == 4); }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(CRUBIT_SIZEOF(class SomeClass) == 4); }
        );
        Ok(())
    }

    #[gtest]
    fn test_struct_vs_typedefed_struct() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct final {
              int x;
            } __attribute__((aligned(16)));
            typedef struct {
              int x;
            } SomeAnonStruct __attribute__((aligned(16)));
        "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;

        // A `struct` is generated for both `SomeStruct` and `SomeAnonStruct`, both
        // in Rust and in C++.
        assert_rs_matches!(rs_api, quote! { pub struct SomeStruct },);
        assert_rs_matches!(rs_api, quote! { pub struct SomeAnonStruct },);
        assert_rs_matches!(rs_api_impl, quote! { struct SomeStruct * __this },);
        assert_rs_matches!(rs_api_impl, quote! { SomeAnonStruct * __this },);

        // In C++, both have align == 16, but size for `SomeAnonStruct` is not aligned.
        // `SomeAnonStruct` won't have `struct` in the assert.
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(alignof(struct SomeStruct) == 16); }
        );
        assert_cc_matches!(rs_api_impl, quote! { static_assert(alignof(SomeAnonStruct) == 16); });
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(CRUBIT_SIZEOF(struct SomeStruct) == 16); }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(CRUBIT_SIZEOF(SomeAnonStruct) == 16); }
        );

        // In Rust, both have align == 16 and size == 16.
        assert_rs_matches!(
            rs_api,
            quote! { assert!(::core::mem::size_of::<crate::SomeStruct>() == 16); }
        );
        assert_rs_matches!(
            rs_api,
            quote! { assert!(::core::mem::align_of::<crate::SomeStruct>() == 16); }
        );
        assert_rs_matches!(
            rs_api,
            quote! { assert!(::core::mem::size_of::<crate::SomeAnonStruct>() == 16); }
        );
        assert_rs_matches!(
            rs_api,
            quote! { assert!(::core::mem::align_of::<crate::SomeAnonStruct>() == 16); }
        );

        Ok(())
    }

    #[gtest]
    fn test_record_with_unsupported_field_type() -> Result<()> {
        // Using a nested struct because it's currently not supported.
        // But... any other unsupported type would also work for this test.
        let ir = ir_from_cc(
            r#"
            struct StructWithUnsupportedField {
              struct NestedStruct {
                int nested_field;
              };

              // Doc comment for `my_field`.
              NestedStruct my_field;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(4))]
                #[__crubit::annotate(cpp_type="StructWithUnsupportedField")]
                pub struct StructWithUnsupportedField {
                    #[doc = " Doc comment for `my_field`.\n \n Reason for representing this field as a blob of bytes:\n Unsupported type 'struct StructWithUnsupportedField::NestedStruct': No generated bindings found for 'NestedStruct'"]
                    pub(crate) my_field: [::core::mem::MaybeUninit<u8>; 4],
                }
                ...
                const _: () = {
                    ...
                    assert!(
                    ::core::mem::offset_of!(crate::StructWithUnsupportedField, my_field) == 0);
                    ...
                };
            }
        );
        Ok(())
    }

    /// This is a regression test for b/283835873 where the alignment of the
    /// generated struct was wrong/missing.
    #[gtest]
    fn test_struct_with_only_bitfields() -> Result<()> {
        let ir = ir_from_cc(
            r#"
                struct SomeStruct {
                  char32_t code_point : 31;
                  enum : char32_t {
                    ok = 0,
                    error = 1
                  } status : 1;
                };
                static_assert(sizeof(SomeStruct) == 4);
                static_assert(alignof(SomeStruct) == 4);
            "#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
               #[repr(C, align(4))]
                #[__crubit::annotate(cpp_type="SomeStruct")]
               pub struct SomeStruct { ...  }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = {
                    ...
                    assert!(::core::mem::size_of::<crate::SomeStruct>() == 4);
                    assert!(::core::mem::align_of::<crate::SomeStruct>() == 4);
                    ...
                };
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_struct_with_unnamed_bitfield_member() -> Result<()> {
        // This test input causes `field_decl->getName()` to return an empty string.
        // This example is based on `struct timex` from
        // /usr/grte/v5/include/bits/timex.h
        let ir = ir_from_cc(
            r#"
            struct SomeStruct {
                int first_field;
                int :32;
                int last_field;
            }; "#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(4))]
                #[__crubit::annotate(cpp_type="SomeStruct")]
                pub struct SomeStruct {
                    pub first_field: ::core::ffi::c_int, ...
                    __bitfields1: [::core::mem::MaybeUninit<u8>; 4],
                    pub last_field: ::core::ffi::c_int,
                }
                ...
                const _: () = {
                    ...
                    assert!(::core::mem::offset_of!(crate::SomeStruct, first_field) == 0);
                    assert!(::core::mem::offset_of!(crate::SomeStruct, last_field) == 8);
                    ...
                };
            }
        );
        Ok(())
    }

    /// Classes with a non-public destructor shouldn't be constructible, not
    /// even via Copy/Clone.
    #[gtest]
    fn test_trivial_nonpublic_destructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct Indestructible final {
              Indestructible() = default;
              Indestructible(int);
              Indestructible(const Indestructible&) = default;
              void Foo() const;
             private:
              ~Indestructible() = default;
            };

            Indestructible ReturnsValue();
            void TakesValue(Indestructible);
            void TakesReference(const Indestructible& x);
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        // It isn't available by value:
        assert_rs_not_matches!(rs_api, quote! {Default});
        assert_rs_not_matches!(rs_api, quote! {From});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Copy ... )});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Clone ... )});
        assert_rs_not_matches!(rs_api, quote! {ReturnsValue});
        assert_rs_not_matches!(rs_api, quote! {TakesValue});
        // ... but it is otherwise available:
        assert_rs_matches!(rs_api, quote! {struct Indestructible});
        assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn TakesReference<'a>(x: &'a crate::Indestructible)});
        Ok(())
    }

    #[gtest]
    fn test_nontrivial_nonpublic_destructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct Indestructible final {
              Indestructible() = default;
              Indestructible(int);
              Indestructible(const Indestructible&) = default;
              void Foo() const;
             private:
              ~Indestructible() {}
            };

            Indestructible ReturnsValue();
            void TakesValue(Indestructible);
            void TakesReference(const Indestructible& x);
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        // It isn't available by value:
        assert_rs_not_matches!(rs_api, quote! {CtorNew});
        assert_rs_not_matches!(rs_api, quote! {ReturnsValue});
        assert_rs_not_matches!(rs_api, quote! {TakesValue});
        // ... but it is otherwise available:
        assert_rs_matches!(rs_api, quote! {struct Indestructible});
        assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn TakesReference<'a>(x: &'a crate::Indestructible)});
        Ok(())
    }

    /// trivial abstract structs shouldn't be constructible, not even via
    /// Copy/Clone.
    ///
    /// Right now, a struct can only be Copy/Clone if it's final, but that
    /// restriction will likely be lifted later.
    #[gtest]
    fn test_trivial_abstract_by_value() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct Abstract final {
              Abstract() = default;
              Abstract(int);
              Abstract(const Abstract&) = default;
              virtual void Foo() const = 0;
              void Nonvirtual() const;
            };
            void TakesAbstract(const Abstract& a);
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        // It isn't available by value:
        assert_rs_not_matches!(rs_api, quote! {Default});
        assert_rs_not_matches!(rs_api, quote! {From});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Copy ... )});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Clone ... )});
        // ... but it is otherwise available:
        assert_rs_matches!(rs_api, quote! {struct Abstract});
        assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn Nonvirtual<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn TakesAbstract<'a>(a: &'a crate::Abstract)});
        Ok(())
    }

    #[gtest]
    fn test_nontrivial_abstract_by_value() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct Abstract final {
              Abstract() {};
              Abstract(int);
              Abstract(const Abstract&) {}
              virtual void Foo() const = 0;
              void Nonvirtual() const;
            };
            void TakesAbstract(const Abstract& a);
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {CtorNew});
        // ... but it is otherwise available:
        assert_rs_matches!(rs_api, quote! {struct Abstract});
        assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn Nonvirtual<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn TakesAbstract<'a>(a: &'a crate::Abstract)});
        Ok(())
    }

    #[gtest]
    fn test_struct_with_unnamed_struct_and_union_members() -> Result<()> {
        // This test input causes `field_decl->getName()` to return an empty string.
        // See also:
        // - https://en.cppreference.com/w/c/language/struct: "[...] an unnamed member
        //   of a struct whose type is a struct without name is known as anonymous
        //   struct."
        // - https://rust-lang.github.io/rfcs/2102-unnamed-fields.html
        let ir = ir_from_cc(
            r#"
            struct StructWithUnnamedMembers {
              int first_field;

              struct {
                int anonymous_struct_field_1;
                int anonymous_struct_field_2;
              };
              union {
                int anonymous_union_field_1;
                int anonymous_union_field_2;
              };

              int last_field;
            }; "#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        // TODO(b/200067824): Once nested structs anhd unions are supported,
        // `__unnamed_field1` and `__unnamed_field2` should have a real, usable
        // type.
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(4))]
                #[__crubit::annotate(cpp_type="StructWithUnnamedMembers")]
                pub struct StructWithUnnamedMembers {
                   pub first_field: ::core::ffi::c_int,
                   #[doc =" Reason for representing this field as a blob of bytes:\n Unsupported type 'struct StructWithUnnamedMembers::(anonymous at ./ir_from_cc_virtual_header.h:7:15)': No generated bindings found for ''"]
                   pub(crate) __unnamed_field1: [::core::mem::MaybeUninit<u8>; 8],
                   #[doc =" Reason for representing this field as a blob of bytes:\n Unsupported type 'union StructWithUnnamedMembers::(anonymous at ./ir_from_cc_virtual_header.h:11:15)': No generated bindings found for ''"]
                   pub(crate) __unnamed_field2: [::core::mem::MaybeUninit<u8>; 4],
                   pub last_field: ::core::ffi::c_int,
                }
                ...
                const _: () = {
                    ...
                    assert!(::core::mem::offset_of!(
                        crate::StructWithUnnamedMembers, first_field) == 0);
                    assert!(::core::mem::offset_of!(
                       crate::StructWithUnnamedMembers, __unnamed_field1) == 4);
                    assert!(::core::mem::offset_of!(
                       crate::StructWithUnnamedMembers, __unnamed_field2) == 12);
                    assert!(::core::mem::offset_of!(
                       crate::StructWithUnnamedMembers, last_field) == 16);
                    ...
                };
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_copy_derives() {
        let record = ir_record("S");
        assert_eq!(generate_derives(&record), &["Clone", "Copy"]);
    }

    #[gtest]
    fn test_copy_derives_not_is_trivial_abi() {
        let mut record = ir_record("S");
        record.is_trivial_abi = false;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    #[gtest]
    fn test_copy_derives_ctor_deleted() {
        let mut record = ir_record("S");
        record.copy_constructor = ir::SpecialMemberFunc::Unavailable;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    #[gtest]
    fn test_copy_derives_ctor_nontrivial_members() {
        let mut record = ir_record("S");
        record.copy_constructor = ir::SpecialMemberFunc::NontrivialMembers;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    #[gtest]
    fn test_copy_derives_ctor_nontrivial_self() {
        let mut record = ir_record("S");
        record.copy_constructor = ir::SpecialMemberFunc::NontrivialUserDefined;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    /// In Rust, a Drop type cannot be Copy.
    #[gtest]
    fn test_copy_derives_dtor_nontrivial_self() {
        let mut record = ir_record("S");
        for definition in
            [ir::SpecialMemberFunc::NontrivialUserDefined, ir::SpecialMemberFunc::NontrivialMembers]
        {
            record.destructor = definition;
            assert_eq!(generate_derives(&record), &["Clone"]);
        }
    }

    #[gtest]
    fn test_base_class_subobject_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            // We use a class here to force `Derived::z` to live inside the tail padding of `Base`.
            // On the Itanium ABI, this would not happen if `Base` were a POD type.
            class Base {__INT64_TYPE__ x; char y;};
            struct Derived final : Base {__INT16_TYPE__ z;};
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                #[__crubit::annotate(cpp_type="Derived")]
                pub struct Derived {
                    __non_field_data: [::core::mem::MaybeUninit<u8>; 10],
                    pub z: ::core::ffi::c_short,
                }
            }
        );
        Ok(())
    }

    /// The same as test_base_class_subobject_layout, but with multiple
    /// inheritance.
    #[gtest]
    fn test_base_class_multiple_inheritance_subobject_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base1 {__INT64_TYPE__ x;};
            class Base2 {char y;};
            struct Derived final : Base1, Base2 {__INT16_TYPE__ z;};
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                #[__crubit::annotate(cpp_type="Derived")]
                pub struct Derived {
                    __non_field_data: [::core::mem::MaybeUninit<u8>; 10],
                    pub z: ::core::ffi::c_short,
                }
            }
        );
        Ok(())
    }

    /// The same as test_base_class_subobject_layout, but with a chain of
    /// inheritance.
    #[gtest]
    fn test_base_class_deep_inheritance_subobject_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base1 {__INT64_TYPE__ x;};
            class Base2 : Base1 {char y;};
            struct Derived final : Base2 {__INT16_TYPE__ z;};
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                #[__crubit::annotate(cpp_type="Derived")]
                pub struct Derived {
                    __non_field_data: [::core::mem::MaybeUninit<u8>; 10],
                    pub z: ::core::ffi::c_short,
                }
            }
        );
        Ok(())
    }

    /// For derived classes with no data members, we can't use the offset of the
    /// first member to determine the size of the base class subobjects.
    #[gtest]
    fn test_base_class_subobject_fieldless_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base {__INT64_TYPE__ x; char y;};
            struct Derived final : Base {};
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                #[__crubit::annotate(cpp_type="Derived")]
                pub struct Derived {
                    __non_field_data: [::core::mem::MaybeUninit<u8>; 16],
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_base_class_subobject_empty_fieldless() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base {};
            struct Derived final : Base {};
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                #[__crubit::annotate(cpp_type="Derived")]
                pub struct Derived {
                    ...
                    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_base_class_subobject_empty() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base {};
            struct Derived final : Base {
                __INT16_TYPE__ x;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[__crubit::annotate(cpp_type="Derived")]
                pub struct Derived {
                    pub x: ::core::ffi::c_short,
                }
            }
        );
        Ok(())
    }

    /// Non-aggregate structs can't be directly initialized, because we add
    /// a zero-sized private field to the bindings.
    #[gtest]
    fn test_non_aggregate_struct_private_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            struct NonAggregate {
                NonAggregate() {}

                __INT16_TYPE__ x = 0;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub struct NonAggregate {
                    __non_field_data:  [::core::mem::MaybeUninit<u8>; 0],
                    pub x: ::core::ffi::c_short,
                }
            }
        );
        Ok(())
    }

    /// When a field is [[no_unique_address]], it occupies the space up to the
    /// next field.
    #[gtest]
    fn test_no_unique_address() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field1 {__INT64_TYPE__ x;};
            class Field2 {char y;};
            struct Struct final {
                [[no_unique_address]] Field1 field1;
                [[no_unique_address]] Field2 field2;
                __INT16_TYPE__ z;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                #[__crubit::annotate(cpp_type="Struct")]
                pub struct Struct {
                    ...
                    pub(crate) field1: [::core::mem::MaybeUninit<u8>; 8],
                    ...
                    pub(crate) field2: [::core::mem::MaybeUninit<u8>; 2],
                    pub z: ::core::ffi::c_short,
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                impl Struct {
                    pub fn field1(&self) -> &crate::Field1 {
                        unsafe {
                            let ptr = (self as *const Self as *const u8).offset(0);
                            &*(ptr as *const crate::Field1)
                        }
                    }
                    pub fn field2(&self) -> &crate::Field2 {
                        unsafe {
                            let ptr = (self as *const Self as *const u8).offset(8);
                            &*(ptr as *const crate::Field2)
                        }
                    }
                }
            }
        );
        Ok(())
    }

    /// When a [[no_unique_address]] field is the last one, it occupies the rest
    /// of the object.
    #[gtest]
    fn test_no_unique_address_last_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field1 {__INT64_TYPE__ x;};
            class Field2 {char y;};
            struct Struct final {
                [[no_unique_address]] Field1 field1;
                [[no_unique_address]] Field2 field2;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                #[__crubit::annotate(cpp_type="Struct")]
                pub struct Struct {
                    ...
                    pub(crate) field1: [::core::mem::MaybeUninit<u8>; 8],
                    ...
                    pub(crate) field2: [::core::mem::MaybeUninit<u8>; 8],
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_no_unique_address_empty() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field {};
            struct Struct final {
                // Doc comment for no_unique_address empty class type field.
                [[no_unique_address]] Field field;
                int x;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                #[__crubit::annotate(cpp_type="Struct")]
                pub struct Struct {
                    pub x: ::core::ffi::c_int,
                }
                ...
                impl Struct {
                  # [doc = " Doc comment for no_unique_address empty class type field."]
                  pub fn field(&self) -> &crate::Field {
                        unsafe {
                            let ptr = (self as *const Self as *const u8).offset(0);
                            &*(ptr as *const crate::Field)
                        }
                      }
                }
                ...
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_base_class_subobject_empty_last_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field {};
            struct Struct final {
                // Doc comment for no_unique_address empty class type field.
                [[no_unique_address]] Field field;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                #[__crubit::annotate(cpp_type="Struct")]
                pub struct Struct {}
                ...
                impl Struct {
                  # [doc = " Doc comment for no_unique_address empty class type field."]
                  pub fn field(&self) -> &crate::Field {
                      unsafe {
                          let ptr = (self as *const Self as *const u8).offset(0);
                          &*(ptr as *const crate::Field)
                      }
                  }
              }
              ...
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_doc_comment_record() -> Result<()> {
        let ir = ir_from_cc(
            "// Doc Comment\n\
            //\n\
            //  * with bullet\n\
            struct SomeStruct final {\n\
                // Field doc\n\
                int field;\
            };",
        )?;

        assert_rs_matches!(
            generate_bindings_tokens(ir)?.rs_api,
            quote! {
                #[doc = " Doc Comment\n \n  * with bullet\n \n Generated from: google3/ir_from_cc_virtual_header.h;l=6"]
                #[derive(Clone, Copy)]
                #[repr(C)]
                #[__crubit::annotate(cpp_type="SomeStruct")]
                pub struct SomeStruct {
                    # [doc = " Field doc"]
                    pub field: ::core::ffi::c_int,
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_basic_union() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            union SomeUnion {
                int some_field;
                long long some_bigger_field;
            };
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C)]
                #[__crubit::annotate(cpp_type="SomeUnion")]
                pub union SomeUnion {
                    pub some_field: ::core::ffi::c_int,
                    pub some_bigger_field: ::core::ffi::c_longlong,
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN9SomeUnionC1Ev(union SomeUnion*__this) {...}
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(CRUBIT_SIZEOF(union SomeUnion)==8) }
        );
        assert_cc_matches!(rs_api_impl, quote! { static_assert(alignof(union SomeUnion)==8) });
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(CRUBIT_OFFSET_OF(some_field, union SomeUnion)==0) }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(CRUBIT_OFFSET_OF(some_bigger_field, union SomeUnion)==0) }
        );
        Ok(())
    }

    #[gtest]
    fn test_union_with_opaque_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            union MyUnion {
                char first_field[56];
                int second_field;
              };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(4))]
                #[__crubit::annotate(cpp_type="MyUnion")]
                pub union MyUnion { ...
                    first_field: [::core::mem::MaybeUninit<u8>; 56],
                    pub second_field: ::core::ffi::c_int,
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = {
                    ...
                    assert!(::core::mem::size_of::<crate::MyUnion>() == 56);
                    assert!(::core::mem::align_of::<crate::MyUnion>() == 4);
                    ...
                };
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_currently_no_offset_assertions_for_unions() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            union SomeUnion {
                int some_field;
                long long some_bigger_field;
            };
            "#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;

        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = {
                    ...
                    assert!(::core::mem::offset_of!(
                        crate::SomeUnion, some_field) == 0);
                    assert!(::core::mem::offset_of!(
                        crate::SomeUnion, some_bigger_field) == 0);
                    ...
                };
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_union_with_private_fields() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            union SomeUnionWithPrivateFields {
              public:
                int public_field;
              private:
                long long private_field;
            };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C, align(8))]
                #[__crubit::annotate(cpp_type="SomeUnionWithPrivateFields")]
                pub union SomeUnionWithPrivateFields {
                    pub public_field: ::core::ffi::c_int,
                    #[doc = " Reason for representing this field as a blob of bytes:\n Types of non-public C++ fields can be elided away"]
                    pub(crate) private_field: [::core::mem::MaybeUninit<u8>; 8],
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = {
                    ...
                    assert!(::core::mem::size_of::<crate::SomeUnionWithPrivateFields>() == 8);
                    assert!(::core::mem::align_of::<crate::SomeUnionWithPrivateFields>() == 8);
                    static_assertions::assert_impl_all!(crate::SomeUnionWithPrivateFields: Clone);
                    static_assertions::assert_impl_all!(crate::SomeUnionWithPrivateFields: Copy);
                    static_assertions::assert_not_impl_any!(crate::SomeUnionWithPrivateFields: Drop);
                    ...
                };
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_nontrivial_unions() -> Result<()> {
        let ir = ir_from_cc_dependency(
            r#"
            union UnionWithNontrivialField {
                NonTrivialStruct my_field;
            };
            "#,
            r#"
            struct NonTrivialStruct {
                NonTrivialStruct(NonTrivialStruct&&);
            };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_not_matches!(rs_api, quote! {derive ( ... Copy ... )});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Clone ... )});
        assert_rs_matches!(
            rs_api,
            quote! {
                #[::ctor::recursively_pinned]
                #[repr(C)]
                #[__crubit::annotate(cpp_type="UnionWithNontrivialField")]
                pub union UnionWithNontrivialField { ... }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_empty_struct() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            struct EmptyStruct final {};
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C)]
                #[__crubit::annotate(cpp_type="EmptyStruct")]
                pub struct EmptyStruct {
                    ...
                    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = {
                    ...
                    assert!(::core::mem::size_of::<crate::EmptyStruct>() == 1);
                    assert!(::core::mem::align_of::<crate::EmptyStruct>() == 1);
                    ...
                };
            }
        );

        Ok(())
    }

    #[gtest]
    fn test_empty_union() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            union EmptyUnion {};
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C)]
                #[__crubit::annotate(cpp_type="EmptyUnion")]
                pub union EmptyUnion {
                    ...
                    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = {
                    ...
                    assert!(::core::mem::size_of::<crate::EmptyUnion>() == 1);
                    assert!(::core::mem::align_of::<crate::EmptyUnion>() == 1);
                    ...
                };
            }
        );

        Ok(())
    }

    #[gtest]
    fn test_union_field_with_nontrivial_destructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            struct NontrivialStruct { ~NontrivialStruct(); };
            union UnionWithNontrivialField {
                int trivial_field;
                NontrivialStruct nontrivial_field;
            };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                #[__crubit::annotate(cpp_type="UnionWithNontrivialField")]
                pub union UnionWithNontrivialField {
                    pub trivial_field: ::core::ffi::c_int,
                    pub nontrivial_field: ::core::mem::ManuallyDrop<crate::NontrivialStruct>,
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = {
                    ...
                    assert!(::core::mem::size_of::<crate::UnionWithNontrivialField>() == 4);
                    assert!(::core::mem::align_of::<crate::UnionWithNontrivialField>() == 4);
                    ...
                };
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_union_with_constructors() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            union UnionWithDefaultConstructors {
                int a;
            };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C)]
                #[__crubit::annotate(cpp_type="UnionWithDefaultConstructors")]
                pub union UnionWithDefaultConstructors {
                    pub a: ::core::ffi::c_int,
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                impl Default for UnionWithDefaultConstructors {
                    #[inline(always)]
                    fn default() -> Self {
                        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN28UnionWithDefaultConstructorsC1Ev(&mut tmp);
                            tmp.assume_init()
                        }
                    }
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> From<::ctor::RvalueReference<'b, Self>> for UnionWithDefaultConstructors {
                    #[inline(always)]
                    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
                        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN28UnionWithDefaultConstructorsC1EOS_(&mut tmp, __param_0);
                            tmp.assume_init()
                        }
                    }
                }
            }
        );

        Ok(())
    }

    #[gtest]
    fn test_unambiguous_public_bases() -> Result<()> {
        let ir = ir_from_cc_dependency(
            "
            struct VirtualBase {};
            struct PrivateBase {};
            struct ProtectedBase {};
            struct UnambiguousPublicBase {};
            struct AmbiguousPublicBase {};
            struct MultipleInheritance : UnambiguousPublicBase, AmbiguousPublicBase {};
            struct Derived : private PrivateBase, protected ProtectedBase, MultipleInheritance, AmbiguousPublicBase, virtual VirtualBase {};
        ",
            "",
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                unsafe impl oops::Inherits<crate::VirtualBase> for crate::Derived {
                    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::VirtualBase {
                        crate::detail::__crubit_dynamic_upcast__7Derived__to__11VirtualBase___2f_2ftest_3atesting_5ftarget(derived)
                    }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! { unsafe impl oops::Inherits<crate::UnambiguousPublicBase> for crate::Derived }
        );
        assert_rs_matches!(
            rs_api,
            quote! { unsafe impl oops::Inherits<crate::MultipleInheritance> for crate::Derived }
        );
        assert_rs_not_matches!(
            rs_api,
            quote! {unsafe impl oops::Inherits<crate::PrivateBase> for crate::Derived}
        );
        assert_rs_not_matches!(
            rs_api,
            quote! {unsafe impl oops::Inherits<crate::ProtectedBase> for crate::Derived}
        );
        assert_rs_not_matches!(
            rs_api,
            quote! {unsafe impl oops::Inherits<crate::AmbiguousPublicBase> for crate::Derived}
        );
        Ok(())
    }

    /// Contrary to intuitions: a base class conversion is ambiguous even if the
    /// ambiguity is from a private base class cast that you can't even
    /// perform.
    ///
    /// Explanation (courtesy James Dennett):
    ///
    /// > Once upon a time, there was a rule in C++ that changing all access
    /// > specifiers to "public" would not change the meaning of code.
    /// > That's no longer true, but some of its effects can still be seen.
    ///
    /// So, we need to be sure to not allow casting to privately-ambiguous
    /// bases.
    #[gtest]
    fn test_unambiguous_public_bases_private_ambiguity() -> Result<()> {
        let ir = ir_from_cc_dependency(
            "
            struct Base {};
            struct Intermediate : public Base {};
            struct Derived : Base, private Intermediate {};
        ",
            "",
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(
            rs_api,
            quote! { unsafe impl oops::Inherits<crate::Base> for Derived }
        );
        Ok(())
    }

    #[gtest]
    fn test_virtual_thunk() -> Result<()> {
        let ir = ir_from_cc("struct Polymorphic { virtual void Foo(); };")?;

        assert_cc_matches!(
            generate_bindings_tokens(ir)?.rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN11Polymorphic3FooEv(struct Polymorphic * __this)
            }
        );
        Ok(())
    }

    /// A trivially relocatable final struct is safe to use in Rust as normal,
    /// and is Unpin.
    #[gtest]
    fn test_no_negative_impl_unpin() -> Result<()> {
        let ir = ir_from_cc("struct Trivial final {};")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {#[::ctor::recursively_pinned]});
        Ok(())
    }

    #[gtest]
    fn test_no_aligned_attr() {
        let ir = ir_from_cc("struct SomeStruct {};").unwrap();
        let rs_api = generate_bindings_tokens(ir).unwrap().rs_api;

        assert_rs_matches! {rs_api, quote! {
            #[repr(C)]
            #[__crubit::annotate(cpp_type="SomeStruct")]
            pub struct SomeStruct { ... }
        }};
    }

    #[gtest]
    fn test_aligned_attr() {
        let ir = ir_from_cc("struct SomeStruct {} __attribute__((aligned(64)));").unwrap();
        let rs_api = generate_bindings_tokens(ir).unwrap().rs_api;

        assert_rs_matches! {rs_api, quote! {
           #[repr(C, align(64))]
            #[__crubit::annotate(cpp_type="SomeStruct")]
           pub struct SomeStruct { ... }
          }
        };
    }

    #[gtest]
    fn test_forward_declared() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct ForwardDeclared;"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                forward_declare::forward_declare!(pub ForwardDeclared = forward_declare::symbol!("ForwardDeclared"));
            }
        );
        assert_rs_not_matches!(rs_api, quote! {struct ForwardDeclared});
        Ok(())
    }

    #[gtest]
    fn test_private_struct_not_present() -> Result<()> {
        let ir = ir_from_cc(&with_lifetime_macros(
            r#"#pragma clang lifetime_elision
            template <typename T> class MyTemplate {};
            class HasPrivateType {
             private:
              struct PrivateType {
                using Foo = MyTemplate<PrivateType>;
                Foo* get();
              };
             protected:
              HasPrivateType(MyTemplate<PrivateType> x) {}
            };"#,
        ))?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_not_matches!(
            rs_api,
            quote! { __CcTemplateInst10MyTemplateIN14HasPrivateType11PrivateTypeEE }
        );
        Ok(())
    }

    #[gtest]
    fn test_implicit_template_specializations_are_sorted_by_mangled_name() -> Result<()> {
        let bindings = generate_bindings_tokens(ir_from_cc(
            r#"
                template <typename T>
                struct MyStruct {
                    T getT();
                };

                using Alias1 = MyStruct<int>;
                using Alias2 = MyStruct<double>;

                namespace test_namespace_bindings {
                    using Alias3 = MyStruct<bool>;
                }
                "#,
        )?)?;

        // Mangled name order: bool < double < int
        let my_struct_bool = make_rs_ident("__CcTemplateInst8MyStructIbE");
        let my_struct_double = make_rs_ident("__CcTemplateInst8MyStructIdE");
        let my_struct_int = make_rs_ident("__CcTemplateInst8MyStructIiE");

        assert_rs_matches!(
            &bindings.rs_api,
            quote! {
                ...
                pub struct #my_struct_bool {...}
                ...
                pub struct #my_struct_double {...}
                ...
                pub struct #my_struct_int {...}
                ...
                const _: () = {
                    ...
                    assert!(::core::mem::size_of::<crate::#my_struct_bool>() == 1);
                    ...
                    assert!(::core::mem::size_of::<crate::#my_struct_double>() == 1);
                    ...
                    assert!(::core::mem::size_of::<crate::#my_struct_int>() == 1);
                    ...
                }
                ...
            }
        );

        // User defined methods in mangled name order
        let my_struct_bool_method =
            make_rs_ident("__rust_thunk___ZN8MyStructIbE4getTEv__2f_2ftest_3atesting_5ftarget");
        let my_struct_double_method =
            make_rs_ident("__rust_thunk___ZN8MyStructIdE4getTEv__2f_2ftest_3atesting_5ftarget");
        let my_struct_int_method =
            make_rs_ident("__rust_thunk___ZN8MyStructIiE4getTEv__2f_2ftest_3atesting_5ftarget");

        assert_cc_matches!(
            &bindings.rs_api_impl,
            quote! {
                ...
                extern "C" bool #my_struct_bool_method(struct MyStruct<bool>*__this) {...} ...
                extern "C" double #my_struct_double_method(struct MyStruct<double>*__this) {...} ...
                extern "C" int #my_struct_int_method(struct MyStruct<int>*__this) {...} ...
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_implicit_template_specialization_namespace_qualifier() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#" #pragma clang lifetime_elision
                namespace test_namespace_bindings {
                    template <typename T>
                    struct MyTemplate final {
                        T value_;
                    };

                    using MyTypeAlias = MyTemplate<int>;
                }"#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                ...
                pub mod test_namespace_bindings {
                    ...
                    pub type MyTypeAlias = crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;
                    ...
                }
                ...
                pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
                    pub value_: ::core::ffi::c_int,
                }
                ...
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_derived_class_inherits_unambiguous_public_functions_bases() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test{
            class Base1 {
              public:
                void NonColliding();
                void Colliding();
            };

            class Base2 {
              public:
                void Colliding();
              private:
                void PrivateFunc();
            };

            class Derived : public Base1, public Base2 {
            };
            }
            "#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                ...
                impl Derived {
                    ...
                    #[inline(always)]
                    pub unsafe fn NonColliding(__this: *mut Self) {
                        crate::detail::__rust_thunk___ZN4test5Base112NonCollidingEv_Derived(oops::UnsafeUpcast::<_>::unsafe_upcast(__this))
                    }
                }
                ...
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_member_in_derived_class_overwrites_inherited_ones() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test{
            class Base1 {
              public:
                void Colliding();
            };

            class Derived : public Base1 {
              public:
                void Colliding();
            };
            }
            "#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                ...
                impl Derived {
                    ...
                    #[inline(always)]
                    pub unsafe fn Colliding(__this: *mut Self) {
                        crate::detail::__rust_thunk___ZN4test7Derived9CollidingEv(__this)
                    }
                }
                ...
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_forward_declared_class_template_specialization_symbol() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test_namespace_bindings {
              template <typename T>
              struct MyTemplate {
                void processT(T t);
              };

              struct Param {};

              template<> struct MyTemplate<Param>;

              using MyTypeAlias = MyTemplate<Param>;
            }"#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                ...
                forward_declare::forward_declare!(pub __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_5ParamEEE = forward_declare::symbol!("__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_5ParamEEE"));
                ...
            }
        );
        Ok(())
    }

    /// Unsupported fields on supported structs are replaced with opaque blobs.
    ///
    /// This is hard to test any other way than token comparison!
    #[gtest]
    fn test_supported_suppressed_field_types() -> Result<()> {
        // Ideally we'd use a cross-platform test, but it's hard to craft an unsupported
        // type that is still returned successfully by db.rs_type_kind(), and so
        // results in a secondary failure when we check afterwards for the
        // required features for the type.
        if multiplatform_testing::test_platform() != multiplatform_testing::Platform::X86Linux {
            return Ok(()); // vectorcall only exists on x86_64, not e.g. aarch64
        }
        let mut ir = ir_from_cc(
            r#"
            struct Nontrivial {
                ~Nontrivial();
            };

            struct Trivial {
                Nontrivial* hidden_field;
                // An example of a field which has a type that is not supported,
                // but _is_ successfully retrieved by db.rs_type_kind().
                void(*hidden_field_2)() [[clang::vectorcall]];
            };
        
        "#,
        )?;
        *ir.target_crubit_features_mut(&ir.current_target().clone()) =
            crubit_feature::CrubitFeature::Supported.into();
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
            struct Trivial {
                ...
                pub(crate) hidden_field: [::core::mem::MaybeUninit<u8>; 8],
                ...
                pub(crate) hidden_field_2: [::core::mem::MaybeUninit<u8>; 8],
                ...
            }}
        );
        Ok(())
    }

    /// Nontrivial fields are replaced with opaque blobs, even if they're
    /// supported!
    #[gtest]
    fn test_supported_nontrivial_field() -> Result<()> {
        let mut ir = ir_from_cc(
            r#"
            struct [[clang::trivial_abi]] Inner {~Inner();};
            struct [[clang::trivial_abi]] Outer {Inner inner_field;};
            "#,
        )?;
        *ir.target_crubit_features_mut(&ir.current_target().clone()) =
            crubit_feature::CrubitFeature::Supported.into();
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        // Note: inner is a supported type, so it isn't being replaced by a blob because
        // it's unsupporter or anything.
        assert_rs_matches!(rs_api, quote! {pub struct Inner});
        // But it _is_ being replaced by a blob!
        assert_rs_matches!(
            rs_api,
            quote! {
            pub struct Outer {
                ...
                pub(crate) inner_field: [::core::mem::MaybeUninit<u8>; 1],
            }}
        );
        Ok(())
    }

    #[gtest]
    fn test_supported_no_unique_address_field() -> Result<()> {
        let mut ir = ir_from_cc(
            r#"
            struct Struct final {
                [[no_unique_address]] char field;
            };
        "#,
        )?;
        *ir.target_crubit_features_mut(&ir.current_target().clone()) =
            crubit_feature::CrubitFeature::Supported.into();
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub struct Struct {
                    ...
                    pub(crate) field: [::core::mem::MaybeUninit<u8>; 1],
                }
            }
        );
        assert_rs_not_matches!(rs_api, quote! {pub fn field});
        Ok(())
    }
}
