// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(cfg_accessible)]
#![feature(rustc_private)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

//! Query the rust compiler.

extern crate rustc_abi;
#[rustversion::since(2026-09-27)]
extern crate rustc_attr_ir;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_infer;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_trait_selection;

use arc_anyhow::Result;
use error_report::anyhow;
#[rustversion::before(2026-05-18)]
use rustc_abi::FieldsShape;
use rustc_abi::{FieldIdx, Layout, Variants};
#[rustversion::since(2026-09-27)]
use rustc_attr_ir::lang_items::LangItem;
#[rustversion::since(2026-08-09)]
#[rustversion::before(2026-09-27)]
use rustc_hir::attrs::lang_items::LangItem;
#[rustversion::before(2026-08-09)]
use rustc_hir::lang_items::LangItem;
use rustc_infer::infer::TyCtxtInferExt;
use rustc_middle::ty::solve::NoSolution;
use rustc_middle::ty::{self, GenericArg, GenericArgKind, GenericParamDefKind, Ty, TyCtxt};
use rustc_span::def_id::DefId;
use rustc_span::symbol::Symbol;
use rustc_trait_selection::infer::InferCtxtExt;
use std::collections::{HashMap, HashSet};

use database::BindingsGenerator;

/// Returns true if `did` is `core::ptr::NonNull`.
#[rustversion::since(2026-08-18)]
pub fn is_std_ptr_non_null(tcx: TyCtxt<'_>, did: DefId) -> bool {
    tcx.is_lang_item(did, LangItem::NonNull)
}

#[rustversion::before(2026-08-18)]
pub fn is_std_ptr_non_null(tcx: TyCtxt<'_>, did: DefId) -> bool {
    tcx.get_diagnostic_item(rustc_span::symbol::sym::NonNull) == Some(did)
}

/// Returns true if pointers to `pointee` are ABI-compatible.
///
/// Only thin pointers are ABI-compatible, not e.g. `&str` or `&[T]`.
fn is_abi_compatible_pointee<'tcx>(db: &BindingsGenerator<'tcx>, pointee: Ty<'tcx>) -> bool {
    !db.portable_abi_compatible()
        || pointee.is_sized(db.tcx(), ty::TypingEnv::fully_monomorphized())
}

/// If `ty` is a reference `&T` or `&mut T`, or `Pin<&T>` or `Pin<&mut T>`, returns the region,
/// referent type, and mutability.
pub fn as_ref_or_pinned_ref<'tcx>(
    ty: Ty<'tcx>,
) -> Option<(ty::Region<'tcx>, Ty<'tcx>, ty::Mutability)> {
    match ty.kind() {
        ty::TyKind::Ref(region, referent, mutability) => Some((*region, *referent, *mutability)),
        _ if let Some(inner) = ty.pinned_ty()
            && let ty::TyKind::Ref(region, referent, mutability) = inner.kind() =>
        {
            Some((*region, *referent, *mutability))
        }
        _ => None,
    }
}

/// Whether functions using `extern "C"` ABI can safely handle values of type
/// `ty` (e.g. when passing by value arguments or return values of such type).
pub fn is_c_abi_compatible_by_value<'tcx>(db: &BindingsGenerator<'tcx>, ty: Ty<'tcx>) -> bool {
    let tcx = db.tcx();
    if let Some((_, pointee, _)) = as_ref_or_pinned_ref(ty) {
        return is_abi_compatible_pointee(db, pointee);
    }
    match ty.kind() {
        // `improper_ctypes_definitions` warning doesn't complain about the following types:
        ty::TyKind::Bool
        | ty::TyKind::Float { .. }
        | ty::TyKind::Int { .. }
        | ty::TyKind::Uint { .. }
        | ty::TyKind::Never
        | ty::TyKind::FnPtr { .. } => true,

        ty::TyKind::RawPtr(pointee, ..) => is_abi_compatible_pointee(db, *pointee),
        ty::TyKind::Tuple(types) if types.is_empty() => true,
        ty::TyKind::Char => !db.portable_abi_compatible(),

        // Crubit's C++ bindings for tuples, structs, and other ADTs may not preserve
        // their ABI (even if they *do* preserve their memory layout).  For example:
        // - In System V ABI replacing a field with a fixed-length array of bytes may affect
        //   whether the whole struct is classified as an integer and passed in general purpose
        //   registers VS classified as SSE2 and passed in floating-point registers like xmm0).
        //   See also b/270454629.
        // - To replicate field offsets, Crubit may insert explicit padding fields. These
        //   extra fields may also impact the ABI of the generated bindings.
        //
        // TODO(lukasza): In the future, some additional performance gains may be realized by
        // returning `true` in a few limited cases (this may require additional complexity to
        // ensure that `generate_adt` never injects explicit padding into such structs):
        // - `#[repr(C)]` structs and unions,
        // - Discriminant-only enums (b/259984090).
        ty::TyKind::Tuple { .. } => false, // An empty tuple (`()` - the unit type) is handled above.
        ty::TyKind::Adt(adt, substs) => {
            if is_std_ptr_non_null(tcx, adt.did()) {
                let pointee = substs[0].expect_ty();
                return is_abi_compatible_pointee(db, pointee);
            }
            if !db.is_cpp_move_constructible(ty) {
                return false;
            }
            let attrs = crubit_attr::get_attrs(tcx, adt.did()).unwrap_or_default();
            if attrs.same_abi {
                return true;
            }
            // NOTE: the below categorizes repr(transparent) types, but that only
            // works if the C++ side uses the _underlying_ type. If it uses the actual
            // same type as Rust, repr(transparent) actually makes it _less_ ABI compatible!
            if db.portable_abi_compatible() {
                return false;
            }
            if !adt.repr().transparent() {
                // If our adt is not transparent, it is not abi compatible by value.
                return false;
            }
            let Some(field) = adt.all_fields().next() else {
                // TODO: b/258259459 - Support zero sized types.
                return false;
            };
            #[rustversion::before(2026-04-19)]
            let mut ty = tcx.type_of(field.did).instantiate(tcx, substs);
            #[rustversion::since(2026-04-19)]
            let mut ty = tcx.type_of(field.did).instantiate(tcx, substs).skip_normalization();

            // Pattern types can be considered by value when they're embedded within an ADT.
            // We dont' want to do that for pattern types at large because they might mean they're
            // in a function signature, and we cannot uphold a pattern types invariants across the
            // FFI boundary leading to UB.
            if let ty::TyKind::Pat(pat_ty, _) = ty.kind() {
                ty = *pat_ty;
            }
            is_c_abi_compatible_by_value(db, ty)
        }
        ty::TyKind::Pat { .. } | ty::TyKind::Coroutine { .. } |
        // Arrays are explicitly not ABI-compatible (though they are layout-compatible).
        ty::TyKind::Array { .. } | ty::TyKind::Alias { .. } |
        // In case we were visited via a repr(transparent) wrapper, instead of `Ref` etc.
        ty::TyKind::Slice { .. } | ty::TyKind::Str => false,

        // `format_ty_for_cc` is expected to fail for other kinds of types
        // and therefore `is_c_abi_compatible_by_value` should never be called for
        // these other types
        ty => panic!("unsupported type kind: {ty:?}"),
    }
}

/// The prefix for deanonymized region names.
pub const ANON_REGION_PREFIX: &str = "'__anon";

#[rustversion::before(2026-04-19)]
pub type PolyFnSig<'tcx> = ty::PolyFnSig<'tcx>;

#[rustversion::since(2026-04-19)]
pub type PolyFnSig<'tcx> = ty::Unnormalized<'tcx, ty::PolyFnSig<'tcx>>;

/// Similar to `TyCtxt::liberate_and_name_late_bound_regions` but also replaces
/// anonymous regions with new names.
pub fn liberate_and_deanonymize_late_bound_regions<'tcx>(
    tcx: TyCtxt<'tcx>,
    sig: PolyFnSig<'tcx>,
    fn_def_id: DefId,
) -> ty::FnSig<'tcx> {
    #[rustversion::since(2026-04-19)]
    let sig = sig.skip_normalization();
    let mut anon_count: u32 = 0;
    let mut translated_kinds: HashMap<ty::BoundVar, ty::BoundRegionKind> = HashMap::new();
    let region_f = |br: ty::BoundRegion<'tcx>| {
        let new_kind: &ty::BoundRegionKind = translated_kinds.entry(br.var).or_insert_with(|| {
            if br.kind.is_named(tcx) {
                let id = br.kind.get_id().unwrap_or(fn_def_id);
                ty::BoundRegionKind::Named(id)
            } else {
                anon_count += 1;
                let name = Symbol::intern(&format!("{ANON_REGION_PREFIX}{anon_count}"));
                ty::BoundRegionKind::NamedForPrinting(name)
            }
        });
        #[cfg_accessible(rustc_middle::ty::RegionExt)]
        use rustc_middle::ty::RegionExt;
        ty::Region::new_late_param(
            tcx,
            fn_def_id,
            ty::LateParamRegionKind::from_bound(br.var, *new_kind),
        )
    };
    tcx.instantiate_bound_regions_uncached(sig, region_f)
}

/// This is mirroring the logic of TyCtxt::try_normalize_after_erasing_regions except it does not
/// erase regions. Because we emit lifetime annotations it's important that we do not erase
/// regions.
pub fn try_normalize<'tcx, T: ty::TypeFoldable<TyCtxt<'tcx>>>(
    tcx: TyCtxt<'tcx>,
    typing_env: ty::TypingEnv<'tcx>,
    value: T,
) -> Result<T, NoSolution> {
    let (infcx, param_env) = tcx.infer_ctxt().build_with_typing_env(typing_env);
    try_normalize_with_infcx(&infcx, param_env, value)
}

pub fn try_normalize_non_body<'tcx, T: ty::TypeFoldable<TyCtxt<'tcx>>>(
    tcx: TyCtxt<'tcx>,
    param_env: ty::ParamEnv<'tcx>,
    value: T,
) -> Result<T, NoSolution> {
    use rustc_trait_selection::infer::canonical::ir::TypingMode;
    let infcx = tcx.infer_ctxt().build(TypingMode::non_body_analysis());
    try_normalize_with_infcx(&infcx, param_env, value)
}

fn try_normalize_with_infcx<'tcx, T: ty::TypeFoldable<TyCtxt<'tcx>>>(
    infcx: &rustc_infer::infer::InferCtxt<'tcx>,
    param_env: ty::ParamEnv<'tcx>,
    value: T,
) -> Result<T, NoSolution> {
    use rustc_trait_selection::traits::query::normalize::QueryNormalizeExt;
    use rustc_trait_selection::traits::{Normalized, ObligationCause};
    let cause = ObligationCause::dummy(); // RESPECTFUL_TERMS_EXCEPTION: rustc code we don't own.
    infcx
        .at(&cause, param_env)
        .query_normalize(value)
        // We ignore obligations, since we know this code already type checks.
        // We're only interested in expanding projections of associated types.
        .map(|Normalized { value, .. }| value)
}

pub fn has_non_lifetime_generics<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> bool {
    tcx.generics_of(def_id)
        .own_params
        .iter()
        .any(|param| !matches!(param.kind, ty::GenericParamDefKind::Lifetime))
}

pub fn post_analysis_typing_env(tcx: TyCtxt, def_id: DefId) -> ty::TypingEnv {
    ty::TypingEnv::post_analysis(tcx, def_id)
}

/// Returns whether `ty` is copyable inside the given environment (e.g. fn or type def).
pub fn is_copy<'tcx>(
    tcx: TyCtxt<'tcx>,
    environment_id: impl Into<Option<DefId>>,
    ty: Ty<'tcx>,
) -> bool {
    // TODO(b/259749095): Once generic ADTs are supported, `is_copy_modulo_regions`
    // might need to be replaced with a more thorough check - see
    // b/258249993#comment4.
    let typing_env = environment_id
        .into()
        .map(|id| post_analysis_typing_env(tcx, id))
        .unwrap_or_else(ty::TypingEnv::fully_monomorphized);
    tcx.type_is_copy_modulo_regions(typing_env, ty)
}

/// Returns whether `ty` contains unrevealed opaque types (or aliases).
///
/// In newer rustc versions, calling `tcx.layout_of` in an empty
/// `TypingEnv::fully_monomorphized()` environment on a type containing unrevealed
/// opaque types (such as `std::env::SplitPaths`, which contains an opaque path
/// iterator) results in an internal compiler error (delayed bug: "unexpected
/// rigid alias in layout_of after normalization"). Checking beforehand allows
/// `get_layout` to gracefully bail and return an error instead.
fn has_unrevealed_opaque_type<'tcx>(
    tcx: TyCtxt<'tcx>,
    ty: Ty<'tcx>,
    seen: &mut HashSet<DefId>,
) -> bool {
    let ty = try_normalize(tcx, ty::TypingEnv::fully_monomorphized(), ty).unwrap_or(ty);
    for generic_arg in ty.walk() {
        if let Some(inner_ty) = generic_arg.as_type() {
            if matches!(inner_ty.kind(), ty::TyKind::Alias(..)) {
                return true;
            }
            if let ty::TyKind::Adt(adt_def, substs) = inner_ty.kind() {
                if !seen.insert(adt_def.did()) || adt_def.is_phantom_data() {
                    continue;
                }
                for variant in adt_def.variants() {
                    for field in &variant.fields {
                        #[rustversion::before(2026-04-19)]
                        let field_ty = field.ty(tcx, substs);
                        #[rustversion::since(2026-04-19)]
                        let field_ty = field.ty(tcx, substs).skip_norm_wip();
                        if has_unrevealed_opaque_type(tcx, field_ty, seen) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn get_layout<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Result<Layout<'tcx>> {
    if has_unrevealed_opaque_type(tcx, ty, &mut HashSet::new()) {
        return Err(anyhow!("Cannot compute layout for type with unrevealed opaque types: {ty}"));
    }
    tcx.layout_of(ty::TypingEnv::fully_monomorphized().as_query_input(ty))
        .map(|ty_and_layout| ty_and_layout.layout)
        .map_err(|layout_err| {
            // Have to use `.map_err`, because `LayoutError` doesn't satisfy the
            // `anyhow::context::ext::StdError` trait bound.
            anyhow!("Error computing the layout: {layout_err}")
        })
}

// Accounts for the offset in the front of a repr(C) enum with multiple
// variants. If given a layout with a single variant, returns 0.
pub fn get_tag_size_with_padding(layout: Layout<'_>) -> u64 {
    match layout.variants() {
        Variants::Single { .. } | Variants::Empty => 0,
        Variants::Multiple { tag: _, tag_encoding: _, tag_field: _, variants } => {
            #[rustversion::before(2026-05-18)]
            let mut variant_offsets = variants.iter().filter_map(|variant| match &variant.fields {
                FieldsShape::Arbitrary { offsets, .. } => {
                    offsets.get(FieldIdx::from_usize(0)).map(|offset| offset.bytes())
                }
                _ => panic!("Internal Error - Detected an enum with non-arbitrary field"),
            });

            #[rustversion::since(2026-05-18)]
            let mut variant_offsets = variants.iter().filter_map(|variant| {
                variant.field_offsets.get(FieldIdx::from_usize(0)).map(|offset| offset.bytes())
            });

            // There are two equivalent ways to express a rust enum:
            // 1. A struct that contains the discriminant and a union of the variants
            // 2. A union where each field begins with a discriminant.
            //
            // Rust internally uses the second representation, and we extract out the
            // discriminant to produce the first.
            //
            //
            // See https://doc.rust-lang.org/beta/nightly-rustc/rustc_abi/enum.FieldsShape.html#variant.Arbitrary
            // and https://doc.rust-lang.org/reference/type-layout.html#reprc-enums-with-fields
            let Some(expected_offset) = variant_offsets.next() else {
                return 0;
            };
            for variant_offset in variant_offsets {
                if variant_offset != expected_offset {
                    panic!("Internal Error - Detected an enum with different tag offsets.")
                }
            }
            expected_offset
        }
    }
}

fn does_type_implement_trait_substs<'tcx>(
    tcx: TyCtxt<'tcx>,
    self_ty: Ty<'tcx>,
    trait_id: DefId,
    generic_args: impl IntoIterator<Item = GenericArg<'tcx>>,
) -> Vec<GenericArg<'tcx>> {
    assert!(tcx.is_trait(trait_id));

    let generics = tcx.generics_of(trait_id);
    assert!(generics.has_self);
    // Self type must be first in our substitution.
    let substs = std::iter::once(GenericArg::from(self_ty)).chain(generic_args).collect::<Vec<_>>();
    // Assert we've provided the expected kind of args for each generic param.
    // For example, we haven't passed a lifetime where a type is expected.
    assert!(generics.own_params.len() == substs.len());
    assert!(generics.own_params.iter().zip(substs.iter()).all(|(param, arg)| matches!(
        (&param.kind, arg.kind()),
        (GenericParamDefKind::Type { .. }, GenericArgKind::Type(_))
            | (GenericParamDefKind::Lifetime, GenericArgKind::Lifetime(_))
            | (GenericParamDefKind::Const { .. }, GenericArgKind::Const(_))
    )));
    substs
}

pub fn does_type_implement_trait<'tcx>(
    tcx: TyCtxt<'tcx>,
    self_ty: Ty<'tcx>,
    trait_id: DefId,
    generic_args: impl IntoIterator<Item = GenericArg<'tcx>>,
) -> bool {
    does_type_implement_trait_with_param_env(
        tcx,
        self_ty,
        trait_id,
        tcx.param_env(trait_id),
        generic_args,
    )
}

pub fn does_type_implement_trait_with_param_env<'tcx>(
    tcx: TyCtxt<'tcx>,
    self_ty: Ty<'tcx>,
    trait_id: DefId,
    param_env: ty::ParamEnv<'tcx>,
    generic_args: impl IntoIterator<Item = GenericArg<'tcx>>,
) -> bool {
    let substs = does_type_implement_trait_substs(tcx, self_ty, trait_id, generic_args);
    use rustc_middle::ty::TypingMode;
    tcx.infer_ctxt()
        .build(TypingMode::non_body_analysis())
        .type_implements_trait(trait_id, substs, param_env)
        .must_apply_modulo_regions()
}
