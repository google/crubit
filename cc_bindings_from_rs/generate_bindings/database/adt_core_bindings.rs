// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_middle;
extern crate rustc_span;

use crate::code_snippet::ApiSnippets;
use proc_macro2::{Ident, TokenStream};
use rustc_middle::ty::Ty;
use rustc_span::def_id::DefId;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// Common bindings for all generated types.
#[derive(Clone)]
pub struct CoreBindingsCommon<'tcx> {
    /// C++ tag - e.g. `struct`, `class`, `enum`, or `union`.  This isn't always
    /// a direct mapping from Rust (e.g. a Rust `enum` might end up being
    /// represented as an opaque C++ `struct`).
    pub keyword: TokenStream,

    /// C++ translation of the ADT identifier - e.g. `SomeStruct`.
    ///
    /// A _short_ name is sufficient (i.e. there is no need to use a
    /// namespace-qualified name), for `CcSnippet`s that are emitted into
    /// the same namespace as the ADT.  (This seems to be all the snippets
    /// today.)
    pub cc_short_name: Ident,

    pub cc_fully_qualified_name: TokenStream,

    pub self_ty: Ty<'tcx>,
    pub alignment_in_bytes: u64,
    pub size_in_bytes: u64,
}

// CoreBindingsCommon is a pure (and memoized...) function of the self_ty.
impl PartialEq for CoreBindingsCommon<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.self_ty == other.self_ty
    }
}

impl Eq for CoreBindingsCommon<'_> {}
impl Hash for CoreBindingsCommon<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.self_ty.hash(state);
    }
}

/// Represents bindings for the "core" part of an algebraic data type (an ADT -
/// a struct, an enum, or a union) in a way that supports later injecting the
/// other parts like so:
///
/// ```
/// quote! {
///     #keyword #alignment #name final {
///         #core
///         #decls_of_other_parts  // (e.g. struct fields, methods, etc.)
///     }
/// }
/// ```
///
/// `keyword`, `name` are stored separately, to support formatting them as a
/// forward declaration - e.g. `struct SomeStruct`.
#[derive(Clone)]
pub struct AdtCoreBindings<'tcx> {
    pub common: Rc<CoreBindingsCommon<'tcx>>,

    /// Structs, enums, and unions. None for tuples.
    pub def_id: Option<DefId>,

    /// Rust spelling of the ADT type - e.g.
    /// `::some_crate::some_module::SomeStruct`.
    pub rs_fully_qualified_name: TokenStream,
}

// AdtCoreBindings are a pure (and memoized...) function of the def_id.
impl PartialEq for AdtCoreBindings<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.def_id == other.def_id && self.common == other.common
    }
}

impl Eq for AdtCoreBindings<'_> {}
impl Hash for AdtCoreBindings<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.def_id.hash(state);
        self.common.hash(state);
    }
}

/// The error type returned by `BindingsGenerator::generate_move_ctor_and_assignment_operator`.
//
// See discussion on http://cl/828812151 for why the type is in this crate/module, not the one that
// defines BindingsGenerator.
#[derive(Clone)]
pub struct NoMoveOrAssign<'tcx> {
    /// An error explaining why we didn't generate the special member functions.
    pub err: arc_anyhow::Error,

    /// Snippets containing explicitly deleted declarations.
    pub explicitly_deleted: ApiSnippets<'tcx>,
}

/// If we were to generate a C++ copy constructor and assignment operator for a
/// Rust-originating type, how would we do it?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CopyCodegenStyle {
    // The Rust type implements `Copy`. Crubit generates trivial `= default;`
    Copy,
    // The Rust type implements `Clone`. Crubit generates a C++ copy constructor
    // and assignment operator that invoke Rust `Clone::clone` and `Clone::clone_from` thunks.
    Clone,
}

impl CopyCodegenStyle {
    /// Returns a `CopyCodegenStyle` based on what combination of `Copy` and `Clone` the type
    /// implements.
    pub fn from_available_traits(implements_copy: bool, implements_clone: bool) -> Option<Self> {
        if implements_copy {
            Some(CopyCodegenStyle::Copy)
        } else if implements_clone {
            Some(CopyCodegenStyle::Clone)
        } else {
            None
        }
    }
}

/// If we were to generate a C++ move constructor and assignment operator for a Rust-originating
/// type, how would we do it?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MoveCodegenStyle {
    // The Rust type does not require drop glue (`!needs_drop`). Crubit generates trivial
    // `= default;` C++ move constructor and move-assignment operator.
    Default,
    // The Rust type implements `Default` and `Unpin`. Crubit generates a C++ move constructor and
    // move-assignment operator that move the value out by swapping with `Default::default()`.
    MemSwap,
    // The Rust type cannot be moved but has a copy constructor and assignment operator that are
    // used in lieu of the move constructor and assignment operator.
    Copy,
}

impl MoveCodegenStyle {
    /// Returns a `MoveCodegenStyle` based on what combination of Default, Unpin, Copy, and Clone
    /// the type implements.
    pub fn from_available_traits(
        does_not_need_drop: bool,
        has_default_ctor: bool,
        is_unpin: bool,
        has_copy_codegen_style: bool,
    ) -> Option<Self> {
        // If our type has no drop glue we can use the default move constructor and assignment operator.
        if does_not_need_drop {
            Some(MoveCodegenStyle::Default)
        } else if has_default_ctor && is_unpin {
            Some(MoveCodegenStyle::MemSwap)
        } else if has_copy_codegen_style {
            Some(MoveCodegenStyle::Copy)
        } else {
            None
        }
    }
}
