// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_abi;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use crate::avoid_colliding_types::{AvoidCollidingTypes, TypeCollisionRisk};
use crate::format_cc_ident;
use crate::format_type::CcParamTy;
use crate::generate_doc_comment;
use crate::generate_function::{
    cc_param_to_c_abi, format_variant_ctor_cc_name, generate_thunk_call, Param, ThunkSelfParameter,
};
use crate::generate_function_thunk::{
    generate_thunk_decl, generate_thunk_impl, replace_all_regions_with_static,
};
use crate::{
    does_type_implement_trait, generate_const, generate_deprecated_tag, generate_must_use_tag,
    generate_trait_thunks, generate_unsupported_def, get_layout, get_scalar_int_type,
    get_tag_size_with_padding, is_bridged_type, is_copy, BridgedBuiltin, RsSnippet, SortedByDef,
    TraitThunks,
};

use arc_anyhow::{Context, Result};
use code_gen_utils::{expect_format_cc_type_name, make_rs_ident, CcInclude};
use database::code_snippet::{
    ApiSnippets, CcPrerequisites, CcSnippet, TemplateSpecialization,
    TraitImplTemplateSpecialization,
};
use database::{AdtCoreBindings, BindingsGenerator, StaticMethodMode, TypeLocation};
use error_report::{anyhow, bail, ensure};
use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream};
use query_compiler::{
    is_c_abi_compatible_by_value, liberate_and_deanonymize_late_bound_regions,
    post_analysis_typing_env, try_normalize,
};
use quote::{format_ident, quote};
#[rustversion::since(2026-05-18)]
use rustc_abi::VariantLayout;
use rustc_abi::{Endian, FieldIdx, FieldsShape, LayoutData, VariantIdx, Variants};

use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::ConstValue;
#[rustversion::since(2026-04-22)]
use rustc_middle::ty::Flags;
use rustc_middle::ty::{self, AssocKind, Ty, TyCtxt, TyKind, TypeFlags, TypingEnv};
use rustc_span::def_id::{CrateNum, DefId, LOCAL_CRATE};
use rustc_span::symbol::{sym, Symbol};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::iter::once;
use std::rc::Rc;

pub(crate) fn has_type_or_const_vars() -> TypeFlags {
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
    let typing_env = bindings
        .def_id
        .map(|id| post_analysis_typing_env(tcx, id))
        .unwrap_or_else(ty::TypingEnv::fully_monomorphized);
    bindings.self_ty.needs_drop(tcx, typing_env)
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
    let field_ty = crate::normalize_ty(
        tcx,
        tcx.param_env(field_def_id),
        tcx.type_of(field_def_id).instantiate_identity(),
    );

    Ok(field_ty)
}

/// Returns the C++ underlying type of the `cpp_enum` struct specified by the given def id.
pub(crate) fn cpp_enum_cpp_underlying_type<'tcx>(
    db: &BindingsGenerator<'tcx>,
    def_id: DefId,
) -> Result<CcSnippet<'tcx>> {
    let field_type = cpp_enum_rust_underlying_type(db.tcx(), def_id)?;
    db.format_ty_for_cc(field_type, TypeLocation::Field)
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
/// enum class MyEnum : ::std::int32_t {
///     VARIANT_0 = 0,
///     VARIANT_1 = 1,
///     // ...
/// };
/// ```
fn generate_cpp_enum<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
) -> ApiSnippets<'tcx> {
    let tcx = db.tcx();
    let enumeration_cc_name = &core.cc_short_name;

    let mut main_api_prereqs = CcPrerequisites::default();
    main_api_prereqs.includes.insert(db.support_header("annotations_internal.h"));

    // Generate relevant attributes.
    let rs_type = core.rs_fully_qualified_name.to_string();
    let mut attributes = vec![quote! {CRUBIT_INTERNAL_RUST_TYPE(#rs_type)}];
    let def_id = core.def_id.expect("cpp_enum requires a valid DefId");
    if let Some(tag) = generate_must_use_tag(tcx, def_id) {
        attributes.push(tag);
    }
    if let Some(tag) = generate_deprecated_tag(tcx, def_id) {
        attributes.push(tag);
    }

    // Generate the enumerator list.
    let enumerator_lines: Vec<TokenStream> = tcx
        .inherent_impls(def_id)
        .iter()
        .copied()
        .sorted_by_def(tcx)
        .flat_map(|impl_id| tcx.associated_items(impl_id).in_definition_order())
        .filter_map(|assoc_item| {
            if !is_supported_associated_item(tcx, assoc_item.def_id) {
                return None;
            }
            let ty::AssocKind::Const { name, .. } = assoc_item.kind else {
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
            let value_kind = *cpp_enum_rust_underlying_type(tcx, def_id).unwrap().kind();
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

    let doc_comment = generate_doc_comment(db, def_id);
    let keyword = &core.keyword;
    let underlying_cc_type_snippet = cpp_enum_cpp_underlying_type(db, def_id).unwrap();
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
        && is_trait_method_of_impl_lookup_stable(tcx, def_id)
}

fn is_trait_method_of_impl_lookup_stable<'tcx>(tcx: TyCtxt<'tcx>, impl_assoc_id: DefId) -> bool {
    let Some(impl_id) = tcx.trait_impl_of_assoc(impl_assoc_id) else {
        // For inherent impls, we don't need to perform this check because the associated item is
        // the only place an unstable attribute could live.
        return true;
    };
    let trait_to_impl_map = tcx.impl_item_implementor_ids(impl_id);
    trait_to_impl_map
        .items()
        .filter_map(
            |(trait_id, impl_id)| {
                if *impl_id == impl_assoc_id {
                    Some(trait_id)
                } else {
                    None
                }
            },
        )
        .get_only()
        .map(|trait_id| {
            tcx.lookup_stability(*trait_id).is_none_or(|stability| stability.is_stable())
        })
        .expect("Associated trait item should be defined on it's trait")
}

pub(crate) fn generate_associated_item<'tcx>(
    db: &BindingsGenerator<'tcx>,
    assoc_item: &ty::AssocItem,
    member_function_names: &mut HashSet<String>,
    method_name_override: Option<&'static str>,
    static_method_mode: StaticMethodMode,
) -> Option<ApiSnippets<'tcx>> {
    let tcx = db.tcx();
    let def_id = assoc_item.def_id;
    if !is_supported_associated_item(tcx, def_id) {
        return None;
    }
    crate::error_scope!(db, def_id);
    let result = match assoc_item.kind {
        ty::AssocKind::Fn { .. } => {
            db.generate_function(def_id, method_name_override, static_method_mode).inspect(|_binding| {
                // If `generate_function` succeeds, record the method in `member_function_names`.
                let unqualified_name = db
                    .symbol_unqualified_name(def_id)
                    .expect("Associated item should have an unqualified name: {def_id:?}");
                let cpp_name = unqualified_name.cpp_name.to_string();
                let was_inserted = member_function_names.insert(cpp_name.clone());
                assert!(
                    was_inserted, // Bindings for Rust/user-named items are given priority.
                    "Unexpected (user-named 'members' are handled first) naming conflict: {cpp_name}",
            );
            })
        }
        ty::AssocKind::Const { .. } => generate_const(db, def_id),
        // TODO: b/405132277 - Rust does not support inherent associated types, but should support
        // associated types when adding traits.
        ty::AssocKind::Type { .. } => {
            assoc_item
                .opt_name()
                .zip(tcx.trait_impl_of_assoc(def_id))
                .ok_or(anyhow!("Associated types with no name are not supported."))
                .and_then(|(name, impl_id)| {
                    #[rustversion::before(2026-04-19)]
                    let trait_ref = tcx.impl_trait_header(impl_id).trait_ref.instantiate_identity();
                    #[rustversion::since(2026-04-19)]
                    let trait_ref = crate::normalize_ty(
                        tcx,
                        tcx.param_env(impl_id),
                        tcx.impl_trait_header(impl_id).trait_ref.instantiate_identity(),
                    );
                    let trait_rs_name = db
                        .symbol_canonical_name(trait_ref.def_id)
                        .expect("Trait impl should have a canonical name.")
                        .format_for_rs();
                    // The first argument of a trait ref is the self type.
                    let self_ty = trait_ref.args.type_at(0);
                    let alias_type = crate::normalize_ty(
                        tcx,
                        tcx.param_env(def_id),
                        tcx.type_of(def_id).instantiate_identity(),
                    );
                    if alias_type.walk().any(|arg| {
                        arg.as_type()
                            .and_then(|ty| ty.ty_adt_def())
                            .is_some_and(|adt| !crate::should_receive_bindings(db, adt.did()))
                    }) {
                        bail!("Associated type `{name}` contains a type that shouldn't receive bindings.");
                    }
                    let rs_type_spelling = format!("<{} as {}>::{}", self_ty, trait_rs_name, name);
                    crate::create_type_alias_with_rs_type(
                        db,
                        def_id,
                        &rs_type_spelling,
                        name.as_str(),
                        alias_type,
                    )
                    .map(|snippet| snippet.into_main_api())
                })
        }
    };
    let result = result.and_then(|snippet| {
        snippet.resolve_feature_requirements(db.crate_features(db.source_crate_num()))
    });
    match result {
        Err(err) => {
            if crubit_attr::get_attrs(tcx, def_id).unwrap().must_bind {
                crate::report_must_bind_error(db, def_id, &err);
            }
            Some(generate_unsupported_def(db, def_id, err).into_main_api())
        }
        Ok(result) => Some(result),
    }
}

fn erase_regions<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
    #[rustversion::any(since(1.94), since(2025-09-10))]
    return tcx.erase_and_anonymize_regions(ty);
    #[rustversion::all(before(1.94), before(2025-09-10))]
    return tcx.erase_regions(ty);
}

fn get_trait_ref_from_impl_id<'tcx>(tcx: TyCtxt<'tcx>, impl_id: DefId) -> ty::TraitRef<'tcx> {
    #[rustversion::any(since(1.94), since(2025-10-17))]
    let middle_trait_header = tcx.impl_trait_header(impl_id);
    #[rustversion::all(before(1.94), before(2025-10-17))]
    let middle_trait_header =
        tcx.impl_trait_header(impl_id).expect("DefId for a trait impl lacked a trait header");
    crate::normalize_ty(
        tcx,
        tcx.param_env(impl_id),
        middle_trait_header.trait_ref.instantiate_identity(),
    )
}

pub fn from_trait_impls_by_argument<'tcx>(
    db: &BindingsGenerator<'tcx>,
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
        let trait_ref = get_trait_ref_from_impl_id(tcx, from_impl_id);
        let ty = trait_ref.args.type_at(1);
        // We want to check if our type has type variables and constant variables, but not
        // region variables. Region variables are fine and we'll replace them with 'static.
        if ty.flags().intersects(has_type_or_const_vars()) {
            continue;
        }

        // We want to work in region-erased types because that's what we will be querying by
        // for lookup.
        let from_self_ty = erase_regions(tcx, ty);
        map.entry(from_self_ty).or_default().push(from_impl_id);
    }
    Rc::new(map)
}

pub fn into_trait_impls_by_destination<'tcx>(
    db: &BindingsGenerator<'tcx>,
    crate_num: CrateNum,
) -> Rc<HashMap<Ty<'tcx>, Vec<DefId>>> {
    let tcx = db.tcx();
    let into_trait = tcx.get_diagnostic_item(sym::Into).expect("Could not find Into trait");
    let impls_iter: Box<dyn Iterator<Item = DefId>> = if crate_num == LOCAL_CRATE {
        Box::new(tcx.local_trait_impls(into_trait).iter().map(|impl_id| impl_id.to_def_id()))
    } else {
        Box::new(
            tcx.implementations_of_trait((crate_num, into_trait))
                .iter()
                .map(|(impl_id, _)| *impl_id),
        )
    };
    let mut map: HashMap<Ty<'tcx>, Vec<DefId>> = HashMap::new();
    for into_impl_id in impls_iter {
        let trait_ref = get_trait_ref_from_impl_id(tcx, into_impl_id);
        let dest_ty = trait_ref.args.type_at(1);
        // We want to check if our type has type variables and constant variables, and still allow
        // region variables.
        if dest_ty.flags().intersects(has_type_or_const_vars()) {
            continue;
        }
        let dest_self_ty = erase_regions(tcx, dest_ty);
        map.entry(dest_self_ty).or_default().push(into_impl_id);
    }
    Rc::new(map)
}

fn generate_into_impls<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
) -> ApiSnippets<'tcx> {
    let tcx = db.tcx();
    let cc_struct_name = &core.cc_short_name;

    let into_trait = tcx.get_diagnostic_item(sym::Into).expect("Could not find Into trait");
    let Some(def_id) = core.def_id else {
        // If we don't have a def_id, we can't generate Into impls.
        return ApiSnippets::default();
    };

    let from_map = db.from_trait_impls_by_argument(def_id.krate);
    let from_impls = from_map.get(&core.self_ty).into_iter().flat_map(|vec| vec.iter()).filter_map(
        |from_impl_id| {
            let trait_ref = get_trait_ref_from_impl_id(tcx, *from_impl_id);
            let from_middle_ty = trait_ref.args.type_at(0);

            // If our type contains type variables or constant variables (but not region variables),
            // we can't generate an `into` impl.
            if from_middle_ty.flags().intersects(has_type_or_const_vars()) {
                return None;
            }
            // We know that our type will always appear in FnReturn position for the `into` method.
            // If our type isn't C++-compatible, we can't generate an `into` impl.
            let cc_ty = db
                .format_ty_for_cc(from_middle_ty, TypeLocation::FnReturn { is_constructor: false })
                .ok()?;
            Some((from_middle_ty, cc_ty, *from_impl_id))
        },
    );
    let into_impls =
        tcx.non_blanket_impls_for_ty(into_trait, core.self_ty).filter_map(|into_impl_id| {
            let trait_ref = get_trait_ref_from_impl_id(tcx, into_impl_id);
            // Index 0 of our trait ref is the self type, so index 1 is the type we're converting
            // into.
            let into_middle_ty = trait_ref.args.type_at(1);
            if into_middle_ty.flags().intersects(has_type_or_const_vars()) {
                return None;
            }

            // If our type isn't C++ compatible, we can't generate an `into` impl.
            let cc_ty = db
                .format_ty_for_cc(into_middle_ty, TypeLocation::FnReturn { is_constructor: false })
                .ok()?;

            Some((into_middle_ty, cc_ty, into_impl_id))
        });

    from_impls
        .chain(into_impls)
        .avoid_colliding_types(tcx, |(middle_ty, _, _)| *middle_ty)
        .into_iter()
        .filter_map(|res| {
            let (middle_ty, cc_ty, def_id) = match res {
                Ok(item) => item,
                Err(TypeCollisionRisk { item: (_, _, def_id), key_type, preferred_type }) => {
                    let err = anyhow!(
                        "Conversion to `{key_type}` is not supported when conversion to \
                         `{preferred_type}` is implemented as they may overlap in C++."
                    );
                    return Some(generate_unsupported_def(db, def_id, err).into_main_api());
                }
            };
            let mut prereqs = CcPrerequisites::default();
            let cc_ty = cc_ty.into_tokens(&mut prereqs);

            // Delay converting this type until we've successfully generated the thunks.
            // We generate thunks for `into` here. This relies on the blanket impls of for `Into` in the stdlib to work.
            let TraitThunks {
                method_name_to_cc_thunk_name,
                cc_thunk_decls,
                rs_thunk_impls: rs_details,
            } = generate_trait_thunks(
                db,
                into_trait,
                &[middle_ty],
                core.self_ty,
                core.def_id,
                core.rs_fully_qualified_name.clone(),
                /*is_constructor=*/ false,
                /*within_template=*/ false,
            )
            .ok()?;

            let thunk_name = method_name_to_cc_thunk_name
                .into_values()
                .exactly_one()
                .expect("Expecting a single `into` method");

            let cc_thunk_decls = cc_thunk_decls.into_tokens(&mut prereqs);
            let doc_comment = generate_doc_comment(db, def_id);

            let self_cpp_ty = db
                .format_ty_for_cc(
                    core.self_ty,
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
                middle_ty,
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
                    ty: core.self_ty,
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

fn is_newtype_relationship<'tcx>(tcx: TyCtxt<'tcx>, tgt_ty: Ty<'tcx>, src_ty: Ty<'tcx>) -> bool {
    if let TyKind::Adt(adt_def, substs) = tgt_ty.kind()
        && adt_def.is_struct()
        && let variants = adt_def.variants()
        && variants.len() == 1
        && let variant = &variants[VariantIdx::from_usize(0)]
        && variant.ctor_kind() == Some(rustc_hir::def::CtorKind::Fn)
        && variant.fields.len() == 1
        && let field = &variant.fields[FieldIdx::from_usize(0)]
        && crate::field_def_is_pub_and_stable(tcx, field).is_ok()
    {
        let field_ty = field.ty(tcx, substs);
        let field_ty = crate::normalize_ty(tcx, tcx.param_env(field.did), field_ty);
        let static_field_ty = replace_all_regions_with_static(tcx, field_ty);
        let static_src_ty = replace_all_regions_with_static(tcx, src_ty);
        static_field_ty == static_src_ty
    } else {
        false
    }
}

fn generate_constructor_impls<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
) -> ApiSnippets<'tcx> {
    let tcx = db.tcx();
    let cc_struct_name = &core.cc_short_name;

    // We need there to be a `def_id` to generate a constructor from.
    let def_id = core.def_id.expect("ADT must have a def_id");

    // Find From impls from the selected ADT
    let from_trait = tcx.get_diagnostic_item(sym::From).expect("Could not find From trait");
    let from_impls = tcx.non_blanket_impls_for_ty(from_trait, core.self_ty).filter_map(|impl_id| {
        let trait_ref = get_trait_ref_from_impl_id(tcx, impl_id);
        let src_ty = trait_ref.args.type_at(1);
        if src_ty.flags().intersects(has_type_or_const_vars()) {
            return None;
        }
        // Skip generating a constructor for the `From` impl if the target type is a
        // newtype with a public field of the same type. In this case, we already
        // synthesize a tuple constructor with the same signature, and generating
        // another one would cause duplicate definition conflicts in C++.
        if is_newtype_relationship(tcx, core.self_ty, src_ty) {
            return None;
        }
        // Skip if source type is Self or a reference to Self (e.g. &Self)
        let src_referent_ty = match src_ty.kind() {
            ty::TyKind::Ref(_, referent_ty, _) => *referent_ty,
            _ => src_ty,
        };
        if src_referent_ty == core.self_ty {
            return None;
        }
        let cc_ty = db
            .format_ty_for_cc(
                src_ty,
                TypeLocation::FnParam { is_self_param: false, elided_is_output: false },
            )
            .ok()?;

        Some((src_ty, cc_ty, impl_id, /*is_from=*/ true))
    });

    // Find Into impls to the selected ADT
    let into_map = db.into_trait_impls_by_destination(def_id.krate);
    let into_impls =
        into_map.get(&core.self_ty).into_iter().flat_map(|vec| vec.iter()).filter_map(|impl_id| {
            let trait_ref = get_trait_ref_from_impl_id(tcx, *impl_id);
            let src_ty = trait_ref.args.type_at(0);
            if src_ty.flags().intersects(has_type_or_const_vars()) {
                return None;
            }
            // Skip if source type is Self or a reference to Self (e.g. &Self)
            let src_referent_ty = match src_ty.kind() {
                ty::TyKind::Ref(_, referent_ty, _) => *referent_ty,
                _ => src_ty,
            };
            if src_referent_ty == core.self_ty {
                return None;
            }
            let cc_ty = db
                .format_ty_for_cc(
                    src_ty,
                    TypeLocation::FnParam { is_self_param: false, elided_is_output: false },
                )
                .ok()?;

            Some((src_ty, cc_ty, *impl_id, /*is_from=*/ false))
        });

    // Avoid collisions in cases where types may map to the same underlying C++ type.
    from_impls
        .chain(into_impls)
        .avoid_colliding_types(tcx, |(src_ty, _, _, _)| *src_ty)
        .into_iter()
        .filter_map(|res| {
            let (src_ty, cc_ty, impl_id, is_from) = match res {
                Ok(item) => item,
                Err(TypeCollisionRisk {
                    item: (_, _, impl_id, is_from),
                    key_type,
                    preferred_type,
                }) => {
                    let trait_name = if is_from { "From" } else { "Into" };
                    let err = anyhow!(
                        "{trait_name} implementation for `{key_type}` is not supported when \
                         `{trait_name}<{preferred_type}>` is implemented as it may overlap."
                    );
                    return Some(generate_unsupported_def(db, impl_id, err).into_main_api());
                }
            };
            let mut prereqs = CcPrerequisites::default();
            let cc_ty = cc_ty.into_tokens(&mut prereqs);
            // Generate thunk names, declarations, and implementations.
            // `From` and `Into` are handled differently Self is different for both
            let (thunk_name, cc_thunk_decls, rs_details) = if is_from {
                // The ADT is the Self in the From impl case so we just use generate_trait_thunks
                let TraitThunks {
                    method_name_to_cc_thunk_name,
                    cc_thunk_decls,
                    rs_thunk_impls: rs_details,
                } = generate_trait_thunks(
                    db,
                    from_trait,
                    &[src_ty],
                    core.self_ty,
                    core.def_id,
                    core.rs_fully_qualified_name.clone(),
                    /*is_constructor=*/ true,
                    /*within_template=*/ false,
                )
                .ok()?;
                let thunk_name = method_name_to_cc_thunk_name
                    .into_values()
                    .exactly_one()
                    .expect("Expecting a single `from` method");
                (thunk_name, cc_thunk_decls, rs_details)
            } else {
                // Since self is not necessarily the ADT, we need to manually construct the thunk
                let into_trait =
                    tcx.get_diagnostic_item(sym::Into).expect("Could not find Into trait");
                let into_trait_assoc_fn = tcx
                    .associated_items(into_trait)
                    .in_definition_order()
                    .find(|item| matches!(item.kind, ty::AssocKind::Fn { .. }))
                    .expect("Into should have a method");

                let trait_args = tcx.mk_args_trait(src_ty, once(core.self_ty.into()));
                let sig = tcx.fn_sig(into_trait_assoc_fn.def_id).instantiate(tcx, trait_args);

                let sig = liberate_and_deanonymize_late_bound_regions(
                    tcx,
                    sig,
                    into_trait_assoc_fn.def_id,
                );
                let sig = try_normalize(
                    tcx,
                    ty::PseudoCanonicalInput {
                        typing_env: TypingEnv::fully_monomorphized(),
                        value: sig,
                    },
                )
                .ok()?;

                // Just a small unique name for the custom Into thunk
                let thunk_name = if db.is_golden_test() {
                    format!(
                        "__crubit_thunk_into_{}_as_{}",
                        code_gen_utils::escape_non_identifier_chars(&format!("{}", src_ty)),
                        code_gen_utils::escape_non_identifier_chars(&format!("{}", core.self_ty))
                    )
                } else {
                    format!(
                        "__crubit_thunk_{:x}_into_{}_as_{}",
                        tcx.stable_crate_id(db.source_crate_num()),
                        code_gen_utils::escape_non_identifier_chars(&format!("{}", src_ty)),
                        code_gen_utils::escape_non_identifier_chars(&format!("{}", core.self_ty))
                    )
                };
                let thunk_name_cc_ident = format_cc_ident(db, &thunk_name).ok()?;
                let cc_thunk_decls = generate_thunk_decl(
                    db,
                    &sig,
                    &thunk_name_cc_ident,
                    /*has_self_param=*/ true,
                    /*is_constructor=*/ true,
                    /*within_template=*/ false,
                )
                .ok()?;
                let static_src_ty = replace_all_regions_with_static(tcx, src_ty);
                let src_rs = db.format_ty_for_rs(static_src_ty).ok()?;
                let foo_rs = &core.rs_fully_qualified_name;
                let fully_qualified_fn_name =
                    quote! { <#src_rs as ::core::convert::Into<#foo_rs>>::into };
                let rs_details = generate_thunk_impl(
                    db,
                    into_trait_assoc_fn.def_id,
                    &sig,
                    &thunk_name,
                    fully_qualified_fn_name,
                    /*is_constructor=*/ true,
                )
                .ok()?;
                (thunk_name_cc_ident, cc_thunk_decls, rs_details)
            };
            let cc_thunk_decls = cc_thunk_decls.into_tokens(&mut prereqs);
            let doc_comment = generate_doc_comment(db, impl_id);
            let mut statements = quote! {};
            let c_abi_expression = cc_param_to_c_abi(
                db,
                format_ident!("value"),
                src_ty,
                ty::TypingEnv::fully_monomorphized(),
                &mut prereqs.includes,
                &mut statements,
            )
            .ok()?;

            let returns_by_value = is_c_abi_compatible_by_value(tcx, core.self_ty);
            let impl_body_tokens = if returns_by_value {
                quote! {
                    #statements
                    *this = __crubit_internal::#thunk_name(#c_abi_expression);
                }
            } else {
                quote! {
                    #statements
                    __crubit_internal::#thunk_name(#c_abi_expression, this);
                }
            };
            prereqs.move_defs_to_fwd_decls();
            Some(ApiSnippets {
                main_api: CcSnippet {
                    tokens: quote! {
                        __NEWLINE__ #doc_comment
                        explicit #cc_struct_name ( #cc_ty value ) ; __NEWLINE__
                        __NEWLINE__
                    },
                    prereqs,
                },
                cc_details: CcSnippet::new(quote! {
                    #cc_thunk_decls
                    inline #cc_struct_name :: #cc_struct_name ( #cc_ty value ) {
                        #impl_body_tokens
                    }
                }),
                rs_details,
            })
        })
        .collect()
}

fn generate_trait_operator_impls<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
) -> ApiSnippets<'tcx> {
    let tcx = db.tcx();

    let query_trait_impls = |trait_def_id: DefId,
                             method_name: &str,
                             operator_name: &'static str|
     -> Vec<ApiSnippets<'_>> {
        let trait_name = tcx.item_name(trait_def_id).to_string();

        tcx.non_blanket_impls_for_ty(trait_def_id, core.self_ty)
            .map(|impl_id| {
                let trait_ref = get_trait_ref_from_impl_id(tcx, impl_id);
                // Index 0 of our trait ref is the self type.
                // If there is a second argument, it is the real type arg (e.g. `T` in `Index<T>`).
                // Otherwise (e.g. for `Neg` or `Not`), we use the self type as a placeholder.
                let trait_args = trait_ref.args;
                let trait_arg_ty = if trait_args.len() > 1 {
                    trait_args.type_at(1)
                } else {
                    trait_args.type_at(0)
                };
                (impl_id, trait_arg_ty)
            })
            .avoid_colliding_types(tcx, |(_impl_id, trait_arg_ty)| *trait_arg_ty)
            .into_iter()
            .map(|res| {
                res.map_err(|TypeCollisionRisk { item: (impl_id, _), key_type, preferred_type }| {
                    let err = anyhow!(
                        "{trait_name} implementation for `{key_type}` is not supported when \
                         `{trait_name}<{preferred_type}>` is implemented as it may overlap."
                    );
                    (err, impl_id)
                })
                .and_then(|(impl_id, trait_arg_ty)| {
                    if trait_arg_ty.flags().intersects(has_type_or_const_vars()) {
                        let err = anyhow!(
                            "{trait_name} impl has uninstantiated generic parameters, \
                                   which is not yet supported {trait_arg_ty}"
                        );
                        return Err((err, impl_id));
                    }

                    let assoc_fn_id = tcx
                        .associated_items(impl_id)
                        .in_definition_order()
                        .find(|item| {
                            item.name().as_str() == method_name
                                && matches!(item.kind, AssocKind::Fn { .. })
                        })
                        .unwrap_or_else(|| {
                            panic!("Caller should ensure {trait_name} has method {method_name}");
                        })
                        .def_id;
                    db.generate_function(assoc_fn_id, Some(operator_name), StaticMethodMode::Infer)
                        .map_err(|e| (e, assoc_fn_id))
                })
                .unwrap_or_else(|(err, def_id)| {
                    generate_unsupported_def(db, def_id, err).into_main_api()
                })
            })
            .collect()
    };

    [
        query_trait_impls(
            tcx.lang_items().index_trait().expect("Could not find Index trait"),
            "index",
            "operator[]",
        ),
        query_trait_impls(
            tcx.lang_items().index_mut_trait().expect("Could not find IndexMut trait"),
            "index_mut",
            "operator[]",
        ),
        query_trait_impls(
            tcx.get_diagnostic_item(sym::PartialEq).expect("Could not find PartialEq trait"),
            "eq",
            "operator==",
        ),
        query_trait_impls(
            tcx.lang_items().add_trait().expect("Could not find Add trait"),
            "add",
            "operator+",
        ),
        query_trait_impls(
            tcx.lang_items().add_assign_trait().expect("Could not find AddAssign trait"),
            "add_assign",
            "operator+=",
        ),
        query_trait_impls(
            tcx.lang_items().bitand_trait().expect("Could not find BitAnd trait"),
            "bitand",
            "operator&",
        ),
        query_trait_impls(
            tcx.lang_items().bitand_assign_trait().expect("Could not find BitAndAssign trait"),
            "bitand_assign",
            "operator&=",
        ),
        query_trait_impls(
            tcx.lang_items().bitor_trait().expect("Could not find BitOr trait"),
            "bitor",
            "operator|",
        ),
        query_trait_impls(
            tcx.lang_items().bitor_assign_trait().expect("Could not find BitOrAssign trait"),
            "bitor_assign",
            "operator|=",
        ),
        query_trait_impls(
            tcx.lang_items().bitxor_trait().expect("Could not find BitXor trait"),
            "bitxor",
            "operator^",
        ),
        query_trait_impls(
            tcx.lang_items().bitxor_assign_trait().expect("Could not find BitXorAssign trait"),
            "bitxor_assign",
            "operator^=",
        ),
        query_trait_impls(
            tcx.lang_items().div_trait().expect("Could not find Div trait"),
            "div",
            "operator/",
        ),
        query_trait_impls(
            tcx.lang_items().div_assign_trait().expect("Could not find DivAssign trait"),
            "div_assign",
            "operator/=",
        ),
        query_trait_impls(
            tcx.lang_items().mul_trait().expect("Could not find Mul trait"),
            "mul",
            "operator*",
        ),
        query_trait_impls(
            tcx.lang_items().mul_assign_trait().expect("Could not find MulAssign trait"),
            "mul_assign",
            "operator*=",
        ),
        query_trait_impls(
            tcx.lang_items().neg_trait().expect("Could not find Neg trait"),
            "neg",
            "operator-",
        ),
        query_trait_impls(
            tcx.lang_items().not_trait().expect("Could not find Not trait"),
            "not",
            "operator!",
        ),
        query_trait_impls(
            tcx.lang_items().rem_trait().expect("Could not find Rem trait"),
            "rem",
            "operator%",
        ),
        query_trait_impls(
            tcx.lang_items().rem_assign_trait().expect("Could not find RemAssign trait"),
            "rem_assign",
            "operator%=",
        ),
        query_trait_impls(
            tcx.lang_items().shl_trait().expect("Could not find Shl trait"),
            "shl",
            "operator<<",
        ),
        query_trait_impls(
            tcx.lang_items().shl_assign_trait().expect("Could not find ShlAssign trait"),
            "shl_assign",
            "operator<<=",
        ),
        query_trait_impls(
            tcx.lang_items().shr_trait().expect("Could not find Shr trait"),
            "shr",
            "operator>>",
        ),
        query_trait_impls(
            tcx.lang_items().shr_assign_trait().expect("Could not find ShrAssign trait"),
            "shr_assign",
            "operator>>=",
        ),
        query_trait_impls(
            tcx.lang_items().sub_trait().expect("Could not find Sub trait"),
            "sub",
            "operator-",
        ),
        query_trait_impls(
            tcx.lang_items().sub_assign_trait().expect("Could not find SubAssign trait"),
            "sub_assign",
            "operator-=",
        ),
        // TODO(b/483382648): Add support for other traits / operators - e.g. `PartialOrd`,
        // (`operator<`, `operator<=`, etc., `operator<=>` seems hard)
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn generate_display_impl<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
) -> ApiSnippets<'tcx> {
    let tcx = db.tcx();
    let Some(def_id) = core.def_id else {
        return ApiSnippets::default();
    };
    let err_snippets = |err| generate_unsupported_def(db, def_id, err).into_main_api();
    let Some(display_trait_id) = tcx.get_diagnostic_item(sym::Display) else {
        return err_snippets(anyhow!(
            "Internal Crubit Error: `std::fmt::Display` trait not found."
        ));
    };

    if !does_type_implement_trait(tcx, core.self_ty, display_trait_id, []) {
        // This isn't an error, our type doesn't implement `Display` and so we don't generate
        // any bindings.
        return ApiSnippets::default();
    }

    let crate_name = tcx.crate_name(db.source_crate_num());
    // Omit Display implementations for the `core` crate, as formatting via
    // `Display` delegates to `ToString` which requires `alloc::string::String`
    // (which is not available in `core` as it doesn't support heap allocation).
    if crate_name.as_str() == "core" {
        return err_snippets(anyhow!("`core` crate does not support `std::fmt::Display` because it requires `alloc::string::String` which is not available in `core`"));
    }

    let Some(to_string_trait_id) = tcx.get_diagnostic_item(Symbol::intern("ToString")) else {
        return err_snippets(anyhow!("Internal Crubit Error: `ToString` trait not found."));
    };

    let TraitThunks { method_name_to_cc_thunk_name, cc_thunk_decls, rs_thunk_impls: rs_details } =
        match generate_trait_thunks(
            db,
            to_string_trait_id,
            &[],
            core.self_ty,
            core.def_id,
            core.rs_fully_qualified_name.clone(),
            /*is_constructor=*/ false,
            /*within_template=*/ true,
        ) {
            Ok(thunks) => thunks,
            Err(err) => return err_snippets(err),
        };

    let to_string_thunk_name = method_name_to_cc_thunk_name
        .into_values()
        .exactly_one()
        .expect("Expecting a single `to_string` method");
    let adt_cc_short_name = &core.cc_short_name;

    let main_api = CcSnippet::new(quote! {
        __NEWLINE__ __COMMENT__ "AbslStringify and std::ostream support via std::fmt::Display"
        template <typename Sink, typename Str = rs::alloc::string::String>
        friend void AbslStringify(Sink& sink, const #adt_cc_short_name& self) {
            crubit::Slot<Str> s;
            #to_string_thunk_name(self, s.Get());
            AbslStringify(sink, ::std::move(s).AssumeInitAndTakeValue().as_str());
        }
        __NEWLINE__
        template <typename Str = rs::alloc::string::String>
        friend ::std::ostream& operator<<(::std::ostream& os, const #adt_cc_short_name& self) {
            crubit::Slot<Str> s;
            #to_string_thunk_name(self, s.Get());
            return os << ::std::string_view(::std::move(s).AssumeInitAndTakeValue().as_str());
        }
        __NEWLINE__
    });

    let cc_details = {
        let mut prereqs = CcPrerequisites::default();
        if let Some(includes) = db.crate_name_to_include_paths().get("alloc") {
            prereqs.includes.extend(includes.iter().cloned());
        }
        prereqs.includes.insert(CcInclude::SystemHeader("string_view".into()));
        prereqs.includes.insert(db.support_header("internal/slot.h"));
        prereqs.includes.insert(CcInclude::SystemHeader("utility".into()));
        prereqs.includes.insert(CcInclude::SystemHeader("ostream".into()));
        let cc_thunk_decls = cc_thunk_decls.into_tokens(&mut prereqs);
        CcSnippet { tokens: cc_thunk_decls, prereqs }
    };

    ApiSnippets { main_api, cc_details, rs_details }
}

/// Formats an algebraic data type (an ADT - a struct, an enum, or a union)
/// represented by `core`.  This function is infallible - after
/// `generate_adt_core` returns success we have committed to emitting C++
/// bindings for the ADT.
pub fn generate_adt<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
) -> ApiSnippets<'tcx> {
    let tcx = db.tcx();
    let adt_cc_name = &core.cc_short_name;

    // Handle `cpp_enum` structs.
    let crubit_attrs =
        core.def_id.and_then(|id| crubit_attr::get_attrs(tcx, id).ok()).unwrap_or_default();
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
        } = generate_trait_thunks(
            db,
            drop_trait_id,
            &[],
            core.self_ty,
            core.def_id,
            core.rs_fully_qualified_name.clone(),
            /*is_constructor=*/ false,
            /*within_template=*/ false,
        )
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
            quote! { static_assert(::std::is_trivially_destructible_v<#adt_cc_name>); },
            CcInclude::type_traits(),
        );
        ApiSnippets { main_api, cc_details, ..Default::default() }
    };

    let copy_ctor_and_assignment_snippets =
        db.generate_copy_ctor_and_assignment_operator(core.clone()).unwrap_or_else(|err| err);

    let move_ctor_and_assignment_snippets = db
        .generate_move_ctor_and_assignment_operator(core.clone())
        .unwrap_or_else(|err| err.explicitly_deleted);

    let relocating_ctor_snippets = generate_relocating_ctor(db, &core.cc_short_name);

    let mut member_function_names = HashSet::<String>::new();
    let impl_items_snippets = core
        .def_id
        .map(|id| tcx.inherent_impls(id))
        .unwrap_or_default()
        .iter()
        .copied()
        .sorted_by_def(tcx)
        .flat_map(|impl_id| tcx.associated_items(impl_id).in_definition_order())
        .flat_map(|assoc_item| {
            generate_associated_item(
                db,
                assoc_item,
                &mut member_function_names,
                None,
                StaticMethodMode::Infer,
            )
        })
        .collect();

    let adt_based_ctors = generate_adt_based_ctors(db, core.clone(), &mut member_function_names);
    let into_operator_snippets = generate_into_impls(db, core.as_ref());
    let trait_operator_snippets = generate_trait_operator_impls(db, core.as_ref());
    let constructor_operator_snippets = generate_constructor_impls(db, core.as_ref());
    let display_snippets = generate_display_impl(db, core.as_ref());
    let into_iterator_snippets = generate_into_iterator_impls(
        db,
        core.as_ref(),
        &mut member_function_names,
    )
    .unwrap_or_else(|err| {
        generate_unsupported_def(db, core.def_id.expect("DefId should be present for an ADT"), err)
            .into_main_api()
    });

    let ApiSnippets {
        main_api: public_functions_main_api,
        cc_details: public_functions_cc_details,
        rs_details: public_functions_rs_details,
    } = [
        default_ctor_snippets,
        adt_based_ctors,
        destructor_snippets,
        move_ctor_and_assignment_snippets,
        copy_ctor_and_assignment_snippets,
        relocating_ctor_snippets,
        impl_items_snippets,
        into_operator_snippets,
        trait_operator_snippets,
        constructor_operator_snippets,
        display_snippets,
        into_iterator_snippets,
    ]
    .into_iter()
    .collect();

    let repr_attrs = core.def_id.map(|id| db.repr_attrs(id)).unwrap_or_default();

    let ApiSnippets {
        main_api: mut fields_main_api,
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

    fields_main_api.prereqs.forward_declare_type(core.self_ty);

    let alignment = Literal::u64_unsuffixed(core.alignment_in_bytes);
    let size = Literal::u64_unsuffixed(core.size_in_bytes);
    let main_api = {
        let rs_type = core.rs_fully_qualified_name.to_string();
        let mut attributes = vec![
            quote! {CRUBIT_INTERNAL_RUST_TYPE(#rs_type)},
            quote! {alignas(#alignment)},
            quote! {[[clang::trivial_abi]]},
        ];
        if !core.lifetime_params.is_empty() {
            let lifetimes = core
                .lifetime_params
                .iter()
                .map(|lt| proc_macro2::Literal::string(lt))
                .collect::<Vec<_>>();
            attributes.push(quote! { CRUBIT_LIFETIME_PARAMS(#(#lifetimes),*) });
        }
        if core
            .def_id
            .map(|id| db.repr_attrs(id).to_vec())
            .unwrap_or_default()
            .iter()
            .any(|repr| matches!(repr, rustc_hir::attrs::ReprPacked { .. }))
        {
            attributes.push(quote! { __attribute__((packed)) })
        }

        // Additional attributes
        if let Some(def_id) = core.def_id {
            if let Some(tag) = generate_must_use_tag(tcx, def_id) {
                attributes.push(tag);
            }
            if let Some(tag) = generate_deprecated_tag(tcx, def_id) {
                attributes.push(tag);
            }
        }

        let doc_comment = core.def_id.map(|id| generate_doc_comment(db, id)).unwrap_or_default();
        let keyword = &core.keyword;

        let mut prereqs = CcPrerequisites::default();
        prereqs.includes.insert(db.support_header("annotations_internal.h"));
        let public_functions_main_api = public_functions_main_api.into_tokens(&mut prereqs);
        let fields_main_api = fields_main_api.into_tokens(&mut prereqs);
        if let Some(def_id) = core.def_id {
            prereqs.fwd_decls.remove(&def_id);
        }

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
        if let Some(def_id) = core.def_id {
            prereqs.defs.insert(def_id);
        }
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
    db: &BindingsGenerator<'tcx>,
    def_id: DefId,
) -> Result<Rc<AdtCoreBindings<'tcx>>> {
    let tcx = db.tcx();
    let attributes = crubit_attr::get_attrs(tcx, def_id).unwrap();

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

    db.generate_adt_core(def_id)
}

/// Implementation of `BindingsGenerator::generate_adt_core`.
pub fn generate_adt_core<'tcx>(
    db: &BindingsGenerator<'tcx>,
    def_id: DefId,
) -> Result<Rc<AdtCoreBindings<'tcx>>> {
    let tcx = db.tcx();
    // Note: we erase regions in order to get bindings regardless of what lifetime parameters are
    // present. We want to generate bindings for functions regardless of their lifetime bounds, as
    // C++ cannot special-case the availability of a function based on lifetimes.
    #[rustversion::before(2026-04-19)]
    let self_ty = erase_regions(tcx, tcx.type_of(def_id).instantiate_identity());
    #[rustversion::since(2026-04-19)]
    let self_ty = erase_regions(
        tcx,
        crate::normalize_ty(tcx, tcx.param_env(def_id), tcx.type_of(def_id).instantiate_identity()),
    );
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
    let cc_fully_qualified_name = fully_qualified_name.format_for_cc(db).with_context(|| {
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

    let generics = tcx.generics_of(def_id);
    let lifetime_params = generics
        .own_params
        .iter()
        .filter(|p| matches!(p.kind, ty::GenericParamDefKind::Lifetime))
        .map(|p| p.name.as_str().strip_prefix('\'').unwrap_or("").to_string())
        .collect::<Vec<_>>();

    Ok(Rc::new(AdtCoreBindings {
        def_id: Some(def_id),
        keyword,
        cc_short_name: cpp_name,
        rs_fully_qualified_name,
        cc_fully_qualified_name,
        lifetime_params,
        self_ty,
        alignment_in_bytes,
        size_in_bytes,
    }))
}

fn anonymous_field_ident(index: usize) -> Ident {
    format_ident!("__field{index}")
}

/// Generates C++ bindings that are equivalent to using Rust constructors.
///
/// For example:
/// * `MyTupleStruct(1,2,3)` is exposed as a C++ constructor that takes field values as arguments.
/// * crubit.rs-enum-ctor-name-and-shape discusses provoding bindings for
///   `MyEnum::NoPayloadVariant` (b/487399481) and
///   `MyEnum::TuplePayloadVariant(1, 2, 3)` (b/487356976)
fn generate_adt_based_ctors<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
    member_function_names: &mut HashSet<String>,
) -> ApiSnippets<'tcx> {
    let TyKind::Adt(adt_def, _) = core.self_ty.kind() else {
        panic!("Attempted to generate constructor for a non-ADT type: {:?}", core.self_ty)
    };

    // Silently suppress errors for synthesized constructors (e.g. a tuple struct constructor
    // doesn't really correspond to an explicit Rust API, so we don't report errors about
    // generating bindings for such "API").
    let should_suppress_errors =
        matches!(adt_def.adt_kind(), ty::AdtKind::Struct | ty::AdtKind::Union);

    adt_def
        .variants()
        .iter_enumerated()
        .map(|(variant_index, variant)| {
            generate_variant_ctor(db, core.clone(), member_function_names, variant_index, variant)
                .unwrap_or_else(|err| {
                    if should_suppress_errors {
                        Default::default()
                    } else {
                        generate_unsupported_def(db, variant.def_id, err).into_main_api()
                    }
                })
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum EnumKind {
    /// Enum with `#[repr(C)]` where bindings get `tag` and "payload" fields.
    ReprC,
    /// Enums (e.g. `#[repr(Rust)]` and `#[repr(u32)]`) that are represented as a blob of bytes
    /// (i.e. bindings only have a single, private `__opaque_blob_of_bytes` field).
    OpaqueBlobOfBytes,
}
fn get_enum_kind<'tcx>(
    db: &BindingsGenerator<'tcx>,
    adt_def: ty::AdtDef<'tcx>,
) -> Option<EnumKind> {
    if !adt_def.is_enum() {
        return None;
    }
    let repr_attrs = db.repr_attrs(adt_def.did());
    if repr_attrs.contains(&rustc_hir::attrs::ReprC) {
        Some(EnumKind::ReprC)
    } else {
        Some(EnumKind::OpaqueBlobOfBytes)
    }
}

fn generate_variant_ctor<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
    member_function_names: &mut HashSet<String>,
    variant_index: VariantIdx,
    variant: &'tcx ty::VariantDef,
) -> Result<ApiSnippets<'tcx>> {
    let tcx = db.tcx();

    if variant.is_field_list_non_exhaustive() {
        // If the definition is marked #[non_exhaustive], don't generate a C++ constructor.
        // #[non_exhaustive] tuple structs do not have a public synthesized constructor.
        bail!("`#[non_exhaustive]` structs don't get public constructors");
    }

    let TyKind::Adt(adt_def, adt_generic_args) = core.self_ty.kind() else {
        panic!("Attempted to generate constructor for a non-ADT type: {:?}", core.self_ty)
    };

    let default_trait_id = tcx.get_diagnostic_item(sym::Default).expect("Default trait not found");
    let clone_trait_id = tcx.lang_items().copy_trait().expect("Copy trait not found");
    let unpin_trait_id = tcx.lang_items().unpin_trait().expect("Unpin trait not found");

    let field_tys = variant
        .fields
        .iter()
        .map(|field_def| {
            if let Err(private_or_unstable) = crate::field_def_is_pub_and_stable(tcx, field_def) {
                // If our synthesized constructor would have a non public
                // visibility, don't generate it as we can't mirror that
                // visibility in C++.
                bail!("Field `{}` is {private_or_unstable}", field_def.name)
            }
            let ty = field_def.ty(tcx, adt_generic_args);
            #[rustversion::since(2026-05-13)]
            let ty = crate::normalize_ty(tcx, tcx.param_env(field_def.did), ty);

            let is_default =
                query_compiler::does_type_implement_trait(tcx, ty, default_trait_id, []);
            let is_clone = query_compiler::does_type_implement_trait(tcx, ty, clone_trait_id, []);
            let is_unpin = query_compiler::does_type_implement_trait(tcx, ty, unpin_trait_id, []);
            let is_movable_in_cpp = (is_default && is_unpin) || is_clone;
            if !is_movable_in_cpp {
                // If one of our fields isn't movable in C++, we can't generate a C++ constructor.
                bail!("Field `{}` is not movable in C++", field_def.name)
            }

            Ok(ty)
        })
        .collect::<Result<Vec<Ty<'tcx>>>>()?;

    // If we fail to convert a field type, don't generate a constructor.
    // Our uncovertible fields will be replaced by a blob of bytes that we do not want to appear
    // in our API, so opt to avoid presenting a constructor for types that contain a blob of
    // bytes.
    let mut main_api_prereqs = CcPrerequisites::default();
    let main_api_params = field_tys
        .into_iter()
        .enumerate()
        .map(|(i, field_ty)| {
            let cpp_type = db.format_ty_for_cc(field_ty, TypeLocation::Other)?;
            let cc_name = anonymous_field_ident(i);
            let cpp_type = cpp_type.into_tokens(&mut main_api_prereqs);
            Ok(quote! { #cpp_type #cc_name })
        })
        .collect::<Result<Vec<TokenStream>>>()?;

    let mut prereqs = main_api_prereqs.clone();
    prereqs.move_defs_to_fwd_decls();

    let Some(ctor_def_id) = variant.ctor_def_id() else {
        bail!("Constructing non-tuple, struct-like enum variants is not supported: b/487357254");
    };

    let adt_cc_name = &core.cc_short_name;
    match adt_def.adt_kind() {
        ty::AdtKind::Struct => {
            let explicit = (main_api_params.len() == 1).then_some(quote! { explicit });
            let initializer_list = (0..main_api_params.len()).map(|i| {
                let cc_name = anonymous_field_ident(i);
                quote! { #cc_name ( ::std::move ( #cc_name ) ) }
            });
            Ok(ApiSnippets {
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
        ty::AdtKind::Enum => {
            let method_name = format_variant_ctor_cc_name(variant.name.as_str());
            if member_function_names.contains(&method_name) {
                bail!("Conflicting member function name: {method_name}");
            }
            let mut mark_method_name_as_used = || {
                let was_inserted = member_function_names.insert(method_name.clone());
                assert!(was_inserted, "Conflicting names rejected earlier (above)");
            };
            if !main_api_params.is_empty() {
                let result = db.generate_function(ctor_def_id, None, StaticMethodMode::Infer);
                if result.is_ok() {
                    mark_method_name_as_used();
                }
                return result;
            }
            let enum_kind = get_enum_kind(db, *adt_def).expect("AtdKindEnum implied EnumKind");
            let body = match enum_kind {
                EnumKind::ReprC => {
                    let discr = core
                        .self_ty
                        .discriminant_for_variant(tcx, variant_index)
                        .expect("Invalid VariantIdx");
                    let (discr_size, _signed) = discr.ty.int_size_and_signed(tcx);
                    let (scalar_int, _) = ty::ScalarInt::truncate_from_uint(discr.val, discr_size);
                    let tag_literal =
                        scalar_value_to_string(tcx, Scalar::Int(scalar_int), *discr.ty.kind())
                            .expect("tag to be a valid scalar constant")
                            .parse::<TokenStream>()
                            .expect("tag string to consist of valid scalar tokens");
                    quote! {
                        return #adt_cc_name(PrivateTagCtorTag{}, Tag { #tag_literal });
                    }
                }
                EnumKind::OpaqueBlobOfBytes => {
                    let (tag_val, tag_size, tag_offset) = {
                        let layout =
                            get_layout(tcx, core.self_ty).expect("Should verify layout earlier");
                        match layout.variants() {
                            rustc_abi::Variants::Empty => {
                                unreachable!("Uninhabited types should be rejected earlier")
                            }
                            rustc_abi::Variants::Single { .. } => (0, rustc_abi::Size::ZERO, 0),
                            rustc_abi::Variants::Multiple { tag_field, .. } => {
                                let typing_env = post_analysis_typing_env(
                                    tcx,
                                    core.def_id.expect("Enums must have a DefId"),
                                );
                                let tag = tcx.tag_for_variant(typing_env.as_query_input((
                                    tcx.erase_and_anonymize_regions(core.self_ty),
                                    variant_index,
                                )));
                                let (val, size) = match tag {
                                    Some(tag) => (tag.to_bits(tag.size()), tag.size()),
                                    None => unreachable!("Multiple variants must have a tag"),
                                };
                                let offset =
                                    layout.fields().offset(tag_field.as_usize()).bytes() as usize;
                                (val, size, offset)
                            }
                        }
                    };
                    let adt_size = core.size_in_bytes as usize;
                    let discr_bytesize = (adt_size - tag_offset).min(tag_size.bytes() as usize);
                    let tag_bytes = match tcx.sess.target.endian {
                        Endian::Little => &tag_val.to_le_bytes()[..discr_bytesize],
                        Endian::Big => {
                            &tag_val.to_be_bytes()[std::mem::size_of::<u128>() - discr_bytesize..]
                        }
                    };
                    let bytes = {
                        let mut bytes = vec![0; adt_size];
                        bytes[tag_offset..tag_offset + tag_bytes.len()].copy_from_slice(tag_bytes);
                        bytes.into_iter().map(Literal::u8_unsuffixed).collect_vec()
                    };
                    quote! {
                        return #adt_cc_name(PrivateBytesTag{}, { #( #bytes ),* });
                    }
                }
            };
            let constexpr = if adt_core_bindings_needs_drop(&core, tcx) {
                // TODO(b/489085607): If we can make destructor `constexpr` (see bug for ideas)
                // then `constexpr` *here* can be used unconditionally.
                quote! {}
            } else {
                quote! { constexpr }
            };
            let doc_comment = generate_doc_comment(db, variant.def_id);
            let method_name = format_cc_ident(db, &method_name)?;
            mark_method_name_as_used();
            Ok(ApiSnippets {
                main_api: CcSnippet {
                    prereqs,
                    tokens: quote! {
                        __NEWLINE__ #doc_comment
                        static #constexpr #adt_cc_name #method_name();
                        __NEWLINE__
                    },
                },
                cc_details: CcSnippet::new(quote! {
                    __NEWLINE__
                    __COMMENT__ "`static` constructor"
                    inline #constexpr #adt_cc_name #adt_cc_name::#method_name() { #body }
                    __NEWLINE__
                }),
                ..Default::default()
            })
        }
        ty::AdtKind::Union => bail!("Crubit doesn't provide bindings for constructing unions"),
    }
}

struct FieldTypeInfo<'tcx> {
    size: u64,
    cpp_type: CcSnippet<'tcx>,
}

struct Field<'tcx> {
    type_info: Result<FieldTypeInfo<'tcx>>,
    cc_name: Ident,
    rs_name: TokenStream,
    is_public: bool,
    index: usize,
    offset: u64,
    offset_of_next_field: u64,
    doc_comment: TokenStream,
    attributes: Vec<TokenStream>,
}

impl<'tcx> Field<'tcx> {
    fn size(&self) -> u64 {
        match self.type_info {
            Err(_) => self.offset_of_next_field - self.offset,
            Ok(FieldTypeInfo { size, .. }) => size,
        }
    }
}

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

struct AdtVariantLayout<'tcx> {
    fields: Vec<Field<'tcx>>,
    size: u64,
}

enum AdtLayout<'tcx> {
    Struct { fields: Vec<Field<'tcx>>, always_omit_padding: bool },
    Union { fields: Vec<Field<'tcx>> },
    Enum { enum_kind: EnumKind, tag_size_with_padding: u64, variants: Vec<AdtVariantLayout<'tcx>> },
}

struct AdtFieldGenerator<'a, 'tcx> {
    db: &'a BindingsGenerator<'tcx>,
    layout: rustc_abi::Layout<'tcx>,
    adt_def: ty::AdtDef<'tcx>,
    adt_generic_args: ty::GenericArgsRef<'tcx>,
    cc_short_name: &'a Ident,
    rs_fully_qualified_name: &'a TokenStream,
    repr_attrs: &'a [rustc_hir::attrs::ReprAttr],
    size_in_bytes: u64,
    alignment_in_bytes: u64,
    member_function_names: &'a HashSet<String>,
}

impl<'a, 'tcx> AdtFieldGenerator<'a, 'tcx> {
    /// Returns the fields of each variant of an ADT. For structs, there will be only one variant.
    ///
    /// If a valid C++ representation is not possible, returns a single error field for the ADT.
    fn variant_fields(
        &self,
        enum_kind: Option<EnumKind>,
        tag_size_with_padding: u64,
    ) -> Vec<Vec<Field<'tcx>>> {
        if let ty::AdtKind::Enum = self.adt_def.adt_kind()
            && enum_kind != Some(EnumKind::ReprC)
        {
            return vec![vec![Field {
                type_info: Err(anyhow!(
                    "No support for bindings of individual non-repr(C) `enum`s"
                )),
                cc_name: format_ident!("__opaque_blob_of_bytes"),
                rs_name: quote! { __opaque_blob_of_bytes },
                is_public: false,
                index: 0,
                offset: 0,
                offset_of_next_field: self.size_in_bytes,
                doc_comment: quote! {},
                attributes: vec![],
            }]];
        };

        let layout = &self.layout;
        let layout_variants = layout.variants();

        #[rustversion::before(2026-05-18)]
        let get_fields =
            |(_, variant): (VariantIdx, &LayoutData<FieldIdx, VariantIdx>)| variant.fields.clone();
        #[rustversion::since(2026-05-18)]
        let get_fields = |(i, _): (VariantIdx, &VariantLayout<FieldIdx>)| {
            LayoutData::for_variant(layout, i).fields
        };
        let variant_layout_field_sizes = match layout_variants {
            Variants::Single { .. } | Variants::Empty => {
                vec![(layout.fields.clone(), layout.size.bytes())]
            }
            Variants::Multiple { tag: _, tag_encoding: _, tag_field: _, variants } => variants
                .iter_enumerated()
                .map(|(variant_index, variant)| {
                    (
                        get_fields((variant_index, variant)),
                        variant.size.bytes() - tag_size_with_padding,
                    )
                })
                .collect_vec(),
        };

        self.adt_def
            .variants()
            .iter()
            .zip(variant_layout_field_sizes)
            .map(|(variant, (field_shape, size))| {
                let enum_adjustment =
                    if enum_kind == Some(EnumKind::ReprC) { tag_size_with_padding } else { 0 };

                let offsets = match field_shape {
                    FieldsShape::Arbitrary { ref offsets, .. } => {
                        offsets.iter().map(|size| size.bytes() - enum_adjustment).collect_vec()
                    }
                    FieldsShape::Union { .. } => (0..variant.fields.len())
                        .map(|i| layout.fields().offset(i).bytes())
                        .collect_vec(),
                    unexpected => panic!("Unexpected FieldsShape: {unexpected:?}"),
                };

                let mut fields = variant
                    .fields
                    .iter()
                    .zip(offsets)
                    .enumerate()
                    .map(|(index, (field_def, offset))| {
                        self.analyze_field(index, field_def, offset)
                    })
                    .collect_vec();

                // We only need to worry about this for variant multiples.
                // TODO: We sort after analyze field because we sort by `field_size` which is
                // determined by `analyze_field`. We could instead determine field size prior and
                // have analyze_field set `next_offset` instead of mutating after the fact.
                if let FieldsShape::Arbitrary { .. } = field_shape {
                    fields.sort_by_key(|field| {
                        let field_size =
                            field.type_info.as_ref().map(|info| info.size).unwrap_or(0);
                        (field.offset, field_size, field.index)
                    });
                }

                let next_offsets = fields
                    .iter()
                    .map(|Field { offset, .. }| offset)
                    .skip(1)
                    .copied()
                    .chain(once(size))
                    .collect_vec();
                for (field, next_offset) in fields.iter_mut().zip(next_offsets) {
                    field.offset_of_next_field = next_offset;
                }

                fields
            })
            .collect_vec()
    }

    fn analyze_layout(&self) -> Result<AdtLayout<'tcx>> {
        let layout = &self.layout;
        let layout_variants = layout.variants();

        let enum_kind = get_enum_kind(self.db, self.adt_def);
        let tag_size_with_padding = match enum_kind {
            Some(EnumKind::ReprC) => get_tag_size_with_padding(*layout),
            None | Some(EnumKind::OpaqueBlobOfBytes) => 0,
        };

        let variants_fields = self.variant_fields(enum_kind, tag_size_with_padding);

        match self.adt_def.adt_kind() {
            ty::AdtKind::Struct => {
                let always_omit_padding = self.repr_attrs.contains(&rustc_hir::attrs::ReprC)
                    && variants_fields.iter().flatten().all(|field| field.type_info.is_ok());
                let fields = variants_fields.into_iter().next().unwrap_or_default();
                Ok(AdtLayout::Struct { fields, always_omit_padding })
            }
            ty::AdtKind::Union => {
                let fields = variants_fields.into_iter().next().unwrap_or_default();
                Ok(AdtLayout::Union { fields })
            }
            ty::AdtKind::Enum => {
                let variant_sizes = match layout_variants {
                    Variants::Multiple { tag: _, tag_encoding: _, tag_field: _, variants } => {
                        variants
                            .iter()
                            .map(|layout| layout.size.bytes() - tag_size_with_padding)
                            .collect_vec()
                    }
                    Variants::Single { .. } | Variants::Empty => vec![self.size_in_bytes],
                };
                let variants = variants_fields
                    .into_iter()
                    .zip(variant_sizes)
                    .map(|(fields, size)| AdtVariantLayout { fields, size })
                    .collect_vec();
                let enum_kind = enum_kind.expect("Enum kind should be set for enums");
                Ok(AdtLayout::Enum { enum_kind, tag_size_with_padding, variants })
            }
        }
    }

    /// Ensures that a given field has a valid C++ type and returns its size and C++ type, if so.
    fn prepare_field_type(&self, field_def: &ty::FieldDef) -> Result<FieldTypeInfo<'tcx>> {
        let tcx = self.db.tcx();
        let ty = field_def.ty(tcx, self.adt_generic_args);
        #[rustversion::since(2026-05-13)]
        let ty = crate::normalize_ty(tcx, tcx.param_env(field_def.did), ty);
        let size = get_layout(tcx, ty).map(|layout| layout.size().bytes())?;

        if is_bridged_type(self.db, ty).is_ok_and(|bridged_type| {
            bridged_type.is_some_and(|bridged_type| !bridged_type.is_layout_compatible())
        }) && !ty
            .ty_adt_def()
            .and_then(|adt_def| BridgedBuiltin::new(self.db, adt_def))
            .is_some_and(|builtin| matches!(builtin, BridgedBuiltin::Option))
        {
            bail!(
                "Field is a bridged type and might not be layout-compatible
                with the C++ type (b/400633609)"
            );
        }

        let cpp_type = self
            .db
            .format_ty_for_cc(ty, TypeLocation::Field)?
            .resolve_feature_requirements(self.db.crate_features(self.db.source_crate_num()))?;

        Ok(FieldTypeInfo { size, cpp_type })
    }

    fn analyze_field(&self, index: usize, field_def: &ty::FieldDef, offset: u64) -> Field<'tcx> {
        let tcx = self.db.tcx();
        let type_info = self.prepare_field_type(field_def);
        let name = field_def.ident(tcx).to_string();
        let cc_name = code_gen_utils::unkeyword_cpp_ident(&name).to_string();
        let cc_name = if self.member_function_names.contains(&cc_name) {
            format!("{cc_name}_")
        } else {
            cc_name
        };
        let cc_name = format_cc_ident(self.db, cc_name.as_str())
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

        let mut attributes = vec![];
        if let Some(cc_deprecated_tag) = generate_deprecated_tag(tcx, field_def.did) {
            attributes.push(cc_deprecated_tag);
        }
        let offset_of_next_field = 0;

        Field {
            type_info,
            cc_name,
            rs_name,
            is_public: crate::field_def_is_pub_and_stable(tcx, field_def).is_ok(),
            index,
            offset,
            offset_of_next_field,
            doc_comment: generate_doc_comment(self.db, field_def.did),
            attributes,
        }
    }

    fn generate_common_assertions(&self, fields: &[Field<'tcx>]) -> ApiSnippets<'tcx> {
        let adt_cc_name = self.cc_short_name;
        let adt_rs_name = self.rs_fully_qualified_name;

        let cc_details = if fields.is_empty() {
            CcSnippet::default()
        } else {
            let cc_assertions: TokenStream = fields
                .iter()
                .filter(|field| field.size() != 0)
                .map(|Field { cc_name, offset, .. }| {
                    let offset = Literal::u64_unsuffixed(*offset);
                    quote! { static_assert(#offset == offsetof(#adt_cc_name, #cc_name)); }
                })
                .collect();

            CcSnippet::with_include(
                quote! {
                    inline void #adt_cc_name::__crubit_field_offset_assertions() {
                        #cc_assertions
                    }
                },
                CcInclude::cstddef(),
            )
        };

        let rs_details: RsSnippet = fields
            .iter()
            .filter(|field| field.is_public)
            .map(|Field { rs_name, offset, .. }| {
                let expected_offset = Literal::u64_unsuffixed(*offset);
                let actual_offset = quote! { ::core::mem::offset_of!(#adt_rs_name, #rs_name) };
                RsSnippet::new(
                    quote! { const _: () = assert!(#actual_offset == #expected_offset); },
                )
            })
            .collect();

        let method_decl = if fields.is_empty() {
            quote! {}
        } else {
            quote! { private: static void __crubit_field_offset_assertions(); }
        };

        ApiSnippets { main_api: CcSnippet::new(method_decl), cc_details, rs_details }
    }

    fn assemble_snippets(
        &self,
        fields_tokens: TokenStream,
        mut prereqs: CcPrerequisites<'tcx>,
        assertions: ApiSnippets<'tcx>,
    ) -> ApiSnippets<'tcx> {
        let method_decl = assertions.main_api.into_tokens(&mut prereqs);
        let main_api = CcSnippet {
            prereqs,
            tokens: quote! {
                #fields_tokens
                #method_decl
            },
        };
        ApiSnippets {
            main_api,
            cc_details: assertions.cc_details,
            rs_details: assertions.rs_details,
        }
    }

    fn emit_field_err(
        &self,
        field: &Field<'tcx>,
        err: &arc_anyhow::Error,
        current_visibility: &mut CcFieldVisState,
    ) -> CcSnippet<'tcx> {
        let cc_name = &field.cc_name;
        let size = field.size();
        let msg = format!("Field type has been replaced with a blob of bytes: {err:#}");

        if size == 0 {
            let msg = format!("Field `{cc_name}` omitted: C++ does not support zero-sized types.");
            return CcSnippet::new(quote! {__NEWLINE__ __COMMENT__ #msg});
        }
        let visibility = current_visibility.set_is_public(false);
        let size = Literal::u64_unsuffixed(size);
        let tokens = quote! {
            #visibility __NEWLINE__
                __COMMENT__ #msg
                ::std::array<unsigned char, #size> #cc_name;
        };
        CcSnippet::with_include(tokens, CcInclude::array())
    }

    fn emit_struct_field(
        &self,
        field: Field<'tcx>,
        current_visibility: &mut CcFieldVisState,
        always_omit_padding: bool,
    ) -> CcSnippet<'tcx> {
        let cc_name = &field.cc_name;
        let bracketed_cc_name = if self.db.kythe_annotations() {
            quote! { __CAPTURE_BEGIN__ #cc_name __CAPTURE_END__ }
        } else {
            quote! { #cc_name }
        };

        let (cpp_type, size) = match field.type_info {
            Err(ref err) => return self.emit_field_err(&field, err, current_visibility),
            Ok(FieldTypeInfo { cpp_type, size }) => (cpp_type, size),
        };

        assert!((field.offset + size) <= field.offset_of_next_field);
        let padding = field.offset_of_next_field - field.offset - size;

        let visibility = current_visibility.set_is_public(field.is_public);
        let mut prereqs = CcPrerequisites::default();
        let cpp_type = cpp_type.into_tokens(&mut prereqs);
        let doc_comment = field.doc_comment;
        let attributes = field.attributes;

        let padding = if always_omit_padding || padding == 0 {
            quote! {}
        } else {
            let padding = Literal::u64_unsuffixed(padding);
            let ident = format_ident!("__padding{}", field.index);
            let padding_visibility = current_visibility.set_is_public(false);
            quote! { #padding_visibility unsigned char #ident[#padding]; }
        };
        let tokens = quote! {
            #visibility __NEWLINE__
                // The anonymous union gives more control over when exactly
                // the field constructors and destructors run. For example,
                // this lets us initialize the fields for the first time via
                // memcpy, in the move or UnsafeRelocateTag constructor, and lets
                // us destroy them only by calling into Rust.
                // See also b/288138612.
                union { __NEWLINE__
                    #doc_comment
                    #(#attributes)*
                    #cpp_type #bracketed_cc_name;
                };
            #padding
        };
        CcSnippet { tokens, prereqs }
    }

    fn emit_union_field(
        &self,
        field: Field<'tcx>,
        current_visibility: &mut CcFieldVisState,
        is_repr_c: bool,
    ) -> CcSnippet<'tcx> {
        let cc_name = &field.cc_name;
        let bracketed_cc_name = if self.db.kythe_annotations() {
            quote! { __CAPTURE_BEGIN__ #cc_name __CAPTURE_END__ }
        } else {
            quote! { #cc_name }
        };

        let cpp_type = match field.type_info {
            Err(ref err) => return self.emit_field_err(&field, err, current_visibility),
            Ok(FieldTypeInfo { cpp_type, .. }) => cpp_type,
        };

        let visibility = current_visibility.set_is_public(field.is_public);
        let mut prereqs = CcPrerequisites::default();
        let cpp_type = cpp_type.into_tokens(&mut prereqs);
        let doc_comment = field.doc_comment;

        let tokens = if is_repr_c {
            quote! {
                #visibility __NEWLINE__
                #doc_comment
                #cpp_type #bracketed_cc_name;
            }
        } else {
            let internal_padding = if field.offset == 0 {
                quote! {}
            } else {
                let internal_padding_size = Literal::u64_unsuffixed(field.offset);
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
        };
        CcSnippet { tokens, prereqs }
    }

    fn emit_enum_field(
        &self,
        field: &Field<'tcx>,
        current_visibility: &mut CcFieldVisState,
    ) -> CcSnippet<'tcx> {
        let cc_name = &field.cc_name;

        let cpp_type = match &field.type_info {
            Err(err) => return self.emit_field_err(field, err, current_visibility),
            Ok(FieldTypeInfo { cpp_type, .. }) => cpp_type,
        };

        let visibility = current_visibility.set_is_public(field.is_public);
        let mut prereqs = CcPrerequisites::default();
        let cpp_type = cpp_type.clone().into_tokens(&mut prereqs);

        let tokens = quote! {
            #visibility __NEWLINE__ #cpp_type #cc_name;
        };
        CcSnippet { tokens, prereqs }
    }

    fn generate_struct(
        &self,
        fields: Vec<Field<'tcx>>,
        always_omit_padding: bool,
    ) -> ApiSnippets<'tcx> {
        let assertions = self.generate_common_assertions(&fields);

        let mut prereqs = CcPrerequisites::default();
        let mut current_visibility = CcFieldVisState::public();
        let fields_tokens: TokenStream = fields
            .into_iter()
            .map(|field| {
                self.emit_struct_field(field, &mut current_visibility, always_omit_padding)
                    .into_tokens(&mut prereqs)
            })
            .collect();

        self.assemble_snippets(fields_tokens, prereqs, assertions)
    }

    fn generate_union(&self, fields: Vec<Field<'tcx>>) -> ApiSnippets<'tcx> {
        let assertions = self.generate_common_assertions(&fields);

        let is_repr_c = self.repr_attrs.contains(&rustc_hir::attrs::ReprC);
        let mut prereqs = CcPrerequisites::default();
        let mut current_visibility = CcFieldVisState::public();
        let fields_tokens: TokenStream = fields
            .into_iter()
            .map(|field| {
                self.emit_union_field(field, &mut current_visibility, is_repr_c)
                    .into_tokens(&mut prereqs)
            })
            .collect();

        self.assemble_snippets(fields_tokens, prereqs, assertions)
    }

    fn generate_enum(
        &self,
        enum_kind: EnumKind,
        tag_size_with_padding: u64,
        variants: Vec<AdtVariantLayout<'tcx>>,
    ) -> ApiSnippets<'tcx> {
        let tcx = self.db.tcx();
        let adt_cc_name = self.cc_short_name;
        let adt_rs_name = self.rs_fully_qualified_name;
        let layout_variants = &self.layout.variants;

        let cc_details = if variants.is_empty() {
            CcSnippet::default()
        } else {
            let cc_assertions: TokenStream = match enum_kind {
                EnumKind::OpaqueBlobOfBytes => variants
                    .iter()
                    .flat_map(|v| &v.fields)
                    .filter(|field| field.size() != 0)
                    .map(|Field { cc_name, offset, .. }| {
                        let offset = Literal::u64_unsuffixed(*offset);
                        quote! { static_assert(#offset == offsetof(#adt_cc_name, #cc_name)); }
                    })
                    .collect(),
                EnumKind::ReprC => {
                    let variant_offset_assertions: TokenStream = self.adt_def.variants().iter().zip(variants.iter())
                        .map(|(variant_def, variant)| {
                            if variant.size == 0 {
                                quote! {}
                            } else {
                                let cc_variant_struct_name = format_cc_ident(self.db, variant_def.ident(tcx).as_str())
                                    .unwrap_or_else(|_err| format_ident!("err_field"));
                                let tag_unsuffixed = Literal::u64_unsuffixed(tag_size_with_padding);
                                quote! { static_assert(#tag_unsuffixed == offsetof(#adt_cc_name, #cc_variant_struct_name)); }
                            }
                        }).collect();
                    let variant_field_assertions: TokenStream = variants
                        .iter()
                        .enumerate()
                        .flat_map(|(variant_index, variant_layout)| {
                            let variant_def = self.adt_def.variant(VariantIdx::from_usize(variant_index));
                            let cc_variant = variant_def.ident(tcx);
                            let qualified_struct_name =
                                expect_format_cc_type_name(&format!("{}::__crubit_{}_struct", adt_cc_name, cc_variant));
                            if variant_def.fields.is_empty() {
                                quote! {}
                            } else {
                                variant_layout.fields.iter().filter(|field| field.type_info.is_ok() && field.size() != 0 ).flat_map(move |Field { cc_name, offset, .. }| {
                                    let offset = Literal::u64_unsuffixed(*offset);
                                    quote! { static_assert(#offset == offsetof(#qualified_struct_name, #cc_name)); }
                                }).collect()
                            }
                    }).collect();
                    quote! {#variant_offset_assertions #variant_field_assertions }
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

        let rs_details: RsSnippet = if enum_kind == EnumKind::ReprC {
            RsSnippet::default()
        } else {
            variants
                .iter()
                .flat_map(|v| &v.fields)
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

        let assertions_method_decl = if variants.is_empty() {
            quote! {}
        } else {
            quote! { private: static void __crubit_field_offset_assertions(); }
        };

        let adt_size = Literal::u64_unsuffixed(self.size_in_bytes);
        let mut prereqs = CcPrerequisites::default();

        let fields = if enum_kind != EnumKind::ReprC {
            variants
                .iter()
                .flat_map(|v| &v.fields)
                .map(|field| {
                    self.emit_enum_field(field, &mut Default::default()).into_tokens(&mut prereqs)
                })
                .collect()
        } else {
            let tag_enum = match layout_variants {
                Variants::Single { .. } | Variants::Empty => quote! {},
                Variants::Multiple { tag, .. } => {
                    let tag_ty = get_scalar_int_type(self.db.tcx(), *tag);

                    let tag_tokens = self
                        .db
                        .format_ty_for_cc(tag_ty, TypeLocation::Other)
                        .expect("discriminant should be a integer type.")
                        .into_tokens(&mut prereqs);

                    let variant_enum_fields: TokenStream = self
                        .adt_def
                        .variants()
                        .iter_enumerated()
                        .map(|(variant_index, variant_def)| {
                            let cc_variant_name =
                                format_cc_ident(self.db, variant_def.name.as_str())
                                    .unwrap_or_else(|_err| format_ident!("err_field"));
                            let (tag_size, _signed) = tag_ty.int_size_and_signed(tcx);
                            let (scalar_int, _) = ty::ScalarInt::truncate_from_uint(
                                self.adt_def.discriminant_for_variant(tcx, variant_index).val,
                                tag_size,
                            );
                            let tag_value = scalar_value_to_string(
                                tcx,
                                Scalar::Int(scalar_int),
                                *tag_ty.kind(),
                            )
                            .expect("tag to be a valid scalar constant")
                            .parse::<TokenStream>()
                            .expect("tag string to consist of valid scalar tokens");
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

            let mut current_visibility = CcFieldVisState::default();
            let tokens_per_variant: Vec<TokenStream> = variants
                .iter()
                .map(|variant_layout| {
                    variant_layout
                        .fields
                        .iter()
                        .map(|field| {
                            self.emit_enum_field(field, &mut current_visibility)
                                .into_tokens(&mut prereqs)
                        })
                        .collect()
                })
                .collect();

            let layout = &self.layout;
            let variant_alignments = match layout_variants {
                Variants::Multiple { variants: layout_vars, .. } => {
                    #[rustversion::before(2026-05-18)]
                    let get_align =
                        |(_, layout): (VariantIdx, &LayoutData<FieldIdx, VariantIdx>)| {
                            layout.align.abi.bytes() - tag_size_with_padding
                        };
                    #[rustversion::since(2026-05-18)]
                    let get_align = |(i, _): (VariantIdx, &VariantLayout<FieldIdx>)| {
                        LayoutData::for_variant(layout, i).align.abi.bytes() - tag_size_with_padding
                    };
                    layout_vars.iter_enumerated().map(get_align).collect_vec()
                }
                Variants::Single { .. } | Variants::Empty => {
                    vec![self.alignment_in_bytes]
                }
            };

            let variant_structs: TokenStream = self
                .adt_def
                .variants()
                .iter_enumerated()
                .map(|(variant_index, variant_def)| {
                    let cc_variant_struct_name = format_cc_ident(
                        self.db,
                        format!("__crubit_{}_struct", variant_def.ident(tcx).as_str()).as_ref(),
                    )
                    .unwrap_or_else(|_err| format_ident!("err_struct"));

                    let fields_for_variant = &tokens_per_variant[variant_index.index()];

                    let variant_alignment =
                        Literal::u64_unsuffixed(variant_alignments[variant_index.index()]);

                    if variants[variant_index.index()].size == 0 {
                        let cc_variant_name = format_cc_ident(self.db, variant_def.name.as_str())
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

            let variants_union_fields: TokenStream = self
                .adt_def
                .variants()
                .iter_enumerated()
                .map(|(variant_index, variant_def)| {
                    let cc_variant_name = format_cc_ident(self.db, variant_def.name.as_str())
                        .unwrap_or_else(|_err| format_ident!("err_field"));
                    let cc_variant_struct_type = format_cc_ident(
                        self.db,
                        format!("__crubit_{}_struct", variant_def.ident(tcx).as_str()).as_ref(),
                    )
                    .unwrap_or_else(|_err| format_ident!("err_struct"));

                    if variants[variant_index.index()].size == 0 {
                        quote! {}
                    } else {
                        quote! {
                            #cc_variant_struct_type #cc_variant_name; __NEWLINE__
                        }
                    }
                })
                .collect();

            let variants_union: TokenStream = {
                let has_no_fields = variants.iter().all(|v| v.size == 0);

                if has_no_fields {
                    quote! {}
                } else {
                    quote! {
                        public: union {
                            #variants_union_fields
                        };
                    }
                }
            };

            quote! {
                #variant_structs __NEWLINE__
                #tag_enum __NEWLINE__
                public: Tag tag; __NEWLINE__
                #variants_union
            }
        };

        let cc_short_name = self.cc_short_name;
        let enum_opaque_bytes_ctor = match enum_kind {
            EnumKind::OpaqueBlobOfBytes => quote! {
                private:
                    struct PrivateBytesTag {};
                    constexpr #cc_short_name(PrivateBytesTag,
                                             ::std::array<unsigned char, #adt_size> bytes)
                        : __opaque_blob_of_bytes(bytes) {}
            },
            EnumKind::ReprC => quote! {
                private:
                    struct PrivateTagCtorTag {};
                    constexpr #cc_short_name(PrivateTagCtorTag, Tag tag)
                        : tag(tag) {}
            },
        };

        let main_api = CcSnippet {
            prereqs,
            tokens: quote! {
                #fields
                #enum_opaque_bytes_ctor
                #assertions_method_decl
            },
        };

        ApiSnippets { main_api, cc_details, rs_details }
    }
}

/// Returns the body of the C++ struct that represents the given ADT.
pub(crate) fn generate_fields<'tcx>(
    db: &BindingsGenerator<'tcx>,
    self_ty: Ty<'tcx>,
    cc_short_name: &Ident,
    rs_fully_qualified_name: &TokenStream,
    repr_attrs: &[rustc_hir::attrs::ReprAttr],
    size_in_bytes: u64,
    alignment_in_bytes: u64,
    member_function_names: &HashSet<String>,
) -> ApiSnippets<'tcx> {
    let TyKind::Adt(adt_def, adt_generic_args) = self_ty.kind() else {
        panic!("Attempted to generate fields for a non-ADT type: {:?}", self_ty)
    };

    let layout = get_layout(db.tcx(), self_ty)
        .expect("Layout should be already verified by `generate_adt_core`");

    let generator = AdtFieldGenerator {
        db,
        layout,
        adt_def: *adt_def,
        adt_generic_args,
        cc_short_name,
        rs_fully_qualified_name,
        repr_attrs,
        size_in_bytes,
        alignment_in_bytes,
        member_function_names,
    };

    let layout_data = generator
        .analyze_layout()
        .expect("Layout analysis should succeed if `generate_adt_core` verified layout");

    match layout_data {
        AdtLayout::Struct { fields, always_omit_padding } => {
            generator.generate_struct(fields, always_omit_padding)
        }
        AdtLayout::Union { fields } => generator.generate_union(fields),
        AdtLayout::Enum { enum_kind, tag_size_with_padding, variants } => {
            generator.generate_enum(enum_kind, tag_size_with_padding, variants)
        }
    }
}

/// Generates the `(UnsafeRelocateTag, T&&)` constructor for the given ADT.
pub(crate) fn generate_relocating_ctor<'tcx>(
    db: &BindingsGenerator<'tcx>,
    adt_cc_name: &Ident,
) -> ApiSnippets<'tcx> {
    let mut main_api = CcSnippet::with_include(
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
                ::std::memcpy(this, &value, sizeof(value));
            }
        },
        db.support_header("internal/slot.h"),
    );
    // We include this for `std::memcpy`.
    main_api.prereqs.includes.insert(CcInclude::cstring());
    main_api.into_main_api()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PassingMode {
    Value,
    SharedRef,
    MutRef,
}

fn get_into_iter_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    self_ty: Ty<'tcx>,
    into_iterator_trait_id: DefId,
) -> Result<Ty<'tcx>> {
    let into_iter_assoc_item = tcx
        .associated_items(into_iterator_trait_id)
        .in_definition_order()
        .find(|item| {
            item.name() == rustc_span::symbol::Symbol::intern("IntoIter")
                && matches!(item.kind, ty::AssocKind::Type { .. })
        })
        .expect("IntoIter to be a required associated item of IntoIterator");

    let projection_ty = Ty::new_projection(tcx, into_iter_assoc_item.def_id, [self_ty]);

    query_compiler::try_normalize(
        tcx,
        ty::PseudoCanonicalInput {
            typing_env: rustc_middle::ty::TypingEnv::fully_monomorphized(),
            value: projection_ty,
        },
    )
    .map_err(|_| anyhow!("Failed to normalize `<{} as IntoIterator>::IntoIter`", self_ty))
}

fn get_into_iter_item_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    self_ty: Ty<'tcx>,
    into_iterator_trait_id: DefId,
) -> Result<Ty<'tcx>> {
    let item_assoc_item = tcx
        .associated_items(into_iterator_trait_id)
        .in_definition_order()
        .find(|item| {
            item.name() == Symbol::intern("Item") && matches!(item.kind, ty::AssocKind::Type { .. })
        })
        .expect("Item to be a required associated item of IntoIterator");

    let projection_ty = Ty::new_projection(tcx, item_assoc_item.def_id, [self_ty]);

    query_compiler::try_normalize(
        tcx,
        ty::PseudoCanonicalInput {
            typing_env: rustc_middle::ty::TypingEnv::fully_monomorphized(),
            value: projection_ty,
        },
    )
    .map_err(|_| anyhow!("Failed to normalize `<{} as IntoIterator>::Item`", self_ty))
}

fn generate_begin_and_end_for_type<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
    into_iterator_trait_id: DefId,
    passing_mode: PassingMode,
) -> Result<Option<ApiSnippets<'tcx>>> {
    let tcx = db.tcx();
    let self_ty = core.self_ty;

    let check_ty = match passing_mode {
        PassingMode::Value => self_ty,
        PassingMode::SharedRef => Ty::new_imm_ref(tcx, tcx.lifetimes.re_erased, self_ty),
        PassingMode::MutRef => Ty::new_mut_ref(tcx, tcx.lifetimes.re_erased, self_ty),
    };

    if let Some(iterator_trait_id) = tcx.get_diagnostic_item(sym::Iterator)
        && does_type_implement_trait(tcx, self_ty, iterator_trait_id, [])
    {
        let PassingMode::MutRef = passing_mode else {
            return Ok(None);
        };

        let adt_cc_name = &core.cc_short_name;
        let mut main_api_prereqs = CcPrerequisites::default();
        main_api_prereqs.includes.insert(db.support_header("rs_std/iterator_adapter.h"));
        main_api_prereqs.move_only_defs_to_fwd_decls();

        let main_api = CcSnippet {
            tokens: quote! {
                template <typename TAdaptedSelf_ = #adt_cc_name>
                inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
                    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
                }
                inline rs::IteratorEnd end() & {
                    return rs::IteratorEnd();
                }
            },
            prereqs: main_api_prereqs,
        };

        return Ok(Some(ApiSnippets { main_api, ..Default::default() }));
    }

    if !does_type_implement_trait(tcx, check_ty, into_iterator_trait_id, []) {
        return Ok(None);
    }

    let into_iter_ty = get_into_iter_ty(tcx, check_ty, into_iterator_trait_id)?;

    let item_ty = get_into_iter_item_ty(tcx, check_ty, into_iterator_trait_id)?;

    let _ = db
        .format_ty_for_cc(item_ty, TypeLocation::Other)
        .context("Failed to format IntoIterator::Item")?;

    let into_iter_cc_ty = db
        .format_ty_for_cc(into_iter_ty, TypeLocation::Other)
        .context("Failed to format IntoIterator::IntoIter")?;

    let static_check_ty = replace_all_regions_with_static(tcx, check_ty);
    let rs_fully_qualified_name = db.format_ty_for_rs(static_check_ty)?;

    let TraitThunks { method_name_to_cc_thunk_name, cc_thunk_decls, rs_thunk_impls } =
        generate_trait_thunks(
            db,
            into_iterator_trait_id,
            &[],
            check_ty,
            core.def_id,
            rs_fully_qualified_name,
            /*is_constructor=*/ false,
            /*within_template=*/ false,
        )?;

    let into_iter_thunk_name = method_name_to_cc_thunk_name
        .get(&sym::into_iter)
        .expect("IntoIterator trait missing into_iter method");

    let into_iter_fn_assoc_item = tcx
        .associated_items(into_iterator_trait_id)
        .in_definition_order()
        .find(|item| item.name() == sym::into_iter && matches!(item.kind, ty::AssocKind::Fn { .. }))
        .expect("IntoIterator should have into_iter method");
    let into_iter_fn_id = into_iter_fn_assoc_item.def_id;

    let adt_cc_name = &core.cc_short_name;
    let param_cc_type_tokens = match passing_mode {
        PassingMode::Value => quote! { #adt_cc_name && },
        PassingMode::SharedRef => quote! { const #adt_cc_name & },
        PassingMode::MutRef => quote! { #adt_cc_name & },
    };

    let param = Param {
        cc_name: format_ident!("self_"),
        cpp_type: CcParamTy {
            snippet: CcSnippet::new(param_cc_type_tokens.clone()),
            is_lifetime_bound: false,
        },
        ty: check_ty,
    };

    let impl_body = generate_thunk_call(
        db,
        into_iter_fn_id,
        into_iter_thunk_name.clone(),
        into_iter_ty,
        ThunkSelfParameter::new(
            /*has_self=*/ false, /*by_copy=*/ false, /*is_trait_method=*/ false,
        ),
        &[param],
    )?;

    let mut main_api_prereqs = CcPrerequisites::default();
    let into_iter_cc_ty_tokens_main = into_iter_cc_ty.clone().into_tokens(&mut main_api_prereqs);
    main_api_prereqs.includes.insert(db.support_header("rs_std/iterator_adapter.h"));
    main_api_prereqs.move_defs_to_fwd_decls();

    let iterator_trait_id = tcx
        .get_diagnostic_item(sym::Iterator)
        .ok_or_else(|| anyhow!("Iterator trait not found"))?;
    let mut impls = tcx.non_blanket_impls_for_ty(iterator_trait_id, into_iter_ty);
    let Some(trait_impl_def_id) = impls.next() else {
        return Ok(None);
    };
    let generics = tcx.generics_of(trait_impl_def_id);
    let has_type_or_const_params = generics.own_params.iter().any(|param| {
        matches!(
            param.kind,
            ty::GenericParamDefKind::Type { .. } | ty::GenericParamDefKind::Const { .. }
        )
    });
    if has_type_or_const_params {
        bail!("IntoIterator/Iterator impls with generic type or const parameters are not supported yet.");
    }
    let specialization = TemplateSpecialization::TraitImpl(TraitImplTemplateSpecialization {
        self_ty_cc_name: into_iter_cc_ty_tokens_main.clone(),
        trait_impl: trait_impl_def_id,
    });
    main_api_prereqs.template_specializations.insert(specialization);

    let mut cc_details_prereqs = CcPrerequisites::default();
    let into_iter_cc_ty_tokens_details = into_iter_cc_ty.into_tokens(&mut cc_details_prereqs);
    cc_details_prereqs.includes.insert(db.support_header("rs_std/iterator_adapter.h"));

    let cc_thunk_decls_tokens = cc_thunk_decls.into_tokens(&mut cc_details_prereqs);
    let impl_body_tokens = impl_body.into_tokens(&mut cc_details_prereqs);
    cc_details_prereqs.move_defs_to_fwd_decls();

    let call_expr = if matches!(into_iter_ty.kind(), ty::Ref(..)) {
        quote! { &call_into_iter() }
    } else {
        quote! { call_into_iter() }
    };

    let (main_api_tokens, cc_details_tokens) = match passing_mode {
        PassingMode::Value => {
            let self_binding = quote! { #adt_cc_name&& self_ = ::std::move(*this); };
            (
                quote! {
                    template <typename TAdaptedSelf_ = #adt_cc_name>
                    inline #into_iter_cc_ty_tokens_main into_iter() &&;
                },
                quote! {
                    #cc_thunk_decls_tokens

                    template <typename TAdaptedSelf_>
                    inline #into_iter_cc_ty_tokens_details #adt_cc_name :: into_iter () && {
                        #self_binding
                        auto call_into_iter = [&]() -> decltype(auto) {
                            #impl_body_tokens
                        };
                        return #call_expr;
                    }
                },
            )
        }
        PassingMode::SharedRef | PassingMode::MutRef => {
            let (ref_qualifiers, self_binding) = match passing_mode {
                PassingMode::SharedRef => {
                    (quote! { const & }, quote! { const #adt_cc_name& self_ = *this; })
                }
                PassingMode::MutRef => (quote! { & }, quote! { #adt_cc_name& self_ = *this; }),
                PassingMode::Value => unreachable!(),
            };
            (
                quote! {
                    template <typename TAdaptedSelf_ = #adt_cc_name>
                    rs::IteratorAdapter< #into_iter_cc_ty_tokens_main > begin() #ref_qualifiers;
                    template <typename TAdaptedSelf_ = #adt_cc_name>
                    rs::IteratorEnd end() #ref_qualifiers;
                },
                quote! {
                    #cc_thunk_decls_tokens

                    template <typename TAdaptedSelf_>
                    inline rs::IteratorAdapter< #into_iter_cc_ty_tokens_details > #adt_cc_name :: begin () #ref_qualifiers {
                        #self_binding
                        auto call_into_iter = [&]() -> decltype(auto) {
                            #impl_body_tokens
                        };
                        return rs::IteratorAdapter< #into_iter_cc_ty_tokens_details >(#call_expr);
                    }
                    template <typename TAdaptedSelf_>
                    inline rs::IteratorEnd #adt_cc_name :: end () #ref_qualifiers {
                        return rs::IteratorEnd();
                    }
                },
            )
        }
    };

    let main_api = CcSnippet { tokens: main_api_tokens, prereqs: main_api_prereqs };

    let cc_details = CcSnippet { tokens: cc_details_tokens, prereqs: cc_details_prereqs };

    Ok(Some(ApiSnippets { main_api, cc_details, rs_details: rs_thunk_impls }))
}

fn generate_into_iterator_impls<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
    member_function_names: &mut HashSet<String>,
) -> Result<ApiSnippets<'tcx>> {
    let tcx = db.tcx();

    if member_function_names.contains("begin")
        || member_function_names.contains("end")
        || member_function_names.contains("into_iter")
    {
        bail!("{} has a method named `begin`, `end`, or `into_iter`, which prevents binding methods for IntoIterator.", core.self_ty);
    }

    let adt_def =
        core.self_ty.ty_adt_def().expect("generate_adt_core should have confirmed this was an ADT");
    let has_conflicting_field = adt_def
        .all_fields()
        .any(|field| matches!(field.name.as_str(), "begin" | "end" | "into_iter"));
    if has_conflicting_field {
        bail!("{} has a field named `begin`, `end`, or `into_iter`, which prevents binding methods for IntoIterator.", core.self_ty);
    }

    let into_iterator_trait_id = tcx
        .get_diagnostic_item(sym::IntoIterator)
        .expect("Could not find IntoIterator trait. Please file a crubit bug.");

    Ok([PassingMode::Value, PassingMode::SharedRef, PassingMode::MutRef]
        .into_iter()
        .map(|mode| generate_begin_and_end_for_type(db, core, into_iterator_trait_id, mode))
        .filter_map(|result| {
            result.unwrap_or_else(|err| {
                core.def_id.map(|def_id| generate_unsupported_def(db, def_id, err).into_main_api())
            })
        })
        .collect())
}
