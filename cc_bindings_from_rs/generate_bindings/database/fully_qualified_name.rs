// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_middle;
extern crate rustc_span;

use crate::db::BindingsGenerator;
use arc_anyhow::{bail, Result};
use code_gen_utils::{format_cc_type_name, make_rs_ident, NamespaceQualifier};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::{CrateNum, DefId};
use rustc_span::symbol::Symbol;
use std::collections::HashMap;
use std::rc::Rc;

/// Represents the unqualified name of a Rust item. For the fully qualified name
// `std::collections::HashMap`, the unqualified name would be `HashMap`.
#[derive(Clone, Copy, Debug, PartialEq)]
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

    /// Number of the crate that defines the item.
    /// This can be different from `def_id.krate` if the item is re-exported.
    pub krate_num: CrateNum,

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
        let features = db.crate_features(self.krate_num);
        if let Some(path) = self.unqualified.cpp_type {
            if path.as_str().contains('{') {
                bail!(
                    "Cannot format generic `cpp_type` `{path}` with unexpanded template placeholders as C++ type"
                );
            }
            // TODO(b/502939407): Until this bug is fixed and cpp_type comes pre-prefixed with `::`,
            // we use this hack here to add the prefix on to generated code manually. This should
            // typically be applied to all types, but because ffi_11 is special and maps to builtin
            // types, we use a match expression to filter those out to avoid things like `::long`.
            return match path.as_str() {
                "decltype(char(0))"
                | "signed char"
                | "unsigned char"
                | "short"
                | "unsigned short"
                | "int"
                | "unsigned int"
                | "long"
                | "unsigned long"
                | "long long"
                | "unsigned long long"
                | "decltype(long(0))"
                | "decltype(nullptr)"
                | "decltype(char8_t(0))"
                | "decltype(char16_t(0))"
                | "decltype(char32_t(0))"
                | "decltype(wchar_t(0))"
                | "wchar_t" => format_cc_type_name(path.as_str(), features),
                path => {
                    let (maybe_const_prefix, path) = if let Some(path) = path.strip_prefix("const ")
                    {
                        ("const ", path)
                    } else {
                        ("", path)
                    };
                    let (universal_qualifier, path) =
                        if path.trim_start().starts_with("::") { ("", path) } else { ("::", path) };
                    format_cc_type_name(
                        &format!("{maybe_const_prefix}{universal_qualifier}{path}"),
                        features,
                    )
                }
            };
        }

        let name = self.unqualified.cpp_name;

        let cpp_top_level_ns = self
            .cpp_top_level_ns
            .iter()
            .map(|ns| db.format_cc_ident(*ns))
            .collect::<Result<Vec<_>>>()?;
        let ns_path = format_ns_path_for_cc(db, &self.cpp_ns_path)?;
        let name = format_cc_type_name(name.as_str(), features)?;
        Ok(quote! { :: #(#cpp_top_level_ns::)* #ns_path #name })
    }

    pub fn cpp_ns_qualifier(&self, db: &BindingsGenerator<'_>) -> NamespaceQualifier {
        let use_leading_colons = db
            .crate_features(self.krate_num)
            .contains(crubit_feature::CrubitFeature::LeadingColonsForCppType);
        NamespaceQualifier::new(
            self.cpp_top_level_ns
                .iter()
                .map(|sym| Rc::<str>::from(sym.as_str()))
                .chain(self.cpp_ns_path.namespaces.iter().cloned()),
            use_leading_colons,
        )
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
    /// The definition that this path names.
    pub def_id: DefId,
    /// Segments of the path.
    pub path: Vec<Symbol>,
    /// If this path aliases a definition, this will be the alias. Otherwise, it will be the
    /// definition name.
    pub name: Symbol,
    /// If this path points as a type alias, rather than a use statement, this will be the DefId of
    /// the type alias.
    pub type_alias_def_id: Option<DefId>,
    /// True if any segment of this path is marked #[doc(hidden)] making it a less preferable path
    /// than any non-hidden path regardless of length.
    pub is_doc_hidden: bool,
    /// True if this path is a reexport.
    pub is_reexport: bool,
    /// The crate that defines this path.
    pub krate: CrateNum,
}

// Checks for a list of known clang builtin macros and renames their namespaces to avoid
// collisions in C++ preprocessing. Generating a namespace with the same name as a macro causes
// illegal expansion during preprocessing.
pub fn rename_clang_builtin_macros(ns: Rc<str>) -> Rc<str> {
    if matches!(ns.as_ref(), "unix" | "linux" | "WIN32" | "WINNT" | "WIN64" | "spirv" | "sun") {
        return Rc::from(format!("rs_{}", ns.as_ref()));
    }
    ns
}

// Checks for a list of global C standard library functions, renaming their namespaces
// when they appear at the top level to avoid scope conflicts in C++ ([basic.scope.scope]).
pub fn rename_c_stdlib_functions(ns: Rc<str>) -> Rc<str> {
    if matches!(
        ns.as_ref(),
        // C standard library global functions
        "remove" | "rename" | "free" | "exit" | "abort" | "signal" | "system" |
        // Memory & String utilities
        "malloc" | "calloc" | "realloc" | "memcpy" | "memmove" | "memset" |
        "strcpy" | "strncpy" | "strcat" | "strncat" | "strcmp" | "strncmp" |
        "strlen" | "strchr" | "strrchr" | "strstr" | "strtok" | "strerror" |
        "memchr" |
        // Math & Utility
        "abs" | "labs" | "llabs" | "div" | "ldiv" | "lldiv" | "rand" | "srand" |
        "bsearch" | "qsort" | "getenv" | "sin" | "cos" | "tan" | "asin" | "acos" |
        "atan" | "atan2" | "sinh" | "cosh" | "tanh" | "exp" | "log" | "log10" |
        "pow" | "sqrt" | "ceil" | "floor" | "fabs" | "frexp" | "ldexp" | "modf" |
        // Time & Date
        "clock" | "difftime" | "mktime" | "time" | "asctime" | "ctime" |
        "gmtime" | "localtime" | "strftime" |
        // I/O
        "printf" | "scanf" | "puts" | "fopen" | "fclose" | "fread" | "fwrite" |
        "fseek" | "ftell" | "rewind" | "perror" | "tmpfile" | "tmpnam"
    ) {
        return Rc::from(format!("rs_{}", ns.as_ref()));
    }
    ns
}

impl ExportedPath {
    pub fn to_namespace_qualifier(&self, db: &BindingsGenerator) -> NamespaceQualifier {
        let features = db.crate_features(self.krate);
        let use_leading_colons =
            features.contains(crubit_feature::CrubitFeature::LeadingColonsForCppType);
        NamespaceQualifier::new(
            self.path
                .iter()
                .map(|s| Rc::<str>::from(s.as_str()))
                .map(rename_clang_builtin_macros)
                .collect::<Vec<_>>(),
            use_leading_colons,
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
            // Between two paths of the same length, prefer the one that is not a reexport.
            .then_with(|| match (self.is_reexport, other.is_reexport) {
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                _ => Ordering::Equal,
            })
            // Failing all else, choose the lexicographically smallest path. Compare the segments
            // as strings: `Symbol`'s `Ord` compares interner indices, which depend on the order
            // in which rustc happened to intern the names.
            .then_with(|| {
                self.path.iter().map(|s| s.as_str()).cmp(other.path.iter().map(|s| s.as_str()))
            })
            .then_with(|| self.name.as_str().cmp(other.name.as_str()))
    }
}

impl PartialOrd for ExportedPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// The ownership priority of a crate that exposes a public path for a `DefId`.
///
/// When an item is re-exported across multiple crates, Crubit uses this rank to choose which
/// crate provides the canonical C++ definition (versus emitting a `using` alias).
///
/// Variants are listed in ascending priority of ownership.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CrateOwnershipRank {
    /// A crate re-exporting an item it did not define (e.g. `pub use std::io::Error;`).
    TransitiveReexport = 0,

    /// A standard library crate (`std` or `alloc`) stably re-exporting an item that was
    /// defined internally/unstably in `core` or `alloc` (e.g. `std::io::Error` is a stable
    /// re-export for the unstable `core::io::error::Error`). We attribute ownership to the
    /// standard library re-export in this case, since it is the canonical owner.
    StdlibStableReexport = 1,

    /// The crate that actually defined the item (`def_id.krate`, e.g. `core` for
    /// `core::option::Option`).
    DefiningCrate = 2,
}

impl CrateOwnershipRank {
    pub fn new(tcx: TyCtxt<'_>, def_id: DefId, krate: CrateNum) -> Self {
        if krate == def_id.krate {
            return Self::DefiningCrate;
        }

        if matches!(tcx.crate_name(def_id.krate).as_str(), "core" | "alloc")
            && matches!(tcx.crate_name(krate).as_str(), "std" | "alloc")
        {
            return Self::StdlibStableReexport;
        }

        Self::TransitiveReexport
    }
}

/// The paths that name one definition, as positions in the table that stores them.
///
/// The canonical path is the preferred path that should be treated as the primary definition for
/// generating bindings. Aliases are other paths the same definition is available at and should
/// generate aliases to the canonical path in the bindings.
///
/// While a table holds the paths of one crate, the aliases are sorted by the paths they refer to.
/// Once `PublicPathTable::extend` has combined several crates, the paths of each crate stay
/// together and keep their order, but the list as a whole is not sorted.
#[derive(Debug)]
struct DefinitionPaths {
    canonical: usize,
    aliases: Vec<usize>,
}

impl DefinitionPaths {
    /// Makes every path of `other` an alias, after the aliases that are already here.
    fn absorb(&mut self, other: Self) {
        self.aliases.push(other.canonical);
        self.aliases.extend(other.aliases);
    }
}

/// The public paths of one crate, or of several, by the definition they name.
///
/// Every path is stored once, and each definition refers to its paths by their position.
#[derive(Debug, Default)]
pub struct PublicPathTable {
    /// The paths of every crate added to the table. `by_def` does not refer to the paths of the
    /// definitions that `extend` was told not to keep.
    paths: Vec<ExportedPath>,
    /// The paths that name each definition.
    by_def: HashMap<DefId, DefinitionPaths>,
}

impl PublicPathTable {
    /// Adds a path of the definition `path.def_id`. The smallest path of a definition is its
    /// canonical path.
    ///
    /// This builds the table of a single crate: it relies on the aliases being sorted, which is
    /// no longer certain after [`Self::extend`].
    pub fn insert(&mut self, path: ExportedPath) {
        use std::collections::hash_map::Entry;
        let index = self.paths.len();
        match self.by_def.entry(path.def_id) {
            Entry::Vacant(vacant) => {
                vacant.insert(DefinitionPaths { canonical: index, aliases: vec![] });
            }
            Entry::Occupied(mut occupied) => {
                let paths = &self.paths;
                let DefinitionPaths { canonical, aliases } = occupied.get_mut();
                let position_among_aliases = |alias: &ExportedPath, aliases: &[usize]| {
                    aliases.binary_search_by(|&other| paths[other].cmp(alias))
                };
                if path < paths[*canonical] {
                    // The canonical path so far becomes an alias, unless an alias equals it.
                    let displaced = std::mem::replace(canonical, index);
                    if let Err(position) = position_among_aliases(&paths[displaced], aliases) {
                        aliases.insert(position, displaced);
                    }
                } else {
                    match position_among_aliases(&path, aliases) {
                        Err(position) => aliases.insert(position, index),
                        // A path equal to one of the aliases is not stored.
                        Ok(_) => return,
                    }
                }
            }
        }
        self.paths.push(path);
    }

    /// Adds the paths that `krate_paths`, the table of one crate, has for the definitions that
    /// `keep` accepts.
    ///
    /// When a definition already has paths from another crate, the canonical path of the crate
    /// with the higher [`CrateOwnershipRank`] for the definition stays canonical; if both crates
    /// rank the same, the smaller canonical path does. The other crate's paths are appended to
    /// the aliases.
    pub fn extend(&mut self, tcx: TyCtxt<'_>, krate_paths: &Self, keep: impl Fn(DefId) -> bool) {
        use std::collections::hash_map::Entry;
        let offset = self.paths.len();
        self.paths.extend_from_slice(&krate_paths.paths);
        for (&def_id, paths) in &krate_paths.by_def {
            if !keep(def_id) {
                continue;
            }
            let mut paths = DefinitionPaths {
                canonical: paths.canonical + offset,
                aliases: paths.aliases.iter().map(|alias| alias + offset).collect(),
            };
            match self.by_def.entry(def_id) {
                Entry::Vacant(vacant) => {
                    vacant.insert(paths);
                }
                Entry::Occupied(mut occupied) => {
                    let existing_paths = occupied.get_mut();
                    let rank = |paths: &DefinitionPaths| {
                        CrateOwnershipRank::new(tcx, def_id, self.paths[paths.canonical].krate)
                    };
                    let canonical = |paths: &DefinitionPaths| &self.paths[paths.canonical];
                    // The higher rank wins, then the smaller canonical path. If those are equal
                    // too, the crate that was added first stays canonical.
                    let new_crate_is_canonical = rank(&paths)
                        .cmp(&rank(existing_paths))
                        .then_with(|| canonical(existing_paths).cmp(canonical(&paths)))
                        .is_gt();
                    if new_crate_is_canonical {
                        std::mem::swap(existing_paths, &mut paths);
                    }
                    existing_paths.absorb(paths);
                }
            }
        }
    }

    /// The definitions that have a path, in no particular order.
    pub fn def_ids(&self) -> impl Iterator<Item = DefId> + use<'_> {
        self.by_def.keys().copied()
    }

    /// The canonical path of `def_id`, if it has any path.
    pub fn canonical(&self, def_id: DefId) -> Option<&ExportedPath> {
        self.by_def.get(&def_id).map(|paths| &self.paths[paths.canonical])
    }

    /// The paths of `def_id` other than its canonical path. They are sorted in the table of a
    /// single crate; after [`Self::extend`], only the aliases of each crate keep their order.
    pub fn aliases(&self, def_id: DefId) -> impl Iterator<Item = &ExportedPath> {
        self.by_def
            .get(&def_id)
            .into_iter()
            .flat_map(move |paths| paths.aliases.iter().map(move |&alias| &self.paths[alias]))
    }
}
