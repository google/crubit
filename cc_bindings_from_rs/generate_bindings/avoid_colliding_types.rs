// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_middle;

use rustc_middle::mir::Mutability;
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
        let results = self.map(Ok).collect();

        // Types that only differ in the width of their integers (e.g. `u64` and `usize`) may
        // map to the same C++ type on some target platforms.
        let results = retain_non_colliding(tcx, results, &key_getter, get_preferred_type);

        // A shared reference `&T` always overlaps with its referent `T`, because the C++
        // parameter is derived from a reference to the key type: `T` is passed as
        // `cpp_type_of(&T)`, whereas `&T` is passed as `cpp_type_of(&T) const&`.  For example
        // `impl PartialEq<str> for String` generates `operator==(rs_std::StrRef)` while
        // `impl PartialEq<&str> for String` generates `operator==(rs_std::StrRef const&)`,
        // and the two overloads are ambiguous at every call site.
        retain_non_colliding(tcx, results, &key_getter, |tcx, ty| {
            get_preferred_type(tcx, peel_shared_refs(ty))
        })
    }
}

impl<'tcx, T: 'tcx, I> AvoidCollidingTypes<'tcx, T> for I where I: Iterator<Item = T> {}

/// Turns the still-`Ok` entries of `results` that risk colliding with a more preferred entry
/// into `Err(TypeCollisionRisk)`.  Entries that are already `Err` are passed through.
///
/// An entry is only at risk when the type preferred over its key type (as reported by
/// `preference`) is itself the key type of another entry.  For example `i32` and `i64` never
/// collide with each other (they map to `int32_t` and `int64_t`), but each of them collides
/// with `isize` (which maps to `intptr_t`).
fn retain_non_colliding<'tcx, T>(
    tcx: TyCtxt<'tcx>,
    results: Vec<Result<T, TypeCollisionRisk<'tcx, T>>>,
    key_getter: &impl Fn(&T) -> Ty<'tcx>,
    preference: impl Fn(TyCtxt<'tcx>, Ty<'tcx>) -> Ty<'tcx>,
) -> Vec<Result<T, TypeCollisionRisk<'tcx, T>>> {
    // `None` for entries that have already been rejected by an earlier pass.
    let annotations: Vec<Option<(Ty<'tcx>, Ty<'tcx>)>> = results
        .iter()
        .map(|result| {
            let key_type = key_getter(result.as_ref().ok()?);
            Some((key_type, preference(tcx, key_type)))
        })
        .collect();

    let present_preferred_types: HashSet<Ty<'tcx>> = annotations
        .iter()
        .flatten()
        .filter(|(key_type, preferred_type)| key_type == preferred_type)
        .map(|(_, preferred_type)| *preferred_type)
        .collect();

    results
        .into_iter()
        .zip(annotations)
        .map(|(result, annotation)| {
            let item = result?;
            let (key_type, preferred_type) =
                annotation.expect("`Ok` entries are always annotated above");
            if key_type == preferred_type || !present_preferred_types.contains(&preferred_type) {
                Ok(item)
            } else {
                Err(TypeCollisionRisk { item, key_type, preferred_type })
            }
        })
        .collect()
}

/// Removes the outermost shared references from `ty` (e.g. maps `&&str` to `str`).
///
/// Only the outermost references are peeled off, because only they affect how the *top-level*
/// C++ parameter type is spelled (e.g. `(&Foo, u64)` and `(Foo, u64)` map to distinct C++
/// types and therefore don't collide).
fn peel_shared_refs<'tcx>(mut ty: Ty<'tcx>) -> Ty<'tcx> {
    while let ty::TyKind::Ref(_, referent, Mutability::Not) = ty.kind() {
        ty = *referent;
    }
    ty
}

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
