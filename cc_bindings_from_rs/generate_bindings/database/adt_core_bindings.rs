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
    /// DefId of the ADT.
    pub def_id: DefId,

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

    /// Rust spelling of the ADT type - e.g.
    /// `::some_crate::some_module::SomeStruct`.
    pub rs_fully_qualified_name: TokenStream,
    pub cc_fully_qualified_name: TokenStream,

    pub self_ty: Ty<'tcx>,
    pub alignment_in_bytes: u64,
    pub size_in_bytes: u64,
}

// AdtCoreBindings are a pure (and memoized...) function of the def_id.
impl PartialEq for AdtCoreBindings<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.def_id == other.def_id && self.self_ty == other.self_ty
    }
}

impl Eq for AdtCoreBindings<'_> {}
impl Hash for AdtCoreBindings<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.def_id.hash(state);
        self.self_ty.hash(state);
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
