// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_middle;

use rustc_middle::ty::{self, Ty, TyCtxt, TypeFoldable, TypeSuperFoldable}; // See also <internal link>/ty.html#import-convention
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeCollisionRisk<'tcx, T> {
    pub item: T,
    pub key_type: Ty<'tcx>,
    pub preferred_type: Ty<'tcx>,
}

pub trait AvoidCollidingTypes<'tcx, T: 'tcx>: Iterator<Item = T> {
    /// Categorizes the items from the iterator, using the type reported by `key_getter`
    /// into either `Ok(item)` or `Err(TypeCollisionRisk)` if they would collide
    /// in C++.  For example, we don't want both `Index<u64>` and `Index<usize>` in the
    /// generated C++, as they may map to the same `operator[](uint64_t)` overload on some
    /// target platforms.
    fn avoid_colliding_types(
        self,
        tcx: TyCtxt<'tcx>,
        key_getter: impl Fn(&T) -> ty::Ty<'tcx>,
    ) -> Vec<Result<T, TypeCollisionRisk<'tcx, T>>>
    where
        Self: Sized,
    {
        let annotated_items: Vec<TypeCollisionRisk<'tcx, T>> = self
            .map(|item| {
                let key_type = key_getter(&item);
                let preferred_type = get_preferred_type(tcx, key_type);
                TypeCollisionRisk { item, key_type, preferred_type }
            })
            .collect();

        let present_preferred_types: HashSet<Ty<'tcx>> = annotated_items
            .iter()
            .filter(|x| x.key_type == x.preferred_type)
            .map(|x| x.preferred_type)
            .collect();

        annotated_items
            .into_iter()
            .map(|annotation| {
                if annotation.key_type == annotation.preferred_type
                    || !present_preferred_types.contains(&annotation.preferred_type)
                {
                    Ok(annotation.item)
                } else {
                    Err(annotation)
                }
            })
            .collect()
    }
}

impl<'tcx, T: 'tcx, I> AvoidCollidingTypes<'tcx, T> for I where I: Iterator<Item = T> {}

/// Returns a type that should be preferred over `ty` (if both are present in the input
/// to `avoid_colliding_types`).
///
/// Implementation covers 3 kinds of types:
/// * Types that may map to the same C++ type (e.g. `usize` and `u64`) are grouped into
///   equivalence classes (e.g. `usize`, `u32`, and `u64`) and one of the types is chosen
///   as the preferred type (e.g. `usize`, because it is common in `Index<T>`).
/// * Structured types that need to be recursively handled (e.g. tuples, refs, slices, etc.,
///   but not ADTs/structs)
/// * Types that don't risk a C++ collision (e.g. `char` and `u8`) are returned as their
///   own preferred type (i.e. their equivalence class contains only 1 type - themselves).
fn get_preferred_type<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
    struct PreferredTypeFolder<'tcx> {
        tcx: TyCtxt<'tcx>,
    }

    impl<'tcx> ty::TypeFolder<TyCtxt<'tcx>> for PreferredTypeFolder<'tcx> {
        fn cx(&self) -> TyCtxt<'tcx> {
            self.tcx
        }

        fn fold_ty(&mut self, ty: Ty<'tcx>) -> Ty<'tcx> {
            use ty::IntTy::*;
            use ty::UintTy::*;
            match ty.kind() {
                ty::TyKind::Int(Isize | I32 | I64) => Ty::new_int(self.tcx, Isize),
                ty::TyKind::Uint(Usize | U32 | U64) => Ty::new_uint(self.tcx, Usize),
                _ => ty.super_fold_with(self),
            }
        }
    }

    ty.fold_with(&mut PreferredTypeFolder { tcx })
}
