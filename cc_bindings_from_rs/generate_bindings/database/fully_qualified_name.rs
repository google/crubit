// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_span;

use crate::db::BindingsGenerator;
use arc_anyhow::Result;
use code_gen_utils::{make_rs_ident, NamespaceQualifier};
use proc_macro2::TokenStream;
use quote::quote;
use rustc_span::def_id::DefId;
use rustc_span::symbol::Symbol;
use std::rc::Rc;

/// Represents the fully qualified name of a Rust item (e.g. of a `struct` or a
/// function).
#[derive(Clone, Debug, PartialEq)]
pub struct FullyQualifiedName {
    /// Name of the crate that defines the item.
    /// For example, this would be `std` for `std::cmp::Ordering`.
    pub krate: Symbol,

    /// Configurable top-level namespace of the C++ bindings.
    /// For example, this would be `::foo` for `foo::bar::baz::qux`.
    pub cpp_top_level_ns: Symbol,

    /// Path to the module where the item is located.
    /// For example, this would be `cmp` for `std::cmp::Ordering`.
    /// The path may contain multiple modules - e.g. `foo::bar::baz`.
    pub rs_mod_path: NamespaceQualifier,
    /// The C++ namespace to use for the symbol excluding the top-level
    /// namespace.
    pub cpp_ns_path: NamespaceQualifier,

    /// Rust name of the item.
    /// For example, this would be:
    /// * `Some("Ordering")` for `std::cmp::Ordering`.
    /// * `None` for `ItemKind::Use` - e.g.: `use submodule::*`
    pub rs_name: Option<Symbol>,

    /// The C++ name to use for the symbol.
    ///
    /// For example, the following struct
    /// ```
    /// #[crubit_annotate::cpp_layout_equivalent(cpp_name="Bar")]
    /// struct Foo { ... }
    /// ```
    /// will be generated as a C++ struct named `Bar` instead of `Foo`.
    pub cpp_name: Option<Symbol>,

    /// The fully-qualified C++ type to use for this, if this was originally a
    /// C++ type.
    ///
    /// For example, if a type has `#[crubit_annotate::cpp_layout_equivalent(cpp_type="x::y")]`,
    /// then cpp_type will be `Some(x::y)`.
    pub cpp_type: Option<Symbol>,
}

fn format_ns_path_for_cc(
    db: &dyn BindingsGenerator<'_>,
    ns: &NamespaceQualifier,
) -> Result<TokenStream> {
    let idents =
        ns.parts().map(|s| db.format_cc_ident(Symbol::intern(s))).collect::<Result<Vec<_>>>()?;
    Ok(quote! { #(#idents::)* })
}

impl FullyQualifiedName {
    /// Computes a `FullyQualifiedName` for `def_id`.
    ///
    /// May panic if `def_id` is an invalid id.
    // TODO(b/259724276): This function's results should be memoized.
    pub fn new(db: &dyn BindingsGenerator<'_>, def_id: DefId) -> Self {
        if let Some(canonical_name) = db.reexported_symbol_canonical_name_mapping().get(&def_id) {
            return canonical_name.clone();
        }

        let tcx = db.tcx();
        let krate = tcx.crate_name(def_id.krate);
        let cpp_top_level_ns = db.format_top_level_ns_for_crate(def_id.krate);

        // Crash OK: these attributes are introduced by crubit itself, and "should
        // never" be malformed.
        let attributes = crubit_attr::get_attrs(tcx, def_id).unwrap();
        let cpp_type = attributes.cpp_type;

        let mut full_path = tcx.def_path(def_id).data; // mod_path + name
        let name = full_path.pop().expect("At least the item's name should be present");
        let rs_name = name.data.get_opt_name();
        let cpp_name = attributes.cpp_name.map(|s| Symbol::intern(s.as_str())).or(rs_name);

        let mod_path = NamespaceQualifier::new(
            full_path
                .into_iter()
                .filter_map(|p| p.data.get_opt_name())
                .map(|s| Rc::<str>::from(s.as_str())),
        );

        Self {
            krate,
            cpp_top_level_ns,
            rs_mod_path: mod_path.clone(),
            cpp_ns_path: mod_path,
            rs_name,
            cpp_name,
            cpp_type,
        }
    }

    pub fn format_for_cc(&self, db: &dyn BindingsGenerator<'_>) -> Result<TokenStream> {
        if let Some(path) = self.cpp_type {
            let path = db.format_cc_ident(path)?;
            return Ok(quote! {#path});
        }

        let name = self.cpp_name.expect("`format_for_cc` can't be called on name-less item kinds");

        let cpp_top_level_ns = db.format_cc_ident(self.cpp_top_level_ns)?;
        let ns_path = format_ns_path_for_cc(db, &self.cpp_ns_path)?;
        let name = db.format_cc_ident(name)?;
        Ok(quote! { :: #cpp_top_level_ns:: #ns_path #name })
    }

    pub fn format_for_rs(&self) -> TokenStream {
        let name =
            self.rs_name.as_ref().expect("`format_for_rs` can't be called on name-less item kinds");

        let krate = make_rs_ident(self.krate.as_str());
        let mod_path = self.rs_mod_path.format_for_rs();
        let name = make_rs_ident(name.as_str());
        quote! { :: #krate :: #mod_path #name }
    }
}
