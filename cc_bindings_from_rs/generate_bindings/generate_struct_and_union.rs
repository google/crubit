// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_abi;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use crate::format_cc_ident;
use crate::format_type::CcParamTy;
use crate::generate_doc_comment;
use crate::generate_function::{generate_thunk_call, Param, ThunkSelfParameter};
use crate::{
    crate_features, generate_const, generate_deprecated_tag, generate_must_use_tag,
    generate_trait_thunks, generate_unsupported_def, get_layout, get_scalar_int_type,
    get_tag_size_with_padding, is_bridged_type, is_copy, BridgedBuiltin, RsSnippet, SortedByDef,
    TraitThunks,
};
use arc_anyhow::{Context, Result};
use code_gen_utils::{expect_format_cc_type_name, make_rs_ident, CcInclude};
use database::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet};
use database::{AdtCoreBindings, BindingsGenerator, SugaredTy, TypeLocation};
use error_report::{anyhow, bail, ensure};
use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream};
use query_compiler::post_analysis_typing_env;
use quote::{format_ident, quote};
use rustc_abi::{FieldsShape, VariantIdx, Variants};
use rustc_hir::attrs::AttributeKind;
use rustc_hir::Attribute;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::ConstValue;
use rustc_middle::ty::{self, Ty, TyCtxt, TyKind, TypeFlags};
use rustc_span::def_id::{CrateNum, DefId, LOCAL_CRATE};
use rustc_span::symbol::sym;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::iter::once;
use std::rc::Rc;

fn has_type_or_const_vars() -> TypeFlags {
    TypeFlags::HAS_TY_PARAM
        | TypeFlags::HAS_CT_PARAM
        | TypeFlags::HAS_TY_INFER
        | TypeFlags::HAS_CT_INFER
        | TypeFlags::HAS_TY_PLACEHOLDER
        | TypeFlags::HAS_CT_PLACEHOLDER
        | TypeFlags::HAS_TY_BOUND
        | TypeFlags::HAS_CT_BOUND
}

pub(crate) fn adt_core_bindings_needs_drop<'tcx>(
    bindings: &AdtCoreBindings<'tcx>,
    tcx: TyCtxt<'tcx>,
) -> bool {
    bindings.self_ty.needs_drop(tcx, post_analysis_typing_env(tcx, bindings.def_id))
}

/// Returns the Rust underlying type of the `cpp_enum` struct specified by the given def id.
pub fn cpp_enum_rust_underlying_type(tcx: TyCtxt, def_id: DefId) -> Result<Ty> {
    let fields = tcx.adt_def(def_id).all_fields().collect::<Vec<_>>();
    if fields.len() != 1 {
        return Err(anyhow!(
            "Expected exactly one field in cpp_enum struct, got {:?}",
            fields.len()
        ));
    }

    let field_def_id = fields[0].did;
    let field_ty = tcx.type_of(field_def_id).instantiate_identity();

    Ok(field_ty)
}

/// Returns the C++ underlying type of the `cpp_enum` struct specified by the given def id.
pub(crate) fn cpp_enum_cpp_underlying_type(
    db: &dyn BindingsGenerator,
    def_id: DefId,
) -> Result<CcSnippet> {
    let tcx = db.tcx();

    let field_middle_ty = cpp_enum_rust_underlying_type(tcx, def_id)?;

    let field_type = SugaredTy::missing_hir(field_middle_ty);

    db.format_ty_for_cc(field_type, TypeLocation::Other)
}

/// Returns a string representation of the value of a given numeric Scalar having a given TyKind.
pub fn scalar_value_to_string(tcx: TyCtxt, scalar: Scalar, kind: TyKind) -> Result<String> {
    let scalar = match scalar {
        Scalar::Int(i) => i,
        Scalar::Ptr(..) => bail!("Pointer values cannot be used as scalar constants."),
    };

    // Print positive integers directly if they fit in an i64, since `int` is guaranteed to be at
    // least 16 bits wide.
    if matches!(kind, TyKind::Uint(_)) {
        let value: u128 = scalar.to_bits_unchecked();
        if value < (i16::MAX as u128) {
            return Ok((value as i16).to_string());
        }
    }

    use ty::FloatTy::*;
    use ty::IntTy::*;
    use ty::TyKind;
    use ty::UintTy::*;

    Ok(match kind {
        TyKind::Bool => scalar.try_to_bool().unwrap().to_string(),
        TyKind::Int(I8) => scalar.to_i8().to_string(),
        TyKind::Int(I16) => scalar.to_i16().to_string(),
        TyKind::Int(I32) => format!("INT32_C({})", scalar.to_i32()),
        TyKind::Uint(U8) => scalar.to_u8().to_string(),
        TyKind::Uint(U16) => format!("UINT16_C({})", scalar.to_u16()),
        TyKind::Uint(U32) => format!("UINT32_C({})", scalar.to_u32()),
        TyKind::Uint(U64) => format!("UINT64_C({})", scalar.to_u64()),
        TyKind::Float(F32) => format!("{}f", scalar.to_f32()),
        TyKind::Float(F64) => format!("{}L", scalar.to_f64()),
        TyKind::Uint(Usize) => format!("UINT64_C({})", scalar.to_target_usize(tcx)),

        // Signed integer minimums cannot be expressed with literals, as `-<int>` parses as a unary
        // minus operator applied to an out-of-range (for signed types) integer literal.
        TyKind::Int(I64) => {
            let value = scalar.to_i64();
            if value == i64::MIN {
                "INT64_MIN".to_string()
            } else {
                format!("INT64_C({value})")
            }
        }
        TyKind::Int(ty::IntTy::Isize) => {
            let value = scalar.to_target_isize(tcx);
            if value == i64::MIN {
                "INT64_MIN".to_string()
            } else {
                format!("INT64_C({value})")
            }
        }
        // Handle ffi_11 wrapper types.
        TyKind::Adt(adt, _) if tcx.crate_name(adt.did().krate).as_str() == "ffi_11" => {
            let name = tcx.item_name(adt.did());
            match name.as_str() {
                "c_char" => scalar.to_u8().to_string(),
                // If ffi_11::c_long is a wrapper type (and not a type alias) it will be 32 bit,
                // same for c_ulong.
                "c_long" => format!("INT32_C({})", scalar.to_i32()),
                "c_ulong" => format!("UINT32_C({})", scalar.to_u32()),
                "c_longlong" => format!("INT64_C({})", scalar.to_i64()),
                "c_ulonglong" => format!("UINT64_C({})", scalar.to_u64()),
                _ => bail!("Unsupported ffi_11 type: {:?}", kind),
            }
        }
        _ => bail!("Unsupported constant type: {:?}", kind),
    })
}

/// Formats a struct that is annotated with the `cpp_enum` attribute.
///
/// The Rust definition for an item annotation with `cpp_enum` is expected to be a repr-transparent
/// struct with a single field. Example:
///
/// ```rs
/// #[crubit_annotate::cpp_enum("enum class")]
/// #[repr(transparent)]
/// pub struct MyEnum(i32);
///
/// impl MyEnum {
///     pub const VARIANT_0: MyEnum = MyEnum(0);
///     pub const VARIANT_1: MyEnum = MyEnum(1);
///     // ...
/// }
/// ```
///
/// This will generate (approximately) the following C++ code:
///
/// ```c++
/// enum class MyEnum : std::int32_t {
///     VARIANT_0 = 0,
///     VARIANT_1 = 1,
///     // ...
/// };
/// ```
fn generate_cpp_enum<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
) -> ApiSnippets {
    let tcx = db.tcx();
    let enumeration_cc_name = &core.cc_short_name;

    let mut main_api_prereqs = CcPrerequisites::default();
    main_api_prereqs.includes.insert(db.support_header("annotations_internal.h"));

    // Generate relevant attributes.
    let rs_type = core.rs_fully_qualified_name.to_string();
    let mut attributes = vec![quote! {CRUBIT_INTERNAL_RUST_TYPE(#rs_type)}];
    if let Some(tag) = generate_must_use_tag(tcx, core.def_id) {
        attributes.push(tag);
    }
    if let Some(tag) = generate_deprecated_tag(tcx, core.def_id) {
        attributes.push(tag);
    }

    // Generate the enumerator list.
    let enumerator_lines: Vec<TokenStream> = tcx
        .inherent_impls(core.def_id)
        .iter()
        .copied()
        .sorted_by_def(tcx)
        .flat_map(|impl_id| tcx.associated_items(impl_id).in_definition_order())
        .filter_map(|assoc_item| {
            if !is_supported_associated_item(tcx, assoc_item.def_id) {
                return None;
            }
            let ty::AssocKind::Const { name } = assoc_item.kind else {
                db.fatal_errors().report(&format!(
                    "C++ enums can only have `const`s as public items, found: {:?}",
                    assoc_item.kind
                ));
                return None;
            };
            let enumerator_name = format_cc_ident(db, name.as_str()).unwrap();
            let (opt_doc_comment, bracketed_enumerator_name) = if db.kythe_annotations() {
                (
                    generate_doc_comment(db, assoc_item.def_id),
                    quote! { __CAPTURE_BEGIN__ #enumerator_name __CAPTURE_END__ },
                )
            } else {
                (TokenStream::new(), quote! { #enumerator_name })
            };
            let value_kind = *cpp_enum_rust_underlying_type(tcx, core.def_id).unwrap().kind();
            let scalar = match tcx.const_eval_poly(assoc_item.def_id).unwrap() {
                ConstValue::Scalar(scalar) => scalar,
                other => {
                    panic!("Unexpected non-scalar ConstValue type in cpp_enum: {other:?}")
                }
            };
            let enumerator_value = scalar_value_to_string(tcx, scalar, value_kind)
                .unwrap()
                .parse::<TokenStream>()
                .unwrap();

            Some(quote! { #opt_doc_comment #bracketed_enumerator_name = #enumerator_value, })
        })
        .collect();

    let doc_comment = generate_doc_comment(db, core.def_id);
    let keyword = &core.keyword;
    let underlying_cc_type_snippet = cpp_enum_cpp_underlying_type(db, core.def_id).unwrap();
    let underlying_cc_type = underlying_cc_type_snippet.tokens;
    let bracketed_enumeration_cc_name = if db.kythe_annotations() {
        quote! { __CAPTURE_BEGIN__ #enumeration_cc_name __CAPTURE_END__ }
    } else {
        quote! { #enumeration_cc_name }
    };

    let main_api = CcSnippet {
        tokens: quote! {
            __NEWLINE__ #doc_comment
            #keyword #(#attributes)* #bracketed_enumeration_cc_name : #underlying_cc_type {
                #( __NEWLINE__ #enumerator_lines)*
            };
            __NEWLINE__
        },
        prereqs: main_api_prereqs + underlying_cc_type_snippet.prereqs,
    };

    let cc_details = CcSnippet::default();
    let rs_details = RsSnippet::new(quote! {});

    ApiSnippets { main_api, cc_details, rs_details }
}

/// Returns true if the associated item should generate bindings.
///
/// Associated items don't have a canonical name because they can be accessed through their parent
/// type even if they're not publicly exported from the crate. Because of this an associated item
/// should receive bindings if it's definition is marked public and it is not marked #[unstable].
///
/// We rely on callers to ensure that we only try to generate bindings for associated items of types
/// that are publicly reachable.
fn is_supported_associated_item<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> bool {
    tcx.visibility(def_id).is_public()
        && tcx.lookup_stability(def_id).is_none_or(|stability| stability.is_stable())
}

pub(crate) fn generate_associated_item<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    assoc_item: &ty::AssocItem,
    member_function_names: &mut HashSet<String>,
) -> Option<ApiSnippets> {
    let tcx = db.tcx();
    let def_id = assoc_item.def_id;
    if !is_supported_associated_item(tcx, def_id) {
        return None;
    }
    crate::error_scope!(db, def_id);
    let result = match assoc_item.kind {
        ty::AssocKind::Fn { .. } => {
            let result = db.generate_function(def_id);
            if result.is_ok() {
                let unqualified_name = db
                    .symbol_unqualified_name(def_id)
                    .expect("Associated item should have an unqualified name: {def_id:?}");
                let cpp_name = unqualified_name.cpp_name.to_string();
                member_function_names.insert(cpp_name);
            }
            result
        }
        ty::AssocKind::Const { .. } => {
            if tcx.trait_impl_of_assoc(def_id).is_some() {
                // Associated constants are not yet supported on traits
                return None;
            }
            generate_const(db, def_id)
        }
        // TODO: b/405132277 - Rust does not support inherent associated types, but should support
        // associated types when adding traits.
        ty::AssocKind::Type { .. } => Err(anyhow!(
            "Associated types are not yet supported, found {:?}. See b/405132277.",
            assoc_item.opt_name()
        )),
    };
    let result = result.and_then(|snippet| {
        snippet.resolve_feature_requirements(crate_features(db, db.source_crate_num()))
    });
    match result {
        Err(err) => {
            if crubit_attr::get_attrs(tcx, def_id).unwrap().must_bind {
                let self_name = crate::item_name(db, tcx.parent(def_id));
                let item_name = crate::item_name(db, def_id);
                let must_bind_message = format!(
                    "Failed to generate bindings for `{self_name}::{item_name}`:\n\
                    {err:?}\n\
                    This is a hard error because `{self_name}::{item_name}` was annotated with \
                    `#[crubit_annotate::must_bind]`"
                );
                db.fatal_errors().report(&must_bind_message);
            }
            Some(generate_unsupported_def(db, def_id, err).into_main_api())
        }
        Ok(result) => Some(result),
    }
}

fn erase_regions<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
    #[rustversion::since(2025-09-10)]
    return tcx.erase_and_anonymize_regions(ty);
    #[rustversion::before(2025-09-10)]
    return tcx.erase_regions(ty);
}

pub fn from_trait_impls_by_argument<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    crate_num: CrateNum,
) -> Rc<HashMap<Ty<'tcx>, Vec<DefId>>> {
    let tcx = db.tcx();
    let from_trait = tcx.get_diagnostic_item(sym::From).expect("Could not find From trait");
    let impls_iter: Box<dyn Iterator<Item = DefId>> = if crate_num == LOCAL_CRATE {
        Box::new(tcx.local_trait_impls(from_trait).iter().map(|impl_id| impl_id.to_def_id()))
    } else {
        Box::new(
            tcx.implementations_of_trait((crate_num, from_trait))
                .iter()
                .map(|(impl_id, _)| *impl_id),
        )
    };
    let mut map: HashMap<Ty<'tcx>, Vec<DefId>> = HashMap::new();
    for from_impl_id in impls_iter {
        #[rustversion::since(2025-10-17)]
        let middle_trait_header = tcx.impl_trait_header(from_impl_id);
        #[rustversion::before(2025-10-17)]
        let middle_trait_header = tcx
            .impl_trait_header(from_impl_id)
            .expect("DefId for an `From` trait impl lacked a trait header");
        let trait_ref = middle_trait_header.trait_ref.instantiate_identity();
        let ty = trait_ref.args.type_at(1);
        // We want to check if our type has type variables and constant variables, but not
        // region variables. Region variables are fine and we'll replace them with 'static.
        if ty.flags().contains(has_type_or_const_vars()) {
            continue;
        }

        // We want to work in region-erased types because that's what we will be querying by
        // for lookup.
        let from_self_ty = erase_regions(tcx, ty);
        map.entry(from_self_ty).or_default().push(from_impl_id);
    }
    Rc::new(map)
}

fn generate_into_impls<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
) -> ApiSnippets {
    let tcx = db.tcx();
    let cc_struct_name = &core.cc_short_name;

    let into_trait = tcx.get_diagnostic_item(sym::Into).expect("Could not find Into trait");

    let from_map = db.from_trait_impls_by_argument(core.def_id.krate);
    let from_impls = from_map.get(&core.self_ty).into_iter().flat_map(|vec| vec.iter()).filter_map(
        |from_impl_id| {
            #[rustversion::since(2025-10-17)]
            let middle_trait_header = tcx.impl_trait_header(*from_impl_id);
            #[rustversion::before(2025-10-17)]
            let middle_trait_header = tcx
                .impl_trait_header(*from_impl_id)
                .expect("DefId for a `From` trait impl lacked a trait header");
            let trait_ref = middle_trait_header.trait_ref.instantiate_identity();

            let from_middle_ty = trait_ref.args.type_at(0);

            // If our type contains type variables or constant variables (but not region variables),
            // we can't generate an `into` impl.
            if from_middle_ty.flags().contains(has_type_or_const_vars()) {
                return None;
            }
            let sugar_ty = SugaredTy::missing_hir(from_middle_ty);
            // We know that our type will always appear in FnReturn position for the `into` method.
            // If our type isn't C++-compatible, we can't generate an `into` impl.
            let cc_ty = db.format_ty_for_cc(sugar_ty, TypeLocation::FnReturn).ok()?;
            Some((from_middle_ty, cc_ty, *from_impl_id))
        },
    );
    let into_impls =
        tcx.non_blanket_impls_for_ty(into_trait, core.self_ty).filter_map(|into_impl_id| {
            #[rustversion::since(2025-10-17)]
            let middle_trait_header = tcx.impl_trait_header(into_impl_id);
            #[rustversion::before(2025-10-17)]
            let middle_trait_header = tcx
                .impl_trait_header(into_impl_id)
                .expect("DefId for an `Into` trait impl lacked a trait header");
            // Index 0 of our trait ref is the self type, so index 1 is the type we're converting
            // into.
            let into_middle_ty =
                middle_trait_header.trait_ref.instantiate_identity().args.type_at(1);

            let sugar_ty = SugaredTy::missing_hir(into_middle_ty);
            // If our type isn't Cxx compatible, we can't generate an `into` impl.
            let cc_ty = db.format_ty_for_cc(sugar_ty, TypeLocation::FnReturn).ok()?;

            Some((into_middle_ty, cc_ty, into_impl_id))
        });

    from_impls
        .chain(into_impls)
        .filter_map(|(middle_ty, cc_ty, def_id)| {
            let mut prereqs = CcPrerequisites::default();
            let cc_ty = cc_ty.into_tokens(&mut prereqs);

            // Delay converting this type until we've successfully generated the thunks.
            // We generate thunks for `into` here. This relies on the blanket impls of for `Into` in the stdlib to work.
            let TraitThunks {
                method_name_to_cc_thunk_name,
                cc_thunk_decls,
                rs_thunk_impls: rs_details,
            } = generate_trait_thunks(db, into_trait, &[middle_ty], core).ok()?;

            let thunk_name = method_name_to_cc_thunk_name
                .into_values()
                .exactly_one()
                .expect("Expecting a single `into` method");

            let cc_thunk_decls = cc_thunk_decls.into_tokens(&mut prereqs);
            let doc_comment = generate_doc_comment(db, def_id);

            let sugar_self_ty = SugaredTy::missing_hir(core.self_ty);
            let self_cpp_ty = db
                .format_ty_for_cc(
                    sugar_self_ty,
                    TypeLocation::FnParam { is_self_param: true, elided_is_output: true },
                )
                .expect(
                    "ADT's self type should be C++-convertible after generate_adt_core succeeds",
                );
            let self_cpp_ty = self_cpp_ty.into_tokens(&mut prereqs);
            let impl_body = generate_thunk_call(
                db,
                def_id,
                thunk_name.clone(),
                SugaredTy::missing_hir(middle_ty),
                ThunkSelfParameter::new(
                    /*has_self=*/ true,
                    is_copy(tcx, def_id, core.self_ty),
                    /*is_trait_method =*/ false,
                ),
                &[Param {
                    cc_name: format_ident!("self"),
                    cpp_type: CcParamTy {
                        snippet: CcSnippet::new(self_cpp_ty),
                        is_lifetime_bound: false,
                    },
                    ty: sugar_self_ty,
                }],
            )
            .expect("Self type of `Into` impl should be bridgeable");

            let impl_body_tokens = impl_body.into_tokens(&mut prereqs);
            prereqs.move_defs_to_fwd_decls();

            Some(ApiSnippets {
                main_api: CcSnippet {
                    tokens: quote! {
                    __NEWLINE__ #doc_comment
                    explicit operator #cc_ty ( ) ; __NEWLINE__
                    __NEWLINE__
                    },
                    prereqs,
                },
                cc_details: CcSnippet::new(quote! {
                    #cc_thunk_decls

                    inline #cc_struct_name :: operator  #cc_ty ( ) {
                        #impl_body_tokens
                    }
                }),
                rs_details,
            })
        })
        .collect()
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

    // Handle `cpp_enum` structs.
    let crubit_attrs = crubit_attr::get_attrs(tcx, core.def_id).unwrap_or_default();
    if crubit_attrs.cpp_enum.is_some() {
        return generate_cpp_enum(db, core);
    }

    let default_ctor_snippets = db.generate_default_ctor(core.clone()).unwrap_or_else(|err| err);

    let destructor_snippets = if adt_core_bindings_needs_drop(&core, tcx) {
        let drop_trait_id =
            tcx.lang_items().drop_trait().expect("`Drop` trait should be present if `needs_drop");
        let TraitThunks {
            method_name_to_cc_thunk_name,
            mut cc_thunk_decls,
            rs_thunk_impls: rs_details,
        } = generate_trait_thunks(db, drop_trait_id, &[], &core)
            .expect("`generate_adt_core` should have already validated `Drop` support");
        // Don't introduce additional feature prerequisites for the `Drop` trait impl, as this
        // will cause type generation to fail based on an API that isn't even user-accessible.
        cc_thunk_decls.prereqs.required_features = flagset::FlagSet::empty();
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

    let move_ctor_and_assignment_snippets = db
        .generate_move_ctor_and_assignment_operator(core.clone())
        .unwrap_or_else(|err| err.explicitly_deleted);

    let relocating_ctor_snippets = generate_relocating_ctor(db, core.clone());

    let tuple_struct_ctor = generate_tuple_struct_ctor(db, core.clone()).unwrap_or_default();

    let mut member_function_names = HashSet::<String>::new();
    let impl_items_snippets = tcx
        .inherent_impls(core.def_id)
        .iter()
        .copied()
        .sorted_by_def(tcx)
        .flat_map(|impl_id| tcx.associated_items(impl_id).in_definition_order())
        .flat_map(|assoc_item| generate_associated_item(db, assoc_item, &mut member_function_names))
        .collect();

    let into_operator_snippets = generate_into_impls(db, core.as_ref());

    let ApiSnippets {
        main_api: public_functions_main_api,
        cc_details: public_functions_cc_details,
        rs_details: public_functions_rs_details,
    } = [
        default_ctor_snippets,
        tuple_struct_ctor,
        destructor_snippets,
        move_ctor_and_assignment_snippets,
        copy_ctor_and_assignment_snippets,
        relocating_ctor_snippets,
        impl_items_snippets,
        into_operator_snippets,
    ]
    .into_iter()
    .collect();

    let repr_attrs = db.repr_attrs(core.def_id);

    let ApiSnippets {
        main_api: fields_main_api,
        cc_details: fields_cc_details,
        rs_details: fields_rs_details,
    } = generate_fields(
        db,
        core.self_ty,
        &core.cc_short_name,
        &core.rs_fully_qualified_name,
        &repr_attrs,
        core.size_in_bytes,
        core.alignment_in_bytes,
        &member_function_names,
    );

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
            .any(|repr| matches!(repr, rustc_hir::attrs::ReprPacked { .. }))
        {
            attributes.push(quote! { __attribute__((packed)) })
        }

        // Additional attributes
        if let Some(tag) = generate_must_use_tag(tcx, core.def_id) {
            attributes.push(tag);
        }
        if let Some(tag) = generate_deprecated_tag(tcx, core.def_id) {
            attributes.push(tag);
        }

        let doc_comment = generate_doc_comment(db, core.def_id);
        let keyword = &core.keyword;

        let mut prereqs = CcPrerequisites::default();
        prereqs.includes.insert(db.support_header("annotations_internal.h"));
        let public_functions_main_api = public_functions_main_api.into_tokens(&mut prereqs);
        let fields_main_api = fields_main_api.into_tokens(&mut prereqs);
        prereqs.fwd_decls.remove(&core.def_id);

        let bracketed_adt_cc_name = if db.kythe_annotations() {
            quote! { __CAPTURE_BEGIN__ #adt_cc_name __CAPTURE_END__ }
        } else {
            quote! { #adt_cc_name }
        };

        CcSnippet {
            prereqs,
            tokens: quote! {
                __NEWLINE__ #doc_comment
                #keyword #(#attributes)* #bracketed_adt_cc_name final {
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
        prereqs.defs.insert(core.def_id);
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

/// Implementation of `BindingsGenerator::adt_needs_bindings`.
pub fn adt_needs_bindings<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    def_id: DefId,
) -> Result<Rc<AdtCoreBindings<'tcx>>> {
    let tcx = db.tcx();
    let attributes = crubit_attr::get_attrs(tcx, def_id).unwrap();

    let has_composable_bridging_attrs = matches!(
        attributes.get_bridging_attrs()?,
        Some(crubit_attr::BridgingAttrs::Composable { .. })
    );

    if !has_composable_bridging_attrs
        && BridgedBuiltin::new(db, tcx.adt_def(def_id)).is_none()
        && query_compiler::has_non_lifetime_generics(tcx, def_id)
    {
        bail!("Generic types are not supported yet (b/259749095)");
    }

    let fully_qualified_name = db
        .symbol_canonical_name(def_id)
        .ok_or_else(|| anyhow!("No public path could be found for type {def_id:?}"))?;
    if let Some(cpp_type) = fully_qualified_name.unqualified.cpp_type {
        let item_name = tcx.def_path_str(def_id);
        bail!(
            "Type bindings for {item_name} suppressed due to being mapped to \
                    an existing C++ type ({cpp_type})"
        );
    }

    db.generate_adt_core(def_id)
}

/// Implementation of `BindingsGenerator::generate_adt_core`.
pub fn generate_adt_core<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    def_id: DefId,
) -> Result<Rc<AdtCoreBindings<'tcx>>> {
    let tcx = db.tcx();
    // Note: we erase regions in order to get bindings regardless of what lifetime parameters are
    // present. We want to generate bindings for functions regardless of their lifetime bounds, as
    // C++ cannot special-case the availability of a function based on lifetimes.
    let self_ty = erase_regions(tcx, tcx.type_of(def_id).instantiate_identity());
    assert!(self_ty.is_adt());
    assert!(db.symbol_canonical_name(def_id).is_some(), "Caller should verify");

    let fully_qualified_name = db
        .symbol_canonical_name(def_id)
        .ok_or_else(|| anyhow!("`generate_adt_core` called on non-reachable type {def_id:?}"))?;
    let rs_fully_qualified_name = fully_qualified_name.format_for_rs();
    let cpp_name = format_cc_ident(db, fully_qualified_name.unqualified.cpp_name.as_str())
        .context("Error formatting item name")?;

    // The check below ensures that `generate_trait_thunks` will succeed for the
    // `Drop`, `Default`, and/or `Clone` trait. Ideally we would directly check
    // if `generate_trait_thunks` or `format_ty_for_cc(..., self_ty, ...)`
    // succeeds, but this would lead to infinite recursion, so we only replicate
    // `format_ty_for_cc` / `TyKind::Adt` checks that are outside of
    // `generate_adt_core`.
    fully_qualified_name.format_for_cc(db).with_context(|| {
        format!("Error formatting the fully-qualified C++ name of `{cpp_name}`")
    })?;

    let adt_def = self_ty.ty_adt_def().expect("`def_id` needs to identify an ADT");
    let crubit_attrs = crubit_attr::get_attrs(tcx, def_id).unwrap_or_default();

    let keyword = match adt_def.adt_kind() {
        ty::AdtKind::Struct => match crubit_attrs.cpp_enum {
            Some(cpp_enum_symbol) => {
                let s = cpp_enum_symbol.as_str();
                match s {
                    "enum" => quote! { enum },
                    "enum class" => quote! { enum class },
                    _ => panic!("Unsupported `cpp_enum` tag: {s}"),
                }
            }
            None => quote! { struct },
        },
        ty::AdtKind::Enum => quote! { struct },
        ty::AdtKind::Union => quote! { union },
    };

    // Verify that `cpp_enum` structs are also repr-transparent.
    if crubit_attrs.cpp_enum.is_some() {
        ensure!(
            adt_def.repr().transparent(),
            "`cpp_enum` struct must be annotated with `#[repr(transparent)]`"
        )
    }

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

struct IndexedVariantField<'tcx> {
    index: usize,
    field_def: &'tcx ty::FieldDef,
}

/// Given ADT bindings, iterates over the variants of that ADT and the fields of each variant.
/// For each field, iteration always provides the middle FieldDef and it's index within it's variant.
/// The hir type of the field will optionally be included if it is available.
fn variant_fields_iter<'tcx>(
    self_ty: Ty<'tcx>,
) -> impl Iterator<Item = impl Iterator<Item = IndexedVariantField<'tcx>>> {
    self_ty.ty_adt_def().expect("`core.def_id` needs to identify an ADT").variants().iter().map(
        |variant| {
            variant
                .fields
                .iter()
                .enumerate()
                .map(move |(index, field_def)| IndexedVariantField { index, field_def })
        },
    )
}

fn anonymous_field_ident(index: usize) -> Ident {
    format_ident!("__field{index}")
}

fn generate_tuple_struct_ctor<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
) -> Option<ApiSnippets> {
    let tcx = db.tcx();
    let TyKind::Adt(adt_def, adt_generic_args) = core.self_ty.kind() else {
        panic!("Attempted to generate constructor for a non-ADT type: {:?}", core.self_ty)
    };

    if !adt_def.has_ctor() {
        // If this is not a struct with a constructor, don't generate a C++ constructor.
        return None;
    }

    if tcx
        .get_all_attrs(core.def_id)
        .iter()
        .any(|attr| matches!(attr, Attribute::Parsed(AttributeKind::NonExhaustive(_))))
    {
        // If the definition is marked #[non_exhaustive], don't generate a C++ constructor.
        // #[non_exhaustive] tuple structs do not have a public synthesized constructor.
        return None;
    }

    let default_trait_id = tcx.get_diagnostic_item(sym::Default).expect("Default trait not found");
    let clone_trait_id = tcx.lang_items().copy_trait().expect("Copy trait not found");
    let unpin_trait_id = tcx.lang_items().unpin_trait().expect("Unpin trait not found");

    let field_tys = variant_fields_iter(core.self_ty)
        .next()
        .expect("Tuple structs must have one variant")
        .map(|IndexedVariantField { field_def, .. }| {
            if field_def.vis != ty::Visibility::Public {
                // If our synthesized constructor would have a non public visibility, don't generate it as
                // we can't mirror that visibility in C++.
                return None;
            }
            let ty = field_def.ty(tcx, adt_generic_args);

            let is_default =
                query_compiler::does_type_implement_trait(tcx, ty, default_trait_id, []);
            let is_clone = query_compiler::does_type_implement_trait(tcx, ty, clone_trait_id, []);
            let is_unpin = query_compiler::does_type_implement_trait(tcx, ty, unpin_trait_id, []);
            let is_movable_in_cpp = (is_default && is_unpin) || is_clone;
            if !is_movable_in_cpp {
                // If one of our fields isn't movable in C++, we can't generate a C++ constructor.
                return None;
            }

            Some(SugaredTy::missing_hir(ty))
        })
        .collect::<Option<Vec<_>>>()?;

    let explicit = (field_tys.len() == 1).then_some(quote! { explicit });

    let mut main_api_prereqs = CcPrerequisites::default();
    let adt_cc_name = &core.cc_short_name;
    let initializer_list = (0..field_tys.len()).map(|i| {
        let cc_name = anonymous_field_ident(i);
        quote! { #cc_name ( std::move ( #cc_name ) ) }
    });

    // If we fail to convert a field type, don't generate a constructor.
    // Our uncovertible fields will be replaced by a blob of bytes that we do not want to appear
    // in our API, so opt to avoid presenting a constructor for types that contain a blob of
    // bytes.
    let main_api_params = field_tys
        .into_iter()
        .enumerate()
        .map(|(i, field_ty)| {
            let cpp_type = db.format_ty_for_cc(field_ty, TypeLocation::Other)?;
            let cc_name = anonymous_field_ident(i);
            let cpp_type = cpp_type.into_tokens(&mut main_api_prereqs);
            Ok(quote! { #cpp_type #cc_name })
        })
        .collect::<Result<Vec<TokenStream>>>()
        .ok()?;

    let mut prereqs = main_api_prereqs.clone();
    prereqs.move_defs_to_fwd_decls();

    Some(ApiSnippets {
        main_api: CcSnippet {
            prereqs,
            tokens: quote! {
              __NEWLINE__ __COMMENT__ "Synthesized tuple constructor"
              #explicit #adt_cc_name (
                  #( #main_api_params ),*
              ) : #( #initializer_list ),* { }
              __NEWLINE__
            },
        },
        ..Default::default()
    })
}

/// Returns the body of the C++ struct that represents the given ADT.
fn generate_fields<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    self_ty: Ty<'tcx>,
    cc_short_name: &Ident,
    rs_fully_qualified_name: &TokenStream,
    repr_attrs: &[rustc_hir::attrs::ReprAttr],
    size_in_bytes: u64,
    alignment_in_bytes: u64,
    member_function_names: &HashSet<String>,
) -> ApiSnippets {
    let tcx = db.tcx();
    let TyKind::Adt(adt_def, adt_generic_args) = self_ty.kind() else {
        panic!("Attempted to generate fields for a non-ADT type: {:?}", self_ty)
    };
    struct FieldTypeInfo {
        size: u64,
        cpp_type: CcSnippet,
    }
    struct Field {
        type_info: Result<FieldTypeInfo>,
        cc_name: Ident,
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

    let layout =
        get_layout(tcx, self_ty).expect("Layout should be already verified by `generate_adt_core`");
    let err_fields = |err| {
        vec![Field {
            type_info: Err(err),
            cc_name: format_ident!("__opaque_blob_of_bytes"),
            rs_name: quote! { __opaque_blob_of_bytes },
            is_public: false,
            index: 0,
            offset: 0,
            offset_of_next_field: size_in_bytes,
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
    let is_supported_enum = adt_def.is_enum() && repr_attrs.contains(&rustc_hir::attrs::ReprC);

    let tag_size_with_padding =
        if is_supported_enum { get_tag_size_with_padding(layout) } else { 0 };

    let variant_sizes = match layout_variants {
        Variants::Multiple { tag: _, tag_encoding: _, tag_field: _, variants } => {
            variants.iter().map(|layout| layout.size.bytes() - tag_size_with_padding).collect_vec()
        }
        Variants::Single { .. } | Variants::Empty => {
            vec![alignment_in_bytes]
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
        ty::AdtKind::Enum if (!repr_attrs.contains(&rustc_hir::attrs::ReprC)) => {
            vec![err_fields(anyhow!("No support for bindings of individual non-repr(C) `enum`s"))]
        }

        // Otherwise, get the fields and determine the memory layout.
        _ => {
            let mut variants_fields = variant_fields_iter(self_ty)
                .map(|field_iter| {
                    field_iter
                        .map(|IndexedVariantField { index, field_def }| {
                            let ty = SugaredTy::missing_hir(field_def.ty(tcx, adt_generic_args));
                            let size =
                                get_layout(tcx, ty.mid()).map(|layout| layout.size().bytes());
                            let type_info = size.and_then(|size| {
                                if is_bridged_type(db, ty.mid())?.is_some() {
                                    bail!(
                                        "Field is a bridged type and might not be layout-compatible
                                    with the C++ type (b/400633609)"
                                    );
                                }

                                Ok(FieldTypeInfo {
                                    size,
                                    cpp_type: db
                                        .format_ty_for_cc(ty, TypeLocation::Other)?
                                        .resolve_feature_requirements(crate_features(
                                            db,
                                            db.source_crate_num(),
                                        ))?,
                                })
                            });
                            let name = field_def.ident(tcx).to_string();
                            let cc_name = code_gen_utils::unkeyword_cpp_ident(&name).to_string();
                            let cc_name = if member_function_names.contains(&cc_name) {
                                // TODO: Handle the case of name_ itself also being taken? e.g. the
                                // Rust struct struct S {a: i32, a_:
                                // i32} impl S { fn a() {} fn a_()
                                // {} fn a__(){}.
                                format!("{cc_name}_")
                            } else {
                                cc_name
                            };
                            let cc_name = format_cc_ident(db, cc_name.as_str())
                                .unwrap_or_else(|_err| anonymous_field_ident(index));
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
                                doc_comment: generate_doc_comment(db, field_def.did),
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
        let adt_cc_name = cc_short_name;
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
                            .unwrap_or_else(|_err| format_ident!("err_field"));
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
                        .iter()
                        .enumerate()
                        .flat_map(|(variant_index, fields_for_variant)| {
                            let variant_def = adt_def.variant(VariantIdx::from_usize(variant_index));
                            let cc_variant = variant_def.ident(tcx);
                            let qualified_struct_name =
                                expect_format_cc_type_name(&format!("{}::__crubit_{}_struct", adt_cc_name, cc_variant));
                            // If the variant has no fields, don't bother generating any assertions.
                            if variant_def.fields.is_empty() {
                                quote! {}
                            } else {
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
        let adt_rs_name = rs_fully_qualified_name;
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
        let always_omit_padding = repr_attrs.contains(&rustc_hir::attrs::ReprC)
            && variants_fields.iter().flatten().all(|field| field.type_info.is_ok());

        let mut prereqs = CcPrerequisites::default();

        #[derive(Debug, Default)]
        struct CcFieldVisState {
            is_public: Option<bool>,
        }

        impl CcFieldVisState {
            fn public() -> Self {
                Self { is_public: Some(true) }
            }
            /// Ensures the current field visibility matches `is_public` by returning tokens to
            /// switch from `private:` to `public:` or vice versa. If the current access specifier
            /// already matches the requested one, no specifier is returned.
            fn set_is_public(&mut self, is_public: bool) -> TokenStream {
                if self.is_public == Some(is_public) {
                    quote! {}
                } else {
                    self.is_public = Some(is_public);
                    if is_public {
                        quote! { public: }
                    } else {
                        quote! { private: }
                    }
                }
            }
        }

        // Takes a field and converts it to a token stream.
        let get_field_tokens = |field: Field,
                                prereqs: &mut CcPrerequisites,
                                current_visibility: &mut CcFieldVisState|
         -> TokenStream {
            let cc_name = &field.cc_name;
            let bracketed_cc_name = if db.kythe_annotations() {
                quote! { __CAPTURE_BEGIN__ #cc_name __CAPTURE_END__ }
            } else {
                quote! { #cc_name }
            };
            match field.type_info {
                Err(ref err) => {
                    let size = field.size();
                    let msg = format!("Field type has been replaced with a blob of bytes: {err:#}");

                    // Empty arrays are ill-formed, but also unnecessary for padding.
                    if size > 0 {
                        let visibility = current_visibility.set_is_public(false);
                        let size = Literal::u64_unsuffixed(size);
                        let tokens = quote! {
                            #visibility __NEWLINE__
                                __COMMENT__ #msg
                                unsigned char #cc_name[#size];
                        };
                        tokens
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

                    // Visibility specifier needed by the current field.
                    // We have to update this field's visibility before calculating its padding,
                    // since the padding may update the current visibility to private.
                    let visibility = current_visibility.set_is_public(field.is_public);

                    let cpp_type = cpp_type.into_tokens(prereqs);
                    let doc_comment = field.doc_comment;
                    let attributes = field.attributes;

                    let tokens = match adt_def.adt_kind() {
                        ty::AdtKind::Struct => {
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
                                let padding_visibility = current_visibility.set_is_public(false);
                                quote! { #padding_visibility unsigned char #ident[#padding]; }
                            };
                            quote! {
                                #visibility __NEWLINE__
                                    // The anonymous union gives more control over when exactly
                                    // the field constructors and destructors run. For example,
                                    // this lets us initialize the fields for the first time via
                                    // memcpy, in the move or UnsafeRelocateTag constructor, and lets
                                    // us destroy them only by calling into Rust.
                                    // See also b/288138612.
                                    union {  __NEWLINE__
                                        #doc_comment
                                        #(#attributes)*
                                        #cpp_type #bracketed_cc_name;
                                    };
                                #padding
                            }
                        }
                        ty::AdtKind::Union => {
                            if repr_attrs.contains(&rustc_hir::attrs::ReprC) {
                                quote! {
                                    #visibility __NEWLINE__
                                    #doc_comment
                                    #cpp_type #bracketed_cc_name;
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
                                    } #bracketed_cc_name;
                                }
                            }
                        }
                        ty::AdtKind::Enum => {
                            // TODO(b/460420108) : Why is there no #doc_comment here? We need one
                            // for __CAPTURE_BEGIN__ et al to work properly.
                            quote! {
                                #visibility __NEWLINE__ #cpp_type #cc_name;
                            }
                        }
                    };
                    tokens
                }
            }
        };

        // For structs and unions, we can just flatten the fields variant. For enums, we
        // need to handle each variant separately.
        let fields = match adt_def.adt_kind() {
            ty::AdtKind::Struct | ty::AdtKind::Union => {
                let mut current_visibility = CcFieldVisState::public();
                variants_fields
                    .into_iter()
                    .flatten()
                    .map(|field| get_field_tokens(field, &mut prereqs, &mut current_visibility))
                    .collect()
            }
            ty::AdtKind::Enum if !is_supported_enum => variants_fields
                .into_iter()
                .flatten()
                .map(|field| get_field_tokens(field, &mut prereqs, &mut Default::default()))
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

                        let tag_tokens = db
                            .format_ty_for_cc(
                                // An enum cannot have repr(c_char), or any other alias, so there's
                                // never sugar.
                                SugaredTy::missing_hir(tag_ty),
                                TypeLocation::Other,
                            )
                            .expect("discriminant should be a integer type.")
                            .into_tokens(&mut prereqs);

                        let variant_enum_fields: TokenStream = adt_def
                            .variants()
                            .iter_enumerated()
                            .map(|(variant_index, variant_def)| {
                                let cc_variant_name =
                                    format_cc_ident(db, variant_def.name.as_str())
                                        .unwrap_or_else(|_err| format_ident!("err_field"));
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
                    let mut current_visibility = CcFieldVisState::default();
                    tokens_per_variant.push(
                        fields_for_variant
                            .into_iter()
                            .map(|field| {
                                get_field_tokens(field, &mut prereqs, &mut current_visibility)
                            })
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
                        vec![alignment_in_bytes]
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
                        .unwrap_or_else(|_err| format_ident!("err_struct"));

                        // Get the corresponding field tokens.
                        let fields_for_variant = &tokens_per_variant[variant_index.index()];

                        // Get the aligment of the variant...
                        let variant_alignment =
                            Literal::u64_unsuffixed(variant_alignments[variant_index.index()]);

                        // Create the actual struct, if the variant has size. Otherwise, make
                        // a note that the variant is empty.
                        if variant_sizes[variant_index.index()] == 0 {
                            let cc_variant_name = format_cc_ident(db, variant_def.name.as_str())
                                .unwrap_or_else(|_err| format_ident!("err_field"));
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
                            .unwrap_or_else(|_err| format_ident!("err_field"));
                        let cc_variant_struct_type = format_cc_ident(
                            db,
                            format!("__crubit_{}_struct", variant_def.ident(tcx).as_str()).as_ref(),
                        )
                        .unwrap_or_else(|_err| format_ident!("err_struct"));

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

/// Generates the `(UnsafeRelocateTag, T&&)` constructor for the given ADT.
fn generate_relocating_ctor<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
) -> ApiSnippets {
    let adt_cc_name = &core.cc_short_name;
    let main_api = CcSnippet::with_include(
        quote! {
            #adt_cc_name(::crubit::UnsafeRelocateTag, #adt_cc_name&& value) {
                // This is a bit tricky. Note that the lifetime of `this` has already begun,
                // so memcpy is only being used to copy the object representation.
                //
                // Second, note that the current type is trivially relocatable
                // (because it came from Rust).
                //
                // Finally, note that none of the fields are initialized yet. (Each is in a
                // union.)
                //
                // So while `memcpy` doesn't usually work, it does here.
                memcpy(this, &value, sizeof(value));
            }
        },
        db.support_header("internal/slot.h"),
    );
    ApiSnippets { main_api, ..Default::default() }
}
