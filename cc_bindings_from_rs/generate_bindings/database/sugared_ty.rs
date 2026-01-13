// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_hir;
extern crate rustc_middle;

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
}

impl<'tcx> std::fmt::Display for SugaredTy<'tcx> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(&self.mid, f)
    }
}

impl<'tcx> SugaredTy<'tcx> {
    pub fn missing_hir(mid: Ty<'tcx>) -> Self {
        Self { mid }
    }

    /// Returns the rustc_middle::Ty this represents.
    pub fn mid(&self) -> Ty<'tcx> {
        self.mid
    }

    pub fn fn_inputs(sig_mid: &ty::FnSig<'tcx>) -> SugaredTyList<'tcx> {
        SugaredTyList { tys: sig_mid.inputs() }
    }

    pub fn fn_output(sig_mid: &ty::FnSig<'tcx>) -> SugaredTy<'tcx> {
        SugaredTy::missing_hir(sig_mid.output())
    }

    /// If this is a tuple type, returns a SugaredTyList of the tuple elements.
    pub fn as_tuple(&self) -> Option<SugaredTyList<'tcx>> {
        let ty::TyKind::Tuple(tys) = self.mid.kind() else { return None };
        Some(SugaredTyList { tys })
    }

    // TODO(b/449759899): Expand this to support all uninhabited types. Rename to `is_uninhabited`.
    pub fn is_never(&self) -> bool {
        *self.mid.kind() == ty::TyKind::Never
    }
}

/// A list of `SugaredTy`s that can be created lazily from a list of `Ty` and
/// an optional list of `rustc_hir::Ty`s.
///
/// This allows for easy `SugaredTy` iteration over existing `Ty` lists, such as those in
/// tuples or function signatures.
pub struct SugaredTyList<'tcx> {
    pub tys: &'tcx [Ty<'tcx>],
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
        SugaredTy::missing_hir(self.tys[i])
    }
}

impl<'tcx> Iterator for SugaredTyList<'tcx> {
    type Item = SugaredTy<'tcx>;
    fn next(&mut self) -> Option<SugaredTy<'tcx>> {
        let (ty, tys_rest) = self.tys.split_first()?;
        self.tys = tys_rest;
        Some(SugaredTy::missing_hir(*ty))
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
