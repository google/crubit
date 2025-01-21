// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::db::BindingsGenerator;
use rustc_hir::HirId;
use rustc_middle::ty::Ty;

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
}
