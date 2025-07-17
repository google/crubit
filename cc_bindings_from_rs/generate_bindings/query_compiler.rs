// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(rustc_private)]
#![deny(rustc::internal)]

//! Query the rust compiler.

extern crate rustc_abi;
extern crate rustc_ast;
extern crate rustc_attr_data_structures;
extern crate rustc_hir;
extern crate rustc_infer;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_trait_selection;

use arc_anyhow::Result;
use error_report::anyhow;
use rustc_abi::IntegerType;
use rustc_abi::{FieldIdx, FieldsShape, Integer, Layout, Primitive, Scalar, Variants};
use rustc_ast::ast::{IntTy as IntT, UintTy as UintT};
use rustc_attr_data_structures::IntType;
use rustc_hir::def::DefKind;
use rustc_infer::infer::TyCtxtInferExt;
use rustc_middle::ty::{self, IntTy, Region, Ty, TyCtxt, UintTy};
use rustc_span::def_id::{DefId, LocalDefId, LocalModDefId};
use rustc_span::symbol::Symbol;
use rustc_trait_selection::infer::InferCtxtExt;
use std::collections::HashMap;
use std::rc::Rc;

/// Whether functions using `extern "C"` ABI can safely handle values of type
/// `ty` (e.g. when passing by value arguments or return values of such type).
pub fn is_c_abi_compatible_by_value(ty: Ty) -> bool {
    match ty.kind() {
        // `improper_ctypes_definitions` warning doesn't complain about the following types:
        ty::TyKind::Bool |
        ty::TyKind::Float{..} |
        ty::TyKind::Int{..} |
        ty::TyKind::Uint{..} |
        ty::TyKind::Never |
        ty::TyKind::RawPtr{..} |
        ty::TyKind::Ref{..} |
        ty::TyKind::FnPtr{..} => true,
        ty::TyKind::Tuple(types) if types.len() == 0 => true,

        // Crubit assumes that `char` is compatible with a certain `extern "C"` ABI.
        // See `rust_builtin_type_abi_assumptions.md` for more details.
        ty::TyKind::Char => true,

        // TODO(b/271016831): When launching `&[T]` (not just `*const T`), consider returning
        // `true` for `TyKind::Ref` and document the rationale for such decision - maybe
        // something like this will be sufficient:
        // - In general `TyKind::Ref` should have the same ABI as `TyKind::RawPtr`
        // - References to slices (`&[T]`) or strings (`&str`) rely on assumptions
        //   spelled out in `rust_builtin_type_abi_assumptions.md`.
        ty::TyKind::Slice{..} => false,

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
        // - `#[repr(transparent)]` struct that wraps an ABI-safe type,
        // - Discriminant-only enums (b/259984090).
        ty::TyKind::Tuple{..} |  // An empty tuple (`()` - the unit type) is handled above.
        ty::TyKind::Adt{..} => false,

        // These kinds of reference-related types are not implemented yet - `is_c_abi_compatible_by_value`
        // should never need to handle them, because `format_ty_for_cc` fails for such types.
        ty::TyKind::Str |
        ty::TyKind::Array{..} => unimplemented!(),

        // `format_ty_for_cc` is expected to fail for other kinds of types
        // and therefore `is_c_abi_compatible_by_value` should never be called for
        // these other types
        _ => unimplemented!(),
    }
}

/// Gets the exactly one region used in this function signature.
///
/// If the function has more than one region, or no regions, returns None.
pub fn count_regions<'tcx>(sig_mid: &ty::FnSig<'tcx>) -> HashMap<Region<'tcx>, u8> {
    use rustc_middle::ty::TypeVisitor;
    struct RegionCounter<'tcx>(HashMap<Region<'tcx>, u8>);
    impl<'tcx> TypeVisitor<TyCtxt<'tcx>> for RegionCounter<'tcx> {
        fn visit_region(&mut self, region: Region<'tcx>) {
            let count = self.0.entry(region).or_default();
            *count = count.saturating_add(1);
        }
    }

    let mut visitor = RegionCounter(Default::default());
    for ty in sig_mid.inputs() {
        visitor.visit_ty(*ty);
    }
    visitor.visit_ty(sig_mid.output());
    visitor.0
}

/// The prefix for deanonymized region names.
pub const ANON_REGION_PREFIX: &str = "'__anon";

/// Similar to `TyCtxt::liberate_and_name_late_bound_regions` but also replaces
/// anonymous regions with new names.
pub fn liberate_and_deanonymize_late_bound_regions<'tcx>(
    tcx: TyCtxt<'tcx>,
    sig: ty::PolyFnSig<'tcx>,
    fn_def_id: DefId,
) -> ty::FnSig<'tcx> {
    let mut anon_count: u32 = 0;
    let mut translated_kinds: HashMap<ty::BoundVar, ty::BoundRegionKind> = HashMap::new();
    let region_f = |br: ty::BoundRegion| {
        let new_kind: &ty::BoundRegionKind = translated_kinds.entry(br.var).or_insert_with(|| {
            if br.kind.is_named(tcx) {
                let id = br.kind.get_id().unwrap_or(fn_def_id);
                ty::BoundRegionKind::Named(id)
            } else {
                anon_count += 1;
                let name = Symbol::intern(&format!("{ANON_REGION_PREFIX}{anon_count}"));
                ty::BoundRegionKind::NamedAnon(name)
            }
        });
        ty::Region::new_late_param(
            tcx,
            fn_def_id,
            ty::LateParamRegionKind::from_bound(br.var, *new_kind),
        )
    };
    tcx.instantiate_bound_regions_uncached(sig, region_f)
}

pub fn has_non_lifetime_generics<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> bool {
    tcx.generics_of(def_id)
        .own_params
        .iter()
        .any(|param| !matches!(param.kind, ty::GenericParamDefKind::Lifetime))
}

pub fn public_free_items_in_mod(tcx: TyCtxt, def_id: DefId) -> Vec<(DefId, DefKind)> {
    let mut items = vec![];
    if def_id.as_local().is_none() {
        return items;
    }
    let local_mod_def_id = LocalModDefId::new_unchecked(def_id.as_local().unwrap());
    for item_id in tcx.hir_module_items(local_mod_def_id).free_items() {
        let item_local_def_id: LocalDefId = item_id.owner_id.def_id;
        let item_def_id = item_local_def_id.to_def_id();
        let item_def_kind = tcx.def_kind(item_def_id);

        if !is_exported(tcx, item_def_id) {
            continue;
        }
        match item_def_kind {
            DefKind::Fn | DefKind::Struct | DefKind::Enum => {
                items.push((item_def_id, item_def_kind));
            }
            _ => {}
        }
    }
    items
}

pub fn post_analysis_typing_env(tcx: TyCtxt, def_id: DefId) -> ty::TypingEnv {
    ty::TypingEnv { typing_mode: ty::TypingMode::PostAnalysis, param_env: tcx.param_env(def_id) }
}

/// Returns whether `ty` is copyable inside the given environment (e.g. fn or type def).
pub fn is_copy<'tcx>(tcx: TyCtxt<'tcx>, environment_id: DefId, ty: Ty<'tcx>) -> bool {
    // TODO(b/259749095): Once generic ADTs are supported, `is_copy_modulo_regions`
    // might need to be replaced with a more thorough check - see
    // b/258249993#comment4.
    tcx.type_is_copy_modulo_regions(post_analysis_typing_env(tcx, environment_id), ty)
}

/// Like `TyCtxt::is_directly_public`, but works not only with `LocalDefId`, but
/// also with `DefId`.
pub fn is_directly_public(tcx: TyCtxt, def_id: DefId) -> bool {
    match def_id.as_local() {
        None => {
            // This mimics the checks in `try_print_visible_def_path_recur` in
            // `compiler/rustc_middle/src/ty/print/pretty.rs`.
            let actual_parent = tcx.opt_parent(def_id);
            let visible_parent = tcx.visible_parent_map(()).get(&def_id).copied();
            actual_parent == visible_parent
        }
        Some(local_def_id) => tcx.effective_visibilities(()).is_directly_public(local_def_id),
    }
}

/// Like `TyCtxt::is_exported`, but works not only with `LocalDefId`, but
/// also with `DefId`.
pub fn is_exported(tcx: TyCtxt, def_id: DefId) -> bool {
    match def_id.as_local() {
        None => {
            // This mimics the checks in `try_print_visible_def_path_recur` in
            // `compiler/rustc_middle/src/ty/print/pretty.rs`.
            let actual_parent = tcx.opt_parent(def_id);
            let visible_parent = tcx.visible_parent_map(()).get(&def_id).copied();
            actual_parent == visible_parent
        }
        Some(local_def_id) => tcx.effective_visibilities(()).is_exported(local_def_id),
    }
}

pub fn get_layout<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Result<Layout<'tcx>> {
    tcx.layout_of(ty::TypingEnv::fully_monomorphized().as_query_input(ty))
        .map(|ty_and_layout| ty_and_layout.layout)
        .map_err(|layout_err| {
            // Have to use `.map_err`, because `LayoutError` doesn't satisfy the
            // `anyhow::context::ext::StdError` trait bound.
            anyhow!("Error computing the layout: {layout_err}")
        })
}

fn convert_interger_type_to_int_type(input: IntegerType) -> IntType {
    match input {
        IntegerType::Pointer(true) => IntType::SignedInt(IntT::Isize),
        IntegerType::Pointer(false) => IntType::UnsignedInt(UintT::Usize),
        IntegerType::Fixed(Integer::I8, false) => IntType::UnsignedInt(UintT::U8),
        IntegerType::Fixed(Integer::I16, false) => IntType::UnsignedInt(UintT::U16),
        IntegerType::Fixed(Integer::I32, false) => IntType::UnsignedInt(UintT::U32),
        IntegerType::Fixed(Integer::I64, false) => IntType::UnsignedInt(UintT::U64),
        IntegerType::Fixed(Integer::I128, false) => IntType::UnsignedInt(UintT::U128),
        IntegerType::Fixed(Integer::I8, true) => IntType::SignedInt(IntT::I8),
        IntegerType::Fixed(Integer::I16, true) => IntType::SignedInt(IntT::I16),
        IntegerType::Fixed(Integer::I32, true) => IntType::SignedInt(IntT::I32),
        IntegerType::Fixed(Integer::I64, true) => IntType::SignedInt(IntT::I64),
        IntegerType::Fixed(Integer::I128, true) => IntType::SignedInt(IntT::I128),
    }
}

/// Implementation of `BindingsGenerator::repr_attrs`.
pub fn repr_attrs(tcx: TyCtxt, def_id: DefId) -> Rc<[rustc_attr_data_structures::ReprAttr]> {
    let mut result = Vec::new();
    let ty = tcx.type_of(def_id).instantiate_identity();
    match ty.kind() {
        ty::TyKind::Adt(adt_def, _) => {
            let repr = adt_def.repr();
            if repr.transparent() {
                result.push(rustc_attr_data_structures::ReprAttr::ReprTransparent);
            }
            if repr.c() {
                result.push(rustc_attr_data_structures::ReprAttr::ReprC);
            }
            if repr.simd() {
                result.push(rustc_attr_data_structures::ReprAttr::ReprSimd);
            }
            if let Some(alignment) = repr.align {
                result.push(rustc_attr_data_structures::ReprAttr::ReprAlign(alignment));
            }
            if let Some(alignment) = repr.pack {
                result.push(rustc_attr_data_structures::ReprAttr::ReprPacked(alignment));
            }
            if let Some(integer) = repr.int {
                result.push(rustc_attr_data_structures::ReprAttr::ReprInt(
                    convert_interger_type_to_int_type(integer),
                ));
            }
            if result.is_empty() {
                result.push(rustc_attr_data_structures::ReprAttr::ReprRust);
            }
            result.into()
        }
        _ => result.into(),
    }
}

// Converts a scalar integer to a Ty.
// We assume the scalar represents an integer, and not a float or a pointer.
// https://doc.rust-lang.org/beta/nightly-rustc/rustc_abi/enum.Primitive.html
pub fn get_scalar_int_type<'tcx>(tcx: TyCtxt<'tcx>, scalar: Scalar) -> Ty<'tcx> {
    match scalar.primitive() {
        Primitive::Int(scalar_int, signed) => {
            // Map the corresponding primitive to rust type.
            match (scalar_int, signed) {
                (Integer::I8, false) => Ty::new_uint(tcx, UintTy::U8),
                (Integer::I16, false) => Ty::new_uint(tcx, UintTy::U16),
                (Integer::I32, false) => Ty::new_uint(tcx, UintTy::U32),
                (Integer::I64, false) => Ty::new_uint(tcx, UintTy::U64),
                (Integer::I128, false) => Ty::new_uint(tcx, UintTy::U128),
                (Integer::I8, true) => Ty::new_int(tcx, IntTy::I8),
                (Integer::I16, true) => Ty::new_int(tcx, IntTy::I16),
                (Integer::I32, true) => Ty::new_int(tcx, IntTy::I32),
                (Integer::I64, true) => Ty::new_int(tcx, IntTy::I64),
                (Integer::I128, true) => Ty::new_int(tcx, IntTy::I128),
            }
        }
        _ => panic!("Internal error: integer scalar is not valid."),
    }
}

// Accounts for the offset in the front of a repr(C) enum with multiple
// variants. If given a layout with a single variant, returns 0.
pub fn get_tag_size_with_padding(layout: Layout<'_>) -> u64 {
    match layout.variants() {
        Variants::Single { .. } | Variants::Empty => 0,
        Variants::Multiple { tag: _, tag_encoding: _, tag_field: _, variants } => {
            let mut variant_offsets = variants.iter().map(|variant| match &variant.fields {
                FieldsShape::Arbitrary { offsets, .. } => {
                    if offsets.is_empty() {
                        variant.size.bytes() // No fields => variant is just the
                                             // tag.
                    } else {
                        offsets[FieldIdx::from_usize(0)].bytes()
                    }
                }
                _ => panic!("Internal Error - Detected an enum with non-arbitrary field"),
            });

            // There are two equivalent ways to express a rust enum:
            // 1. A struct that contains the discriminant and a union of the variants
            // 2. A union where each field begins with a discriminant.
            //
            // Rust interally uses the second representation, and we extract out the
            // discriminant to produce the first.
            //
            //
            // See https://doc.rust-lang.org/beta/nightly-rustc/rustc_abi/enum.FieldsShape.html#variant.Arbitrary
            // and https://doc.rust-lang.org/reference/type-layout.html#reprc-enums-with-fields
            let expected_offset = variant_offsets.next().expect("At least one variant is required");
            for variant_offset in variant_offsets {
                if variant_offset != expected_offset {
                    panic!("Internal Error - Detected an enum with different tag offsets.")
                }
            }
            expected_offset
        }
    }
}

pub fn does_type_implement_trait<'tcx>(
    tcx: TyCtxt<'tcx>,
    self_ty: Ty<'tcx>,
    trait_id: DefId,
) -> bool {
    assert!(tcx.is_trait(trait_id));

    let generics = tcx.generics_of(trait_id);
    assert!(generics.has_self);
    assert_eq!(
        generics.count(),
        1, // Only `Self`
        "Generic traits are not supported yet (b/286941486)",
    );
    let substs = [self_ty];

    use rustc_middle::ty::TypingMode;
    tcx.infer_ctxt()
        .build(TypingMode::non_body_analysis())
        .type_implements_trait(trait_id, substs, tcx.param_env(trait_id))
        .must_apply_modulo_regions()
}
