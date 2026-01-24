// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_span;

use crate::db::BindingsGenerator;
use arc_anyhow::Result;
use code_gen_utils::{format_cc_type_name, make_rs_ident, NamespaceQualifier};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use rustc_span::def_id::DefId;
use rustc_span::symbol::Symbol;
use std::rc::Rc;

/// Represents the unqualified name of a Rust item. For the fully qualified name
// `std::collections::HashMap`, the unqualified name would be `HashMap`.
#[derive(Clone, Debug, PartialEq)]
pub struct UnqualifiedName {
    /// Rust name of the item.
    /// For example, this would be:
    /// * `Ordering` for `std::cmp::Ordering`.
    pub rs_name: Symbol,

    /// The C++ name to use for the symbol.
    ///
    /// For example, the following struct
    /// ```
    /// #[crubit_annotate::cpp_layout_equivalent(cpp_name="Bar")]
    /// struct Foo { ... }
    /// ```
    /// will be generated as a C++ struct named `Bar` instead of `Foo`.
    pub cpp_name: Symbol,

    /// The fully-qualified C++ type to use for this, if this was originally a C++ type.
    ///
    /// For example, if a type has `#[crubit_annotate::cpp_layout_equivalent(cpp_type="x::y")]`,
    /// then cpp_type will be `Some(x::y)`.
    pub cpp_type: Option<Symbol>,
}

/// Represents the fully qualified name of a Rust item (e.g. of a `struct` or a
/// function).
#[derive(Clone, Debug, PartialEq)]
pub struct FullyQualifiedName {
    /// Name of the crate that defines the item.
    /// For example, this would be `std` for `std::cmp::Ordering`.
    pub krate: Symbol,

    /// Configurable top-level namespace of the C++ bindings.
    /// For example, this would be `::foo` for `foo::bar::baz::qux`.
    pub cpp_top_level_ns: Rc<[Symbol]>,

    /// Path to the module where the item is located.
    /// For example, this would be `cmp` for `std::cmp::Ordering`.
    /// The path may contain multiple modules - e.g. `foo::bar::baz`.
    pub rs_mod_path: NamespaceQualifier,
    /// The C++ namespace to use for the symbol excluding the top-level
    /// namespace.
    pub cpp_ns_path: NamespaceQualifier,

    /// The unqualified name of the item.
    pub unqualified: UnqualifiedName,
}

fn format_ns_path_for_cc(
    db: &BindingsGenerator<'_>,
    ns: &NamespaceQualifier,
) -> Result<TokenStream> {
    let idents =
        ns.parts().map(|s| db.format_cc_ident(Symbol::intern(s))).collect::<Result<Vec<_>>>()?;
    Ok(quote! { #(#idents::)* })
}

impl FullyQualifiedName {
    pub fn format_for_cc(&self, db: &BindingsGenerator<'_>) -> Result<TokenStream> {
        if let Some(path) = self.unqualified.cpp_type {
            let path = format_cc_type_name(path.as_str())?;
            return Ok(path);
        }

        let name = self.unqualified.cpp_name;

        let cpp_top_level_ns = self
            .cpp_top_level_ns
            .iter()
            .map(|ns| db.format_cc_ident(*ns))
            .collect::<Result<Vec<_>>>()?;
        let ns_path = format_ns_path_for_cc(db, &self.cpp_ns_path)?;
        let name = format_cc_type_name(name.as_str())?;
        Ok(quote! { :: #(#cpp_top_level_ns::)* #ns_path #name })
    }

    pub fn format_for_rs(&self) -> TokenStream {
        let iter = self.rs_name_parts();
        quote! { #( ::#iter )* }
    }

    /// Returns an iterator of parts to spell out the Rust name.
    pub fn rs_name_parts(&self) -> impl Iterator<Item = Ident> + use<'_> {
        let krate = make_rs_ident(self.krate.as_str());
        let mod_path = self.rs_mod_path.parts_with_snake_case_record_names();
        let name = self.unqualified.rs_name;
        let name = make_rs_ident(name.as_str());
        std::iter::once(krate).chain(mod_path).chain(std::iter::once(name))
    }
}

/// A publicly exported path for a definition.
///
/// This path is not necessarily semantically meaningful (i.e. it won't always have a corresponding
/// scope and DefId), but is always valid to spell out in syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportedPath {
    /// Segments of the path.
    pub path: Vec<Symbol>,
    /// If this path aliases a definition, this will be the alias. Otherwise, it will be the definition name.
    pub name: Symbol,
    /// If this path points as a type alias, rather than a use statement, this will be the DefId of
    /// the type alias.
    pub type_alias_def_id: Option<DefId>,
    /// True if any segment of this path is marked #[doc(hidden)] making it a less preferable path
    /// than any non-hidden path regardless of length.
    pub is_doc_hidden: bool,
}

impl From<&ExportedPath> for NamespaceQualifier {
    fn from(this: &ExportedPath) -> Self {
        NamespaceQualifier::new(
            this.path.iter().map(|s| Rc::<str>::from(s.as_str())).collect::<Vec<_>>(),
        )
    }
}

impl Ord for ExportedPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        // Prefer paths that are do not contain an item marked #[doc(hidden)].
        self.is_doc_hidden
            .cmp(&other.is_doc_hidden)
            // Prefer the shortest path.
            .then_with(|| self.path.len().cmp(&other.path.len()))
            // Between two paths of the same length, prefer the one that is not a type alias.
            .then_with(|| match (self.type_alias_def_id, other.type_alias_def_id) {
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                _ => Ordering::Equal,
            })
            // Failing all else, choose the lexicographically smallest path.
            .then_with(|| self.path.cmp(&other.path))
            .then_with(|| self.name.as_str().cmp(other.name.as_str()))
    }
}

impl PartialOrd for ExportedPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// The set of public paths for a definition.
///
/// The canonical path is the preferred path that should be treated as the primary definition for
/// generating bindings. Aliases are other paths the same definition is available at and should
/// generate aliases to the canonical path in the bindings.
///
/// Aliases are sorted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicPaths {
    canonical_path: ExportedPath,
    aliases: Vec<ExportedPath>,
}
impl PublicPaths {
    pub fn new(canonical_path: ExportedPath) -> Self {
        Self { canonical_path, aliases: vec![] }
    }

    pub fn insert(&mut self, path: ExportedPath) {
        let alias = if path < self.canonical_path {
            std::mem::replace(&mut self.canonical_path, path)
        } else {
            path
        };
        if let Err(index) = self.aliases.binary_search(&alias) {
            self.aliases.insert(index, alias);
        }
    }

    pub fn canonical(&self) -> &ExportedPath {
        &self.canonical_path
    }

    /// Turn `self` into it's canonical path and aliases. This should only be used on a definition
    /// defined in the source crate.
    pub fn into_canonical_and_aliases(self) -> (ExportedPath, Vec<ExportedPath>) {
        (self.canonical_path, self.aliases)
    }

    /// Turn `self` into a list of aliases for a definition defined in an external crate.
    /// External crate definitions should only be aliased, not used as the canonical definition.
    /// The returned vector is sorted.
    pub fn into_extern_aliases(mut self) -> Vec<ExportedPath> {
        self.aliases.insert(0, self.canonical_path);
        self.aliases
    }
}
