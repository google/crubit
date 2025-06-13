// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_hir;
extern crate rustc_middle;

use crate::db::BindingsGenerator;
use rustc_hir::HirId;
use rustc_middle::ty::{self, Ty};

/// A Ty, optionally attached to its `hir::Ty` counterpart, if any.
///
/// The rustc_hir::Ty is used only for detecting type aliases (or other
/// optional sugar), unrelated to the actual concrete type. It
/// necessarily disappears if, for instance, the type is plugged in from
/// a generic. There's no way to tell, in the bindings for
/// Vec<c_char>::len(), that `T` came from the type alias
/// `c_char`, instead of a plain `i8` or `u8`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SugaredTy<'tcx> {
    mid: Ty<'tcx>,
    /// The HirId of the corresponding HirTy. We store it as a HirId so that
    /// it's hashable.
    hir_id: Option<HirId>,
}

impl<'tcx> std::fmt::Display for SugaredTy<'tcx> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(&self.mid, f)
    }
}

impl<'tcx> SugaredTy<'tcx> {
    pub fn new(mid: Ty<'tcx>, hir: Option<&rustc_hir::Ty<'tcx>>) -> Self {
        Self { mid, hir_id: hir.map(|hir| hir.hir_id) }
    }

    pub fn missing_hir(mid: Ty<'tcx>) -> Self {
        Self { mid, hir_id: None }
    }

    /// Returns the rustc_middle::Ty this represents.
    pub fn mid(&self) -> Ty<'tcx> {
        self.mid
    }

    /// Returns the rustc_hir::Ty this represents, if any.
    pub fn hir(&self, db: &dyn BindingsGenerator<'tcx>) -> Option<&'tcx rustc_hir::Ty<'tcx>> {
        let hir_id = self.hir_id?;
        let hir_ty = db.tcx().hir_node(hir_id).expect_ty();
        debug_assert_eq!(hir_ty.hir_id, hir_id);
        Some(hir_ty)
    }

    pub fn fn_inputs(
        sig_mid: &ty::FnSig<'tcx>,
        sig_hir: Option<&rustc_hir::FnDecl<'tcx>>,
    ) -> SugaredTyList<'tcx> {
        let hir_tys = sig_hir.map(|sig_hir| sig_hir.inputs);
        SugaredTyList { tys: sig_mid.inputs(), hir_tys }
    }

    pub fn fn_output(
        sig_mid: &ty::FnSig<'tcx>,
        sig_hir: Option<&rustc_hir::FnDecl<'tcx>>,
    ) -> SugaredTy<'tcx> {
        let hir_output = sig_hir.and_then(|sig_hir| match sig_hir.output {
            rustc_hir::FnRetTy::Return(hir_ty) => Some(hir_ty),
            _ => None,
        });
        SugaredTy::new(sig_mid.output(), hir_output)
    }

    /// If this is a tuple type, returns a SugaredTyList of the tuple elements.
    pub fn as_tuple(&self, db: &dyn BindingsGenerator<'tcx>) -> Option<SugaredTyList<'tcx>> {
        let ty::TyKind::Tuple(tys) = self.mid.kind() else { return None };
        let hir_tys = self.hir(db).and_then(|hir_ty| {
            if let rustc_hir::TyKind::Tup(tys) = hir_ty.kind {
                Some(tys)
            } else {
                None
            }
        });
        Some(SugaredTyList { tys, hir_tys })
    }
}

/// A list of `SugaredTy`s that can be created lazily from a list of `Ty` and
/// an optional list of `rustc_hir::Ty`s.
///
/// This allows for easy `SugaredTy` iteration over existing `Ty` lists, such as those in
/// tuples or function signatures.
pub struct SugaredTyList<'tcx> {
    pub tys: &'tcx [Ty<'tcx>],
    pub hir_tys: Option<&'tcx [rustc_hir::Ty<'tcx>]>,
}

impl<'tcx> SugaredTyList<'tcx> {
    pub fn len(&self) -> usize {
        self.tys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tys.is_empty()
    }

    // NOTE: This is not the `Index` trait because the `Index` trait must return a reference.
    pub fn index(&self, i: usize) -> SugaredTy<'tcx> {
        SugaredTy::new(self.tys[i], self.hir_tys.map(|hir_tys| hir_tys[i]).as_ref())
    }
}

impl<'tcx> Iterator for SugaredTyList<'tcx> {
    type Item = SugaredTy<'tcx>;
    fn next(&mut self) -> Option<SugaredTy<'tcx>> {
        let (ty, tys_rest) = self.tys.split_first()?;
        self.tys = tys_rest;
        let hir_ty = self.hir_tys.and_then(|hir_tys| {
            let (hir_ty, hir_tys_rest) = hir_tys.split_first()?;
            self.hir_tys = Some(hir_tys_rest);
            Some(hir_ty)
        });
        Some(SugaredTy::new(*ty, hir_ty))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl std::iter::ExactSizeIterator for SugaredTyList<'_> {}

impl<'tcx> From<SugaredTy<'tcx>> for Ty<'tcx> {
    fn from(ty: SugaredTy<'tcx>) -> Ty<'tcx> {
        ty.mid()
    }
}
