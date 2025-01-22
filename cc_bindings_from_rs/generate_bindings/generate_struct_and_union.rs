// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// TODO(b/381888123): Seperate out enum generation.
use crate::format_cc_ident;
use crate::generate_doc_comment;
use crate::{
    crate_features, format_ty_for_cc, generate_const, generate_deprecated_tag,
    generate_trait_thunks, generate_unsupported_def, get_layout, get_scalar_int_type,
    get_tag_size_with_padding, is_exported, is_public_or_supported_export,
    post_analysis_typing_env, RsSnippet, TraitThunks,
};
use arc_anyhow::{Context, Result};
use code_gen_utils::make_rs_ident;
use code_gen_utils::CcInclude;
use database::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet};
use database::{AdtCoreBindings, BindingsGenerator, FullyQualifiedName, SugaredTy, TypeLocation};
use error_report::{anyhow, ensure};
use itertools::Itertools;
use proc_macro2::{Literal, TokenStream};
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use rustc_hir::{AssocItemKind, ItemKind};
use rustc_middle::ty::{self, TyCtxt};
use rustc_span::def_id::{DefId, LOCAL_CRATE};
use rustc_target::abi::{FieldsShape, VariantIdx, Variants};
use std::collections::{BTreeSet, HashSet};
use std::iter::once;
use std::rc::Rc;

pub(crate) fn adt_core_bindings_needs_drop<'tcx>(
    bindings: &AdtCoreBindings<'tcx>,
    tcx: TyCtxt<'tcx>,
) -> bool {
    bindings.self_ty.needs_drop(tcx, post_analysis_typing_env(tcx, bindings.def_id))
}

/// Formats an algebraic data type (an ADT - a struct, an enum, or a union)
/// represented by `core`.  This function is infallible - after
/// `generate_adt_core` returns success we have committed to emitting C++
/// bindings for the ADT.
pub fn generate_adt<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
) -> ApiSnippets {
    let tcx = db.tcx();
    let adt_cc_name = &core.cc_short_name;

    // `generate_adt` should only be called for local ADTs.
    let local_def_id = core.def_id.expect_local();

    let default_ctor_snippets = db.generate_default_ctor(core.clone()).unwrap_or_else(|err| err);

    let destructor_snippets = if adt_core_bindings_needs_drop(&core, tcx) {
        let drop_trait_id =
            tcx.lang_items().drop_trait().expect("`Drop` trait should be present if `needs_drop");
        let TraitThunks {
            method_name_to_cc_thunk_name,
            cc_thunk_decls,
            rs_thunk_impls: rs_details,
        } = generate_trait_thunks(db, drop_trait_id, &core)
            .expect("`generate_adt_core` should have already validated `Drop` support");
        let drop_thunk_name = method_name_to_cc_thunk_name
            .into_values()
            .exactly_one()
            .expect("Expecting a single `drop` method");
        let main_api = CcSnippet::new(quote! {
            __NEWLINE__ __COMMENT__ "Drop::drop"
            ~#adt_cc_name(); __NEWLINE__
            __NEWLINE__
        });
        let cc_details = {
            let mut prereqs = CcPrerequisites::default();
            let cc_thunk_decls = cc_thunk_decls.into_tokens(&mut prereqs);
            let tokens = quote! {
                #cc_thunk_decls
                inline #adt_cc_name::~#adt_cc_name() {
                    __crubit_internal::#drop_thunk_name(*this);
                }
            };
            CcSnippet { tokens, prereqs }
        };
        ApiSnippets { main_api, cc_details, rs_details }
    } else {
        let main_api = CcSnippet::new(quote! {
            __NEWLINE__ __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
            ~#adt_cc_name() = default; __NEWLINE__
        });
        let cc_details = CcSnippet::with_include(
            quote! { static_assert(std::is_trivially_destructible_v<#adt_cc_name>); },
            CcInclude::type_traits(),
        );
        ApiSnippets { main_api, cc_details, ..Default::default() }
    };

    let copy_ctor_and_assignment_snippets =
        db.generate_copy_ctor_and_assignment_operator(core.clone()).unwrap_or_else(|err| err);

    let move_ctor_and_assignment_snippets =
        db.generate_move_ctor_and_assignment_operator(core.clone()).unwrap_or_else(|err| err);

    let mut member_function_names = HashSet::<String>::new();
    let impl_items_snippets = tcx
        .inherent_impls(core.def_id)
        .iter()
        .map(|impl_id| tcx.hir().expect_item(impl_id.expect_local()))
        .flat_map(|item| match &item.kind {
            ItemKind::Impl(impl_) => impl_.items,
            other => panic!("Unexpected `ItemKind` from `inherent_impls`: {other:?}"),
        })
        .sorted_by_key(|impl_item_ref| {
            let def_id = impl_item_ref.id.owner_id.def_id;
            tcx.def_span(def_id)
        })
        .filter_map(|impl_item_ref| {
            let def_id = impl_item_ref.id.owner_id.def_id;
            if !is_exported(db.tcx(), def_id.to_def_id()) {
                return None;
            }
            let result = match impl_item_ref.kind {
                AssocItemKind::Fn { .. } => {
                    let result = db.generate_function(def_id);
                    if result.is_ok() {
                        let cpp_name = FullyQualifiedName::new(db, def_id.into())
                            .cpp_name
                            .unwrap()
                            .to_string();
                        member_function_names.insert(cpp_name);
                    }
                    result
                }
                AssocItemKind::Const => generate_const(db, def_id),
                other => Err(anyhow!("Unsupported `impl` item kind: {other:?}")),
            };
            let result = result.and_then(|snippet| {
                snippet.resolve_feature_requirements(crate_features(db, LOCAL_CRATE))
            });
            match result {
                Err(err) => Some(generate_unsupported_def(db, def_id, err)),
                Ok(result) => Some(result),
            }
        })
        .collect();

    let ApiSnippets {
        main_api: public_functions_main_api,
        cc_details: public_functions_cc_details,
        rs_details: public_functions_rs_details,
    } = [
        default_ctor_snippets,
        destructor_snippets,
        move_ctor_and_assignment_snippets,
        copy_ctor_and_assignment_snippets,
        impl_items_snippets,
    ]
    .into_iter()
    .collect();

    let ApiSnippets {
        main_api: fields_main_api,
        cc_details: fields_cc_details,
        rs_details: fields_rs_details,
    } = generate_fields(db, &core, &member_function_names);

    let alignment = Literal::u64_unsuffixed(core.alignment_in_bytes);
    let size = Literal::u64_unsuffixed(core.size_in_bytes);
    let main_api = {
        let rs_type = core.rs_fully_qualified_name.to_string();
        let mut attributes = vec![
            quote! {CRUBIT_INTERNAL_RUST_TYPE(#rs_type)},
            quote! {alignas(#alignment)},
            quote! {[[clang::trivial_abi]]},
        ];
        if db
            .repr_attrs(core.def_id)
            .iter()
            .any(|repr| matches!(repr, rustc_attr_data_structures::ReprPacked { .. }))
        {
            attributes.push(quote! { __attribute__((packed)) })
        }

        // Attribute: must_use
        if let Some(must_use_attr) = tcx.get_attr(core.def_id, rustc_span::symbol::sym::must_use) {
            match must_use_attr.value_str() {
                None => attributes.push(quote! {[[nodiscard]]}),
                Some(symbol) => {
                    let message = symbol.as_str();
                    attributes.push(quote! {[[nodiscard(#message)]]});
                }
            }
        }

        // Attribute: deprecated
        if let Some(cc_deprecated_tag) = generate_deprecated_tag(tcx, core.def_id) {
            attributes.push(cc_deprecated_tag);
        }

        let doc_comment = generate_doc_comment(tcx, core.def_id.expect_local());
        let keyword = &core.keyword;

        let mut prereqs = CcPrerequisites::default();
        prereqs.includes.insert(db.support_header("internal/attribute_macros.h"));
        let public_functions_main_api = public_functions_main_api.into_tokens(&mut prereqs);
        let fields_main_api = fields_main_api.into_tokens(&mut prereqs);
        prereqs.fwd_decls.remove(&local_def_id);

        CcSnippet {
            prereqs,
            tokens: quote! {
                __NEWLINE__ #doc_comment
                #keyword #(#attributes)* #adt_cc_name final {
                    public: __NEWLINE__
                        #public_functions_main_api
                    #fields_main_api
                };
                __NEWLINE__
            },
        }
    };
    let cc_details = {
        let mut prereqs = CcPrerequisites::default();
        let public_functions_cc_details = public_functions_cc_details.into_tokens(&mut prereqs);
        let fields_cc_details = fields_cc_details.into_tokens(&mut prereqs);
        prereqs.defs.insert(local_def_id);
        CcSnippet {
            prereqs,
            tokens: quote! {
                __NEWLINE__
                static_assert(
                    sizeof(#adt_cc_name) == #size,
                    "Verify that ADT layout didn't change since this header got generated");
                static_assert(
                    alignof(#adt_cc_name) == #alignment,
                    "Verify that ADT layout didn't change since this header got generated");
                __NEWLINE__
                #public_functions_cc_details
                #fields_cc_details
            },
        }
    };
    let rs_details = {
        let adt_rs_name = &core.rs_fully_qualified_name;
        let mut extern_c_decls = BTreeSet::new();
        let public_functions_rs_details =
            public_functions_rs_details.into_tokens(&mut extern_c_decls);
        let fields_rs_details = fields_rs_details.into_tokens(&mut extern_c_decls);
        RsSnippet {
            tokens: quote! {
                const _: () = assert!(::std::mem::size_of::<#adt_rs_name>() == #size);
                const _: () = assert!(::std::mem::align_of::<#adt_rs_name>() == #alignment);
                #public_functions_rs_details
                #fields_rs_details
            },
            extern_c_decls,
        }
    };
    ApiSnippets { main_api, cc_details, rs_details }
}

/// Formats the core of an algebraic data type (an ADT - a struct, an enum, or a
/// union) represented by `def_id`.
///
/// The "core" means things that are necessary for a succesful binding (e.g.
/// inability to generate a correct C++ destructor means that the ADT cannot
/// have any bindings).  "core" excludes things that are A) infallible (e.g.
/// struct or union fields which can always be translated into private, opaque
/// blobs of bytes) or B) optional (e.g. a problematic instance method
/// can just be ignored, unlike a problematic destructor).  The split between
/// fallible "core" and non-fallible "rest" is motivated by the need to avoid
/// cycles / infinite recursion (e.g. when processing fields that refer back to
/// the struct type, possible with an indirection of a pointer).
///
/// `generate_adt_core` is used both to 1) format bindings for the core of an
/// ADT, and 2) check if formatting would have succeeded (e.g. when called from
/// `format_ty`).  The 2nd case is needed for ADTs defined in any crate - this
/// is why the `def_id` parameter is a DefId rather than LocalDefId.
pub fn generate_adt_core<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    def_id: DefId,
) -> Result<Rc<AdtCoreBindings<'tcx>>> {
    let tcx = db.tcx();
    let self_ty = tcx.type_of(def_id).instantiate_identity();
    assert!(self_ty.is_adt());
    assert!(is_public_or_supported_export(db, def_id), "Caller should verify");

    let fully_qualified_name = FullyQualifiedName::new(db, def_id);
    let rs_fully_qualified_name = fully_qualified_name.format_for_rs();
    let cpp_name = format_cc_ident(db, fully_qualified_name.cpp_name.unwrap().as_str())
        .context("Error formatting item name")?;

    // The check below ensures that `generate_trait_thunks` will succeed for the
    // `Drop`, `Default`, and/or `Clone` trait. Ideally we would directly check
    // if `generate_trait_thunks` or `format_ty_for_cc(..., self_ty, ...)`
    // succeeds, but this would lead to infinite recursion, so we only replicate
    // `format_ty_for_cc` / `TyKind::Adt` checks that are outside of
    // `generate_adt_core`.
    fully_qualified_name
        .format_for_cc(db)
        .with_context(|| format!("Error formatting the fully-qualified C++ name of `{cpp_name}"))?;

    let adt_def = self_ty.ty_adt_def().expect("`def_id` needs to identify an ADT");
    let keyword = match adt_def.adt_kind() {
        ty::AdtKind::Struct | ty::AdtKind::Enum => quote! { struct },
        ty::AdtKind::Union => quote! { union },
    };

    let layout = get_layout(tcx, self_ty)
        .with_context(|| format!("Error computing the layout of #{cpp_name}"))?;
    ensure!(
        layout.backend_repr().is_sized(),
        "Bindings for dynamically sized types are not supported."
    );
    let alignment_in_bytes = {
        // Only the ABI-mandated alignment is considered (i.e. `AbiAndPrefAlign::pref`
        // is ignored), because 1) Rust's `std::mem::align_of` returns the
        // ABI-mandated alignment and 2) the generated C++'s `alignas(...)`
        // should specify the minimal/mandatory alignment.
        layout.align().abi.bytes()
    };
    let size_in_bytes = layout.size().bytes();
    ensure!(size_in_bytes != 0, "Zero-sized types (ZSTs) are not supported (b/258259459)");

    Ok(Rc::new(AdtCoreBindings {
        def_id,
        keyword,
        cc_short_name: cpp_name,
        rs_fully_qualified_name,
        self_ty,
        alignment_in_bytes,
        size_in_bytes,
    }))
}

/// Returns the body of the C++ struct that represents the given ADT.
fn generate_fields<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
    member_function_names: &HashSet<String>,
) -> ApiSnippets {
    let tcx = db.tcx();

    // TODO(b/259749095): Support non-empty set of generic parameters.
    let substs_ref = ty::List::empty();

    let repr_attrs = db.repr_attrs(core.def_id);

    struct FieldTypeInfo {
        size: u64,
        cpp_type: CcSnippet,
    }
    struct Field {
        type_info: Result<FieldTypeInfo>,
        cc_name: TokenStream,
        rs_name: TokenStream,
        is_public: bool,
        index: usize,
        offset: u64,
        offset_of_next_field: u64,
        doc_comment: TokenStream,
        attributes: Vec<TokenStream>,
    }
    impl Field {
        fn size(&self) -> u64 {
            match self.type_info {
                Err(_) => self.offset_of_next_field - self.offset,
                Ok(FieldTypeInfo { size, .. }) => size,
            }
        }
    }

    let layout = get_layout(tcx, core.self_ty)
        .expect("Layout should be already verified by `generate_adt_core`");
    let adt_def = core.self_ty.ty_adt_def().expect("`core.def_id` needs to identify an ADT");
    let err_fields = |err| {
        vec![Field {
            type_info: Err(err),
            cc_name: quote! { __opaque_blob_of_bytes },
            rs_name: quote! { __opaque_blob_of_bytes },
            is_public: false,
            index: 0,
            offset: 0,
            offset_of_next_field: core.size_in_bytes,
            doc_comment: quote! {},
            attributes: vec![],
        }]
    };

    let layout_variants = layout.variants();

    // If the ADT has one variant, then just use the fields in `layout.fields`.
    // If the ADT has multiple variants, then we need to use the layout of each
    // variant. The `layout.fields` just contains the tag.
    let fields_shape = match layout_variants {
        Variants::Single { .. } | Variants::Empty => vec![&layout.fields],
        Variants::Multiple { tag: _, tag_encoding: _, tag_field: _, variants } => {
            variants.iter().map(|variant| &variant.fields).collect_vec()
        }
    };

    // Used for generating enum bindings.
    let is_supported_enum = adt_def.is_enum()
        && repr_attrs.contains(&rustc_attr_data_structures::ReprC)
        && crate_features(db, core.def_id.krate)
            .contains(crubit_feature::CrubitFeature::Experimental);

    let tag_size_with_padding =
        if is_supported_enum { get_tag_size_with_padding(layout) } else { 0 };

    let variant_sizes = match layout_variants {
        Variants::Multiple { tag: _, tag_encoding: _, tag_field: _, variants } => {
            variants.iter().map(|layout| layout.size.bytes() - tag_size_with_padding).collect_vec()
        }
        Variants::Single { .. } | Variants::Empty => {
            vec![core.alignment_in_bytes]
        }
    };

    // The size of each variant. Note for enums, this removes the size (and padding)
    // for the tag.
    let layout_size = match layout_variants {
        Variants::Single { .. } | Variants::Empty => vec![layout.size().bytes()],
        Variants::Multiple { tag: _, tag_encoding: _, tag_field: _, variants } => variants
            .iter()
            .map(|variant| variant.size.bytes() - tag_size_with_padding)
            .collect_vec(),
    };
    let variants_fields: Vec<Vec<Field>> = match adt_def.adt_kind() {
        // Handle cases of unsupported ADTs.
        ty::AdtKind::Enum if (!repr_attrs.contains(&rustc_attr_data_structures::ReprC)) => {
            vec![err_fields(anyhow!("No support for bindings of individual non-repr(C) `enum`s"))]
        }
        ty::AdtKind::Enum if !is_supported_enum => {
            vec![err_fields(anyhow!(
                "support for repr(C) enums requires //features:experimental"
            ))]
        }
        ty::AdtKind::Union
            if !repr_attrs.contains(&rustc_attr_data_structures::ReprC)
                && !crate_features(db, core.def_id.krate)
                    .contains(crubit_feature::CrubitFeature::Experimental) =>
        {
            vec![err_fields(anyhow!(
              "support for non-repr(C) unions requires //features:experimental"
          ))]
        }

        // Otherwise, get the fields and determine the memory layout.
        _ => {
            let rustc_hir::Node::Item(item) = tcx.hir_node_by_def_id(core.def_id.expect_local())
            else {
                panic!("internal error: def_id referring to an ADT was not a HIR Item.");
            };
            let variants = match item.kind {
                rustc_hir::ItemKind::Struct(variant, _) => vec![variant],
                rustc_hir::ItemKind::Union(variant, _) => vec![variant],
                rustc_hir::ItemKind::Enum(enum_info, _) => {
                    enum_info.variants.iter().map(|variant| variant.data).collect()
                }
                _ => panic!(
                    "internal error: def_id referring to a non-enum ADT was not a struct or union."
                ),
            };
            let hir_fields: Vec<Vec<_>> = variants
                .iter()
                .map(|variant| variant.fields().iter().sorted_by_key(|f| f.span).collect())
                .collect();

            let mut variants_fields = core
                .self_ty
                .ty_adt_def()
                .expect("`core.def_id` needs to identify an ADT")
                .variants()
                .iter_enumerated()
                .map(|(variant_index, variant)| {
                    variant
                        .fields
                        .iter()
                        .sorted_by_key(|f| tcx.def_span(f.did))
                        .enumerate()
                        .map(|(index, field_def)| {
                            // *Not* using zip, in order to crash on length mismatch.
                            let hir_field = hir_fields
                                .get(variant_index.index())
                                .expect("HIR ADT had fewer variants than rustc_middle")
                                .get(index)
                                .expect(
                                    "HIR ADT had fewer fields than rustc_middle for this variant",
                                );
                            assert!(field_def.did == hir_field.def_id.to_def_id());
                            let ty =
                                SugaredTy::new(field_def.ty(tcx, substs_ref), Some(hir_field.ty));
                            let size =
                                get_layout(tcx, ty.mid()).map(|layout| layout.size().bytes());
                            let type_info = size.and_then(|size| {
                                Ok(FieldTypeInfo {
                                    size,
                                    cpp_type: db
                                        .format_ty_for_cc(ty, TypeLocation::Other)?
                                        .resolve_feature_requirements(crate_features(
                                            db,
                                            LOCAL_CRATE,
                                        ))?,
                                })
                            });
                            let name = field_def.ident(tcx).to_string();
                            let cc_name = if member_function_names.contains(&name) {
                                // TODO: Handle the case of name_ itself also being taken? e.g. the
                                // Rust struct struct S {a: i32, a_:
                                // i32} impl S { fn a() {} fn a_()
                                // {} fn a__(){}.
                                format!("{name}_")
                            } else {
                                name.clone()
                            };
                            let cc_name =
                                format_cc_ident(db, cc_name.as_str()).unwrap_or_else(|_err| {
                                    format_ident!("__field{index}").into_token_stream()
                                });
                            let rs_name = {
                                let name_starts_with_digit = name
                                    .as_str()
                                    .chars()
                                    .next()
                                    .expect("Empty names are unexpected (here and in general)")
                                    .is_ascii_digit();
                                if name_starts_with_digit {
                                    let index = Literal::usize_unsuffixed(index);
                                    quote! { #index }
                                } else {
                                    let name = make_rs_ident(name.as_str());
                                    quote! { #name }
                                }
                            };

                            // `offset` and `offset_of_next_field` will be fixed by
                            // FieldsShape::Arbitrary branch below.
                            let offset = 0;
                            let offset_of_next_field = 0;

                            // Populate attributes.
                            let mut attributes = vec![];
                            if let Some(cc_deprecated_tag) =
                                generate_deprecated_tag(tcx, field_def.did)
                            {
                                attributes.push(cc_deprecated_tag);
                            }

                            Field {
                                type_info,
                                cc_name,
                                rs_name,
                                is_public: field_def.vis == ty::Visibility::Public,
                                index,
                                offset,
                                offset_of_next_field,
                                doc_comment: generate_doc_comment(
                                    tcx,
                                    field_def.did.expect_local(),
                                ),
                                attributes,
                            }
                        })
                        .collect_vec()
                })
                .collect_vec();

            for (variant_index, variant_fields) in fields_shape.iter().enumerate() {
                match variant_fields {
                    // Struct/Enum case
                    FieldsShape::Arbitrary { offsets, .. } => {
                        for (index, offset) in offsets.iter().enumerate() {
                            // Documentation of `FieldsShape::Arbitrary says that the offsets are
                            // "ordered to match the source definition order".
                            // We can coorelate them with elements
                            // of the `fields` vector because we've explicitly `sorted_by_key` using
                            // `def_span`.
                            variants_fields[variant_index][index].offset = offset.bytes();

                            if is_supported_enum {
                                // Find the offset for the variant, and take it into
                                // account.
                                variants_fields[variant_index][index].offset -=
                                    tag_size_with_padding;
                            }
                        }
                        // Sort by offset first; ZSTs in the same offset are sorted by source order.
                        // Use `field_size` to ensure ZSTs at the same offset as
                        // non-ZSTs sort first to avoid weird offset issues later on.
                        variants_fields[variant_index].sort_by_key(|field| {
                            let field_size =
                                field.type_info.as_ref().map(|info| info.size).unwrap_or(0);
                            (field.offset, field_size, field.index)
                        });
                    }
                    FieldsShape::Union(num_fields) => {
                        // Compute the offset of each field
                        for index in 0..num_fields.get() {
                            variants_fields[variant_index][index].offset =
                                layout.fields().offset(index).bytes();
                        }
                    }
                    unexpected => panic!("Unexpected FieldsShape: {unexpected:?}"),
                }
            }

            for (variant_index, variant_fields) in variants_fields.iter_mut().enumerate() {
                let next_offsets = variant_fields
                    .iter()
                    .map(|Field { offset, .. }| *offset)
                    .skip(1)
                    .chain(once(layout_size[variant_index]))
                    .collect_vec();
                for (field, next_offset) in variant_fields.iter_mut().zip(next_offsets) {
                    field.offset_of_next_field = next_offset;
                }
            }
            variants_fields
        }
    };

    let cc_details = if variants_fields.is_empty() {
        CcSnippet::default()
    } else {
        let adt_cc_name = &core.cc_short_name;
        let cc_assertions: TokenStream = match adt_def.adt_kind() {
            ty::AdtKind::Struct | ty::AdtKind::Union => {
                variants_fields
                    .iter()
                    .flatten()
                    // TODO(b/298660437): Add support for ZST fields.
                    .filter(|field| field.size() != 0)
                    .map(|Field { cc_name, offset, .. }| {
                        let offset = Literal::u64_unsuffixed(*offset);
                        quote! { static_assert(#offset == offsetof(#adt_cc_name, #cc_name)); }
                    })
                    .collect()
            }
            ty::AdtKind::Enum => {
                // Check if each variant has the tag (and appropriate padding) in the front.
                if !is_supported_enum {
                    variants_fields
                        .iter()
                        .flatten()
                        // TODO(b/298660437): Add support for ZST fields.
                        .filter(|field| field.size() != 0)
                        .map(|Field { cc_name, offset, .. }| {
                            let offset = Literal::u64_unsuffixed(*offset);
                            quote! { static_assert(#offset == offsetof(#adt_cc_name, #cc_name)); }
                        })
                        .collect()
                } else {
                    let variant_offset_assertions: TokenStream = adt_def.variants().iter_enumerated().map(|(variant_index, variant_def)| {
                  let cc_variant_struct_name = format_cc_ident(db, variant_def.ident(tcx).as_str())
                      .unwrap_or_else(|_err| format_ident!("err_field").into_token_stream());
                  let tag_unsuffixed = Literal::u64_unsuffixed(tag_size_with_padding);
                  // If the variant has no fields, don't bother generating any assertions.
                  if variant_sizes[variant_index.index()] == 0  {
                      quote! {}
                  } else {
                      quote! { static_assert(#tag_unsuffixed == offsetof(#adt_cc_name, #cc_variant_struct_name)); }
                  }
              }).collect();
                    // Check for each field's offsets within the variant.
                    let variant_field_assertions: TokenStream = variants_fields
                  .iter().enumerate().flat_map(|(variant_index, fields_for_variant)| {
                      let variant_def = adt_def.variant(VariantIdx::from_usize(variant_index));
                      let cc_variant = variant_def.ident(tcx);
                      let qualified_struct_name =
                          format_cc_ident(db, format!("{}::__crubit_{}_struct", adt_cc_name, cc_variant).as_ref())
                              .unwrap();
                      // If the variant has no fields, don't bother generating any assertions.
                      if variant_def.fields.is_empty() {
                          quote! {}
                      }
                      else {
                          //
                      fields_for_variant.iter().filter(|field| field.type_info.is_ok() && field.size() != 0 ).flat_map(move |Field { cc_name, offset, .. }| {
                      let offset = Literal::u64_unsuffixed(*offset);
                          quote! { static_assert(#offset == offsetof(#qualified_struct_name, #cc_name)); }
                  }).collect()
              }
                  }).collect();
                    quote! {#variant_offset_assertions #variant_field_assertions }
                }
            }
        };

        CcSnippet::with_include(
            quote! {
                inline void #adt_cc_name::__crubit_field_offset_assertions() {
                    #cc_assertions
                }
            },
            CcInclude::cstddef(),
        )
    };

    let rs_details: RsSnippet = if is_supported_enum {
        // Offsets for enums is an experimental feature.
        // TODO(b/355642210): Add these assertions once they're not
        // experiemtnal. let adt_rs_name =
        // &core.rs_fully_qualified_name; variants_fields
        //     .iter()
        //     .enumerate()
        //     .map(|(variant_index, fields)| {
        //         let variant_def =
        // adt_def.variant(VariantIdx::from_usize(variant_index));         let
        // variant_name = make_rs_ident(variant_def.ident(tcx).as_str());
        //         let variant_offset_assertions: TokenStream = fields
        //             .iter()
        //             .map(|Field { rs_name, offset, .. }| {
        //                 let expected_offset =
        // Literal::u64_unsuffixed(*offset);                 let
        // actual_offset =                     quote! {
        // ::core::mem::offset_of!(#adt_rs_name, #variant_name.#rs_name)
        // };                 quote! { const _: () =
        // assert!(#actual_offset == #expected_offset); }             })
        //             .collect();
        //         variant_offset_assertions
        //     })
        //     .collect()
        RsSnippet::default()
    } else {
        let adt_rs_name = &core.rs_fully_qualified_name;
        variants_fields
            .iter()
            .flatten()
            // TODO(b/298660437): Even though we don't generate bindings for ZST fields,
            // we'd still like to make sure we computed the offset of
            // ZST fields correctly on the Rust side, so we still emit
            // offset assertions for ZST fields here. TODO(b/298660437):
            // Remove the comment above when ZST fields are supported.
            .filter(|field| field.is_public)
            .map(|Field { rs_name, offset, .. }| {
                let expected_offset = Literal::u64_unsuffixed(*offset);
                let actual_offset = quote! { ::core::mem::offset_of!(#adt_rs_name, #rs_name) };
                RsSnippet::new(
                    quote! { const _: () = assert!(#actual_offset == #expected_offset); },
                )
            })
            .collect()
    };
    let main_api = {
        let assertions_method_decl = if variants_fields.is_empty() {
            quote! {}
        } else {
            // We put the assertions in a method so that they can read private member
            // variables.
            quote! { private: static void __crubit_field_offset_assertions(); }
        };

        // If all fields are known, and the type is repr(C), then we don't need padding
        // fields, and can instead use the natural padding from alignment.
        //
        // Note: it does need to be repr(C) to be guaranteed, since the compiler might
        // reasonably place a field later than it has to for layout
        // randomization purposes. For example, in `#[repr(align(4))] struct
        // Foo(i8);` there are four different places the `i8` could be.
        // If it was placed in the second byte, for any reason, then we would need
        // explicit padding bytes.
        let always_omit_padding = repr_attrs.contains(&rustc_attr_data_structures::ReprC)
            && variants_fields.iter().flatten().all(|field| field.type_info.is_ok());

        let mut prereqs = CcPrerequisites::default();
        // Takes a field and converts it to a token stream.
        let get_field_tokens = |field: Field, prereqs: &mut CcPrerequisites| -> TokenStream {
            let cc_name = &field.cc_name;
            match field.type_info {
                Err(ref err) => {
                    let size = field.size();
                    let msg = format!("Field type has been replaced with a blob of bytes: {err:#}");

                    // Empty arrays are ill-formed, but also unnecessary for padding.
                    if size > 0 {
                        let size = Literal::u64_unsuffixed(size);
                        quote! {
                            private: __NEWLINE__
                                __COMMENT__ #msg
                                unsigned char #cc_name[#size];
                        }
                    } else {
                        // TODO(b/258259459): Generate bindings for ZST fields.
                        let msg = format!(
                            "Skipped bindings for field `{cc_name}`: \
                             ZST fields are not supported (b/258259459)"
                        );
                        quote! {__NEWLINE__ __COMMENT__ #msg}
                    }
                }
                Ok(FieldTypeInfo { cpp_type, size }) => {
                    let padding = match adt_def.adt_kind() {
                        ty::AdtKind::Struct | ty::AdtKind::Enum => {
                            assert!((field.offset + size) <= field.offset_of_next_field);
                            field.offset_of_next_field - field.offset - size
                        }
                        ty::AdtKind::Union => field.offset,
                    };

                    // Omit explicit padding if:
                    //   1. The type is repr(C) and has known types for all fields, so we can reuse
                    //      the natural repr(C) padding.
                    //   2. There is no padding
                    // TODO(jeanpierreda): also omit padding for the final field?
                    let padding = if always_omit_padding || padding == 0 {
                        quote! {}
                    } else {
                        let padding = Literal::u64_unsuffixed(padding);
                        let ident = format_ident!("__padding{}", field.index);
                        quote! { private: unsigned char #ident[#padding]; }
                    };
                    let visibility = if field.is_public {
                        quote! { public: }
                    } else {
                        quote! { private: }
                    };
                    let cpp_type = cpp_type.into_tokens(prereqs);
                    let doc_comment = field.doc_comment;
                    let attributes = field.attributes;

                    match adt_def.adt_kind() {
                        ty::AdtKind::Struct => quote! {
                            #visibility __NEWLINE__
                                // The anonymous union gives more control over when exactly
                                // the field constructors and destructors run.  See also
                                // b/288138612.
                                union {  __NEWLINE__
                                    #doc_comment
                                    #(#attributes)*
                                    #cpp_type #cc_name;
                                };
                            #padding
                        },
                        ty::AdtKind::Union => {
                            if repr_attrs.contains(&rustc_attr_data_structures::ReprC) {
                                quote! {
                                    #visibility __NEWLINE__
                                    #doc_comment
                                    #cpp_type #cc_name;
                                }
                            } else {
                                let internal_padding = if field.offset == 0 {
                                    quote! {}
                                } else {
                                    let internal_padding_size =
                                        Literal::u64_unsuffixed(field.offset);
                                    quote! {char __crubit_internal_padding[#internal_padding_size]}
                                };
                                quote! {
                                    #visibility __NEWLINE__
                                    #doc_comment
                                    struct {
                                        #internal_padding
                                        #cpp_type value;
                                    } #cc_name;
                                }
                            }
                        }
                        ty::AdtKind::Enum => {
                            quote! {
                                #visibility __NEWLINE__ #cpp_type #cc_name;
                            }
                        }
                    }
                }
            }
        };

        // For structs and unions, we can just flatten the fields variant. For enums, we
        // need to handle each variant separately.
        let fields = match adt_def.adt_kind() {
            ty::AdtKind::Struct | ty::AdtKind::Union => variants_fields
                .into_iter()
                .flatten()
                .map(|field| get_field_tokens(field, &mut prereqs))
                .collect(),
            ty::AdtKind::Enum if !is_supported_enum => variants_fields
                .into_iter()
                .flatten()
                .map(|field| get_field_tokens(field, &mut prereqs))
                .collect(),
            ty::AdtKind::Enum => {
                // We need three things:
                // 1. A representation of the tag (tag_enum).
                // 2. A representation of the fields in each variant (variant_structs).
                // 3. A union of the results of (2) (variants_union).

                // Step 1 is ignored if there is only one variant.

                // See https://doc.rust-lang.org/reference/type-layout.html#reprc-enums-with-fields

                // Get tokens for the tag, if it exists.
                let tag_enum = match layout_variants {
                    Variants::Single { .. } | Variants::Empty => quote! {},
                    Variants::Multiple { tag, .. } => {
                        let tag_ty = get_scalar_int_type(db.tcx(), *tag);

                        let tag_tokens = format_ty_for_cc(
                            db,
                            // An enum cannot have repr(c_char), or any other alias, so there's
                            // never sugar.
                            SugaredTy::new(tag_ty, None),
                            TypeLocation::Other,
                        )
                        .expect("discriminant should be a integer type.")
                        .into_tokens(&mut prereqs);

                        let variant_enum_fields: TokenStream = adt_def
                            .variants()
                            .iter_enumerated()
                            .map(|(variant_index, variant_def)| {
                                let cc_variant_name =
                                    format_cc_ident(db, variant_def.name.as_str()).unwrap_or_else(
                                        |_err| format_ident!("err_field").into_token_stream(),
                                    );
                                let tag_value = Literal::u128_unsuffixed(
                                    adt_def.discriminant_for_variant(tcx, variant_index).val,
                                );
                                quote! {
                                    __NEWLINE__ #cc_variant_name = #tag_value,
                                }
                            })
                            .collect();
                        quote! {
                            __NEWLINE__ enum class Tag : #tag_tokens {
                                #variant_enum_fields
                            }; __NEWLINE__
                        }
                    }
                };

                let mut tokens_per_variant: Vec<TokenStream> =
                    Vec::with_capacity(variants_fields.len());

                for fields_for_variant in variants_fields.into_iter() {
                    tokens_per_variant.push(
                        fields_for_variant
                            .into_iter()
                            .map(|field| get_field_tokens(field, &mut prereqs))
                            .collect(),
                    );
                }

                // We need to get the alignment of each variant struct.
                let variant_alignments = match layout_variants {
                    Variants::Multiple { tag: _, tag_encoding: _, tag_field: _, variants } => {
                        variants
                            .iter()
                            .map(|layout| layout.align.abi.bytes() - tag_size_with_padding)
                            .collect_vec()
                    }
                    Variants::Single { .. } | Variants::Empty => {
                        vec![core.alignment_in_bytes]
                    }
                };

                let variant_structs: TokenStream = adt_def
                    .variants()
                    .iter_enumerated()
                    .map(|(variant_index, variant_def)| {
                        // Get the variant name.
                        let cc_variant_struct_name = format_cc_ident(
                            db,
                            format!("__crubit_{}_struct", variant_def.ident(tcx).as_str()).as_ref(),
                        )
                        .unwrap_or_else(|_err| format_ident!("err_struct").into_token_stream());

                        // Get the corresponding field tokens.
                        let fields_for_variant = &tokens_per_variant[variant_index.index()];

                        // Get the aligment of the variant...
                        let variant_alignment =
                            Literal::u64_unsuffixed(variant_alignments[variant_index.index()]);

                        // Create the actual struct, if the variant has size. Otherwise, make
                        // a note that the variant is empty.
                        if variant_sizes[variant_index.index()] == 0 {
                            let cc_variant_name = format_cc_ident(db, variant_def.name.as_str())
                                .unwrap_or_else(|_err| {
                                    format_ident!("err_field").into_token_stream()
                                });
                            let msg = format!(
                                "Variant {} has no size, so no struct is generated.",
                                cc_variant_name
                            );
                            quote! {__NEWLINE__
                            __COMMENT__ #msg}
                        } else {
                            quote! {
                                __NEWLINE__
                                struct alignas(#variant_alignment) #cc_variant_struct_name {
                                    #fields_for_variant
                                };
                            }
                        }
                    })
                    .collect();

                let variants_union_fields: TokenStream = adt_def
                    .variants()
                    .iter_enumerated()
                    .map(|(variant_index, variant_def)| {
                        // Get the variant name.
                        let cc_variant_name = format_cc_ident(db, variant_def.name.as_str())
                            .unwrap_or_else(|_err| format_ident!("err_field").into_token_stream());
                        let cc_variant_struct_type = format_cc_ident(
                            db,
                            format!("__crubit_{}_struct", variant_def.ident(tcx).as_str()).as_ref(),
                        )
                        .unwrap_or_else(|_err| format_ident!("err_struct").into_token_stream());

                        // If the variant has no fields (i.e. the struct is empty), we can skip
                        // this declaration.
                        if variant_sizes[variant_index.index()] == 0 {
                            quote! {}
                        } else {
                            quote! {
                                #cc_variant_struct_type #cc_variant_name; __NEWLINE__
                            }
                        }
                    })
                    .collect();

                let variants_union: TokenStream = {
                    let has_no_fields =
                        variant_sizes.iter().all(|size_of_variant| *size_of_variant == 0);

                    if has_no_fields {
                        // If there are no fields in any variant, we must skip this union
                        quote! {}
                    } else {
                        quote! {
                            public: union  {
                                #variants_union_fields
                            };
                        }
                    }
                };

                // Combine everything together.
                quote! {
                    #variant_structs __NEWLINE__
                    #tag_enum __NEWLINE__
                    public: Tag tag; __NEWLINE__
                    #variants_union
                }
            }
        };

        CcSnippet {
            prereqs,
            tokens: quote! {
                #fields
                #assertions_method_decl
            },
        }
    };

    ApiSnippets { main_api, cc_details, rs_details }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::*;
    use quote::quote;
    use token_stream_matchers::{assert_cc_matches, assert_rs_matches};

    /// The `test_generated_bindings_struct` test covers only a single example
    /// of an ADT (struct/enum/union) that should get a C++ binding.
    /// Additional coverage of how items are formatted is provided by
    /// `test_format_item_..._struct_...`, `test_format_item_..._enum_...`,
    /// and `test_format_item_..._union_...` tests.
    ///
    /// We don't want to duplicate coverage already provided by
    /// `test_format_item_struct_with_fields`, but we do want to verify that
    /// * `format_crate` will actually find and process the struct
    ///   (`test_format_item_...` doesn't cover this aspect - it uses a
    ///   test-only `find_def_id_by_name` instead)
    /// * The actual shape of the bindings still looks okay at this level.
    #[test]
    fn test_generated_bindings_struct() {
        let test_src = r#"
                pub struct Point {
                    pub x: i32,
                    pub y: i32,
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    namespace rust_out {
                        ...
                        struct CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: Point") alignas(4) [[clang::trivial_abi]] Point final {
                            // No point replicating test coverage of
                            // `test_format_item_struct_with_fields`.
                            ...
                        };
                        static_assert(sizeof(Point) == 8, ...);
                        static_assert(alignof(Point) == 4, ...);
                        ... // Other static_asserts are covered by
                            // `test_format_item_struct_with_fields`
                    }  // namespace rust_out
                }
            );
            assert_rs_matches!(
                bindings.cc_api_impl,
                quote! {
                    // No point replicating test coverage of
                    // `test_format_item_struct_with_fields`.
                    const _: () = assert!(::std::mem::size_of::<::rust_out::Point>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::Point>() == 4);
                    const _: () = assert!(::core::mem::offset_of!(::rust_out::Point, x) == 0);
                    const _: () = assert!(::core::mem::offset_of!(::rust_out::Point, y) == 4);
                }
            );
        });
    }

    #[test]
    fn test_format_bridged_type_in_generic_types() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(
                  cpp_type="cpp_ns::CppType",
                  cpp_type_include="cpp_ns/cpp_type.h",
                  rust_to_cpp_converter="convert_rust_to_cpp_type",
                  cpp_to_rust_converter="convert_cpp_to_rust_type",
                )]
                pub struct RustType {
                    pub x: i32,
                }

                #[unsafe(no_mangle)]
                pub fn foo(_: Box<RustType>) {}

                #[unsafe(no_mangle)]
                pub fn bar(_: Option<Box<Result<RustType, ()>>>) {}
        "#;
        test_format_item(test_src, "foo", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error handling parameter #0: Can't format ADT as it has a generic type \
                    `RustType` that is a bridged type"
            );
        });

        test_format_item(test_src, "bar", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error handling parameter #0: Can't format ADT as it has a generic type \
                    `RustType` that is a bridged type"
            );
        });
    }

    #[test]
    fn test_format_struct_cpp_name() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(cpp_name="Bar")]
                pub struct Foo {
                    pub x: i32,
                }
            "#;
        test_format_item(test_src, "Foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());

            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::Foo>() == 4);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::Foo>() == 4);
                    const _: () = assert!(::core::mem::offset_of!(::rust_out::Foo, x) == 0);
                }
            );

            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: Foo") alignas(4)
                    [[clang::trivial_abi]] Bar final
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_type_generic_struct() {
        let test_src = r#"
                pub struct Point<T> {
                    pub x: T,
                    pub y: T,
                }
            "#;
        test_format_item(test_src, "Point", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic types are not supported yet (b/259749095)");
        });
    }

    #[test]
    fn test_format_item_unsupported_lifetime_generic_struct() {
        let test_src = r#"
                pub struct Point<'a> {
                    pub x: &'a i32,
                    pub y: &'a i32,
                }

                impl<'a> Point<'a> {
                    // Some lifetimes are bound at the `impl` / `struct` level (the lifetime is
                    // hidden underneath the `Self` type), and some at the `fn` level.
                    pub fn new<'b, 'c>(_x: &'b i32, _y: &'c i32) -> Self { unimplemented!() }
                }
            "#;
        test_format_item(test_src, "Point", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic types are not supported yet (b/259749095)");
        });
    }

    #[test]
    fn test_format_item_unsupported_generic_enum() {
        let test_src = r#"
                pub enum Point<T> {
                    Cartesian{x: T, y: T},
                    Polar{angle: T, dist: T},
                }
            "#;
        test_format_item(test_src, "Point", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic types are not supported yet (b/259749095)");
        });
    }

    #[test]
    fn test_format_item_unsupported_generic_union() {
        let test_src = r#"
                pub union SomeUnion<T> {
                    pub x: std::mem::ManuallyDrop<T>,
                    pub y: i32,
                }
            "#;
        test_format_item(test_src, "SomeUnion", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic types are not supported yet (b/259749095)");
        });
    }

    /// This is a test for a regular struct - a struct with named fields.
    /// https://doc.rust-lang.org/reference/items/structs.html refers to this kind of struct as
    /// `StructStruct` or "nominal struct type".
    #[test]
    fn test_format_item_struct_with_fields() {
        let test_src = r#"
                pub struct SomeStruct {
                    pub x: i32,
                    pub y: i32,
                }

                const _: () = assert!(std::mem::size_of::<SomeStruct>() == 8);
                const _: () = assert!(std::mem::align_of::<SomeStruct>() == 4);
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeStruct final {
                        public:
                            __COMMENT__ "`SomeStruct` doesn't implement the `Default` trait"
                            SomeStruct() = delete;

                            __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                            ~SomeStruct() = default;
                            SomeStruct(SomeStruct&&) = default;
                            SomeStruct& operator=(SomeStruct&&) = default;

                            __COMMENT__ "`SomeStruct` doesn't implement the `Clone` trait"
                            SomeStruct(const SomeStruct&) = delete;
                            SomeStruct& operator=(const SomeStruct&) = delete;
                        public: union { ... std::int32_t x; };
                        public: union { ... std::int32_t y; };
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(SomeStruct) == 8, ...);
                    static_assert(alignof(SomeStruct) == 4, ...);
                    static_assert(std::is_trivially_destructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                    inline void SomeStruct::__crubit_field_offset_assertions() {
                      static_assert(0 == offsetof(SomeStruct, x));
                      static_assert(4 == offsetof(SomeStruct, y));
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, x) == 0);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, y) == 4);
                }
            );
        });
    }

    /// This is a test for `TupleStruct` or "tuple struct" - for more details
    /// please refer to https://doc.rust-lang.org/reference/items/structs.html
    #[test]
    fn test_format_item_struct_with_tuple() {
        let test_src = r#"
                pub struct TupleStruct(pub i32, pub i32);
                const _: () = assert!(std::mem::size_of::<TupleStruct>() == 8);
                const _: () = assert!(std::mem::align_of::<TupleStruct>() == 4);
            "#;
        test_format_item(test_src, "TupleStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] TupleStruct final {
                        public:
                            __COMMENT__ "`TupleStruct` doesn't implement the `Default` trait"
                            TupleStruct() = delete;

                            __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                            ~TupleStruct() = default;
                            TupleStruct(TupleStruct&&) = default;
                            TupleStruct& operator=(TupleStruct&&) = default;

                            __COMMENT__ "`TupleStruct` doesn't implement the `Clone` trait"
                            TupleStruct(const TupleStruct&) = delete;
                            TupleStruct& operator=(const TupleStruct&) = delete;
                        public: union { ... std::int32_t __field0; };
                        public: union { ... std::int32_t __field1; };
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(TupleStruct) == 8, ...);
                    static_assert(alignof(TupleStruct) == 4, ...);
                    static_assert(std::is_trivially_destructible_v<TupleStruct>);
                    static_assert(std::is_trivially_move_constructible_v<TupleStruct>);
                    static_assert(std::is_trivially_move_assignable_v<TupleStruct>);
                    inline void TupleStruct::__crubit_field_offset_assertions() {
                      static_assert(0 == offsetof(TupleStruct, __field0));
                      static_assert(4 == offsetof(TupleStruct, __field1));
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::TupleStruct>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::TupleStruct>() == 4);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::TupleStruct, 0) == 0);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::TupleStruct, 1) == 4);
                }
            );
        });
    }

    /// This test the scenario where Rust lays out field in a different order
    /// than the source order.
    #[test]
    fn test_format_item_struct_with_reordered_field_offsets() {
        let test_src = r#"
                pub struct SomeStruct {
                    pub field1: i16,
                    pub field2: i32,
                    pub field3: i16,
                }

                const _: () = assert!(std::mem::size_of::<SomeStruct>() == 8);
                const _: () = assert!(std::mem::align_of::<SomeStruct>() == 4);
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeStruct final {
                        ...
                        // The particular order below is not guaranteed,
                        // so we may need to adjust this test assertion
                        // (if Rust changes how it lays out the fields).
                        public: union { ... std::int32_t field2; };
                        public: union { ... std::int16_t field1; };
                        public: union { ... std::int16_t field3; };
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(SomeStruct) == 8, ...);
                    static_assert(alignof(SomeStruct) == 4, ...);
                    static_assert(std::is_trivially_destructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                    inline void SomeStruct::__crubit_field_offset_assertions() {
                      static_assert(0 == offsetof(SomeStruct, field2));
                      static_assert(4 == offsetof(SomeStruct, field1));
                      static_assert(6 == offsetof(SomeStruct, field3));
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, field2)
                                           == 0);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, field1)
                                           == 4);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, field3)
                                           == 6);
                }
            );
        });
    }

    #[test]
    fn test_format_item_struct_with_packed_layout() {
        let test_src = r#"
                #[repr(packed(1))]
                pub struct SomeStruct {
                    pub field1: u16,
                    pub field2: u32,
                }
                const _: () = assert!(::std::mem::size_of::<SomeStruct>() == 6);
                const _: () = assert!(::std::mem::align_of::<SomeStruct>() == 1);
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(1) [[clang::trivial_abi]] __attribute__((packed)) SomeStruct final {
                        ...
                        public: union { ... std::uint16_t field1; };
                        public: union { ... std::uint32_t field2; };
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(SomeStruct) == 6, ...);
                    static_assert(alignof(SomeStruct) == 1, ...);
                    static_assert(std::is_trivially_destructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                    inline void SomeStruct::__crubit_field_offset_assertions() {
                      static_assert(0 == offsetof(SomeStruct, field1));
                      static_assert(2 == offsetof(SomeStruct, field2));
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 6);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 1);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, field1)
                                           == 0);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, field2)
                                           == 2);
                }
            );
        });
    }

    #[test]
    fn test_format_item_struct_with_explicit_padding_in_generated_code() {
        let test_src = r#"
                pub struct SomeStruct {
                    pub f1: u8,
                    pub f2: u32,
                }
                const _: () = assert!(::std::mem::size_of::<SomeStruct>() == 8);
                const _: () = assert!(::std::mem::align_of::<SomeStruct>() == 4);
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeStruct final {
                        ...
                        public: union { ... std::uint32_t f2; };
                        public: union { ... std::uint8_t f1; };
                        private: unsigned char __padding0[3];
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(SomeStruct) == 8, ...);
                    static_assert(alignof(SomeStruct) == 4, ...);
                    static_assert(std::is_trivially_destructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                    inline void SomeStruct::__crubit_field_offset_assertions() {
                      static_assert(0 == offsetof(SomeStruct, f2));
                      static_assert(4 == offsetof(SomeStruct, f1));
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, f2) == 0);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, f1) == 4);
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_struct_with_name_that_is_reserved_keyword() {
        let test_src = r#"
                #[allow(non_camel_case_types)]
                pub struct reinterpret_cast {
                    pub x: i32,
                    pub y: i32,
                }
            "#;
        test_format_item(test_src, "reinterpret_cast", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... reinterpret_cast_ final
                }
            );
        });
    }

    #[test]
    fn test_format_item_struct_with_unsupported_field_type() {
        let test_src = r#"
                pub struct SomeStruct {
                    pub successful_field: i32,
                    pub unsupported_field: Option<[i32; 3]>,
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let broken_field_msg = "Field type has been replaced with a blob of bytes: \
                                    Generic types are not supported yet (b/259749095)";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        private:
                            __COMMENT__ #broken_field_msg
                            unsigned char unsupported_field[16];
                        public:
                            union { ... std::int32_t successful_field; };
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                    ...
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(SomeStruct) == 20, ...);
                    static_assert(alignof(SomeStruct) == 4, ...);
                    static_assert(std::is_trivially_destructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                    inline void SomeStruct::__crubit_field_offset_assertions() {
                      static_assert(0 == offsetof(SomeStruct, unsupported_field));
                      static_assert(16 == offsetof(SomeStruct, successful_field));
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 20);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct,
                                                                 unsupported_field) == 0);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct,
                                                                 successful_field) == 16);
                }
            );
        });
    }

    /// This test verifies how reference type fields are represented in the
    /// generated bindings.  See b/286256327.
    ///
    /// In some of the past discussions we tentatively decided that the
    /// generated bindings shouldn't use C++ references in fields - instead
    /// a C++ pointer should be used.  One reason is that C++ references
    /// cannot be assigned to (i.e. rebound), and therefore C++ pointers
    /// more accurately represent the semantics of Rust fields.  The pointer
    /// type should probably use some form of C++ annotations to mark it as
    /// non-nullable.
    #[test]
    fn test_format_item_struct_with_unsupported_field_of_reference_type() {
        let test_src = r#"
                // `'static` lifetime can be used in a non-generic struct - this let's us
                // test reference fieles without requiring support for generic structs.
                pub struct NonGenericSomeStruct {
                    pub reference_field: &'static i32,
                }
            "#;
        test_format_item(test_src, "NonGenericSomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let broken_field_msg = "Field type has been replaced with a blob of bytes: \
                                    Can't format `&'static i32`, because references \
                                    are only supported in function parameter types and \
                                    return types (b/286256327)";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    private:
                        __COMMENT__ #broken_field_msg
                        unsigned char reference_field[8];
                    ...
                }
            );
        });
    }

    /// This test verifies that `generate_trait_thunks(..., drop_trait_id,
    /// ...).expect(...)` won't panic - the `generate_adt_core` needs to
    /// verify that formatting of the fully qualified C++ name of the struct
    /// works fine.
    #[test]
    fn test_format_item_unsupported_struct_with_custom_drop_impl_in_reserved_name_module() {
        let test_src = r#"
                // This mimics the name of a public module used by
                // `icu_locid` in `extensions/mod.rs`.
                pub mod private {
                    #[derive(Default)]
                    pub struct SomeStruct {
                        pub x: i32,
                        pub y: i32,
                    }

                    impl Drop for SomeStruct {
                        fn drop(&mut self) {}
                    }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let cc_details = &result.cc_details;
            assert_cc_matches!(
                cc_details.tokens,
                quote! {
                    ::rust_out::private_::SomeStruct
                }
            );
        });
    }

    /// This test covers how ZSTs (zero-sized-types) are handled.
    /// https://doc.rust-lang.org/reference/items/structs.html refers to this kind of struct as a
    /// "unit-like struct".
    #[test]
    fn test_format_item_unsupported_struct_zero_sized_type_with_no_fields() {
        let test_src = r#"
                pub struct ZeroSizedType1;
                pub struct ZeroSizedType2();
                pub struct ZeroSizedType3{}
            "#;
        for name in ["ZeroSizedType1", "ZeroSizedType2", "ZeroSizedType3"] {
            test_format_item(test_src, name, |result| {
                let err = result.unwrap_err();
                assert_eq!(err, "Zero-sized types (ZSTs) are not supported (b/258259459)");
            });
        }
    }

    #[test]
    fn test_format_item_unsupported_struct_with_only_zero_sized_type_fields() {
        let test_src = r#"
                pub struct ZeroSizedType;
                pub struct SomeStruct {
                    pub zst1: ZeroSizedType,
                    pub zst2: ZeroSizedType,
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Zero-sized types (ZSTs) are not supported (b/258259459)",);
        });
    }

    #[test]
    fn test_format_item_unsupported_struct_with_some_zero_sized_type_fields() {
        let test_src = r#"
                pub struct ZeroSizedType;
                pub struct SomeStruct {
                    pub zst1: ZeroSizedType,
                    pub successful_field: i32,
                    pub zst2: ZeroSizedType,
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let broken_field_msg_zst1 =
                "Skipped bindings for field `zst1`: ZST fields are not supported (b/258259459)";
            let broken_field_msg_zst2 =
                "Skipped bindings for field `zst2`: ZST fields are not supported (b/258259459)";

            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        public:
                            union { ... std::int32_t successful_field; };
                        __COMMENT__ #broken_field_msg_zst1
                        __COMMENT__ #broken_field_msg_zst2
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                    ...
                }
            );

            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(SomeStruct) == 4, ...);
                    static_assert(alignof(SomeStruct) == 4, ...);
                    static_assert(std::is_trivially_destructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                    static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                    inline void SomeStruct::__crubit_field_offset_assertions() {
                    static_assert(0 == offsetof(SomeStruct, successful_field));
                    }
                }
            );

            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 4);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, successful_field) == 0);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, zst1) == 4);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, zst2) == 4);

                }
            );
        });
    }

    #[test]
    fn test_format_item_struct_with_dynamically_sized_field() {
        let test_src = r#"
                #![allow(dead_code)]
                pub struct DynamicallySizedStruct {
                    /// Having a non-ZST field avoids hitting the following error:
                    /// "Zero-sized types (ZSTs) are not supported (b/258259459)"
                    _non_zst_field: f32,
                    _dynamically_sized_field: [i32],
                }
            "#;
        test_format_item(test_src, "DynamicallySizedStruct", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Bindings for dynamically sized types are not supported.");
        });
    }

    #[test]
    fn test_format_item_struct_fields_with_doc_comments() {
        let test_src = r#"
                pub struct SomeStruct {
                    /// Documentation of `successful_field`.
                    pub successful_field: i32,

                    /// Documentation of `unsupported_field`.
                    pub unsupported_field: Option<[i32; 3]>,
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let comment_for_successful_field = " Documentation of `successful_field`.\n\n\
                  Generated from: <crubit_unittests.rs>;l=4";
            let comment_for_unsupported_field =
                "Field type has been replaced with a blob of bytes: \
                 Generic types are not supported yet (b/259749095)";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        private:
                            __COMMENT__ #comment_for_unsupported_field
                            unsigned char unsupported_field[16];
                        public:
                            union {
                                __COMMENT__ #comment_for_successful_field
                                std::int32_t successful_field;
                            };
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                    ...
                }
            );
        });
    }

    /// This is a test for an enum that only has `EnumItemDiscriminant` items
    /// (and doesn't have `EnumItemTuple` or `EnumItemStruct` items).  See
    /// also https://doc.rust-lang.org/reference/items/enumerations.html
    #[test]
    fn test_format_item_enum_with_only_discriminant_items() {
        let test_src = r#"
                pub enum SomeEnum {
                    Red,
                    Green = 123,
                    Blue,
                }

                const _: () = assert!(std::mem::size_of::<SomeEnum>() == 1);
                const _: () = assert!(std::mem::align_of::<SomeEnum>() == 1);
            "#;
        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let no_fields_msg = "Field type has been replaced with a blob of bytes: \
                                 No support for bindings of individual non-repr(C) `enum`s";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(1) [[clang::trivial_abi]] SomeEnum final {
                        public:
                            __COMMENT__ "`SomeEnum` doesn't implement the `Default` trait"
                            SomeEnum() = delete;

                            __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                            ~SomeEnum() = default;
                            SomeEnum(SomeEnum&&) = default;
                            SomeEnum& operator=(SomeEnum&&) = default;

                            __COMMENT__ "`SomeEnum` doesn't implement the `Clone` trait"
                            SomeEnum(const SomeEnum&) = delete;
                            SomeEnum& operator=(const SomeEnum&) = delete;
                        private:
                            __COMMENT__ #no_fields_msg
                            unsigned char __opaque_blob_of_bytes[1];
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(SomeEnum) == 1, ...);
                    static_assert(alignof(SomeEnum) == 1, ...);
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeEnum>() == 1);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeEnum>() == 1);
                }
            );
        });
    }

    /// This is a test for an enum that has `EnumItemTuple` and `EnumItemStruct`
    /// items. See also https://doc.rust-lang.org/reference/items/enumerations.html
    #[test]
    fn test_format_item_enum_with_tuple_and_struct_items() {
        let test_src = r#"
                pub enum Point {
                    Cartesian(f32, f32),
                    Polar{ dist: f32, angle: f32 },
                }

                const _: () = assert!(std::mem::size_of::<Point>() == 12);
                const _: () = assert!(std::mem::align_of::<Point>() == 4);
            "#;
        test_format_item(test_src, "Point", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let no_fields_msg = "Field type has been replaced with a blob of bytes: \
                                 No support for bindings of individual non-repr(C) `enum`s";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] Point final {
                        public:
                            __COMMENT__ "`Point` doesn't implement the `Default` trait"
                            Point() = delete;

                            __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                            ~Point() = default;
                            Point(Point&&) = default;
                            Point& operator=(Point&&) = default;

                            __COMMENT__ "`Point` doesn't implement the `Clone` trait"
                            Point(const Point&) = delete;
                            Point& operator=(const Point&) = delete;
                        private:
                            __COMMENT__ #no_fields_msg
                            unsigned char __opaque_blob_of_bytes[12];
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(Point) == 12, ...);
                    static_assert(alignof(Point) == 4, ...);
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::Point>() == 12);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::Point>() == 4);
                }
            );
        });
    }

    /// This test covers how zero-variant enums are handled.  See also
    /// https://doc.rust-lang.org/reference/items/enumerations.html#zero-variant-enums
    #[test]
    fn test_format_item_unsupported_enum_zero_variants() {
        let test_src = r#"
                pub enum ZeroVariantEnum {}
            "#;
        test_format_item(test_src, "ZeroVariantEnum", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Zero-sized types (ZSTs) are not supported (b/258259459)");
        });
    }

    /// This is a test for a `union`.  See also
    /// https://doc.rust-lang.org/reference/items/unions.html
    #[test]
    fn test_format_item_union() {
        let test_src = r#"
                pub union SomeUnion {
                    pub i: i32,
                    pub f: f64,
                }

                const _: () = assert!(std::mem::size_of::<SomeUnion>() == 8);
                const _: () = assert!(std::mem::align_of::<SomeUnion>() == 8);
            "#;
        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    union CRUBIT_INTERNAL_RUST_TYPE(...) alignas(8) [[clang::trivial_abi]] SomeUnion final {
                        public:
                            __COMMENT__ "`SomeUnion` doesn't implement the `Default` trait"
                            SomeUnion() = delete;

                            __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                            ~SomeUnion() = default;
                            SomeUnion(SomeUnion&&) = default;
                            SomeUnion& operator=(SomeUnion&&) = default;

                            __COMMENT__ "`SomeUnion` doesn't implement the `Clone` trait"
                            SomeUnion(const SomeUnion&) = delete;
                            SomeUnion& operator=(const SomeUnion&) = delete;
                        ...
                        struct {
                            ...
                            std::int32_t value;
                        } i;
                        ...
                        struct {
                            ...
                            double value;
                        } f;
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(SomeUnion) == 8, ...);
                    static_assert(alignof(SomeUnion) == 8, ...);
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeUnion>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeUnion>() == 8);
                }
            );
        });
    }

    #[test]
    fn test_format_item_doc_comments_union() {
        let test_src = r#"
            /// Doc for some union.
            pub union SomeUnionWithDocs {
                /// Doc for a field in a union.
                pub i: i32,
                pub f: f64
            }
        "#;
        test_format_item(test_src, "SomeUnionWithDocs", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let comment = " Doc for some union.\n\n\
                           Generated from: <crubit_unittests.rs>;l=3";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    union ... SomeUnionWithDocs final {
                        ...
                    }
                    ...
                }
            );
        });
    }

    #[test]
    fn test_format_item_doc_comments_enum() {
        let test_src = r#"
            /** Doc for some enum. */
            pub enum SomeEnumWithDocs {
                Kind1(i32),
            }
        "#;
        test_format_item(test_src, "SomeEnumWithDocs", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let comment = " Doc for some enum. \n\n\
                            Generated from: <crubit_unittests.rs>;l=3";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    struct ... SomeEnumWithDocs final {
                        ...
                    }
                    ...
                }
            );
        });
    }

    #[test]
    fn test_format_item_doc_comments_struct() {
        let test_src = r#"
            #![allow(dead_code)]
            #[doc = "Doc for some struct."]
            pub struct SomeStructWithDocs {
                #[doc = "Doc for first field."]
                some_field : i32,
            }
        "#;
        test_format_item(test_src, "SomeStructWithDocs", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let comment = "Doc for some struct.\n\n\
                           Generated from: <crubit_unittests.rs>;l=4";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    struct ... SomeStructWithDocs final {
                        ...
                    }
                    ...
                }
            );
        });
    }

    #[test]
    fn test_format_item_doc_comments_tuple_struct() {
        let test_src = r#"
            #![allow(dead_code)]

            /// Doc for some tuple struct.
            pub struct SomeTupleStructWithDocs(i32);
        "#;
        test_format_item(test_src, "SomeTupleStructWithDocs", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let comment = " Doc for some tuple struct.\n\n\
                           Generated from: <crubit_unittests.rs>;l=5";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    struct ... SomeTupleStructWithDocs final {
                        ...
                    }
                    ...
                },
            );
        });
    }

    #[test]
    fn test_repr_c_enum_fields() {
        let test_src = r#"
        #[repr(C, i32)]
        pub enum SomeEnum {
            A(i32),
            B{x: u32},
            C,
            D{foo: i32, bar: i32} = 3,
        }

        const _: () = assert!(std::mem::size_of::<SomeEnum>() == 12);
        const _: () = assert!(std::mem::align_of::<SomeEnum>() == 4);
        "#;

        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) ... [[clang::trivial_abi]] SomeEnum final {
                        public:
                            ...
                            __COMMENT__ "`SomeEnum` doesn't implement the `Default` trait"
                            SomeEnum() = delete;
                            ...
                            __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                            ~SomeEnum() = default;
                            SomeEnum(SomeEnum&&) = default;
                            SomeEnum& operator=(SomeEnum&&) = default;

                            __COMMENT__ "`SomeEnum` doesn't implement the `Clone` trait"
                            SomeEnum(const SomeEnum&) = delete;
                            SomeEnum& operator=(const SomeEnum&) = delete;
                            ...
                            struct alignas(...) __crubit_A_struct {
                                public:
                                    std::int32_t __field0;
                            };
                            ...
                            struct alignas(...) __crubit_B_struct {
                                public:
                                    std::uint32_t x;
                            };
                            ...
                            __COMMENT__ "Variant C has no size, so no struct is generated."
                            ...
                            struct alignas(...) __crubit_D_struct {
                                public:
                                    std::int32_t foo;
                                public:
                                    std::int32_t bar;
                            };
                            ...
                            enum class Tag : std::int32_t {
                                A = 0,
                                B = 1,
                                C = 2,
                                D = 3,
                            };
                            ...
                            public:
                                Tag tag;
                            ...
                            public:
                                union {
                                    __crubit_A_struct A;
                                    __crubit_B_struct B;
                                    __crubit_D_struct D;
                                };
                            ...
                        ...
                    };
                }
            );
        })
    }

    #[test]
    fn test_repr_c_enum_with_zst() {
        let test_src = r#"
        #[repr(C, i32)]
        pub enum SomeEnum {
            A(()),
        }

        const _: () = assert!(std::mem::size_of::<SomeEnum>() == 4);
        const _: () = assert!(std::mem::align_of::<SomeEnum>() == 4);
        "#;

        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) ... [[clang::trivial_abi]] SomeEnum final {
                        public:
                            ...
                            __COMMENT__ "`SomeEnum` doesn't implement the `Default` trait"
                            SomeEnum() = delete;
                            ...
                            __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                            ~SomeEnum() = default;
                            SomeEnum(SomeEnum&&) = default;
                            SomeEnum& operator=(SomeEnum&&) = default;

                            __COMMENT__ "`SomeEnum` doesn't implement the `Clone` trait"
                            SomeEnum(const SomeEnum&) = delete;
                            SomeEnum& operator=(const SomeEnum&) = delete;
                            ...
                            __COMMENT__ "Variant A has no size, so no struct is generated."
                            ...
                            enum class Tag : std::int32_t {
                                A = 0,
                            };
                            ...
                            public:
                                Tag tag;
                            ...
                        ...
                    };
                }
            );
        })
    }

    #[test]
    fn test_repr_c_union_fields() {
        let test_src = r#"
        #[repr(C)]
        pub union SomeUnion {
            pub x: u16,
            pub y: u32,
        }

        const _: () = assert!(std::mem::size_of::<SomeUnion>() == 4);
        const _: () = assert!(std::mem::align_of::<SomeUnion>() == 4);
        "#;

        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    union CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeUnion final {
                        public:
                            ...
                            __COMMENT__ "`SomeUnion` doesn't implement the `Default` trait"
                            SomeUnion() = delete;
                            ...
                            __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                            ~SomeUnion() = default;
                            SomeUnion(SomeUnion&&) = default;
                            SomeUnion& operator=(SomeUnion&&) = default;

                            __COMMENT__ "`SomeUnion` doesn't implement the `Clone` trait"
                            SomeUnion(const SomeUnion&) = delete;
                            SomeUnion& operator=(const SomeUnion&) = delete;
                            ...
                            std::uint16_t x;
                            ...
                            std::uint32_t y;

                        private:
                            static void __crubit_field_offset_assertions();
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(SomeUnion) == 4, ...);
                    static_assert(alignof(SomeUnion) == 4, ...);
                    static_assert(std::is_trivially_destructible_v<SomeUnion>);
                    static_assert(std::is_trivially_move_constructible_v<SomeUnion>);
                    static_assert(std::is_trivially_move_assignable_v<SomeUnion>);
                    inline void SomeUnion::__crubit_field_offset_assertions() {
                      static_assert(0 == offsetof(SomeUnion, x));
                      static_assert(0 == offsetof(SomeUnion, y));
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeUnion>() == 4);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeUnion>() == 4);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeUnion, x) == 0);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeUnion, y) == 0);
                }
            );
        })
    }

    #[test]
    fn test_union_fields() {
        let test_src = r#"
        pub union SomeUnion {
            pub x: u16,
            pub y: u32,
        }

        const _: () = assert!(std::mem::size_of::<SomeUnion>() == 4);
        const _: () = assert!(std::mem::align_of::<SomeUnion>() == 4);
        "#;

        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    union CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeUnion final {
                        public:
                            ...
                            __COMMENT__ "`SomeUnion` doesn't implement the `Default` trait"
                            SomeUnion() = delete;
                            ...
                            __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                            ~SomeUnion() = default;
                            SomeUnion(SomeUnion&&) = default;
                            SomeUnion& operator=(SomeUnion&&) = default;

                            __COMMENT__ "`SomeUnion` doesn't implement the `Clone` trait"
                            SomeUnion(const SomeUnion&) = delete;
                            SomeUnion& operator=(const SomeUnion&) = delete;
                            ...
                            struct {
                                ...
                                std::uint16_t value;
                            } x;
                            ...
                            struct {
                                ...
                                std::uint32_t value;
                            } y;
                        private:
                            static void __crubit_field_offset_assertions();
                    };
                }
            );

            // Note: we don't check for offsets here, because we don't know necessarily know
            // what the offset will be.
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(SomeUnion) == 4, ...);
                    static_assert(alignof(SomeUnion) == 4, ...);
                    static_assert(std::is_trivially_destructible_v<SomeUnion>);
                    static_assert(std::is_trivially_move_constructible_v<SomeUnion>);
                    static_assert(std::is_trivially_move_assignable_v<SomeUnion>);
                    inline void SomeUnion::__crubit_field_offset_assertions() {
                      ...
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeUnion>() == 4);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeUnion>() == 4);
                    ...
                }
            );
        })
    }

    #[test]
    fn test_repr_c_union_unknown_fields() {
        let test_src = r#"
        #[repr(C)]
        pub union SomeUnion {
            pub z: std::mem::ManuallyDrop<i64>,
        }

        const _: () = assert!(std::mem::size_of::<SomeUnion>() == 8);
        const _: () = assert!(std::mem::align_of::<SomeUnion>() == 8);
        "#;

        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    union CRUBIT_INTERNAL_RUST_TYPE(...) alignas(8) [[clang::trivial_abi]] SomeUnion final {
                        public:
                            ...
                        private:
                            __COMMENT__ "Field type has been replaced with a blob of bytes: Generic types are not supported yet (b/259749095)"
                            unsigned char z[8];
                        ...
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(sizeof(SomeUnion) == 8, ...);
                    static_assert(alignof(SomeUnion) == 8, ...);
                    ...
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeUnion>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeUnion>() == 8);
                    const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeUnion, z) == 0);
                }
            );
        })
    }

    #[test]
    fn test_format_cpp_name_for_struct() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(cpp_type="cpp_ns::CppType")]
                pub struct RustType {
                    pub x: i32,
                }
            "#;
        test_format_item(test_src, "RustType", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Type bindings for RustType suppressed \
                    due to being mapped to an existing C++ type (cpp_ns::CppType)"
            );
        });
    }

    #[test]
    fn test_must_use_attr_for_struct_no_msg() {
        let test_src = r#"
        #[must_use]
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... [[nodiscard]] ... SomeStruct final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_format_item_rename_field_with_conflicting_name() {
        let test_src = r#"
        pub struct X {
            pub a: i32,
            b: i32,
            #[allow(dead_code)]
            c: i32,
        }

        impl X {
            pub fn a(&self) -> i32 {
                self.a
            }
            pub fn b(&self) -> i32 {
                self.b
            }
        }
        "#;

        test_format_item(test_src, "X", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t a_;
                }
            );
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t b_;
                }
            );
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t c;
                }
            );
            // Check that the fields are not renamed in the Rust side.
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    ::core::mem::offset_of!(::rust_out::X, a) == 0
                }
            );
        })
    }

    #[test]
    fn test_must_use_attr_for_struct_msg() {
        let test_src = r#"
        #[must_use = "foo"]
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... [[nodiscard("foo")]] ... SomeStruct final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_must_use_attr_for_enum_no_msg() {
        let test_src = r#"
        #[must_use]
        pub enum SomeEnum {
            A(i32),
            B(u32),
        }"#;

        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... [[nodiscard]] ... SomeEnum final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_must_use_attr_for_enum_msg() {
        let test_src = r#"
        #[must_use = "foo"]
        pub enum SomeEnum {
            A(i32),
            B(u32),
        }"#;

        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... [[nodiscard("foo")]] ... SomeEnum final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_must_use_attr_for_union_no_msg() {
        let test_src = r#"
        #[must_use]
        pub union SomeUnion {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    union ... [[nodiscard]] ... SomeUnion final {
                        ...
                    };
                }
            )
        })
    }
    #[test]
    fn test_must_use_attr_for_union_msg() {
        let test_src = r#"
        #[must_use = "foo"]
        pub union SomeUnion {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    union ... [[nodiscard("foo")]] ... SomeUnion final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_struct_no_args() {
        let test_src = r#"
        #[deprecated]
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... [[deprecated]] ... SomeStruct final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_struct_with_message() {
        let test_src = r#"
        #[deprecated = "Use AnotherStruct instead"]
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... [[deprecated("Use AnotherStruct instead")]] ... SomeStruct final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_struct_with_named_args() {
        let test_src = r#"
        #[deprecated(since = "3.14", note = "Use AnotherStruct instead")]
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... [[deprecated("Use AnotherStruct instead")]] ... SomeStruct final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_union_with_named_args() {
        let test_src = r#"
        #[deprecated(since = "3.14", note = "Use AnotherUnion instead")]
        pub struct SomeUnion {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... [[deprecated("Use AnotherUnion instead")]] ... SomeUnion final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_enum_with_named_args() {
        let test_src = r#"
        #[deprecated(since = "3.14", note = "Use AnotherEnum instead")]
        pub enum SomeEnum {
            Integer(i32),
            FloatingPoint(f64),
        }"#;

        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... [[deprecated("Use AnotherEnum instead")]] ... SomeEnum final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_struct_fields() {
        let test_src = r#"
        pub struct SomeStruct {
            #[deprecated = "Use `y` instead"]
            pub x: u32,

            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... SomeStruct final {
                        ...
                        union {
                            ...
                            [[deprecated("Use `y` instead")]] std::uint32_t x;
                        }
                        ...
                        union {
                            ...
                            std::uint32_t y;
                        }
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_impl_block() {
        let test_src = r#"
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }

        #[deprecated = "Use AnotherStruct instead"]
        impl SomeStruct {
            pub fn sum(&self) -> u32 {
                self.x + self.y
            }

            pub fn product(&self) -> u32 {
                self.x * self.y
            }
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... SomeStruct final {
                        ...
                        ... [[deprecated("Use AnotherStruct instead")]] std::uint32_t sum() const ...
                        ...
                        ... [[deprecated("Use AnotherStruct instead")]] std::uint32_t product() const ...
                        ...
                    };
                }
            )
        })
    }
}
