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
use error_report::{anyhow, bail, ensure};
use itertools::Itertools;
#[rustversion::before(2026-05-18)]
use rustc_abi::FieldsShape;
use rustc_abi::{BackendRepr, FieldIdx, HasDataLayout, Layout, Scalar, TargetDataLayout, Variants};
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
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display, Formatter};
use std::path::Path;

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

/// Whether the layout is from a type that implements [`std::marker::PointerLike`].
///
/// Currently, that means that the type is pointer-sized, pointer-aligned,
/// and has a initialized (non-union), scalar ABI.
pub fn has_pointer_abi(from: Layout, data_layout: &TargetDataLayout) -> bool {
    from.size() == data_layout.pointer_size()
        && from.align().abi == data_layout.pointer_align().abi
        && matches!(from.backend_repr(), BackendRepr::Scalar(Scalar::Initialized { .. }))
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
            if let Some(layout_equiv) = attrs
                .get_cpp_layout_equivalent_attrs(db.crubit_support_path_format())
                .ok()
                .flatten()
                && layout_equiv
                    .cpp_type
                    .as_str()
                    .parse()
                    .is_ok_and(|ts| database::is_cpp_pointer_type(ts).is_some())
            {
                return adt.repr().transparent()
                    && get_layout(tcx, ty)
                        .is_ok_and(|layout| has_pointer_abi(layout, tcx.data_layout()));
            }
            if attrs.cpp_type.is_some() {
                return false;
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
        ty::TyKind::Param { .. } => false,

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

/// Returns whether `ty` is copyable.
pub fn is_copy<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    tcx.type_is_copy_modulo_regions(ty::TypingEnv::fully_monomorphized(), ty)
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

pub fn get_validated_layout<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Result<Layout<'tcx>> {
    let layout = get_layout(tcx, ty)?;
    ensure!(
        layout.backend_repr().is_sized(),
        "Bindings for dynamically sized types are not supported."
    );
    ensure!(layout.size().bytes() != 0, "Zero-sized types (ZSTs) are not supported (b/258259459)");
    Ok(layout)
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

pub fn is_in_ignore_symbols_from_files<'tcx>(db: &BindingsGenerator<'tcx>, def_id: DefId) -> bool {
    let def_span = db.tcx().def_span(def_id);
    let filepath = db.tcx().sess.source_map().span_to_filename(def_span);
    let file_name = filepath.prefer_local_unconditionally().to_string();
    let file_name = file_name.strip_prefix("./").unwrap_or(file_name.as_str());
    db.ignore_symbols_from_files().contains(Path::new(file_name))
}

pub fn should_receive_bindings<'tcx>(db: &BindingsGenerator<'tcx>, def_id: DefId) -> Result<()> {
    let tcx = db.tcx();
    let attributes = crubit_attr::get_attrs(tcx, def_id).unwrap_or_default();
    if attributes.do_not_bind {
        let name = tcx.def_path_str(def_id);
        bail!("`{name}` does not receive bindings because it is marked `#[do_not_bind]`.");
    }
    // If the type has a `cpp_type` attribute, it should receive bindings even if it is defined in
    // an ignored file.
    if attributes.cpp_type.is_some() {
        return Ok(());
    }
    if is_in_ignore_symbols_from_files(db, def_id) {
        let name = tcx.def_path_str(def_id);
        bail!(
            "`{name}` does not receive bindings because it is defined in an \
            additional Rust source on a `rust_api_from_cpp` rule. See \
            crubit.rs/cpp/best_practices#types-in-srcs-do-not-receive-cpp-bindings."
        );
    }
    Ok(())
}

/// Checks whether a definition matches a specific qualified name by matching its definition path
/// against `name`. Name must include the crate in its path.
pub fn matches_qualified_name(db: &BindingsGenerator<'_>, item_did: DefId, name: &[&str]) -> bool {
    let tcx = db.tcx();
    let path = tcx.def_path(item_did);
    if path.data.len() + 1 != name.len() {
        return false;
    }
    // This will always return false for anonymous path data because the caller won't be able to
    // specify the `disambiguator` that gets inserted into the symbol. That's fine. It is expected
    // this function will only be called to check for non-anonymous paths.
    [tcx.crate_name(path.krate)]
        .into_iter()
        .chain(path.data.into_iter().map(|seg| seg.as_sym(/* verbose= */ false)))
        .zip(name.iter().map(|s| Symbol::intern(s)))
        .all(|(sym, expected)| sym == expected)
}

/// Compares two `DefId` s by their path and then by their path hash, which is stable across
/// compilations (the same ordering as `NodeSortKey` in `generate_bindings`).
pub fn stable_def_id_cmp<'tcx>(tcx: TyCtxt<'tcx>, lhs_id: DefId, rhs_id: DefId) -> Ordering {
    let sort_key =
        |def_id: DefId| (tcx.def_path_str(def_id), tcx.def_path_hash(def_id).local_hash().as_u64());
    sort_key(lhs_id).cmp(&sort_key(rhs_id))
}

pub trait SortedByDef: Iterator + Sized {
    fn sorted_by_def<'tcx>(self, tcx: TyCtxt<'tcx>) -> std::vec::IntoIter<Self::Item>
    where
        Self::Item: Copy + Into<DefId>,
    {
        self.sorted_by_def_with(tcx, |item| (*item).into())
    }

    fn sorted_by_def_with<'tcx>(
        self,
        tcx: TyCtxt<'tcx>,
        mut item_to_def_id: impl FnMut(&Self::Item) -> DefId,
    ) -> std::vec::IntoIter<Self::Item> {
        self.sorted_unstable_by(|lhs, rhs| {
            stable_def_id_cmp(tcx, item_to_def_id(lhs), item_to_def_id(rhs))
        })
    }
}
impl<T: Iterator + Sized> SortedByDef for T {}

#[rustversion::before(2026-04-19)]
pub fn normalize_ty<'tcx, T>(_tcx: TyCtxt<'tcx>, _param_env: ty::ParamEnv<'tcx>, val: T) -> T {
    val
}

#[rustversion::since(2026-04-19)]
pub fn try_normalize_ty<'tcx, T>(
    tcx: TyCtxt<'tcx>,
    param_env: ty::ParamEnv<'tcx>,
    val: ty::Unnormalized<'tcx, T>,
) -> Result<T>
where
    T: ty::TypeFoldable<TyCtxt<'tcx>>,
{
    try_normalize_non_body(tcx, param_env, val.skip_normalization())
        .map_err(|_| anyhow!("val contained unresolved type variables and could not be normalized"))
}

#[rustversion::since(2026-04-19)]
pub fn normalize_ty<'tcx, T>(
    tcx: TyCtxt<'tcx>,
    param_env: ty::ParamEnv<'tcx>,
    val: ty::Unnormalized<'tcx, T>,
) -> T
where
    T: ty::TypeFoldable<TyCtxt<'tcx>> + std::fmt::Debug,
{
    try_normalize_ty(tcx, param_env, val).expect(
        "normalize_ty called on a type that could not be normalized. Consider try_normalize_ty",
    )
}

/// Returns true if the field is public and stable.
///
/// Notably, this rejects public fields that are unstable, which may exist for macro accessibility
/// reasons.
pub fn field_def_is_pub_and_stable(
    tcx: TyCtxt<'_>,
    field_def: &ty::FieldDef,
) -> Result<(), PrivateOrUnstableField> {
    if field_def.vis != ty::Visibility::Public {
        return Err(PrivateOrUnstableField::Private);
    }
    if tcx.lookup_stability(field_def.did).is_some_and(|stability| stability.is_unstable()) {
        return Err(PrivateOrUnstableField::Unstable);
    }
    Ok(())
}

pub enum PrivateOrUnstableField {
    Private,
    Unstable,
}

impl Display for PrivateOrUnstableField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PrivateOrUnstableField::Private => write!(f, "private"),
            PrivateOrUnstableField::Unstable => write!(f, "unstable"),
        }
    }
}
