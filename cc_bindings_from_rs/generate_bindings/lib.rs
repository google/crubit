// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(rustc_private)]
#![deny(rustc::internal)]

extern crate rustc_ast;
extern crate rustc_attr;
extern crate rustc_hir;
extern crate rustc_infer;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;
extern crate rustc_type_ir;

mod code_snippet;
mod db;
mod format_type;
mod generate_function;
mod generate_function_thunk;
mod generate_struct_and_union;
mod query_compiler;

use crate::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet, ExternCDecl, RsSnippet};
pub use crate::db::{BindingsGenerator, Database};
use crate::format_type::{
    create_canonical_name_from_foreign_path, ensure_ty_is_pointer_like, format_cc_ident,
    format_ns_path_for_cc, format_param_types_for_cc, format_region_as_cc_lifetime,
    format_region_as_rs_lifetime, format_ret_ty_for_cc, format_top_level_ns_for_crate,
    format_ty_for_cc, format_ty_for_rs, is_bridged_type, BridgedType, BridgedTypeConversionInfo,
};
use crate::generate_function::generate_function;
use crate::generate_function_thunk::{generate_trait_thunks, TraitThunks};
use crate::generate_struct_and_union::{generate_adt, generate_adt_core, AdtCoreBindings};
use crate::query_compiler::{
    count_regions, does_type_implement_trait, get_layout, get_scalar_int_type,
    get_tag_size_with_padding, is_c_abi_compatible_by_value, is_directly_public, is_exported,
    liberate_and_deanonymize_late_bound_regions, post_analysis_typing_env,
    public_free_items_in_mod, repr_attrs,
};
use arc_anyhow::{Context, Error, Result};
use code_gen_utils::{
    format_cc_includes, make_rs_ident, CcConstQualifier, CcInclude, NamespaceQualifier,
};
use error_report::{anyhow, bail, ensure};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustc_attr::find_deprecation;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{HirId, Item, ItemKind, Node, UseKind, UsePath};
use rustc_middle::dep_graph::DepContext;
use rustc_middle::mir::ConstValue;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::def_id::{CrateNum, DefId, LocalDefId, LOCAL_CRATE};
use rustc_span::symbol::{sym, Symbol};
use rustc_target::abi::{AddressSpace, BackendRepr, Integer, Primitive, Scalar};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::iter::once;
use std::rc::Rc;
use std::slice;

#[derive(Clone, Debug, Hash)]
pub enum IncludeGuard {
    PragmaOnce,
    Guard(String),
}

fn support_header<'tcx>(db: &dyn BindingsGenerator<'tcx>, suffix: &'tcx str) -> CcInclude {
    CcInclude::support_lib_header(db.crubit_support_path_format(), suffix.into())
}

pub struct BindingsTokens {
    pub cc_api: TokenStream,
    pub cc_api_impl: TokenStream,
}

fn add_include_guard(db: &dyn BindingsGenerator<'_>, cc_api: TokenStream) -> Result<TokenStream> {
    match db.h_out_include_guard() {
        IncludeGuard::PragmaOnce => Ok(quote! {
            __HASH_TOKEN__ pragma once __NEWLINE__
            __NEWLINE__

            #cc_api
        }),
        IncludeGuard::Guard(include_guard_str) => {
            let include_guard = format_cc_ident(db, include_guard_str.as_str())?;
            Ok(quote! {
                __HASH_TOKEN__ ifndef #include_guard __NEWLINE__
                __HASH_TOKEN__ define #include_guard __NEWLINE__
                __NEWLINE__

                #cc_api

                __HASH_TOKEN__ endif __COMMENT__ #include_guard_str __NEWLINE__
            })
        }
    }
}

pub fn generate_bindings(db: &Database) -> Result<BindingsTokens> {
    let tcx = db.tcx();

    let top_comment = {
        let crate_name = tcx.crate_name(LOCAL_CRATE);
        let crubit_features = {
            let mut crubit_features: Vec<&str> = crate_features(db, LOCAL_CRATE)
                .into_iter()
                .map(|feature| feature.short_name())
                .collect();
            crubit_features.sort();
            if crubit_features.is_empty() {
                "<none>".to_string()
            } else {
                crubit_features.join(", ")
            }
        };
        let txt = format!(
            "Automatically @generated C++ bindings for the following Rust crate:\n\
             {crate_name}\n\
             Features: {crubit_features}"
        );
        quote! { __COMMENT__ #txt __NEWLINE__ }
    };

    let BindingsTokens { cc_api, cc_api_impl } = generate_crate(db).unwrap_or_else(|err| {
        let txt = format!("Failed to generate bindings for the crate: {err}");
        let src = quote! { __COMMENT__ #txt };
        BindingsTokens { cc_api: src.clone(), cc_api_impl: src }
    });
    let cc_api = add_include_guard(db, cc_api)?;
    let cc_api = quote! {
        #top_comment

        #cc_api
    };

    let mut extern_crate_decls: Vec<TokenStream> = vec![];
    for (name, renamed) in db.crate_renames().iter() {
        let name = format_ident!("{}", name.to_string());
        let renamed = format_ident!("{}", renamed.to_string());

        extern_crate_decls.push(quote! {
            extern crate #name as #renamed;
        });
    }

    let cc_api_impl = quote! {
        #top_comment

        // `rust_builtin_type_abi_assumptions.md` documents why the generated
        // bindings need to relax the `improper_ctypes_definitions` warning
        // for `char` (and possibly for other built-in types in the future).
        #![allow(improper_ctypes_definitions)] __NEWLINE__

        __NEWLINE__

        #(#extern_crate_decls)*

        __NEWLINE__

        #cc_api_impl
    };

    Ok(BindingsTokens { cc_api, cc_api_impl })
}

fn crate_features(
    db: &dyn BindingsGenerator,
    krate: CrateNum,
) -> flagset::FlagSet<crubit_feature::CrubitFeature> {
    let crate_features = db.crate_name_to_features();
    let features = if krate == LOCAL_CRATE {
        crate_features.get("self")
    } else {
        crate_features.get(db.tcx().crate_name(krate).as_str())
    };
    features.copied().unwrap_or_else(|| db.default_features())
}

fn check_feature_enabled_on_self_and_all_deps(
    db: &dyn BindingsGenerator,
    feature: FineGrainedFeature,
) -> bool {
    for (_, crate_features) in db.crate_name_to_features().iter() {
        if feature.ensure_crubit_feature(*crate_features).is_err() {
            return false;
        }
    }
    true
}

fn format_with_cc_body(
    db: &dyn BindingsGenerator,
    ns: &NamespaceQualifier,
    body: TokenStream,
    attributes: Vec<TokenStream>,
) -> Result<TokenStream> {
    if ns.0.is_empty() {
        Ok(body)
    } else {
        let namespace_cc_idents =
            ns.0.iter().map(|s| format_cc_ident(db, s)).collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            __NEWLINE__ #(#attributes)* namespace #(#namespace_cc_idents)::* { __NEWLINE__
                #body
            __NEWLINE__ }  __NEWLINE__
        })
    }
}

flagset::flags! {
    /// An "expanded" version of CrubitFeature that includes specific cc_bindings_from_rs features.
    /// This allows them to be converted into more readable error messages: rather than simply
    /// stating "<big thing> requires experimental", we can say it requires experimental because
    /// it needs e.g. "references".
    enum FineGrainedFeature : u8 {
        References,
        LifetimeReuse,
        PossibleMutableAliasing,
        NonFreeReferenceParams,
        EscapeCppReservedKeyword,
        RustChar,
    }
}

impl FineGrainedFeature {
    fn ensure_crubit_feature(
        self,
        crubit_features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    ) -> Result<()> {
        use crubit_feature::CrubitFeature::*;
        match self {
            Self::References => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for references of non-function-param types requires {}",
                    Experimental.aspect_hint()
                )
            }
            Self::LifetimeReuse => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for multiple uses of a lifetime parameter requires {}",
                    Experimental.aspect_hint()
                )
            }
            Self::NonFreeReferenceParams => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for bound reference lifetimes (such as 'static) requires {}",
                    Experimental.aspect_hint()
                )
            }
            Self::PossibleMutableAliasing => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for functions taking a mutable reference, and which may alias in C++, requires {}",
                    Experimental.aspect_hint()
                )
            }
            Self::EscapeCppReservedKeyword => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for escaping C++ reserved keywords requires {}",
                    Experimental.aspect_hint()
                )
            }
            Self::RustChar => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for the Rust `char` type requires {}",
                    Experimental.aspect_hint()
                )
            }
        }
        Ok(())
    }
}

/// Represents the fully qualified name of a Rust item (e.g. of a `struct` or a
/// function).
#[derive(Clone, Debug, PartialEq)]
struct FullyQualifiedName {
    /// Name of the crate that defines the item.
    /// For example, this would be `std` for `std::cmp::Ordering`.
    krate: Symbol,

    /// Configurable top-level namespace of the C++ bindings.
    /// For example, this would be `::foo` for `foo::bar::baz::qux`.
    cpp_top_level_ns: Symbol,

    /// Path to the module where the item is located.
    /// For example, this would be `cmp` for `std::cmp::Ordering`.
    /// The path may contain multiple modules - e.g. `foo::bar::baz`.
    rs_mod_path: NamespaceQualifier,
    /// The C++ namespace to use for the symbol excluding the top-level
    /// namespace.
    cpp_ns_path: NamespaceQualifier,

    /// Rust name of the item.
    /// For example, this would be:
    /// * `Some("Ordering")` for `std::cmp::Ordering`.
    /// * `None` for `ItemKind::Use` - e.g.: `use submodule::*`
    rs_name: Option<Symbol>,

    /// The C++ name to use for the symbol.
    ///
    /// For example, the following struct
    /// ```
    /// #[__crubit::annotate(cpp_name="Bar")]
    /// struct Foo { ... }
    /// ```
    /// will be generated as a C++ struct named `Bar` instead of `Foo`.
    cpp_name: Option<Symbol>,

    /// The fully-qualified C++ type to use for this, if this was originally a
    /// C++ type.
    ///
    /// For example, if a type has `#[__crubit::annotate(cpp_type="x::y")]`,
    /// then cpp_type will be `Some(x::y)`.
    cpp_type: Option<Symbol>,
}

impl FullyQualifiedName {
    /// Computes a `FullyQualifiedName` for `def_id`.
    ///
    /// May panic if `def_id` is an invalid id.
    // TODO(b/259724276): This function's results should be memoized.
    fn new(db: &dyn BindingsGenerator<'_>, def_id: DefId) -> Self {
        if let Some(canonical_name) = db.reexported_symbol_canonical_name_mapping().get(&def_id) {
            return canonical_name.clone();
        }

        let tcx = db.tcx();
        let krate = tcx.crate_name(def_id.krate);
        let cpp_top_level_ns = format_top_level_ns_for_crate(db, def_id.krate);

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

    fn format_for_cc(&self, db: &dyn BindingsGenerator<'_>) -> Result<TokenStream> {
        if let Some(path) = self.cpp_type {
            let path = format_cc_ident(db, path.as_str())?;
            return Ok(quote! {#path});
        }

        let name = self
            .cpp_name
            .as_ref()
            .expect("`format_for_cc` can't be called on name-less item kinds");

        let cpp_top_level_ns = format_cc_ident(db, self.cpp_top_level_ns.as_str())?;
        let ns_path = format_ns_path_for_cc(db, &self.cpp_ns_path)?;
        let name = format_cc_ident(db, name.as_str())?;
        Ok(quote! { :: #cpp_top_level_ns:: #ns_path #name })
    }

    fn format_for_rs(&self) -> TokenStream {
        let name =
            self.rs_name.as_ref().expect("`format_for_rs` can't be called on name-less item kinds");

        let krate = make_rs_ident(self.krate.as_str());
        let mod_path = self.rs_mod_path.format_for_rs();
        let name = make_rs_ident(name.as_str());
        quote! { :: #krate :: #mod_path #name }
    }
}

mod sugared_ty {
    use super::*;
    /// A Ty, optionally attached to its `hir::Ty` counterpart, if any.
    ///
    /// The rustc_hir::Ty is used only for detecting type aliases (or other
    /// optional sugar), unrelated to the actual concrete type. It
    /// necessarily disappears if, for instance, the type is plugged in from
    /// a generic. There's no way to tell, in the bindings for
    /// Vec<c_char>::len(), that `T` came from the type alias
    /// `c_char`, instead of a plain `i8` or `u8`.
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub(super) struct SugaredTy<'tcx> {
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
}
use sugared_ty::SugaredTy;

/// Location where a type is used.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum TypeLocation {
    /// The top-level return type.
    ///
    /// The "top-level" part can be explained by looking at an example of `fn
    /// foo() -> *const T`:
    /// - The top-level return type `*const T` is in the `FnReturn` location
    /// - The nested pointee type `T` is in the `Other` location
    FnReturn,

    /// The top-level parameter type.
    ///
    /// The "top-level" part can be explained by looking at an example of:
    /// `fn foo(param: *const T)`:
    /// - The top-level parameter type `*const T` is in the `FnParam` location
    /// - The nested pointee type `T` is in the `Other` location
    // TODO(b/278141494, b/278141418): Once `const` and `static` items are supported,
    // we may want to apply parameter-like formatting to their types (e.g. have
    // `format_ty_for_cc` emit `T&` rather than `T*`).
    FnParam,

    /// Other location (e.g. pointee type, field type, etc.).
    Other,
}

fn symbols_from_extern_crate(db: &dyn BindingsGenerator<'_>) -> Vec<(DefId, FullyQualifiedName)> {
    use rustc_hir::intravisit::Visitor;
    let tcx = db.tcx();
    struct ForeignSymbols<'a, 'tcx> {
        pub symbols: Vec<(DefId, FullyQualifiedName)>,
        pub db: &'a dyn BindingsGenerator<'tcx>,
    }

    impl<'tcx> Visitor<'tcx> for ForeignSymbols<'_, '_> {
        fn visit_path(&mut self, path: &rustc_hir::Path<'tcx>, _id: rustc_hir::HirId) {
            let db = self.db;
            if let Some((def_id, fully_qualified_name)) =
                create_canonical_name_from_foreign_path(db, path.segments, &path.res)
            {
                self.symbols.push((def_id, fully_qualified_name));
            }
        }
    }

    let mut visitor = ForeignSymbols { symbols: Vec::new(), db };
    tcx.hir().visit_all_item_likes_in_crate(&mut visitor);

    visitor.symbols
}

/// Computes a mapping from a `DefId` to a `FullyQualifiedName` for all
/// not-directly-public symbols that are reexported by a `use` statement.
// TODO(b/350772554): Don't generate bindings for ambiguous symbols.
fn reexported_symbol_canonical_name_mapping(
    db: &dyn BindingsGenerator<'_>,
) -> HashMap<DefId, FullyQualifiedName> {
    let tcx = db.tcx();
    let mut name_map: HashMap<DefId, FullyQualifiedName> = HashMap::new();

    #[derive(Debug)]
    struct AliasInfo {
        using_name: String,
        local_def_id: LocalDefId,
        type_def_id: DefId,
        def_kind: DefKind,
    }
    let create_canonical_name = |name_map: &mut HashMap<DefId, FullyQualifiedName>,
                                 alias_info: &AliasInfo|
     -> Option<FullyQualifiedName> {
        let alias_name = &alias_info.using_name;
        let alias_local_def_id = alias_info.local_def_id;
        let aliased_entity_def_id = alias_info.type_def_id;
        let rs_name = Symbol::intern(alias_name);
        if let Some(canonical_name) = name_map.get(&aliased_entity_def_id) {
            // We keep the lexicographically smallest name.
            if canonical_name.rs_name.unwrap().as_str() < rs_name.as_str() {
                return None;
            }
        }
        let def_id = alias_local_def_id.to_def_id();
        let tcx = db.tcx();

        // We only handle local reexported private symbols for `pub use`.
        if !tcx.effective_visibilities(()).is_directly_public(alias_local_def_id) // not pub use
                || !aliased_entity_def_id.is_local() // symbols from other crates
                || tcx.effective_visibilities(()).is_directly_public(aliased_entity_def_id.expect_local())
        {
            return None;
        }
        let item_name = tcx.opt_item_name(aliased_entity_def_id)?;
        let krate = tcx.crate_name(def_id.krate);
        let cpp_top_level_ns = format_top_level_ns_for_crate(db, def_id.krate);
        let parent_def_key = tcx.def_key(def_id).parent?;
        let parent_def_id = DefId::local(parent_def_key);

        // If the parent is being aliased, we use its canonical name and we always
        // process parents before their children.
        let full_path_strs: Vec<Rc<str>> = if let Some(con_name) = name_map.get(&parent_def_id) {
            con_name.rs_mod_path.0.clone()
        } else {
            let mut full_path = tcx.def_path(def_id).data; // mod_path + name
            full_path.pop().expect("At least the use exists");
            full_path
                .into_iter()
                .filter_map(|p| p.data.get_opt_name())
                .map(|s| Rc::<str>::from(s.as_str()))
                .collect()
        };

        let rs_mod_path = NamespaceQualifier::new(full_path_strs.clone());
        let cpp_ns_path = NamespaceQualifier::new(
            full_path_strs.into_iter().chain([Rc::from("__crubit_internal")]),
        );
        let attributes = crubit_attr::get_attrs(tcx, aliased_entity_def_id).unwrap();
        let cpp_type = attributes.cpp_type;
        Some(FullyQualifiedName {
            cpp_name: Some(item_name),
            cpp_ns_path,
            rs_name: Some(rs_name),
            krate,
            cpp_top_level_ns,
            rs_mod_path,
            cpp_type,
        })
    };

    for (def_id, fully_qualified_name) in symbols_from_extern_crate(db).into_iter() {
        name_map.insert(def_id, fully_qualified_name);
    }
    let aliases =
        tcx.hir()
            .items()
            .filter_map(|item_id| {
                let local_def_id: LocalDefId = item_id.owner_id.def_id;
                if let Item { ident, kind: ItemKind::Use(use_path, use_kind), .. } =
                    tcx.hir().expect_item(local_def_id)
                {
                    // TODO(b/350772554): Preserve the errors.
                    collect_alias_from_use(db, ident.as_str(), use_path, use_kind).ok().map(
                        |aliases| {
                            aliases.into_iter().map(move |(using_name, type_def_id, def_kind)| {
                                AliasInfo { using_name, local_def_id, type_def_id, def_kind }
                            })
                        },
                    )
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<AliasInfo>>();

    // TODO(b/350772554): Support mod.
    // We should process the aliases in the path order: mod -> struct ->
    // function/etc. Otherwise, for example, the function will still use the
    // private fully qualified name as it doesn't know the canonical struct name
    // yet.
    let (struct_like_aliases, other_aliases): (Vec<AliasInfo>, Vec<AliasInfo>) =
        aliases.into_iter().partition(|AliasInfo { def_kind, .. }| {
            matches!(*def_kind, DefKind::Struct | DefKind::Enum | DefKind::Union)
        });
    for alias_info in struct_like_aliases.into_iter().chain(other_aliases.into_iter()) {
        if let Some(canonical_name) = create_canonical_name(&mut name_map, &alias_info) {
            name_map.insert(alias_info.type_def_id, canonical_name);
        }
    }

    name_map
}

/// Checks whether an definition matches a specific qualified name.
fn matches_qualified_name(
    db: &dyn BindingsGenerator<'_>,
    item_did: DefId,
    name_to_compare: &str,
) -> bool {
    // TODO(b/372153103): Compare the name via `tcx.def_path(adt.did())`.
    let type_name = FullyQualifiedName::new(db, item_did);
    type_name.format_for_rs().to_string() == name_to_compare
}

/// Checks that `ty` has the same ABI as `rs_std::SliceRef`.
fn check_slice_layout<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) {
    // Check the assumption from `rust_builtin_type_abi_assumptions.md` that Rust's
    // slice has the same ABI as `rs_std::SliceRef`.
    let layout = tcx
        .layout_of(
            ty::TypingEnv {
                typing_mode: ty::TypingMode::PostAnalysis,
                param_env: ty::ParamEnv::empty(),
            }
            .as_query_input(ty),
        )
        .expect("`layout_of` is expected to succeed for `{ty}` type")
        .layout;
    assert_eq!(8, layout.align().abi.bytes());
    assert_eq!(16, layout.size().bytes());
    assert!(matches!(
        layout.backend_repr(),
        BackendRepr::ScalarPair(
            Scalar::Initialized { value: Primitive::Pointer(AddressSpace(_)), .. },
            Scalar::Initialized {
                value: Primitive::Int(Integer::I64, /* signedness = */ false),
                ..
            }
        )
    ));
}

#[derive(Debug, Clone, PartialEq)]
enum CcType {
    Pointer { cpp_type: Symbol, cv: CcConstQualifier },
    Other(Symbol),
}

impl AsRef<str> for CcType {
    fn as_ref(&self) -> &str {
        match self {
            CcType::Other(cpp_type) => cpp_type.as_str(),
            CcType::Pointer { cpp_type, .. } => cpp_type.as_str(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AllowReferences {
    /// Only allow references when it is safe.
    Safe,
    /// Allow references unconditionally, and rely on users to check for
    /// aliasing.
    UnsafeAll,
}

/// Returns the C++ deprecated tag for the item identified by `def_id`, if it is
/// deprecated. Otherwise, returns None.
fn generate_deprecated_tag(tcx: TyCtxt, def_id: DefId) -> Option<TokenStream> {
    if let Some(deprecated_attr) = tcx.get_attr(def_id, rustc_span::symbol::sym::deprecated) {
        if let Some((deprecation, _span)) =
            find_deprecation(tcx.sess(), tcx.features(), slice::from_ref(deprecated_attr))
        {
            let cc_deprecated_tag = match deprecation.note {
                None => quote! {[[deprecated]]},
                Some(note_symbol) => {
                    let note = note_symbol.as_str();
                    quote! {[[deprecated(#note)]]}
                }
            };
            return Some(cc_deprecated_tag);
        }
    }
    None
}

fn generate_using_statement(
    db: &dyn BindingsGenerator<'_>,
    using_name: &str,
    def_id: DefId,
    def_kind: DefKind,
) -> Result<ApiSnippets> {
    let tcx = db.tcx();

    match def_kind {
        DefKind::Fn => {
            let mut prereqs;
            // TODO(b/350772554): Support exporting private functions.
            if let Some(local_id) = def_id.as_local() {
                if let Ok(snippet) = db.generate_function(local_id) {
                    prereqs = snippet.main_api.prereqs;
                } else {
                    bail!("Ignoring the use because the bindings for the target is not generated");
                }
            } else {
                bail!("Unsupported checking for external function");
            }
            let fully_qualified_fn_name = FullyQualifiedName::new(db, def_id);
            let formatted_fully_qualified_fn_name = fully_qualified_fn_name.format_for_cc(db)?;
            let main_api_fn_name =
                format_cc_ident(db, fully_qualified_fn_name.cpp_name.unwrap().as_str())
                    .context("Error formatting function name")?;
            let using_name =
                format_cc_ident(db, using_name).context("Error formatting using name")?;

            prereqs.defs.insert(def_id.expect_local());
            let tokens = if format!("{}", using_name) == format!("{}", main_api_fn_name) {
                quote! {using #formatted_fully_qualified_fn_name;}
            } else {
                // TODO(b/350772554): Support function alias.
                bail!("Unsupported function alias");
            };
            Ok(ApiSnippets {
                main_api: CcSnippet { prereqs, tokens },
                cc_details: CcSnippet::default(),
                rs_details: RsSnippet::default(),
            })
        }
        DefKind::Struct | DefKind::Enum => {
            // This points directly to a type definition, not an alias or compound data
            // type, so we can drop the hir type.
            let use_type = SugaredTy::new(tcx.type_of(def_id).instantiate_identity(), None);
            create_type_alias(db, def_id, using_name, use_type)
        }
        DefKind::TyAlias => {
            let hir_ty = if def_id.is_local() {
                let local_def_id = def_id.as_local().unwrap();
                let Item { kind: ItemKind::TyAlias(hir_ty, ..), .. } =
                    tcx.hir().expect_item(local_def_id)
                else {
                    panic!("{:#?} is not a type alias", def_id);
                };
                Some(*hir_ty)
            } else {
                None
            };
            let alias_type = SugaredTy::new(tcx.type_of(def_id).instantiate_identity(), hir_ty);
            create_type_alias(db, def_id, using_name, alias_type)
        }
        _ => {
            bail!("Unsupported use statement that refers to this type of the entity: {:#?}", def_id)
        }
    }
}

// Collect all the aliases (alias_name, underlying_type_def_id,
// underlying_type_def_kind) created by the `use` statement. For example, `pub
// use some_mod::*` will return all the free items that are exported.
fn collect_alias_from_use(
    db: &dyn BindingsGenerator<'_>,
    using_name: &str,
    use_path: &UsePath,
    use_kind: &UseKind,
) -> Result<Vec<(String, DefId, DefKind)>> {
    let tcx = db.tcx();
    // TODO(b/350772554): Support multiple items with the same name in `use`
    // statements.`
    if use_path.res.len() != 1 {
        bail!(
            "use statements which resolve to multiple items with the same name are not supported yet"
        );
    }

    let (def_kind, def_id) = match use_path.res[0] {
        // TODO(b/350772554): Support PrimTy.
        // TODO(b/350772554): Support `use some_module`.
        Res::Def(def_kind, def_id) if def_kind != DefKind::Mod || use_kind == &UseKind::Glob => {
            (def_kind, def_id)
        }
        _ => {
            bail!(
                "Unsupported use statement that refers to this type of the entity: {:#?}",
                use_path.res[0]
            );
        }
    };

    let mut aliases = vec![];
    if def_kind == DefKind::Mod {
        for (item_def_id, item_def_kind) in public_free_items_in_mod(db, def_id) {
            let item_name = tcx.item_name(item_def_id).to_string();
            // TODO(b/350772554): Support export Enum fields.
            if !item_name.is_empty() {
                aliases.push((item_name, item_def_id, item_def_kind));
            }
        }
    } else {
        // TODO(b/350772554): Support export Enum fields.
        if !using_name.is_empty() {
            aliases.push((using_name.to_string(), def_id, def_kind));
        }
    }
    Ok(aliases)
}

fn generate_use(
    db: &dyn BindingsGenerator<'_>,
    using_name: &str,
    use_path: &UsePath,
    use_kind: &UseKind,
) -> Result<ApiSnippets> {
    let aliases = collect_alias_from_use(db, using_name, use_path, use_kind)?;
    // TODO(b/350772554): Expose the errors. If any of the types in the `use`
    // statement is not supported, we currently ignore it and discard the
    // errors.
    Ok(aliases
        .into_iter()
        .filter_map(|(using_name, def_id, def_kind)| {
            if is_public_or_supported_export(db, def_id) {
                Some(generate_using_statement(db, &using_name, def_id, def_kind))
            } else {
                None
            }
        })
        .filter_map(Result::ok)
        .collect())
}

fn generate_const(db: &dyn BindingsGenerator<'_>, local_def_id: LocalDefId) -> Result<ApiSnippets> {
    let tcx = db.tcx();
    let def_id: DefId = local_def_id.to_def_id();
    let unsupported_node_item_msg = "Called `generate_const` with a `rustc_hir::Node` that is not a `Node::Item` or `Node::ImplItem`";
    let hir_node = tcx.hir_node_by_def_id(local_def_id);

    let hir_ty = match hir_node {
        Node::Item(item) => item.expect_const().0,
        Node::ImplItem(item) => item.expect_const().0,
        _ => panic!("{}", unsupported_node_item_msg),
    };
    let ty = tcx.type_of(def_id).instantiate_identity();
    let cc_type_snippet =
        format_ty_for_cc(db, SugaredTy::new(ty, Some(hir_ty)), TypeLocation::Other)?;

    let cc_type = cc_type_snippet.tokens;
    let cc_name = format_cc_ident(db, tcx.item_name(def_id).as_str())?;
    let cc_value = match tcx.const_eval_poly(def_id).unwrap() {
        ConstValue::Scalar(scalar) => {
            macro_rules! eval {
                ($method:ident $(,$arg:expr)?) => {
                    Ok(scalar.$method($($arg)?).unwrap().to_string())
                };
            }
            match ty.kind() {
                ty::TyKind::Bool => eval!(to_bool),
                ty::TyKind::Int(ty::IntTy::I8) => eval!(to_i8),
                ty::TyKind::Int(ty::IntTy::I16) => eval!(to_i16),
                ty::TyKind::Int(ty::IntTy::I32) => eval!(to_i32),
                ty::TyKind::Int(ty::IntTy::I64) => eval!(to_i64),
                ty::TyKind::Uint(ty::UintTy::U8) => eval!(to_u8),
                ty::TyKind::Uint(ty::UintTy::U16) => eval!(to_u16),
                ty::TyKind::Uint(ty::UintTy::U32) => eval!(to_u32),
                ty::TyKind::Uint(ty::UintTy::U64) => eval!(to_u64),
                ty::TyKind::Float(ty::FloatTy::F32) => eval!(to_f32),
                ty::TyKind::Float(ty::FloatTy::F64) => eval!(to_f64),
                ty::TyKind::Uint(ty::UintTy::Usize) => eval!(to_target_usize, &tcx),
                ty::TyKind::Int(ty::IntTy::Isize) => eval!(to_target_isize, &tcx),
                _ => Err(anyhow!("Unsupported constant type")),
            }
        }
        _ => Err(anyhow!("Unexpected ConstValue type")),
    }?
    .parse::<TokenStream>()
    .unwrap();

    Ok(ApiSnippets {
        main_api: CcSnippet {
            tokens: match hir_node {
                Node::Item(_) => {
                    quote! {
                        constexpr #cc_type #cc_name = #cc_value;
                    }
                }
                Node::ImplItem(_) => {
                    quote! {
                        static constexpr #cc_type #cc_name = #cc_value;
                    }
                }
                _ => panic!("{}", unsupported_node_item_msg),
            },
            ..cc_type_snippet
        },
        cc_details: CcSnippet::default(),
        rs_details: RsSnippet::default(),
    })
}

fn generate_type_alias(
    db: &dyn BindingsGenerator<'_>,
    local_def_id: LocalDefId,
) -> Result<ApiSnippets> {
    let tcx = db.tcx();
    let def_id: DefId = local_def_id.to_def_id();
    let Item { kind: ItemKind::TyAlias(hir_ty, ..), .. } = tcx.hir().expect_item(local_def_id)
    else {
        panic!("called generate_type_alias on a non-type-alias");
    };
    let alias_type = SugaredTy::new(tcx.type_of(def_id).instantiate_identity(), Some(*hir_ty));
    create_type_alias(db, def_id, tcx.item_name(def_id).as_str(), alias_type)
}

fn create_type_alias<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    def_id: DefId,
    alias_name: &str,
    alias_type: SugaredTy<'tcx>,
) -> Result<ApiSnippets> {
    let cc_bindings = format_ty_for_cc(db, alias_type, TypeLocation::Other)?;
    let mut main_api_prereqs = CcPrerequisites::default();
    let actual_type_name = cc_bindings.into_tokens(&mut main_api_prereqs);

    let alias_name = format_cc_ident(db, alias_name).context("Error formatting type alias name")?;

    let mut attributes = vec![];
    if let Some(cc_deprecated_tag) = generate_deprecated_tag(db.tcx(), def_id) {
        attributes.push(cc_deprecated_tag);
    }

    let tokens = quote! {using #alias_name #(#attributes)* = #actual_type_name;};

    Ok(ApiSnippets {
        main_api: CcSnippet { prereqs: main_api_prereqs, tokens },
        cc_details: CcSnippet::default(),
        rs_details: RsSnippet::default(),
    })
}

fn is_public_or_supported_export(db: &dyn BindingsGenerator<'_>, def_id: DefId) -> bool {
    is_directly_public(db.tcx(), def_id)
        || ((is_exported(db.tcx(), def_id) || !def_id.is_local())
            && db.reexported_symbol_canonical_name_mapping().contains_key(&def_id))
}

/// Formats a default constructor for an ADT if possible (i.e. if the `Default`
/// trait is implemented for the ADT).  Returns an error otherwise (e.g. if
/// there is no `Default` impl, then the default constructor will be
/// `=delete`d in the returned snippet).
fn generate_default_ctor<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
) -> Result<ApiSnippets, ApiSnippets> {
    fn fallible_format_default_ctor<'tcx>(
        db: &dyn BindingsGenerator<'tcx>,
        core: Rc<AdtCoreBindings<'tcx>>,
    ) -> Result<ApiSnippets> {
        let tcx = db.tcx();
        let trait_id = tcx
            .get_diagnostic_item(sym::Default)
            .ok_or(anyhow!("Couldn't find `core::default::Default`"))?;
        let TraitThunks {
            method_name_to_cc_thunk_name,
            cc_thunk_decls,
            rs_thunk_impls: rs_details,
        } = generate_trait_thunks(db, trait_id, &core)?;

        let cc_struct_name = &core.cc_short_name;
        let main_api = CcSnippet::new(quote! {
            __NEWLINE__ __COMMENT__ "Default::default"
            #cc_struct_name(); __NEWLINE__ __NEWLINE__
        });
        let cc_details = {
            let thunk_name = method_name_to_cc_thunk_name
                .into_values()
                .exactly_one()
                .expect("Expecting a single `default` method");

            let mut prereqs = CcPrerequisites::default();
            let cc_thunk_decls = cc_thunk_decls.into_tokens(&mut prereqs);

            let tokens = quote! {
                #cc_thunk_decls
                inline #cc_struct_name::#cc_struct_name() {
                    __crubit_internal::#thunk_name(this);
                }
            };
            CcSnippet { tokens, prereqs }
        };
        Ok(ApiSnippets { main_api, cc_details, rs_details })
    }
    fallible_format_default_ctor(db, core.clone()).map_err(|err| {
        let msg = format!("{err:#}");
        let adt_cc_name = &core.cc_short_name;
        ApiSnippets {
            main_api: CcSnippet::new(quote! {
                __NEWLINE__ __COMMENT__ #msg
                #adt_cc_name() = delete; __NEWLINE__
            }),
            ..Default::default()
        }
    })
}

/// Formats the copy constructor and the copy-assignment operator for an ADT if
/// possible (i.e. if the `Clone` trait is implemented for the ADT).  Returns an
/// error otherwise (e.g. if there is no `Clone` impl, then the copy constructor
/// and assignment operator will be `=delete`d in the returned snippet).
fn generate_copy_ctor_and_assignment_operator<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
) -> Result<ApiSnippets, ApiSnippets> {
    fn fallible_format_copy_ctor_and_assignment_operator<'tcx>(
        db: &dyn BindingsGenerator<'tcx>,
        core: Rc<AdtCoreBindings<'tcx>>,
    ) -> Result<ApiSnippets> {
        let tcx = db.tcx();
        let cc_struct_name = &core.cc_short_name;

        let is_copy = {
            // TODO(b/259749095): Once generic ADTs are supported, `is_copy_modulo_regions`
            // might need to be replaced with a more thorough check - see
            // b/258249993#comment4.
            core.self_ty.is_copy_modulo_regions(tcx, post_analysis_typing_env(tcx, core.def_id))
        };
        if is_copy {
            let msg = "Rust types that are `Copy` get trivial, `default` C++ copy constructor \
                       and assignment operator.";
            let main_api = CcSnippet::new(quote! {
                __NEWLINE__ __COMMENT__ #msg
                #cc_struct_name(const #cc_struct_name&) = default;  __NEWLINE__
                #cc_struct_name& operator=(const #cc_struct_name&) = default;
            });
            let cc_details = CcSnippet::with_include(
                quote! {
                    static_assert(std::is_trivially_copy_constructible_v<#cc_struct_name>);
                    static_assert(std::is_trivially_copy_assignable_v<#cc_struct_name>);
                },
                CcInclude::type_traits(),
            );

            return Ok(ApiSnippets { main_api, cc_details, rs_details: RsSnippet::default() });
        }

        let trait_id = tcx
            .lang_items()
            .clone_trait()
            .ok_or_else(|| anyhow!("Can't find the `Clone` trait"))?;
        let TraitThunks {
            method_name_to_cc_thunk_name,
            cc_thunk_decls,
            rs_thunk_impls: rs_details,
        } = generate_trait_thunks(db, trait_id, &core)?;
        let main_api = CcSnippet::new(quote! {
            __NEWLINE__ __COMMENT__ "Clone::clone"
            #cc_struct_name(const #cc_struct_name&); __NEWLINE__
            __NEWLINE__ __COMMENT__ "Clone::clone_from"
            #cc_struct_name& operator=(const #cc_struct_name&); __NEWLINE__ __NEWLINE__
        });
        let cc_details = {
            // `unwrap` calls are okay because `Clone` trait always has these methods.
            let clone_thunk_name = method_name_to_cc_thunk_name.get(&sym::clone).unwrap();
            let clone_from_thunk_name = method_name_to_cc_thunk_name.get(&sym::clone_from).unwrap();

            let mut prereqs = CcPrerequisites::default();
            let cc_thunk_decls = cc_thunk_decls.into_tokens(&mut prereqs);

            let tokens = quote! {
                #cc_thunk_decls
                inline #cc_struct_name::#cc_struct_name(const #cc_struct_name& other) {
                    __crubit_internal::#clone_thunk_name(other, this);
                }
                inline #cc_struct_name& #cc_struct_name::operator=(const #cc_struct_name& other) {
                    if (this != &other) {
                        __crubit_internal::#clone_from_thunk_name(*this, other);
                    }
                    return *this;
                }
            };
            CcSnippet { tokens, prereqs }
        };
        Ok(ApiSnippets { main_api, cc_details, rs_details })
    }
    fallible_format_copy_ctor_and_assignment_operator(db, core.clone()).map_err(|err| {
        let msg = format!("{err:#}");
        let adt_cc_name = &core.cc_short_name;
        ApiSnippets {
            main_api: CcSnippet::new(quote! {
                __NEWLINE__ __COMMENT__ #msg
                #adt_cc_name(const #adt_cc_name&) = delete;  __NEWLINE__
                #adt_cc_name& operator=(const #adt_cc_name&) = delete;
            }),
            ..Default::default()
        }
    })
}

/// Formats the move constructor and the move-assignment operator for an ADT if
/// possible (it depends on various factors like `needs_drop`, `is_unpin` and
/// implementations of `Default` and/or `Clone` traits).  Returns an error
/// otherwise (the error's `ApiSnippets` contain a `=delete`d declaration).
fn generate_move_ctor_and_assignment_operator<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
) -> Result<ApiSnippets, ApiSnippets> {
    fn fallible_format_move_ctor_and_assignment_operator<'tcx>(
        db: &dyn BindingsGenerator<'tcx>,
        core: Rc<AdtCoreBindings<'tcx>>,
    ) -> Result<ApiSnippets> {
        let tcx = db.tcx();
        let adt_cc_name = &core.cc_short_name;
        if core.needs_drop(tcx) {
            let has_default_ctor = db.generate_default_ctor(core.clone()).is_ok();
            let is_unpin = core.self_ty.is_unpin(tcx, post_analysis_typing_env(tcx, core.def_id));
            if has_default_ctor && is_unpin {
                let main_api = CcSnippet::new(quote! {
                    #adt_cc_name(#adt_cc_name&&); __NEWLINE__
                    #adt_cc_name& operator=(#adt_cc_name&&); __NEWLINE__
                });
                let mut prereqs = CcPrerequisites::default();
                prereqs.includes.insert(db.support_header("internal/memswap.h"));
                prereqs.includes.insert(CcInclude::utility()); // for `std::move`
                let tokens = quote! {
                    inline #adt_cc_name::#adt_cc_name(#adt_cc_name&& other)
                            : #adt_cc_name() {
                        *this = std::move(other);
                    }
                    inline #adt_cc_name& #adt_cc_name::operator=(#adt_cc_name&& other) {
                        crubit::MemSwap(*this, other);
                        return *this;
                    }
                };
                let cc_details = CcSnippet { tokens, prereqs };
                Ok(ApiSnippets { main_api, cc_details, ..Default::default() })
            } else if db.generate_copy_ctor_and_assignment_operator(core).is_ok() {
                // The class will have a custom copy constructor and copy assignment operator
                // and *no* move constructor nor move assignment operator. This
                // way, when a move is requested, a copy is performed instead
                // (this is okay, this is what happens if a copyable pre-C++11
                // class is compiled in C++11 mode and moved).
                //
                // We can't use the `=default` move constructor, because it is elementwise and
                // semantically incorrect.  We can't `=delete` the move constructor because it
                // would make `SomeStruct(MakeSomeStruct())` select the deleted move constructor
                // and fail to compile.
                Ok(ApiSnippets::default())
            } else {
                bail!(
                    "C++ moves are deleted \
                       because there's no non-destructive implementation available."
                );
            }
        } else {
            let main_api = CcSnippet::new(quote! {
                // The generated bindings have to follow Rust move semantics:
                // * All Rust types are memcpy-movable (e.g. <internal link>/constructors.html says
                //   that "Every type must be ready for it to be blindly memcopied to somewhere
                //   else in memory")
                // * The only valid operation on a moved-from non-`Copy` Rust struct is to assign to
                //   it.
                //
                // The generated C++ bindings below match the required semantics because they:
                // * Generate trivial` C++ move constructor and move assignment operator. Per
                //   <internal link>/cpp/language/move_constructor#Trivial_move_constructor: "A trivial
                //   move constructor is a constructor that performs the same action as the trivial
                //   copy constructor, that is, makes a copy of the object representation as if by
                //   std::memmove."
                // * Generate trivial C++ destructor.
                //
                // In particular, note that the following C++ code and Rust code are exactly
                // equivalent (except that in Rust, reuse of `y` is forbidden at compile time,
                // whereas in C++, it's only prohibited by convention):
                // * C++, assumming trivial move constructor and trivial destructor:
                //   `auto x = std::move(y);`
                // * Rust, assumming non-`Copy`, no custom `Drop` or drop glue:
                //   `let x = y;`
                //
                // TODO(b/258251148): If the ADT provides a custom `Drop` impls or requires drop
                // glue, then extra care should be taken to ensure the C++ destructor can handle
                // the moved-from object in a way that meets Rust move semantics.  For example, the
                // generated C++ move constructor might need to assign `Default::default()` to the
                // moved-from object.
                #adt_cc_name(#adt_cc_name&&) = default; __NEWLINE__
                #adt_cc_name& operator=(#adt_cc_name&&) = default; __NEWLINE__
                __NEWLINE__
            });
            let cc_details = CcSnippet::with_include(
                quote! {
                    static_assert(std::is_trivially_move_constructible_v<#adt_cc_name>);
                    static_assert(std::is_trivially_move_assignable_v<#adt_cc_name>);
                },
                CcInclude::type_traits(),
            );
            Ok(ApiSnippets { main_api, cc_details, ..Default::default() })
        }
    }
    fallible_format_move_ctor_and_assignment_operator(db, core.clone()).map_err(|err| {
        let msg = format!("{err:#}");
        let adt_cc_name = &core.cc_short_name;
        ApiSnippets {
            main_api: CcSnippet::new(quote! {
                __NEWLINE__ __COMMENT__ #msg
                #adt_cc_name(#adt_cc_name&&) = delete;  __NEWLINE__
                #adt_cc_name& operator=(#adt_cc_name&&) = delete;
            }),
            ..Default::default()
        }
    })
}

/// Formats the forward declaration of an algebraic data type (an ADT - a
/// struct, an enum, or a union), returning something like
/// `quote!{ struct SomeStruct; }`.
///
/// Will panic if `def_id` doesn't identify an ADT that can be successfully
/// handled by `generate_adt_core`.
fn generate_fwd_decl(db: &Database<'_>, def_id: LocalDefId) -> TokenStream {
    let def_id = def_id.to_def_id(); // LocalDefId -> DefId conversion.

    // `generate_fwd_decl` should only be called for items from
    // `CcPrerequisites::fwd_decls` and `fwd_decls` should only contain ADTs
    // that `generate_adt_core` succeeds for.
    let core_bindings = db
        .generate_adt_core(def_id)
        .expect("`generate_fwd_decl` should only be called if `generate_adt_core` succeeded");
    let AdtCoreBindings { keyword, cc_short_name, .. } = &*core_bindings;

    quote! { #keyword #cc_short_name; }
}

fn generate_source_location(tcx: TyCtxt, local_def_id: LocalDefId) -> String {
    let def_span = tcx.def_span(local_def_id);
    let rustc_span::FileLines { file, lines } =
        match tcx.sess().source_map().span_to_lines(def_span) {
            Ok(filelines) => filelines,
            Err(_) => return "unknown location".to_string(),
        };
    let file_name = file.name.prefer_local().to_string();
    // Note: line_index starts at 0, while CodeSearch starts indexing at 1.
    let line_number = lines[0].line_index + 1;
    let google3_prefix = {
        // If rustc_span::FileName isn't a 'real' file, then it's surrounded by by angle
        // brackets, thus don't prepend "google3/" prefix.
        if file.name.is_real() {
            "google3/"
        } else {
            ""
        }
    };
    format!("{google3_prefix}{file_name};l={line_number}")
}

/// Formats the doc comment (if any) associated with the item identified by
/// `local_def_id`, and appends the source location at which the item is
/// defined.
fn generate_doc_comment(tcx: TyCtxt, local_def_id: LocalDefId) -> TokenStream {
    let hir_id = tcx.local_def_id_to_hir_id(local_def_id);
    let doc_comment = tcx
        .hir()
        .attrs(hir_id)
        .iter()
        .filter_map(|attr| attr.doc_str())
        .map(|symbol| symbol.to_string())
        .chain(once(format!("Generated from: {}", generate_source_location(tcx, local_def_id))))
        .join("\n\n");
    quote! { __COMMENT__ #doc_comment}
}

/// Formats a HIR item idenfied by `def_id`.  Returns `None` if the item
/// can be ignored. Returns an `Err` if the definition couldn't be formatted.
///
/// Will panic if `def_id` is invalid (i.e. doesn't identify a HIR item).
fn generate_item(
    db: &dyn BindingsGenerator<'_>,
    def_id: LocalDefId,
) -> Result<Option<ApiSnippets>> {
    let tcx = db.tcx();

    // TODO(b/350772554): Support `use` mod.
    if !is_public_or_supported_export(db, def_id.to_def_id()) {
        return Ok(None);
    }

    let item = match tcx.hir().expect_item(def_id) {
        Item { kind: ItemKind::Struct(_, generics) |
                     ItemKind::Enum(_, generics) |
                     ItemKind::Union(_, generics),
               .. } if !generics.params.is_empty() => {
            bail!("Generic types are not supported yet (b/259749095)");
        },
        Item { kind: ItemKind::Fn(..), .. } => db.generate_function(def_id).map(Some),
        Item { kind: ItemKind::Struct(..) | ItemKind::Enum(..) | ItemKind::Union(..), .. } => {
            let attributes = crubit_attr::get_attrs(tcx, def_id.to_def_id()).unwrap();
            if let Some(cpp_type) = attributes.cpp_type {
                let item_name = tcx.def_path_str(def_id.to_def_id());
                bail!(
                    "Type bindings for {item_name} suppressed due to being mapped to \
                            an existing C++ type ({cpp_type})"
                );
            }
            db.generate_adt_core(def_id.to_def_id())
                .map(|core| Some(generate_adt(db, core)))
        }
        Item { kind: ItemKind::TyAlias(..), ..} => generate_type_alias(db, def_id).map(Some),
        Item { ident, kind: ItemKind::Use(use_path, use_kind), ..} => {
            generate_use(db, ident.as_str(), use_path, use_kind).map(Some)
        },
        Item { kind: ItemKind::Const(..), .. } => generate_const(db, def_id).map(Some),
        Item { kind: ItemKind::Impl(_), .. } |  // Handled by `generate_adt`
        Item { kind: ItemKind::Mod(_), .. } =>  // Handled by `generate_crate`
            Ok(None),
        Item { kind, .. } => bail!("Unsupported rustc_hir::hir::ItemKind: {}", kind.descr()),
    };

    if let Ok(Some(item)) = item {
        Ok(Some(item.resolve_feature_requirements(crate_features(db, LOCAL_CRATE))?))
    } else {
        item
    }
}

/// Formats a C++ comment explaining why no bindings have been generated for
/// `local_def_id`.
fn generate_unsupported_def(
    db: &dyn BindingsGenerator<'_>,
    local_def_id: LocalDefId,
    err: Error,
) -> ApiSnippets {
    let tcx = db.tcx();
    db.errors().insert(&err);
    let source_loc = generate_source_location(tcx, local_def_id);
    let name = tcx.def_path_str(local_def_id.to_def_id());

    // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
    // says: To print causes as well [...], use the alternate selector “{:#}”.
    let msg = format!("Error generating bindings for `{name}` defined at {source_loc}: {err:#}");
    let main_api = CcSnippet::new(quote! { __NEWLINE__ __NEWLINE__ __COMMENT__ #msg __NEWLINE__ });

    ApiSnippets { main_api, cc_details: CcSnippet::default(), rs_details: RsSnippet::default() }
}

/// Formats namespace-bound snippets, given an iterator over (namespace_def_id,
/// namespace_qualifier, tokens) and the TyCtxt.
///
/// (The namespace_def_id is optional, where None corresponds to the top-level
/// namespace.)
///
/// For example, `[(id, ns, tokens)]` will be formatted as:
///
///     ```
///     namespace ns {
///     #tokens
///     }
///     ```
///
/// `format_namespace_bound_cc_tokens` tries to give a nice-looking output - for
/// example it combines consecutive items that belong to the same namespace,
/// when given `[(id, ns, tokens1), (id, ns, tokens2)]` as input:
///
///     ```
///     namespace ns {
///     #tokens1
///     #tokens2
///     }
///     ```
///
/// `format_namespace_bound_cc_tokens` also knows that top-level items (e.g.
/// ones where `NamespaceQualifier` doesn't contain any namespace names) should
/// be emitted at the top-level (not nesting them under a `namespace` keyword).
/// For example, `[(None, toplevel_ns, tokens)]` will be formatted as just:
///
///     ```
///     #tokens
///     ```
fn format_namespace_bound_cc_tokens(
    db: &dyn BindingsGenerator<'_>,
    iter: impl IntoIterator<Item = (Option<DefId>, NamespaceQualifier, TokenStream)>,
    tcx: TyCtxt,
) -> TokenStream {
    let iter = iter
        .into_iter()
        .coalesce(|(id1, ns1, mut tokens1), (id2, ns2, tokens2)| {
            // Coalesce tokens if consecutive items belong to the same namespace.
            if (id1 == id2) && (ns1 == ns2) {
                tokens1.extend(tokens2);
                Ok((id1, ns1, tokens1))
            } else {
                Err(((id1, ns1, tokens1), (id2, ns2, tokens2)))
            }
        })
        .map(|(ns_def_id_opt, ns, tokens)| {
            let mut ns_attributes = vec![];
            if let Some(ns_def_id) = ns_def_id_opt {
                if let Some(cc_deprecated_tag) = generate_deprecated_tag(tcx, ns_def_id) {
                    ns_attributes.push(cc_deprecated_tag);
                }
            }
            format_with_cc_body(db, &ns, tokens, ns_attributes).unwrap_or_else(|err| {
                let name = ns.0.iter().join("::");
                let err = format!("Failed to format namespace name `{name}`: {err}");
                quote! { __COMMENT__ #err }
            })
        });

    // Using fully-qualified syntax to avoid the warning that `intersperse`
    // may be added to the standard library in the future.
    //
    // TODO(https://github.com/rust-lang/rust/issues/79524): Use `.intersperse(...)` syntax once
    // 1) this stdlib feature gets stabilized and
    // 2) the method with conflicting name gets removed from `itertools`.
    let iter = itertools::Itertools::intersperse(iter, quote! { __NEWLINE__ __NEWLINE__ });

    iter.collect()
}

/// Formats all public items from the Rust crate being compiled.
fn generate_crate(db: &Database) -> Result<BindingsTokens> {
    let tcx = db.tcx();
    let mut cc_details_prereqs = CcPrerequisites::default();
    let mut cc_details: Vec<(LocalDefId, TokenStream)> = vec![];
    let mut cc_api_impl = TokenStream::default();
    let mut extern_c_decls = BTreeSet::new();
    let mut main_apis = HashMap::<LocalDefId, CcSnippet>::new();
    let formatted_items = tcx
        .hir()
        .items()
        .filter_map(|item_id| {
            let def_id: LocalDefId = item_id.owner_id.def_id;
            db.generate_item(def_id)
                .unwrap_or_else(|err| Some(generate_unsupported_def(db, def_id, err)))
                .map(|api_snippets| (def_id, api_snippets))
        })
        .sorted_by_key(|(def_id, _)| tcx.def_span(*def_id));
    for (def_id, api_snippets) in formatted_items {
        let old_item = main_apis.insert(def_id, api_snippets.main_api);
        assert!(old_item.is_none(), "Duplicated key: {def_id:?}");

        // `cc_details` don't participate in the toposort, because
        // `CcPrerequisites::defs` always use `main_api` as the predecessor
        // - `chain`ing `cc_details` after `ordered_main_apis` trivially
        // meets the prerequisites.
        cc_details.push((def_id, api_snippets.cc_details.into_tokens(&mut cc_details_prereqs)));
        cc_api_impl.extend(api_snippets.rs_details.into_tokens(&mut extern_c_decls));
    }

    // Find the order of `main_apis` that 1) meets the requirements of
    // `CcPrerequisites::defs` and 2) makes a best effort attempt to keep the
    // `main_apis` in the same order as the source order of the Rust APIs.
    let ordered_ids = {
        let toposort::TopoSortResult { ordered: ordered_ids, failed: failed_ids } = {
            let nodes = main_apis.keys().copied();
            let deps = main_apis.iter().flat_map(|(&successor, main_api)| {
                let predecessors = main_api.prereqs.defs.iter().copied();
                predecessors.map(move |predecessor| toposort::Dependency { predecessor, successor })
            });
            toposort::toposort(nodes, deps, move |lhs_id, rhs_id| {
                tcx.def_span(*lhs_id).cmp(&tcx.def_span(*rhs_id))
            })
        };
        assert_eq!(
            0,
            failed_ids.len(),
            "There are no known scenarios where CcPrerequisites::defs can form \
                    a dependency cycle. These `LocalDefId`s form an unexpected cycle: {}",
            failed_ids.into_iter().map(|id| format!("{:?}", id)).join(",")
        );
        ordered_ids
    };

    // Destructure/rebuild `main_apis` (in the same order as `ordered_ids`) into
    // `includes`, and `ordered_cc` (mixing in `fwd_decls` and `cc_details`).
    let (includes, ordered_cc) = {
        let mut already_declared = HashSet::new();
        let mut fwd_decls = HashSet::new();
        let mut includes = cc_details_prereqs.includes;
        let mut ordered_main_apis: Vec<(LocalDefId, TokenStream)> = Vec::new();
        for def_id in ordered_ids.into_iter() {
            let CcSnippet {
                tokens: cc_tokens,
                prereqs: CcPrerequisites {
                    includes: mut inner_includes,
                    fwd_decls: inner_fwd_decls,
                    .. // `defs` have already been utilized by `toposort` above
                }
            } = main_apis.remove(&def_id).unwrap();

            fwd_decls.extend(inner_fwd_decls.difference(&already_declared).copied());
            already_declared.insert(def_id);
            already_declared.extend(inner_fwd_decls.into_iter());

            includes.append(&mut inner_includes);
            ordered_main_apis.push((def_id, cc_tokens));
        }

        let fwd_decls = fwd_decls
            .into_iter()
            .sorted_by_key(|def_id| tcx.def_span(*def_id))
            .map(|local_def_id| (local_def_id, generate_fwd_decl(db, local_def_id)));

        // The first item of the tuple here is the DefId of the namespace.
        let ordered_cc: Vec<(Option<DefId>, NamespaceQualifier, TokenStream)> = fwd_decls
            .into_iter()
            .chain(ordered_main_apis)
            .chain(cc_details)
            .map(|(local_def_id, tokens)| {
                let ns_def_id = tcx.opt_parent(local_def_id.to_def_id());
                let mod_path = FullyQualifiedName::new(db, local_def_id.to_def_id()).cpp_ns_path;
                (ns_def_id, mod_path, tokens)
            })
            .collect_vec();

        (includes, ordered_cc)
    };

    // Generate top-level elements of the C++ header file.
    let cc_api = {
        let cpp_top_level_ns = format_top_level_ns_for_crate(db, LOCAL_CRATE);
        let cpp_top_level_ns = format_cc_ident(db, cpp_top_level_ns.as_str())?;

        let includes = format_cc_includes(&includes);
        let ordered_cc = format_namespace_bound_cc_tokens(db, ordered_cc, tcx);
        quote! {
            #includes
            __NEWLINE__ __NEWLINE__
            namespace #cpp_top_level_ns {
                __NEWLINE__
                #ordered_cc
                __NEWLINE__
            }
            __NEWLINE__
        }
    };

    let mut decls = quote! {};
    for ExternCDecl { decl, .. } in extern_c_decls.into_iter() {
        decls.extend(decl);
    }

    if !decls.is_empty() {
        cc_api_impl = quote! {
            #cc_api_impl

           extern "C" {
               #decls
           }
        };
    }

    Ok(BindingsTokens { cc_api, cc_api_impl })
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use quote::quote;

    use error_report::IgnoreErrors;
    use run_compiler_test_support::{find_def_id_by_name, run_compiler_for_testing};
    use token_stream_matchers::{
        assert_cc_matches, assert_cc_not_matches, assert_rs_matches, assert_rs_not_matches,
    };

    /// This test covers only a single example of a function that should get a
    /// C++ binding. The test focuses on verification that the output from
    /// `generate_function` gets propagated all the way to
    /// `GenerateBindings::new`. Additional coverage of how functions are
    /// formatted is provided by `test_format_item_..._fn_...` tests (which
    /// work at the `generate_function` level).
    #[test]
    fn test_generated_bindings_fn_no_mangle_extern_c() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn public_function() {
                    println!("foo");
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    extern "C" void public_function();
                }
            );

            // No Rust thunks should be generated in this test scenario.
            assert_rs_not_matches!(bindings.cc_api_impl, quote! { public_function });
        });
    }

    /// `test_generated_bindings_fn_export_name` covers a scenario where
    /// `MixedSnippet::cc` is present but `MixedSnippet::rs` is empty
    /// (because no Rust thunks are needed).
    #[test]
    fn test_generated_bindings_fn_export_name() {
        let test_src = r#"
                #[unsafe(export_name = "export_name")]
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    namespace rust_out {
                        ...
                        double public_function(double x, double y);
                        namespace __crubit_internal {
                            extern "C" double export_name(double, double);
                        }
                        inline double public_function(double x, double y) {
                            return __crubit_internal::export_name(x, y);
                        }
                    }
                }
            );
        });
    }

    /// The `test_generated_bindings_impl` test covers only a single example of
    /// a non-trait `impl`. Additional coverage of how items are formatted
    /// should be provided in the future by `test_format_item_...` tests.
    ///
    /// We don't want to duplicate coverage already provided by
    /// `test_format_item_static_method`, but we do want to verify that
    /// * `generate_crate` won't process the `impl` as a standalone HIR item
    /// * The actual shape of the bindings still looks okay at this level.
    #[test]
    fn test_generated_bindings_impl() {
        let test_src = r#"
                #![allow(dead_code)]

                pub struct SomeStruct(i32);

                impl SomeStruct {
                    pub fn public_static_method() -> i32 { 123 }

                    fn private_static_method() -> i32 { 123 }
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    namespace rust_out {
                        ...
                        struct ... SomeStruct ... {
                            // No point replicating test coverage of
                            // `test_format_item_static_method`.
                            ...
                            std::int32_t public_static_method();
                            ...
                        };
                        ...
                        std::int32_t SomeStruct::public_static_method() {
                            ...
                        }
                        ...
                    }  // namespace rust_out
                }
            );
            assert_rs_matches!(
                bindings.cc_api_impl,
                quote! {
                    extern "C" fn ...() -> i32 {
                        ::rust_out::SomeStruct::public_static_method()
                    }
                }
            );
        });
    }

    #[test]
    fn test_generated_bindings_includes() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn public_function(i: i32, d: isize, u: u64) {
                    dbg!(i);
                    dbg!(d);
                    dbg!(u);
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    __HASH_TOKEN__ include <cstdint> ...
                    namespace ... {
                        ...
                        extern "C" void public_function(
                            std::int32_t i,
                            std::intptr_t d,
                            std::uint64_t u);
                    }
                }
            );
        });
    }

    /// Tests that `toposort` is used to reorder item bindings.
    #[test]
    fn test_generated_bindings_prereq_defs_field_deps_require_reordering() {
        let test_src = r#"
                #![allow(dead_code)]

                // In the generated bindings `Outer` needs to come *after* `Inner`.
                pub struct Outer(Inner);
                pub struct Inner(bool);
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    namespace rust_out {
                    ...
                        struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(1) [[clang::trivial_abi]] Inner final {
                          ... union { ... bool __field0; }; ...
                        };
                    ...
                        struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(1) [[clang::trivial_abi]] Outer final {
                          ... union { ... ::rust_out::Inner __field0; }; ...
                        };
                    ...
                    }  // namespace rust_out
                }
            );
        });
    }

    /// Tests that a forward declaration is present when it is required to
    /// preserve the original source order.  In this test the
    /// `CcPrerequisites::fwd_decls` dependency comes from a pointer parameter.
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_for_ptr_param() {
        let test_src = r#"
                #![allow(dead_code)]

                // To preserve original API order we need to forward declare S.
                pub fn f(_: *const S) {}
                pub struct S(bool);
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    namespace rust_out {
                        ...
                        // Verifing the presence of this forward declaration
                        // it the essence of this test.  The order of the items
                        // below also matters.
                        struct S;
                        ...
                        void f(::rust_out::S const* __param_0);
                        ...
                        struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(...) [[clang::trivial_abi]] S final { ... }
                        ...
                        inline void f(::rust_out::S const* __param_0) { ... }
                        ...
                    }  // namespace rust_out
                }
            );
        });
    }

    /// Tests that a forward declaration is present when it is required to
    /// preserve the original source order.  In this test the
    /// `CcPrerequisites::fwd_decls` dependency comes from a
    /// function declaration that has a parameter that takes a struct by value.
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_for_cpp_fn_decl() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn f(s: S) -> bool { s.0 }

                #[repr(C)]
                pub struct S(bool);
            "#;

        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    namespace rust_out {
                        ...
                        // Verifing the presence of this forward declaration
                        // is the essence of this test.  The order also matters:
                        // 1. The fwd decl of `S` should come first,
                        // 2. Declaration of `f` and definition of `S` should come next
                        //    (in their original order - `f` first and then `S`).
                        struct S;
                        ...
                        // `CcPrerequisites` of `f` declaration below (the main api of `f`) should
                        // include `S` as a `fwd_decls` edge, rather than as a `defs` edge.
                        bool f(::rust_out::S s);
                        ...
                        struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(...) [[clang::trivial_abi]] S final { ... }
                        ...
                    }  // namespace rust_out
                }
            );
        });
    }

    /// This test verifies that a forward declaration for a given ADT is only
    /// emitted once (and not once for every API item that requires the
    /// forward declaration as a prerequisite).
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_no_duplication() {
        let test_src = r#"
                #![allow(dead_code)]

                // All three functions below require a forward declaration of S.
                pub fn f1(_: *const S) {}
                pub fn f2(_: *const S) {}
                pub fn f3(_: *const S) {}

                pub struct S(bool);

                // This function also includes S in its CcPrerequisites::fwd_decls
                // (although here it is not required, because the definition of S
                // is already available above).
                pub fn f4(_: *const S) {}
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap().cc_api.to_string();

            // Only a single forward declaration is expected.
            assert_eq!(1, bindings.matches("struct S ;").count(), "bindings = {bindings}");
        });
    }

    /// This test verifies that forward declarations are emitted in a
    /// deterministic order. The particular order doesn't matter _that_
    /// much, but it definitely shouldn't change every time
    /// `cc_bindings_from_rs` is invoked again.  The current order preserves
    /// the original source order of the Rust API items.
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_deterministic_order() {
        let test_src = r#"
                #![allow(dead_code)]

                // To try to mix things up, the bindings for the functions below
                // will *ask* for forward declarations in a different order:
                // * Different from the order in which the forward declarations
                //   are expected to be *emitted* (the original source order).
                // * Different from alphabetical order.
                pub fn f1(_: *const b::S3) {}
                pub fn f2(_: *const a::S2) {}
                pub fn f3(_: *const a::S1) {}

                pub mod a {
                    pub struct S1(bool);
                    pub struct S2(bool);
                }

                pub mod b {
                    pub struct S3(bool);
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    namespace rust_out {
                        ...
                        // Verifying that we get the same order in each test
                        // run is the essence of this test.
                        namespace a {
                        struct S1;
                        struct S2;
                        }
                        namespace b {
                        struct S3;
                        }
                        ...
                        void f1 ...
                        void f2 ...
                        void f3 ...

                        namespace a { ...
                        struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(...) [[clang::trivial_abi]] S1 final { ... } ...
                        struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(...) [[clang::trivial_abi]] S2 final { ... } ...
                        } ...
                        namespace b { ...
                        struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(...) [[clang::trivial_abi]] S3 final { ... } ...
                        } ...
                    }  // namespace rust_out
                }
            );
        });
    }

    /// This test verifies that forward declarations are not emitted if they are
    /// not needed (e.g. if bindings the given `struct` or other ADT have
    /// already been defined earlier).  In particular, we don't want to emit
    /// forward declarations for *all* `structs` (regardless if they are
    /// needed or not).
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_not_needed_because_of_initial_order() {
        let test_src = r#"
                #[allow(dead_code)]

                pub struct S(bool);

                // S is already defined above - no need for forward declaration in C++.
                pub fn f(_s: *const S) {}
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_not_matches!(bindings.cc_api, quote! { struct S; });
            assert_cc_matches!(bindings.cc_api, quote! { void f(::rust_out::S const* _s); });
        });
    }

    /// This test verifies that a method declaration doesn't ask for a forward
    /// declaration to the struct.
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_not_needed_inside_struct_definition() {
        let test_src = r#"
                #![allow(dead_code)]

                pub struct S {
                    // This shouldn't require a fwd decl of S.
                    field: *const S,
                }

                impl S {
                    // This shouldn't require a fwd decl of S.
                    pub fn create() -> S { Self{ field: std::ptr::null() } }
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_not_matches!(bindings.cc_api, quote! { struct S; });
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    static ::rust_out::S create(); ...
                    union { ... ::rust_out::S const* field; }; ...
                }
            );
        });
    }

    #[test]
    fn test_generated_bindings_module_basics() {
        let test_src = r#"
                pub mod some_module {
                    pub fn some_func() {}
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    namespace rust_out {
                        namespace some_module {
                            ...
                            inline void some_func() { ... }
                            ...
                        }  // namespace some_module
                    }  // namespace rust_out
                }
            );
            assert_rs_matches!(
                bindings.cc_api_impl,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C"
                    fn ...() -> () {
                        ::rust_out::some_module::some_func()
                    }
                }
            );
        });
    }

    #[test]
    fn test_generated_bindings_module_name_is_cpp_reserved_keyword() {
        let test_src = r#"
                pub mod reinterpret_cast {
                    pub fn working_module_f1() {}
                    pub fn working_module_f2() {}
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();

            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    namespace rust_out {
                        namespace reinterpret_cast_ {
                            ...
                            void working_module_f1();
                            ...
                            void working_module_f2();
                            ...
                        }  // namespace reinterpret_cast_

                    }  // namespace rust_out
                }
            );
        });
    }

    /// `test_generated_bindings_non_pub_items` verifies that non-public items
    /// are not present/propagated into the generated bindings.
    #[test]
    fn test_generated_bindings_non_pub_items() {
        let test_src = r#"
                #![allow(dead_code)]

                extern "C" fn private_function() {
                    println!("foo");
                }

                struct PrivateStruct {
                    x: i32,
                    y: i32,
                }

                pub struct PublicStruct(i32);

                impl PublicStruct {
                    fn private_method() {}
                }

                pub mod public_module {
                    fn priv_func_in_pub_module() {}
                }

                mod private_module {
                    pub fn pub_func_in_priv_module() { priv_func_in_priv_module() }
                    fn priv_func_in_priv_module() {}
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_not_matches!(bindings.cc_api, quote! { private_function });
            assert_rs_not_matches!(bindings.cc_api_impl, quote! { private_function });
            assert_cc_not_matches!(bindings.cc_api, quote! { PrivateStruct });
            assert_rs_not_matches!(bindings.cc_api_impl, quote! { PrivateStruct });
            assert_cc_not_matches!(bindings.cc_api, quote! { private_method });
            assert_rs_not_matches!(bindings.cc_api_impl, quote! { private_method });
            assert_cc_not_matches!(bindings.cc_api, quote! { priv_func_in_priv_module });
            assert_rs_not_matches!(bindings.cc_api_impl, quote! { priv_func_in_priv_module });
            assert_cc_not_matches!(bindings.cc_api, quote! { priv_func_in_pub_module });
            assert_rs_not_matches!(bindings.cc_api_impl, quote! { priv_func_in_pub_module });
            assert_cc_not_matches!(bindings.cc_api, quote! { private_module });
            assert_rs_not_matches!(bindings.cc_api_impl, quote! { private_module });
            assert_cc_not_matches!(bindings.cc_api, quote! { pub_func_in_priv_module });
            assert_rs_not_matches!(bindings.cc_api_impl, quote! { pub_func_in_priv_module });
        });
    }

    #[test]
    fn test_generated_bindings_top_level_items() {
        let test_src = "pub fn public_function() {}";
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            let expected_comment_txt =
                "Automatically @generated C++ bindings for the following Rust crate:\n\
                 rust_out\n\
                 Features: experimental, supported";
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    __COMMENT__ #expected_comment_txt
                    ...
                    __HASH_TOKEN__ pragma once
                    ...
                    namespace rust_out {
                        ...
                    }
                }
            );
            assert_cc_matches!(
                bindings.cc_api_impl,
                quote! {
                    __COMMENT__ #expected_comment_txt
                }
            );
        })
    }

    #[test]
    fn test_format_item_reexport_private_type() {
        let test_src = r#"
            #![allow(dead_code)]
            mod test_mod {
                pub struct ReExportedStruct{
                    pub field: i32
                }
                pub struct NotReExportedStruct{
                    pub field: i32
                }
            }

            pub use crate::test_mod::ReExportedStruct as Z;
            pub use crate::test_mod::ReExportedStruct as X;
            pub use crate::test_mod::ReExportedStruct as Y;
            #[allow(unused_imports)]
            use crate::test_mod::ReExportedStruct as PrivateUse;
            "#;
        test_format_item(test_src, "NotReExportedStruct", |result| {
            let result = result.unwrap();
            assert!(result.is_none());
        });

        test_format_item(test_src, "PrivateUse", |result| {
            let result = result.unwrap();
            assert!(result.is_none());
        });

        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    ...
                    namespace __crubit_internal {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: X") alignas(4)
                    [[clang::trivial_abi]] ReExportedStruct final
                    ...
                    }
                }
            );

            assert_rs_matches!(
                bindings.cc_api_impl,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::X>() == 4);
                }
            );

            assert_rs_not_matches!(bindings.cc_api_impl, quote! { ::rust_out::Y });
            assert_rs_not_matches!(bindings.cc_api_impl, quote! { ::rust_out::Z });
        });
    }

    #[test]
    fn test_generated_bindings_module_deprecated_no_args() {
        let test_src = r#"
                #[deprecated]
                pub mod some_module {
                    pub fn some_function() {}
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    ...
                        [[deprecated]]
                        namespace some_module {
                            ...
                        }  // namespace some_module
                    ...
                }
            );
        });
    }

    #[test]
    fn test_generated_bindings_module_deprecated_with_message() {
        let test_src = r#"
                #[deprecated = "Use other_module instead"]
                pub mod some_module {
                    pub fn some_function() {}
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    ...
                        [[deprecated("Use other_module instead")]]
                        namespace some_module {
                            ...
                        }  // namespace some_module
                    ...
                }
            );
        });
    }

    #[test]
    fn test_generated_bindings_module_deprecated_named_args() {
        let test_src = r#"
                #[deprecated(since = "3.14", note = "Use other_module instead")]
                pub mod some_module {
                    pub fn some_function() {}
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    ...
                        [[deprecated("Use other_module instead")]]
                        namespace some_module {
                            ...
                        }  // namespace some_module
                    ...
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_extern_c_no_mangle_no_params_no_return_type() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn public_function() {}
            "#;
        test_format_item(test_src, "public_function", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void public_function();
                }
            );

            // Sufficient to just re-declare the Rust API in C++.
            // (i.e. there is no need to have a C++-side definition of `public_function`).
            assert!(result.cc_details.tokens.is_empty());

            // There is no need to have a separate thunk for an `extern "C"` function.
            assert!(result.rs_details.tokens.is_empty());
        });
    }

    #[test]
    fn test_format_const() {
        let test_src = r#"
            pub const BOOL_TRUE: bool = true;
            pub const BOOL_FALSE: bool = false;
            pub const INT_POS: i32 = 42;
            pub const INT_NEG: i32 = -17;
            pub const FLOAT_32: f32 = 0.125; // 2^(-3)
            pub const FLOAT_64: f64 = 0.0078125; // 2^(-7)
            pub const LARGE_INT: i64 = 9223372036854775807;
            pub const UNSIGNED_INT: u32 = 4294967295;
            pub const SLICE_LENGTH: usize = "hello world".len();
            pub const ISIZE: isize = 42;
            use core::ffi::c_char;
            pub const CHAR: c_char = 42;
        "#;

        test_format_item(test_src, "BOOL_TRUE", |result| {
            let result = result.unwrap().unwrap();
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr bool BOOL_TRUE = true;
                }
            );
        });

        test_format_item(test_src, "BOOL_FALSE", |result| {
            let result = result.unwrap().unwrap();
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr bool BOOL_FALSE = false;
                }
            );
        });

        test_format_item(test_src, "INT_POS", |result| {
            let result = result.unwrap().unwrap();
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr std::int32_t INT_POS = 42;
                }
            );
        });

        test_format_item(test_src, "LARGE_INT", |result| {
            let result = result.unwrap().unwrap();
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr std::int64_t LARGE_INT = 9223372036854775807;
                }
            );
        });

        test_format_item(test_src, "UNSIGNED_INT", |result| {
            let result = result.unwrap().unwrap();
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr std::uint32_t UNSIGNED_INT = 4294967295;
                }
            );
        });

        test_format_item(test_src, "INT_NEG", |result| {
            let result = result.unwrap().unwrap();
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr std::int32_t INT_NEG = -17;
                }
            );
        });

        test_format_item(test_src, "FLOAT_32", |result| {
            let result = result.unwrap().unwrap();
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr float FLOAT_32 = 0.125;
                }
            );
        });

        test_format_item(test_src, "FLOAT_64", |result| {
            let result = result.unwrap().unwrap();
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr double FLOAT_64 = 0.0078125;
                }
            );
        });

        test_format_item(test_src, "SLICE_LENGTH", |result| {
            let result = result.unwrap().unwrap();
            assert_eq!("hello world".len(), 11);
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr std::uintptr_t SLICE_LENGTH = 11;
                }
            );
        });

        test_format_item(test_src, "ISIZE", |result| {
            let result = result.unwrap().unwrap();
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr std::intptr_t ISIZE = 42;
                }
            );
        });

        test_format_item(test_src, "CHAR", |result| {
            let result = result.unwrap().unwrap();
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr char CHAR = 42;
                }
            );
        });

        let test_src = r#"
                #![allow(nonstandard_style)]
                pub const reinterpret_cast: u32 = 42;
            "#;
        test_format_item(test_src, "reinterpret_cast", |result| {
            let result = result.unwrap().unwrap();
            assert_cc_matches!(
                result.main_api.tokens,
                quote! {
                    constexpr std::uint32_t reinterpret_cast_ = 42;
                }
            );
        });
    }

    /// The `test_format_item_fn_explicit_unit_return_type` test below is very
    /// similar to the
    /// `test_format_item_fn_extern_c_no_mangle_no_params_no_return_type` above,
    /// except that the return type is explicitly spelled out.  There is no
    /// difference in `ty::FnSig` so our code behaves exactly the same, but the
    /// test has been planned based on earlier, hir-focused approach and having
    /// this extra test coverage shouldn't hurt. (`hir::FnSig`
    /// and `hir::FnRetTy` _would_ see a difference between the two tests, even
    /// though there is no different in the current `bindings.rs` code).
    #[test]
    fn test_format_item_fn_explicit_unit_return_type() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn explicit_unit_return_type() -> () {}
            "#;
        test_format_item(test_src, "explicit_unit_return_type", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void explicit_unit_return_type();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_never_return_type() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn never_returning_function() -> ! {
                    panic!("This function panics and therefore never returns");
                }
            "#;
        test_format_item(test_src, "never_returning_function", |result| {
            // TODO(b/254507801): The function should be annotated with the `[[noreturn]]`
            // attribute.
            // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
            // details).
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void never_returning_function();
                }
            );
        })
    }

    /// `test_format_item_fn_mangling` checks that bindings can be generated for
    /// `extern "C"` functions that do *not* have `#[unsafe(no_mangle)]`
    /// attribute.  The test elides away the mangled name in the
    /// `assert_cc_matches` checks below, but end-to-end test coverage
    /// should eventually be provided by `test/functions` (see b/262904507).
    #[test]
    fn test_format_item_fn_mangling() {
        let test_src = r#"
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_format_item(test_src, "public_function", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    double public_function(double x, double y);
                }
            );
            // TODO(b/262904507): omit the thunk and uncomment the next line.
            // assert!(result.rs_details.tokens.is_empty());
            assert!(result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double, double);
                    }
                    ...
                    inline double public_function(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_export_name() {
        let test_src = r#"
                #[unsafe(export_name = "export_name")]
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_format_item(test_src, "public_function", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    double public_function(double x, double y);
                }
            );

            // There is no need to have a separate thunk for an `extern "C"` function.
            assert!(result.rs_details.tokens.is_empty());

            // We generate a C++-side definition of `public_function` so that we
            // can call a differently-named (but same-signature) `export_name` function.
            assert!(result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double export_name(double, double);
                    }
                    ...
                    inline double public_function(double x, double y) {
                        return __crubit_internal::export_name(x, y);
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_extern_c_unsafe() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn foo() {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    void foo();
                }
            );
            assert!(result.rs_details.tokens.is_empty());
        });
    }

    /// For non-extern "C" unsafe functions, we need a thunk, and it needs some
    /// `unsafe`.
    ///
    /// The thunk itself needs to be unsafe, because it wraps an unsafe function
    /// and is still in-principle itself directly callable. It also needs to
    /// have an unsafe block inside of it due to RFC #2585
    /// `unsafe_block_in_unsafe_fn`.
    #[test]
    fn test_format_item_fn_unsafe() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub unsafe fn foo() {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    void foo();
                }
            );
            assert_cc_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C" fn __crubit_thunk_foo() -> () {
                        unsafe { ::rust_out::foo() }
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_references() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo(_x: &i32, _y: &i32) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                let main_api = result.unwrap().unwrap().main_api;
                assert_cc_matches!(
                    main_api.tokens,
                    quote! {
                        void foo(
                            std::int32_t const& [[clang::annotate_type("lifetime" , "__anon1")]] _x,
                            std::int32_t const& [[clang::annotate_type("lifetime" , "__anon2")]] _y );
                    }
                );
            },
        );
    }

    #[test]
    fn test_format_item_fn_risky_mut_reference() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo(_x: &mut i32, _y: &i32) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "support for functions taking a mutable reference, and which may alias in C++, requires //features:experimental"
                )
            },
        );
    }

    #[test]
    fn test_format_item_fn_static_reference() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo(_x: &'static i32) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "support for bound reference lifetimes (such as 'static) requires //features:experimental"
                )
            },
        );
    }

    // NOTE: If we gain support for references as non-parameter types, we must
    // _still_ require :experimental.
    #[test]
    fn test_format_item_fn_nested_reference() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo(_x: &&i32) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "Error handling parameter #0: Failed to format the referent of the reference type `&'__anon1 &'__anon2 i32`: Can't format `&'__anon2 i32`, because references are only supported in function parameter types and return types (b/286256327)"
                )
            },
        );
    }

    #[test]
    fn test_format_item_fn_returned_static_reference() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo() -> &'static i32 {todo!()}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "support for references of non-function-param types requires //features:experimental"
                )
            },
        );
    }

    #[test]
    fn test_format_item_fn_reused_reference_lifetime() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo<'a>(_x: &'a i32, _y: &'a i32) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "support for multiple uses of a lifetime parameter requires //features:experimental"
                )
            },
        );
    }

    // NOTE: If we gain support for lifetime generic parameters, we must _still_
    // require :experimental.
    #[test]
    fn test_format_item_fn_reused_reference_lifetime_struct() {
        let test_src = r#"
                pub struct Foo<'a>(pub i32, core::marker::PhantomData<&'a i32>);
                #[unsafe(no_mangle)]
                pub fn foo<'a>(_x: &'a Foo<'a>) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "Error handling parameter #0: Failed to format the referent of the reference type `&'a Foo<'a>`: Generic types are not supported yet (b/259749095)"
                )
            },
        );
    }

    #[test]
    fn test_format_item_fn_char() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo(_x: char) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "support for the Rust `char` type requires //features:experimental"
                )
            },
        );
    }

    #[test]
    fn test_format_bridged_type_pointer_like_errors() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(
                  cpp_type="const CppType*",
                  cpp_type_include="cpp_ns/cpp_type.h",
                )]
                pub struct MissingReprTransparent {
                    pub cpp_type: *const core::ffi::c_void,
                }

                #[unsafe(no_mangle)]
                pub fn with_missing_repr_transparent(_: MissingReprTransparent) {}

                #[__crubit::annotate(
                  cpp_type="const CppType*",
                  cpp_type_include="cpp_ns/cpp_type.h",
                )]
                #[repr(transparent)]
                pub struct NotPointerLike {
                    pub value: i32,
                }

                #[unsafe(no_mangle)]
                pub fn not_pointer_like(_: NotPointerLike) {}
        "#;

        test_format_item(test_src, "with_missing_repr_transparent", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error handling parameter #0: Can't convert MissingReprTransparent to a C++ \
                    pointer as it's not `repr(transparent)`"
            );
        });

        test_format_item(test_src, "not_pointer_like", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error handling parameter #0: Can't convert NotPointerLike to a C++ pointer as \
                    its layout is not pointer-like. To be considered pointer-like it may only have \
                    one non-ZST field that needs to be a C ABI compatible pointer."
            );
        });
    }

    #[test]
    fn test_format_bridged_func_arg_pointer_like() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(
                  cpp_type="const CppType*",
                  cpp_type_include="cpp_ns/cpp_type.h",
                )]
                #[repr(transparent)]
                pub struct RustTypeView {
                    pub cpp_type: *const core::ffi::c_void,
                }

                #[unsafe(no_mangle)]
                pub fn foo(_: RustTypeView) {}
        "#;
        test_format_item(test_src, "RustTypeView", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Type bindings for RustTypeView suppressed \
                    due to being mapped to an existing C++ type (const CppType*)"
            );
        });

        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;

            assert_eq!(main_api.prereqs.includes.len(), 1);

            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    void foo(const CppType* __param_0);
                }
            );

            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void __crubit_thunk_foo(const CppType*);
                    }

                    inline void foo(const CppType* __param_0) {
                        return __crubit_internal::__crubit_thunk_foo(__param_0);
                    }
                }
            );

            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn __crubit_thunk_foo(__param_0: *const core::ffi::c_void) -> () {
                        let mut __crubit___param_0_uninit =
                            ::core::mem::MaybeUninit::<::rust_out::RustTypeView>::uninit();
                        unsafe {
                            __crubit___param_0_uninit.write(::core::mem::transmute(__param_0));
                        }
                        ::rust_out::foo(unsafe { __crubit___param_0_uninit.assume_init() })
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_bridged_func_arg_by_pointer() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(
                  cpp_type="CppType const*",
                  cpp_type_include="cpp_ns/cpp_type.h",
                  cpp_to_rust_converter="cpp_pointer_to_rust_struct",
                  rust_to_cpp_converter="rust_struct_to_cpp_pointer",
                )]
                pub struct RustTypeView {
                    pub cpp_type: *const core::ffi::c_void,
                }

                #[unsafe(no_mangle)]
                pub fn foo(_: RustTypeView) {}
        "#;
        test_format_item(test_src, "RustTypeView", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Type bindings for RustTypeView suppressed \
                    due to being mapped to an existing C++ type (CppType const*)"
            );
        });

        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;

            assert_eq!(main_api.prereqs.includes.len(), 1);

            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    void foo(CppType const* __param_0);
                }
            );

            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void __crubit_thunk_foo(CppType const*);
                    }

                    inline void foo(CppType const* __param_0) {
                        return __crubit_internal::__crubit_thunk_foo(__param_0);
                    }
                }
            );

            let extern_c_decl = result.rs_details.extern_c_decls.first().unwrap();
            assert_eq!(extern_c_decl.symbol, Symbol::intern("cpp_pointer_to_rust_struct"));
            assert_rs_matches!(
                extern_c_decl.decl,
                quote! {
                    fn cpp_pointer_to_rust_struct(cpp_in: *const core::ffi::c_void,
                        rs_out: *mut core::ffi::c_void);
                }
            );

            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn __crubit_thunk_foo(__param_0: *const core::ffi::c_void) -> () {
                        let mut __crubit___param_0_uninit =
                            ::core::mem::MaybeUninit::<::rust_out::RustTypeView>::uninit();
                        unsafe {
                            cpp_pointer_to_rust_struct(
                                __param_0,
                                __crubit___param_0_uninit.as_mut_ptr() as *mut core::ffi::c_void
                            );
                        }
                        ::rust_out::foo(unsafe { __crubit___param_0_uninit.assume_init() })
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_bridged_func_arg_by_value() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(
                  cpp_type="cpp_ns::CppType",
                  cpp_type_include="cpp_ns/cpp_type.h",
                  rust_to_cpp_converter="convert_rust_to_cpp_type",
                  cpp_to_rust_converter="convert_cpp_to_rust_type",
                )]
                pub struct RustType {
                    pub x: i32,
                }

                #[unsafe(no_mangle)]
                pub fn foo(_a: RustType) {}
        "#;
        test_format_item(test_src, "RustType", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Type bindings for RustType suppressed \
                    due to being mapped to an existing C++ type (cpp_ns::CppType)"
            );
        });
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;

            assert_eq!(main_api.prereqs.includes.len(), 1);
            assert_eq!(
                *main_api.prereqs.includes.first().unwrap(),
                CcInclude::user_header("cpp_ns/cpp_type.h".into())
            );

            assert_eq!(result.rs_details.extern_c_decls.len(), 1);

            let extern_c_decl = result.rs_details.extern_c_decls.first().unwrap();
            assert_eq!(extern_c_decl.symbol, Symbol::intern("convert_cpp_to_rust_type"));
            assert_rs_matches!(
                extern_c_decl.decl,
                quote! {
                    fn convert_cpp_to_rust_type(cpp_in: *const core::ffi::c_void,
                        rs_out: *mut core::ffi::c_void);
                }
            );

            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn __crubit_thunk_foo(_a: *const core::ffi::c_void) -> () {
                        let mut __crubit__a_uninit =
                            ::core::mem::MaybeUninit::<::rust_out::RustType>::uninit();
                        unsafe {
                            convert_cpp_to_rust_type(
                                _a,
                                __crubit__a_uninit.as_mut_ptr() as *mut core::ffi::c_void
                            );
                        }
                        ::rust_out::foo(unsafe { __crubit__a_uninit.assume_init() })
                    }
                }
            );

            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    void foo(cpp_ns::CppType _a);
                }
            );

            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void __crubit_thunk_foo(cpp_ns::CppType*);
                    }

                    inline void foo(cpp_ns::CppType _a) {
                        return __crubit_internal::__crubit_thunk_foo(&_a);
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_bridged_return_type_pointer_like() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(
                  cpp_type="CppType*",
                  cpp_type_include="cpp_ns/cpp_type.h",
                )]
                #[repr(transparent)]
                pub struct RustTypeOwned {
                    pub cpp_type: *mut core::ffi::c_void,
                }

                #[unsafe(no_mangle)]
                pub fn foo() -> RustTypeOwned { todo!() }
        "#;
        test_format_item(test_src, "RustTypeOwned", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Type bindings for RustTypeOwned suppressed \
                    due to being mapped to an existing C++ type (CppType*)"
            );
        });
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;

            assert_eq!(main_api.prereqs.includes.len(), 1);

            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn __crubit_thunk_foo(__ret_ptr: *mut *mut core::ffi::c_void) -> () {
                        let rs_val = ::rust_out::foo();
                        unsafe { __ret_ptr.write(::core::mem::transmute(rs_val)); }
                    }
                }
            );

            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    CppType* foo();
                }
            );

            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void __crubit_thunk_foo(CppType** __ret_ptr);
                    }

                    inline CppType* foo() {
                        union __crubit_return_union {
                            constexpr __crubit_return_union() {}
                            ~__crubit_return_union() { std::destroy_at(&this->val); }
                            CppType* val;
                        } __ret_val_holder;
                        __crubit_internal::__crubit_thunk_foo(&__ret_val_holder.val);
                        return std::move(__ret_val_holder.val);
                    }
                }
            );
        })
    }

    #[test]
    fn test_format_brided_type_deduplicate_extern_c_decls() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(
                  cpp_type="CppType*",
                  cpp_type_include="cpp_ns/cpp_type.h",
                  rust_to_cpp_converter="rust_struct_to_cpp_pointer",
                  cpp_to_rust_converter="cpp_pointer_to_rust_struct",
                )]
                pub struct RustType1 {
                    pub cpp_type: *const core::ffi::c_void,
                }

                #[__crubit::annotate(
                  cpp_type="CppType*",
                  cpp_type_include="cpp_ns/cpp_type.h",
                  rust_to_cpp_converter="rust_struct_to_cpp_pointer",
                  cpp_to_rust_converter="cpp_pointer_to_rust_struct",
                )]
                pub struct RustType2 {
                    pub cpp_type: *const core::ffi::c_void,
                }

                #[unsafe(no_mangle)]
                pub fn foo(_: RustType1, _: RustType2) { todo!() }
        "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();

            assert_eq!(result.rs_details.extern_c_decls.len(), 1);
            let extern_c_decl = result.rs_details.extern_c_decls.first().unwrap();
            assert_eq!(extern_c_decl.symbol, Symbol::intern("cpp_pointer_to_rust_struct"));
            assert_rs_matches!(
                extern_c_decl.decl,
                quote! {
                    fn cpp_pointer_to_rust_struct(cpp_in: *mut core::ffi::c_void,
                        rs_out: *mut core::ffi::c_void);
                }
            );
        });
    }

    #[test]
    fn test_format_bridged_return_type_by_pointer() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(
                  cpp_type="CppType*",
                  cpp_type_include="cpp_ns/cpp_type.h",
                  rust_to_cpp_converter="rust_struct_to_cpp_pointer",
                  cpp_to_rust_converter="cpp_pointer_to_rust_struct",
                )]
                pub struct RustTypeOwned {
                    pub cpp_type: *const core::ffi::c_void,
                }

                #[unsafe(no_mangle)]
                pub fn foo() -> RustTypeOwned { todo!() }
        "#;
        test_format_item(test_src, "RustTypeOwned", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Type bindings for RustTypeOwned suppressed \
                    due to being mapped to an existing C++ type (CppType*)"
            );
        });
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;

            assert_eq!(main_api.prereqs.includes.len(), 1);

            let extern_c_decl = result.rs_details.extern_c_decls.first().unwrap();
            assert_eq!(extern_c_decl.symbol, Symbol::intern("rust_struct_to_cpp_pointer"));
            assert_rs_matches!(
                extern_c_decl.decl,
                quote! {
                    fn rust_struct_to_cpp_pointer(
                        rs_in: *const core::ffi::c_void,
                        cpp_out: *mut *mut core::ffi::c_void
                    );
                }
            );

            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn __crubit_thunk_foo(__ret_ptr: *mut *mut core::ffi::c_void) -> () {
                        let rs_val = ::rust_out::foo();
                        unsafe {
                            rust_struct_to_cpp_pointer(
                                std::ptr::from_ref(&rs_val) as *const core::ffi::c_void,
                                __ret_ptr
                            );
                        }
                    }
                }
            );

            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    CppType* foo();
                }
            );

            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void __crubit_thunk_foo(CppType** __ret_ptr);
                    }

                    inline CppType* foo() {
                        union __crubit_return_union {
                            constexpr __crubit_return_union() {}
                            ~__crubit_return_union() { std::destroy_at(&this->val); }
                            CppType* val;
                        } __ret_val_holder;
                        __crubit_internal::__crubit_thunk_foo(&__ret_val_holder.val);
                        return std::move(__ret_val_holder.val);
                    }
                }
            );
        })
    }

    #[test]
    fn test_format_bridged_return_type_by_value() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(
                  cpp_type="cpp_ns::CppType",
                  cpp_type_include="cpp_ns/cpp_type.h",
                  rust_to_cpp_converter="rust_to_cpp_converter",
                  cpp_to_rust_converter="cpp_to_rust_converter",
                )]
                pub struct RustType {
                    pub x: i32,
                }

                #[unsafe(no_mangle)]
                pub fn foo() -> RustType {
                    RustType { x: 10 }
                }
        "#;
        test_format_item(test_src, "RustType", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Type bindings for RustType suppressed \
                    due to being mapped to an existing C++ type (cpp_ns::CppType)"
            );
        });
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;

            assert_eq!(main_api.prereqs.includes.len(), 1);
            assert_eq!(
                *main_api.prereqs.includes.first().unwrap(),
                CcInclude::user_header("cpp_ns/cpp_type.h".into())
            );

            let extern_c_decl = result.rs_details.extern_c_decls.first().unwrap();
            assert_eq!(extern_c_decl.symbol, Symbol::intern("rust_to_cpp_converter"));
            assert_rs_matches!(
                extern_c_decl.decl,
                quote! {
                    fn rust_to_cpp_converter(rs_in: *const core::ffi::c_void,
                        cpp_out: *mut core::ffi::c_void);
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn __crubit_thunk_foo(__ret_ptr: *mut core::ffi::c_void) -> () {
                        let rs_val = ::rust_out::foo();
                        unsafe {
                            rust_to_cpp_converter(
                                std::ptr::from_ref(&rs_val) as *const core::ffi::c_void,
                                __ret_ptr
                            );
                        }
                    }
                }
            );

            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    cpp_ns::CppType foo();
                }
            );

            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void __crubit_thunk_foo(cpp_ns::CppType* __ret_ptr);
                    }

                    inline cpp_ns::CppType foo() {
                        union __crubit_return_union {
                            constexpr __crubit_return_union() {}
                            ~__crubit_return_union() { std::destroy_at(&this->val); }
                            cpp_ns::CppType val;
                        } __ret_val_holder;
                        __crubit_internal::__crubit_thunk_foo(&__ret_val_holder.val);
                        return std::move(__ret_val_holder.val);
                    }
                }
            );
        })
    }

    #[test]
    fn test_bridged_type_unsupported() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(
                  cpp_type="cpp_ns::CppType",
                  cpp_type_include="cpp_ns/cpp_type.h",
                  rust_to_cpp_converter="convert_rust_to_cpp_type",
                  cpp_to_rust_converter="convert_cpp_to_rust_type",
                )]
                pub struct RustType {
                    pub x: i32,
                }

                #[unsafe(no_mangle)]
                pub fn unsupported_thunk_arg(_: fn() -> RustType) {}

                #[unsafe(no_mangle)]
                pub fn unsupported_return_ref() -> &'static RustType { todo!(); }

                #[unsafe(no_mangle)]
                pub fn unsupported_return_ptr() -> *const RustType { todo!(); }

                #[unsafe(no_mangle)]
                pub fn unsupported_accept_ref<'a>(_: &'a RustType) {}

                #[unsafe(no_mangle)]
                pub fn unsupported_accept_ptr(_: *const RustType) {}
        "#;

        test_format_item(test_src, "unsupported_thunk_arg", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error handling parameter #0: Function pointers can't have a thunk: Any calling \
                    convention other than `extern \"C\"` requires a thunk"
            );
        });

        test_format_item(test_src, "unsupported_return_ref", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Can't format reference type `&'static RustType` because the referent is a \
                bridged type. Passing bridged types by reference is not supported."
            );
        });

        test_format_item(test_src, "unsupported_return_ptr", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Can't format pointer type `*const RustType` because the pointee is a bridged \
                type. Passing bridged types by pointer is not supported."
            );
        });

        test_format_item(test_src, "unsupported_accept_ref", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Can't format reference type `&'a RustType` because the referent is a \
                bridged type. Passing bridged types by reference is not supported."
            );
        });

        test_format_item(test_src, "unsupported_accept_ptr", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Can't format pointer type `*const RustType` because the pointee is a bridged \
                type. Passing bridged types by pointer is not supported."
            );
        });
    }

    #[test]
    fn test_format_fn_cpp_name2() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[__crubit::annotate(cpp_type="cpp_ns::CppType")]
                pub struct RustType {
                    pub x: i32,
                }
            "#;
        test_format_item(test_src, "RustType", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Type bindings for RustType suppressed \
                    due to being mapped to an existing C++ type (cpp_ns::CppType)"
            );
        });
    }

    #[test]
    fn test_format_fn_cpp_name() {
        let test_src = r#"
                #![feature(register_tool)]
                #![register_tool(__crubit)]

                #[unsafe(no_mangle)]
                #[__crubit::annotate(cpp_name="Create")]
                pub fn foo() {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());

            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn __crubit_thunk_foo() -> () {
                         ::rust_out::foo()
                    }
                }
            );

            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    void Create();
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void __crubit_thunk_foo();
                    }
                    ...
                    inline void Create() {
                        return __crubit_internal::__crubit_thunk_foo();
                    }
                }
            );
        });
    }

    /// `test_format_item_fn_const` tests how bindings for an `const fn` are
    /// generated.
    ///
    /// Right now the `const` qualifier is ignored, but one can imagine that in
    /// the (very) long-term future such functions (including their bodies)
    /// could be translated into C++ `consteval` functions.
    #[test]
    fn test_format_item_fn_const() {
        let test_src = r#"
                pub const fn foo(i: i32) -> i32 { i * 42 }
            "#;
        test_format_item(test_src, "foo", |result| {
            // TODO(b/254095787): Update test expectations below once `const fn` from Rust
            // is translated into a `consteval` C++ function.
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t foo(std::int32_t i);
                }
            );
            assert!(!result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" std::int32_t ...( std::int32_t);
                    }
                    ...
                    inline std::int32_t foo(std::int32_t i) {
                        return __crubit_internal::...(i);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C"
                    fn ...(i: i32) -> i32 {
                        ::rust_out::foo(i)
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_c_unwind_abi() {
        // See also https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
        let test_src = r#"
                #![feature(c_unwind)]

                #[unsafe(no_mangle)]
                pub extern "C-unwind" fn may_throw() {}
            "#;
        test_format_item(test_src, "may_throw", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void may_throw();
                }
            );
        });
    }

    /// This test mainly verifies that `generate_item` correctly propagates
    /// `CcPrerequisites` of parameter types and return type.
    #[test]
    fn test_format_item_fn_cc_prerequisites_if_cpp_definition_needed() {
        let test_src = r#"
                #![allow(dead_code)]

                pub fn foo(_i: i32) -> S { panic!("foo") }
                pub struct S(i32);
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;

            // Minimal coverage, just to double-check that the test setup works.
            //
            // Note that this is a definition, and therefore `S` should be defined
            // earlier (not just forward declared).
            assert_cc_matches!(main_api.tokens, quote! { S foo(std::int32_t _i);});
            assert_cc_matches!(result.cc_details.tokens, quote! { S foo(std::int32_t _i) { ... }});

            // Main checks: `CcPrerequisites::includes`.
            assert_cc_matches!(
                format_cc_includes(&main_api.prereqs.includes),
                quote! { include <cstdint> }
            );
            assert_cc_matches!(
                format_cc_includes(&result.cc_details.prereqs.includes),
                quote! { include <cstdint> }
            );

            // Main checks: `CcPrerequisites::defs` and `CcPrerequisites::fwd_decls`.
            //
            // Verifying the actual def_id is tricky, because `test_format_item` doesn't
            // expose `tcx` to the verification function (and therefore calling
            // `find_def_id_by_name` is not easily possible).
            //
            // Note that `main_api` and `impl_details` have different expectations.
            assert_eq!(0, main_api.prereqs.defs.len());
            assert_eq!(1, main_api.prereqs.fwd_decls.len());
            assert_eq!(1, result.cc_details.prereqs.defs.len());
            assert_eq!(0, result.cc_details.prereqs.fwd_decls.len());
        });
    }

    /// This test verifies that `generate_item` uses
    /// `CcPrerequisites::fwd_decls` rather than `CcPrerequisites::defs` for
    /// function declarations in the `main_api`.
    #[test]
    fn test_format_item_fn_cc_prerequisites_if_only_cpp_declaration_needed() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn foo(s: S) -> bool { s.0 }

                #[repr(C)]
                pub struct S(bool);
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;

            // Minimal coverage, just to double-check that the test setup works.
            //
            // Note that this is only a function *declaration* (not a function definition -
            // there is no function body), and therefore `S` just needs to be
            // forward-declared earlier.
            assert_cc_matches!(main_api.tokens, quote! { bool foo(::rust_out::S s); });

            // Main checks: `CcPrerequisites::defs` and `CcPrerequisites::fwd_decls`.
            //
            // Verifying the actual def_id is tricky, because `test_format_item` doesn't
            // expose `tcx` to the verification function (and therefore calling
            // `find_def_id_by_name` is not easily possible).
            assert_eq!(0, main_api.prereqs.defs.len());
            assert_eq!(1, main_api.prereqs.fwd_decls.len());
        });
    }

    #[test]
    fn test_format_item_fn_with_type_aliased_return_type() {
        // Type aliases disappear at the `rustc_middle::ty::Ty` level and therefore in
        // the short-term the generated bindings also ignore type aliases.
        //
        // TODO(b/254096006): Consider preserving `type` aliases when generating
        // bindings.
        let test_src = r#"
                type MyTypeAlias = f64;

                #[unsafe(no_mangle)]
                pub extern "C" fn type_aliased_return() -> MyTypeAlias { 42.0 }
            "#;
        test_format_item(test_src, "type_aliased_return", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" double type_aliased_return();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_doc_comment_with_unmangled_name() {
        let test_src = r#"
            /// Outer line doc.
            /** Outer block doc that spans lines.
             */
            #[doc = "Doc comment via doc attribute."]
            #[unsafe(no_mangle)]
            pub extern "C" fn fn_with_doc_comment_with_unmangled_name() {}
          "#;
        test_format_item(test_src, "fn_with_doc_comment_with_unmangled_name", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            let doc_comments = [
                " Outer line doc.",
                "",
                " Outer block doc that spans lines.",
                "             ",
                "",
                "Doc comment via doc attribute.",
                "",
                "Generated from: <crubit_unittests.rs>;l=7",
            ]
            .join("\n");
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #doc_comments
                    extern "C" void fn_with_doc_comment_with_unmangled_name();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_inner_doc_comment_with_unmangled_name() {
        let test_src = r#"
            /// Outer doc comment.
            #[unsafe(no_mangle)]
            pub extern "C" fn fn_with_inner_doc_comment_with_unmangled_name() {
                //! Inner doc comment.
            }
          "#;
        test_format_item(test_src, "fn_with_inner_doc_comment_with_unmangled_name", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            let doc_comments = [
                " Outer doc comment.",
                " Inner doc comment.",
                "Generated from: <crubit_unittests.rs>;l=4",
            ]
            .join("\n\n");
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #doc_comments
                    extern "C" void fn_with_inner_doc_comment_with_unmangled_name();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_doc_comment_with_mangled_name() {
        let test_src = r#"
                /// Doc comment of a function with mangled name.
                pub extern "C" fn fn_with_doc_comment_with_mangled_name() {}
            "#;
        test_format_item(test_src, "fn_with_doc_comment_with_mangled_name", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            let comment = " Doc comment of a function with mangled name.\n\n\
                           Generated from: <crubit_unittests.rs>;l=3";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    void fn_with_doc_comment_with_mangled_name();
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_name_is_reserved_cpp_keyword() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn reinterpret_cast() -> () {}
            "#;
        test_format_item(test_src, "reinterpret_cast", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void reinterpret_cast_();
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_ret_type() {
        let test_src = r#"
                pub fn foo() -> (i32, i32) { (123, 456) }
            "#;
        test_format_item(test_src, "foo", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error formatting function return type: \
                       Tuples are not supported yet: (i32, i32) (b/254099023)"
            );
        });
    }

    /// This test verifies handling of inferred, anonymous lifetimes.
    ///
    /// Note that `Region::get_name_or_anon()` may return the same name (e.g.
    /// `"anon"` for both lifetimes, but bindings should use 2 distinct
    /// lifetime names in the generated bindings and in the thunk impl.
    #[test]
    fn test_format_item_lifetime_generic_fn_with_inferred_lifetimes() {
        let test_src = r#"
                pub fn foo(arg: &i32) -> &i32 {
                    unimplemented!("arg = {arg}")
                }
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]]
                    foo(std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]] arg);
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C"
                    std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]] ...(
                        std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]]);
                    }
                    inline
                    std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]]
                    foo(std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]] arg) {
                      return __crubit_internal::...(arg);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn ...<'__anon1>(arg: &'__anon1 i32) -> &'__anon1 i32 {
                        ::rust_out::foo(arg)
                    }
                }
            );
        });
    }

    /// This test verifies handling of various explicit (i.e. non-inferred)
    /// lifetimes.
    ///
    /// * Note that the two `'_` specify two distinct lifetimes (i.e. two
    ///   distinct names need to be used in the generated bindings and thunk
    ///   impl).
    /// * Note that `'static` doesn't need to be listed in the generic
    ///   parameters of the thunk impl
    /// * Note that even though `'foo` is used in 2 parameter types, it should
    ///   only appear once in the list of generic parameters of the thunk impl
    /// * Note that in the future the following translation may be preferable:
    ///     * `'a` => `$a` (no parens)
    ///     * `'foo` => `$(foo)` (note the extra parens)
    #[test]
    fn test_format_item_lifetime_generic_fn_with_various_lifetimes() {
        let test_src = r#"
                pub fn foo<'a, 'foo>(
                    arg1: &'a i32,  // Single letter lifetime = `$a` is possible
                    arg2: &'foo i32,  // Multi-character lifetime
                    arg3: &'foo i32,  // Same lifetime used for 2 places
                    arg4: &'static i32,
                    arg5: &'_ i32,
                    arg6: &'_ i32,
                ) -> &'foo i32 {
                    unimplemented!("args: {arg1}, {arg2}, {arg3}, {arg4}, {arg5}, {arg6}")
                }
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                  std::int32_t const& [[clang::annotate_type("lifetime", "foo")]]
                  foo(
                    std::int32_t const& [[clang::annotate_type("lifetime", "a")]] arg1,
                    std::int32_t const& [[clang::annotate_type("lifetime", "foo")]] arg2,
                    std::int32_t const& [[clang::annotate_type("lifetime", "foo")]] arg3,
                    std::int32_t const& [[clang::annotate_type("lifetime", "static")]] arg4,
                    std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]] arg5,
                    std::int32_t const& [[clang::annotate_type("lifetime", "__anon2")]] arg6);
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C"
                    std::int32_t const& [[clang::annotate_type("lifetime", "foo")]]
                    ...(
                        std::int32_t const& [[clang::annotate_type("lifetime", "a")]],
                        std::int32_t const& [[clang::annotate_type("lifetime", "foo")]],
                        std::int32_t const& [[clang::annotate_type("lifetime", "foo")]],
                        std::int32_t const& [[clang::annotate_type("lifetime", "static")]],
                        std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]],
                        std::int32_t const& [[clang::annotate_type("lifetime", "__anon2")]]);
                    }
                    inline
                    std::int32_t const& [[clang::annotate_type("lifetime", "foo")]]
                    foo(
                        std::int32_t const& [[clang::annotate_type("lifetime", "a")]] arg1,
                        std::int32_t const& [[clang::annotate_type("lifetime", "foo")]] arg2,
                        std::int32_t const& [[clang::annotate_type("lifetime", "foo")]] arg3,
                        std::int32_t const& [[clang::annotate_type("lifetime", "static")]] arg4,
                        std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]] arg5,
                        std::int32_t const& [[clang::annotate_type("lifetime", "__anon2")]] arg6) {
                      return __crubit_internal::...(arg1, arg2, arg3, arg4, arg5, arg6);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn ...<'a, 'foo, '__anon1, '__anon2>(
                        arg1: &'a i32,
                        arg2: &'foo i32,
                        arg3: &'foo i32,
                        arg4: &'static i32,
                        arg5: &'__anon1 i32,
                        arg6: &'__anon2 i32
                    ) -> &'foo i32 {
                        ::rust_out::foo(arg1, arg2, arg3, arg4, arg5, arg6)
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_slice() {
        let test_src = r#"
                pub fn foo(_a: *const [u32], _b: *const [u8], _c: *mut [i16], _d: *mut [bool]) { todo!() }
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                  void
                  foo(
                    rs_std::SliceRef<const std::uint32_t> _a,
                    rs_std::SliceRef<const std::uint8_t> _b,
                    rs_std::SliceRef<std::int16_t> _c,
                    rs_std::SliceRef<bool> _d
                  );
                }
            );
        });
    }

    /// Test of lifetime-generic function with a `where` clause.
    ///
    /// The `where` constraint below is a bit silly (why not just use `'static`
    /// directly), but it seems prudent to test and confirm that we disable
    /// generation of bindings for generic functions with `where` clauses
    /// (because it is unclear if such constraints can be replicated
    /// in C++).
    #[test]
    fn test_format_item_lifetime_generic_fn_with_where_clause() {
        let test_src = r#"
                pub fn foo<'a>(arg: &'a i32) where 'a : 'static {
                    unimplemented!("{arg}")
                }
            "#;
        test_format_item(test_src, "foo", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic functions are not supported yet (b/259749023)");
        });
    }

    #[test]
    fn test_format_item_unsupported_type_generic_fn() {
        let test_src = r#"
                use std::fmt::Display;
                pub fn generic_function<T: Default + Display>() {
                    println!("{}", T::default());
                }
            "#;
        test_format_item(test_src, "generic_function", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic functions are not supported yet (b/259749023)");
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_async() {
        let test_src = r#"
                pub async fn async_function() {}
            "#;
        test_format_item(test_src, "async_function", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error formatting function return type: \
                             The following Rust type is not supported yet: \
                             impl std::future::Future<Output = ()>"
            );
        });
    }

    #[test]
    fn test_format_item_fn_rust_abi() {
        let test_src = r#"
                pub fn add(x: f64, y: f64) -> f64 { x * y }
            "#;
        test_format_item(test_src, "add", |result| {
            // TODO(b/261074843): Re-add thunk name verification once we are using stable
            // name mangling (which may be coming in Q1 2023).  (This might mean
            // reverting cl/492333432 + manual review and tweaks.)
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    double add(double x, double y);
                }
            );
            assert!(result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double, double);
                    }
                    ...
                    inline double add(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C"
                    fn ...(x: f64, y: f64) -> f64 {
                        ::rust_out::add(x, y)
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_rust_abi_with_param_taking_struct_by_value22() {
        let test_src = r#"
                use std::slice;
                pub struct S(i32);
                pub unsafe fn transmute_slice(
                    slice_ptr: *const u8,
                    slice_len: usize,
                    element_size: usize,
                    s: S,
                ) -> i32 {
                    let len_in_bytes = slice_len * element_size;
                    let b = slice::from_raw_parts(slice_ptr as *const u8, len_in_bytes);
                    if b.len() == len_in_bytes {
                        s.0
                    } else {
                        0
                    }
                }
            "#;
        test_format_item(test_src, "transmute_slice", |result| {
            let result = result.unwrap().unwrap();
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C"
                    fn ...(...) -> i32 {
                        unsafe {
                            ::rust_out::transmute_slice(..., ..., ..., s.assume_init_read() )
                        }
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_rust_abi_with_param_taking_struct_by_value() {
        let test_src = r#"
                pub struct S(i32);
                pub fn into_i32(s: S) -> i32 { s.0 }
            "#;
        test_format_item(test_src, "into_i32", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t into_i32(::rust_out::S s);
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" std::int32_t ...(::rust_out::S*);
                    }
                    ...
                    inline std::int32_t into_i32(::rust_out::S s) {
                        return __crubit_internal::...(&s);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C"
                    fn ...(s: &mut ::core::mem::MaybeUninit<::rust_out::S>) -> i32 {
                        ::rust_out::into_i32(unsafe { s.assume_init_read() })
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_rust_abi_returning_struct_by_value() {
        let test_src = r#"
                #![allow(dead_code)]

                pub struct S(i32);
                pub fn create(i: i32) -> S { S(i) }
            "#;
        test_format_item(test_src, "create", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ::rust_out::S create(std::int32_t i);
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void ...(std::int32_t, ::rust_out::S* __ret_ptr);
                    }
                    ...
                    inline ::rust_out::S create(std::int32_t i) {
                        crubit::Slot<::rust_out::S> __ret_slot;
                        __crubit_internal::...(i, __ret_slot.Get());
                        return std::move(__ret_slot).AssumeInitAndTakeValue();
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C"
                    fn ...(
                        i: i32,
                        __ret_slot: &mut ::core::mem::MaybeUninit<::rust_out::S>
                    ) -> () {
                        __ret_slot.write({ ::rust_out::create(i) });
                    }
                }
            );
        });
    }

    /// `test_format_item_fn_rust_abi` tests a function call that is not a
    /// C-ABI, and is not the default Rust ABI.  It can't use `"stdcall"`,
    /// because it is not supported on the targets where Crubit's tests run.
    /// So, it ended up using `"vectorcall"`.
    ///
    /// This test almost entirely replicates `test_format_item_fn_rust_abi`,
    /// except for the `extern "vectorcall"` part in the `test_src` test
    /// input.
    ///
    /// This test verifies the current behavior that gives reasonable and
    /// functional FFI bindings.  OTOH, in the future we may decide to avoid
    /// having the extra thunk for cases where the given non-C-ABI function
    /// call convention is supported by both C++ and Rust
    /// (see also `format_cc_call_conv_as_clang_attribute` in
    /// `rs_bindings_from_cc/src_code_gen.rs`)
    #[test]
    fn test_format_item_fn_vectorcall_abi() {
        let test_src = r#"
                #![feature(abi_vectorcall)]
                pub extern "vectorcall" fn add(x: f64, y: f64) -> f64 { x * y }
            "#;
        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    double add(double x, double y);
                }
            );
            assert!(result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double, double);
                    }
                    ...
                    inline double add(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C"
                    fn ...(x: f64, y: f64) -> f64 {
                        ::rust_out::add(x, y)
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_variadic() {
        let test_src = r#"
                #![feature(c_variadic)]

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn variadic_function(_fmt: *const u8, ...) {}
            "#;
        test_format_item(test_src, "variadic_function", |result| {
            // TODO(b/254097223): Add support for variadic functions.
            let err = result.unwrap_err();
            assert_eq!(err, "C variadic functions are not supported (b/254097223)");
        });
    }

    #[test]
    fn test_format_item_fn_params() {
        let test_src = r#"
                #[allow(unused_variables)]
                #[unsafe(no_mangle)]
                pub extern "C" fn foo(b: bool, f: f64) {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    extern "C" void foo(bool b, double f);
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_param_name_reserved_keyword() {
        let test_src = r#"
                #[allow(unused_variables)]
                #[unsafe(no_mangle)]
                pub extern "C" fn some_function(reinterpret_cast: f64) {}
            "#;
        test_format_item(test_src, "some_function", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    extern "C" void some_function(double reinterpret_cast_);
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_multiple_anonymous_parameter_names() {
        let test_src = r#"
                pub fn foo(_: f64, _: f64) {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    void foo(double __param_0, double __param_1);
                }
            );
            assert!(result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void ...(double, double);
                    }
                    ...
                    inline void foo(double __param_0, double __param_1) {
                        return __crubit_internal::...(__param_0, __param_1);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn ...(__param_0: f64, __param_1: f64) -> () {
                        ::rust_out::foo(__param_0, __param_1)
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_destructuring_parameter_name() {
        let test_src = r#"
                pub struct S {
                    pub f1: i32,
                    pub f2: i32,
                }

                // This test mostly focuses on the weird parameter "name" below.
                // See also
                // https://doc.rust-lang.org/reference/items/functions.html#function-parameters
                // which points out that function parameters are just irrefutable patterns.
                pub fn func(S{f1, f2}: S) -> i32 { f1 + f2 }
            "#;
        test_format_item(test_src, "func", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t func(::rust_out::S __param_0);
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" std::int32_t ...(::rust_out::S*);
                    }
                    ...
                    inline std::int32_t func(::rust_out::S __param_0) {
                        return __crubit_internal::...(&__param_0);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn ...(
                        __param_0: &mut ::core::mem::MaybeUninit<::rust_out::S>
                    ) -> i32 {
                        ::rust_out::func(unsafe {__param_0.assume_init_read() })
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_param_type() {
        let test_src = r#"
                pub fn foo(_param: (i32, i32)) {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error handling parameter #0: \
                             Tuples are not supported yet: (i32, i32) (b/254099023)"
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_param_type_unit() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn fn_with_params(_param: ()) {}
            "#;
        test_format_item(test_src, "fn_with_params", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error handling parameter #0: \
                             `()` / `void` is only supported as a return type (b/254507801)"
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_param_type_never() {
        let test_src = r#"
                #![feature(never_type)]

                #[unsafe(no_mangle)]
                pub extern "C" fn fn_with_params(_param: !) {}
            "#;
        test_format_item(test_src, "fn_with_params", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error handling parameter #0: \
                 The never type `!` is only supported as a return type (b/254507801)"
            );
        });
    }

    #[test]
    fn test_format_item_static_method() {
        let test_src = r#"
                #![allow(dead_code)]

                /// No-op `f32` placeholder is used, because ZSTs are not supported
                /// (b/258259459).
                pub struct Math(f32);

                impl Math {
                    pub fn add_i32(x: f32, y: f32) -> f32 {
                        x + y
                    }
                }
            "#;
        test_format_item(test_src, "Math", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... Math final {
                        ...
                        public:
                          ...
                          static float add_i32(float x, float y);
                        ...
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" float ... (float, float);
                    }
                    inline float Math::add_i32(float x, float y) {
                      return __crubit_internal::...(x, y);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn ...(x: f32, y: f32) -> f32 {
                        ::rust_out::Math::add_i32(x, y)
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_static_method_with_generic_type_parameters() {
        let test_src = r#"
                #![allow(dead_code)]

                /// No-op `f32` placeholder is used, because ZSTs are not supported
                /// (b/258259459).
                pub struct SomeStruct(f32);

                impl SomeStruct {
                    // To make this testcase distinct / non-overlapping wrt
                    // test_format_item_static_method_with_generic_lifetime_parameters
                    // `t` is taken by value below.
                    pub fn generic_method<T: Clone>(t: T) -> T {
                        t.clone()
                    }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let unsupported_msg = "Error generating bindings for `SomeStruct::generic_method` \
                                   defined at <crubit_unittests.rs>;l=12: \
                                   Generic functions are not supported yet (b/259749023)";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        __COMMENT__ #unsupported_msg
                        ...
                    };
                    ...
                }
            );
            assert_cc_not_matches!(result.cc_details.tokens, quote! { SomeStruct::generic_method },);
            assert_rs_not_matches!(result.rs_details.tokens, quote! { generic_method },);
        });
    }

    #[test]
    fn test_format_item_static_method_with_generic_lifetime_parameters_at_fn_level() {
        let test_src = r#"
                #![allow(dead_code)]

                /// No-op `f32` placeholder is used, because ZSTs are not supported
                /// (b/258259459).
                pub struct SomeStruct(f32);

                impl SomeStruct {
                    pub fn fn_taking_reference<'a>(x: &'a i32) -> i32 { *x }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        static std::int32_t fn_taking_reference(
                            std::int32_t const& [[clang::annotate_type("lifetime", "a")]] x);
                        ...
                    };
                    ...
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C" std::int32_t ...(
                        std::int32_t const& [[clang::annotate_type("lifetime", "a")]]);
                    }
                    inline std::int32_t SomeStruct::fn_taking_reference(
                        std::int32_t const& [[clang::annotate_type("lifetime", "a")]] x) {
                      return __crubit_internal::...(x);
                    }
                },
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn ...<'a>(x: &'a i32) -> i32 {
                        ::rust_out::SomeStruct::fn_taking_reference(x)
                    }
                },
            );
        });
    }

    #[test]
    fn test_format_item_static_method_with_generic_lifetime_parameters_at_impl_level() {
        let test_src = r#"
                #![allow(dead_code)]

                /// No-op `f32` placeholder is used, because ZSTs are not supported
                /// (b/258259459).
                pub struct SomeStruct(f32);

                impl<'a> SomeStruct {
                    pub fn fn_taking_reference(x: &'a i32) -> i32 { *x }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let unsupported_msg =
                "Error generating bindings for `SomeStruct::fn_taking_reference` \
                                   defined at <crubit_unittests.rs>;l=9: \
                                   Generic functions are not supported yet (b/259749023)";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        __COMMENT__ #unsupported_msg
                        ...
                    };
                    ...
                }
            );
            assert_cc_not_matches!(
                result.cc_details.tokens,
                quote! { SomeStruct::fn_taking_reference },
            );
            assert_rs_not_matches!(result.rs_details.tokens, quote! { fn_taking_reference },);
        });
    }

    fn test_format_item_method_taking_self_by_value(test_src: &str) {
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        float into_f32() &&;
                        ...
                    };
                    ...
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C" float ...(::rust_out::SomeStruct*);
                    }
                    inline float SomeStruct::into_f32() && {
                      return __crubit_internal::...(this);
                    }
                },
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    ...
                    #[unsafe(no_mangle)]
                    extern "C" fn ...(__self: &mut ::core::mem::MaybeUninit<::rust_out::SomeStruct>) -> f32 {
                        ::rust_out::SomeStruct::into_f32(unsafe { __self.assume_init_read() })
                    }
                    ...
                },
            );
        });
    }

    #[test]
    fn test_format_item_method_taking_self_by_value_implicit_type() {
        let test_src = r#"
                pub struct SomeStruct(pub f32);

                impl SomeStruct {
                    pub fn into_f32(self) -> f32 {
                        self.0
                    }
                }
            "#;
        test_format_item_method_taking_self_by_value(test_src);
    }

    /// One difference from
    /// `test_format_item_method_taking_self_by_value_implicit_type` is that
    /// `fn_sig.decl.implicit_self` is `ImplicitSelfKind::None` here (vs
    /// `ImplicitSelfKind::Imm` in the other test).
    #[test]
    fn test_format_item_method_taking_self_by_value_explicit_type() {
        let test_src = r#"
                pub struct SomeStruct(pub f32);

                impl SomeStruct {
                    pub fn into_f32(self: SomeStruct) -> f32 {
                        self.0
                    }
                }
            "#;
        test_format_item_method_taking_self_by_value(test_src);
    }

    fn test_format_item_method_taking_self_by_const_ref(test_src: &str) {
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        float get_f32() const [[clang::annotate_type("lifetime", "__anon1")]];
                        ...
                    };
                    ...
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C" float ...(
                        ::rust_out::SomeStruct const& [[clang::annotate_type("lifetime",
                                                                             "__anon1")]]);
                    }
                    inline float SomeStruct::get_f32()
                        const [[clang::annotate_type("lifetime", "__anon1")]] {
                      return __crubit_internal::...(*this);
                    }
                },
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn ...<'__anon1>(__self: &'__anon1 ::rust_out::SomeStruct) -> f32 {
                        ::rust_out::SomeStruct::get_f32(__self)
                    }
                    ...
                },
            );
        });
    }

    #[test]
    fn test_format_item_method_taking_self_by_const_ref_implicit_type() {
        let test_src = r#"
                pub struct SomeStruct(pub f32);

                impl SomeStruct {
                    pub fn get_f32(&self) -> f32 {
                        self.0
                    }
                }
            "#;
        test_format_item_method_taking_self_by_const_ref(test_src);
    }

    #[test]
    fn test_format_item_method_taking_self_by_const_ref_explicit_type() {
        let test_src = r#"
                pub struct SomeStruct(pub f32);

                impl SomeStruct {
                    pub fn get_f32(self: &SomeStruct) -> f32 {
                        self.0
                    }
                }
            "#;
        test_format_item_method_taking_self_by_const_ref(test_src);
    }

    fn test_format_item_method_taking_self_by_mutable_ref(test_src: &str) {
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        void set_f32(float new_value)
                            [[clang::annotate_type("lifetime", "__anon1")]];
                        ...
                    };
                    ...
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C" void ...(
                        ::rust_out::SomeStruct& [[clang::annotate_type("lifetime", "__anon1")]],
                        float);
                    }
                    inline void SomeStruct::set_f32(float new_value)
                            [[clang::annotate_type("lifetime", "__anon1")]] {
                      return __crubit_internal::...(*this, new_value);
                    }
                },
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn ...<'__anon1>(
                        __self: &'__anon1 mut ::rust_out::SomeStruct,
                        new_value: f32
                    ) -> () {
                        ::rust_out::SomeStruct::set_f32(__self, new_value)
                    }
                    ...
                },
            );
        });
    }

    #[test]
    fn test_format_item_method_taking_self_by_mutable_ref_implicit_type() {
        let test_src = r#"
                pub struct SomeStruct(pub f32);

                impl SomeStruct {
                    pub fn set_f32(&mut self, new_value: f32) {
                        self.0 = new_value;
                    }
                }
            "#;
        test_format_item_method_taking_self_by_mutable_ref(test_src);
    }

    #[test]
    fn test_format_item_method_taking_self_by_mutable_ref_explicit_type() {
        let test_src = r#"
                pub struct SomeStruct(pub f32);

                impl SomeStruct {
                    pub fn set_f32(self: &mut SomeStruct, new_value: f32) {
                        self.0 = new_value;
                    }
                }
            "#;
        test_format_item_method_taking_self_by_mutable_ref(test_src);
    }

    #[test]
    fn test_format_item_method_taking_self_by_arc() {
        let test_src = r#"
                use std::sync::Arc;

                pub struct SomeStruct(pub f32);

                impl SomeStruct {
                    pub fn get_f32(self: Arc<Self>) -> f32 {
                        self.0
                    }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let unsupported_msg = "Error generating bindings for `SomeStruct::get_f32` \
                                   defined at <crubit_unittests.rs>;l=7: \
                                   Error handling parameter #0: \
                                   Generic types are not supported yet (b/259749095)";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        __COMMENT__ #unsupported_msg
                        ...
                    };
                    ...
                }
            );
            assert_cc_not_matches!(result.cc_details.tokens, quote! { SomeStruct::get_f32 },);
            assert_rs_not_matches!(result.rs_details.tokens, quote! { get_f32 },);
        });
    }

    #[test]
    fn test_format_item_method_taking_self_by_pinned_mut_ref() {
        let test_src = r#"
                use core::pin::Pin;

                pub struct SomeStruct(f32);

                impl SomeStruct {
                    pub fn set_f32(mut self: Pin<&mut Self>, f: f32) {
                        self.0 = f;
                    }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let unsupported_msg = "Error generating bindings for `SomeStruct::set_f32` \
                                   defined at <crubit_unittests.rs>;l=7: \
                                   Error handling parameter #0: \
                                   Generic types are not supported yet (b/259749095)";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        __COMMENT__ #unsupported_msg
                        ...
                    };
                    ...
                }
            );
            assert_cc_not_matches!(result.cc_details.tokens, quote! { SomeStruct::set_f32 },);
            assert_rs_not_matches!(result.rs_details.tokens, quote! { set_f32 },);
        });
    }

    #[test]
    fn test_format_item_struct_with_default_constructor() {
        let test_src = r#"
                #![allow(dead_code)]

                #[derive(Default)]
                pub struct Point(i32, i32);
            "#;
        test_format_item(test_src, "Point", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... Point final {
                        ...
                        public:
                          __COMMENT__ "Default::default"
                          Point();
                        ...
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void ...(::rust_out::Point* __ret_ptr);
                    }
                    inline Point::Point() {
                        ...(this);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                   #[unsafe(no_mangle)]
                   extern "C" fn ...(
                       __ret_slot: &mut ::core::mem::MaybeUninit<::rust_out::Point>
                   ) -> () {
                       __ret_slot.write({ <::rust_out::Point as ::core::default::Default>::default() });
                   }
                }
            );
        });
    }

    #[test]
    fn test_format_item_struct_with_copy_trait() {
        let test_src = r#"
                #![allow(dead_code)]

                #[derive(Clone, Copy)]
                pub struct Point(i32, i32);
            "#;
        let msg = "Rust types that are `Copy` get trivial, `default` C++ copy constructor \
                   and assignment operator.";
        test_format_item(test_src, "Point", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... Point final {
                        ...
                        public:
                          ...
                          __COMMENT__ #msg
                          Point(const Point&) = default;
                          Point& operator=(const Point&) = default;
                          ...
                    };
                }
            );

            // Trivial copy doesn't require any C++ details except `static_assert`s.
            assert_cc_not_matches!(result.cc_details.tokens, quote! { Point::Point(const Point&) },);
            assert_cc_not_matches!(
                result.cc_details.tokens,
                quote! { Point::operator=(const Point&) },
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    static_assert(std::is_trivially_copy_constructible_v<Point>);
                    static_assert(std::is_trivially_copy_assignable_v<Point>);
                },
            );

            // Trivial copy doesn't require any Rust details.
            assert_rs_not_matches!(result.rs_details.tokens, quote! { Copy });
            assert_rs_not_matches!(result.rs_details.tokens, quote! { copy });
        });
    }

    /// Test of `generate_copy_ctor_and_assignment_operator` when the ADT
    /// implements a `Clone` trait.
    ///
    /// Notes:
    /// * `Copy` trait is covered in `test_format_item_struct_with_copy_trait`.
    /// * The test below implements `clone` and uses the default `clone_from`.
    #[test]
    fn test_format_item_struct_with_clone_trait() {
        let test_src = r#"
                #![allow(dead_code)]

                pub struct Point(i32, i32);
                impl Clone for Point {
                    fn clone(&self) -> Self {
                        unimplemented!()
                    }
                }
            "#;
        test_format_item(test_src, "Point", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... Point final {
                        ...
                        public:
                          ...
                          __COMMENT__ "Clone::clone"
                          Point(const Point&);

                          __COMMENT__ "Clone::clone_from"
                          Point& operator=(const Point&);
                        ...
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C" void ...(
                        ::rust_out::Point const& [[clang::annotate_type("lifetime",
                                                                        "__anon1")]],
                        ::rust_out::Point* __ret_ptr);
                    }
                    namespace __crubit_internal {
                    extern "C" void ...(
                        ::rust_out::Point& [[clang::annotate_type("lifetime", "__anon1")]],
                        ::rust_out::Point const& [[clang::annotate_type("lifetime",
                                                                        "__anon2")]]);
                    }
                    inline Point::Point(const Point& other) {
                      __crubit_internal::...(other, this);
                    }
                    inline Point& Point::operator=(const Point& other) {
                      if (this != &other) {
                        __crubit_internal::...(*this, other);
                      }
                      return *this;
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn ...<'__anon1>(
                        __self: &'__anon1 ::rust_out::Point,
                        __ret_slot: &mut ::core::mem::MaybeUninit<::rust_out::Point>
                    ) -> () {
                        __ret_slot.write(
                            { <::rust_out::Point as ::core::clone::Clone>::clone(__self) }
                        );
                    }
                    #[unsafe(no_mangle)]
                    extern "C" fn ...<'__anon1, '__anon2>(
                        __self: &'__anon1 mut ::rust_out::Point,
                        source: &'__anon2 ::rust_out::Point
                    ) -> () {
                        <::rust_out::Point as ::core::clone::Clone>::clone_from(__self, source)
                    }
                }
            );
        });
    }

    fn test_format_item_struct_with_custom_drop_and_no_default_nor_clone_impl(
        test_src: &str,
        pass_by_value_line_number: i32,
    ) {
        test_format_item(test_src, "TypeUnderTest", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let move_deleted_msg = "C++ moves are deleted \
                                    because there's no non-destructive implementation available.";
            let pass_by_value_msg = format!(
                "Error generating bindings for `TypeUnderTest::pass_by_value` \
                        defined at <crubit_unittests.rs>;l={pass_by_value_line_number}: \
                 Can't pass the return type by value without a move constructor"
            );
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... TypeUnderTest final {
                        ...
                        public:
                          ...
                          __COMMENT__ "Drop::drop"
                          ~TypeUnderTest();

                          __COMMENT__ #move_deleted_msg
                          TypeUnderTest(TypeUnderTest&&) = delete;
                          TypeUnderTest& operator=(TypeUnderTest&&) = delete;
                          ...
                          __COMMENT__ #pass_by_value_msg
                          ...
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C" void ...(  // `drop` thunk decl
                        ::rust_out::TypeUnderTest& [[clang::annotate_type(
                            "lifetime", "__anon1")]]);
                    }
                    inline TypeUnderTest::~TypeUnderTest() {
                      __crubit_internal::...(*this);
                    }
                }
            );
            assert_cc_not_matches!(result.cc_details.tokens, quote! { pass_by_value });
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    ...
                    #[unsafe(no_mangle)]
                    extern "C" fn ...(
                        __self: &mut ::core::mem::MaybeUninit<::rust_out::TypeUnderTest>
                    ) {
                        unsafe { __self.assume_init_drop() };
                    }
                    ...
                }
            );
            assert_rs_not_matches!(result.rs_details.tokens, quote! { pass_by_value });
        });
    }

    #[test]
    fn test_format_item_struct_with_custom_drop_impl_and_no_default_nor_clone_impl() {
        let test_src = r#"
                pub struct TypeUnderTest {
                    pub x: i32,
                    pub y: i32,
                }

                impl Drop for TypeUnderTest {
                    fn drop(&mut self) {}
                }

                impl TypeUnderTest {
                    pub fn pass_by_value() -> Self { unimplemented!() }
                }
            "#;
        let pass_by_value_line_number = 12;
        test_format_item_struct_with_custom_drop_and_no_default_nor_clone_impl(
            test_src,
            pass_by_value_line_number,
        );
    }

    #[test]
    fn test_format_item_struct_with_custom_drop_glue_and_no_default_nor_clone_impl() {
        let test_src = r#"
                #![allow(dead_code)]

                // `i32` is present to avoid hitting the ZST checks related to (b/258259459)
                struct StructWithCustomDropImpl(i32);

                impl Drop for StructWithCustomDropImpl {
                    fn drop(&mut self) {
                        println!("dropping!");
                    }
                }

                pub struct TypeUnderTest {
                    field: StructWithCustomDropImpl,
                }

                impl TypeUnderTest {
                    pub fn pass_by_value() -> Self { unimplemented!() }
                }
            "#;
        let pass_by_value_line_number = 18;
        test_format_item_struct_with_custom_drop_and_no_default_nor_clone_impl(
            test_src,
            pass_by_value_line_number,
        );
    }

    fn test_format_item_struct_with_custom_drop_and_with_default_impl(test_src: &str) {
        test_format_item(test_src, "TypeUnderTest", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... TypeUnderTest final {
                        ...
                        public:
                          ...
                          __COMMENT__ "Drop::drop"
                          ~TypeUnderTest();
                          TypeUnderTest(TypeUnderTest&&);
                          TypeUnderTest& operator=(
                              TypeUnderTest&&);
                          ...
                          static ::rust_out::TypeUnderTest pass_by_value();
                          ...
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C" void ...(  // `drop` thunk decl
                        ::rust_out::TypeUnderTest& [[clang::annotate_type(
                            "lifetime", "__anon1")]]);
                    }
                    inline TypeUnderTest::~TypeUnderTest() {
                      __crubit_internal::...(*this);
                    }
                    inline TypeUnderTest::TypeUnderTest(
                        TypeUnderTest&& other)
                        : TypeUnderTest() {
                      *this = std::move(other);
                    }
                    inline TypeUnderTest& TypeUnderTest::operator=(
                        TypeUnderTest&& other) {
                      crubit::MemSwap(*this, other);
                      return *this;
                    }
                    namespace __crubit_internal {  // `pass_by_value` thunk decl
                    extern "C" void ...(::rust_out::TypeUnderTest* __ret_ptr);
                    }
                    inline ::rust_out::TypeUnderTest TypeUnderTest::pass_by_value() {
                      crubit::Slot<::rust_out::TypeUnderTest> __ret_slot;
                      __crubit_internal::...(__ret_slot.Get());
                      return std::move(__ret_slot).AssumeInitAndTakeValue();
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    ...
                    #[unsafe(no_mangle)]
                    extern "C" fn ...(
                        __self: &mut ::core::mem::MaybeUninit<::rust_out::TypeUnderTest>
                    ) {
                        unsafe { __self.assume_init_drop() };
                    }
                    #[unsafe(no_mangle)]
                    extern "C" fn ...(
                        __ret_slot: &mut ::core::mem::MaybeUninit<::rust_out::TypeUnderTest>
                    ) -> () {
                        __ret_slot.write({ ::rust_out::TypeUnderTest::pass_by_value() });
                    }
                    ...
                }
            );
        });
    }

    #[test]
    fn test_format_item_struct_with_custom_drop_impl_and_with_default_impl() {
        let test_src = r#"
                #[derive(Default)]
                pub struct TypeUnderTest {
                    pub x: i32,
                    pub y: i32,
                }

                impl Drop for TypeUnderTest {
                    fn drop(&mut self) {}
                }

                impl TypeUnderTest {
                    pub fn pass_by_value() -> Self { unimplemented!() }
                }
            "#;
        test_format_item_struct_with_custom_drop_and_with_default_impl(test_src);
    }

    #[test]
    fn test_format_item_struct_with_custom_drop_glue_and_with_default_impl() {
        let test_src = r#"
                #![allow(dead_code)]

                // `i32` is present to avoid hitting the ZST checks related to (b/258259459)
                #[derive(Default)]
                struct StructWithCustomDropImpl(i32);

                impl Drop for StructWithCustomDropImpl {
                    fn drop(&mut self) {
                        println!("dropping!");
                    }
                }

                #[derive(Default)]
                pub struct TypeUnderTest {
                    field: StructWithCustomDropImpl,
                }

                impl TypeUnderTest {
                    pub fn pass_by_value() -> Self { unimplemented!() }
                }
            "#;
        test_format_item_struct_with_custom_drop_and_with_default_impl(test_src);
    }

    fn test_format_item_struct_with_custom_drop_and_no_default_and_clone(test_src: &str) {
        test_format_item(test_src, "TypeUnderTest", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... TypeUnderTest final {
                        ...
                        public:
                          ...
                          __COMMENT__ "Drop::drop"
                          ~TypeUnderTest();
                          ...
                          static ::rust_out::TypeUnderTest pass_by_value();
                          ...
                    };
                }
            );

            // Implicit, but not `=default`-ed move constructor and move assignment
            // operator.
            assert_cc_not_matches!(main_api.tokens, quote! { TypeUnderTest(TypeUnderTest&&) });
            assert_cc_not_matches!(main_api.tokens, quote! { operator=(TypeUnderTest&&) });
            // No definition of a custom move constructor nor move assignment operator.
            assert_cc_not_matches!(
                result.cc_details.tokens,
                quote! { TypeUnderTest(TypeUnderTest&&) },
            );
            assert_cc_not_matches!(result.cc_details.tokens, quote! { operator=(TypeUnderTest&&) },);

            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C" void ...(  // `drop` thunk decl
                        ::rust_out::TypeUnderTest& [[clang::annotate_type(
                            "lifetime", "__anon1")]]);
                    }
                    ...
                    namespace __crubit_internal {  // `pass_by_value` thunk decl
                    extern "C" void ...(::rust_out::TypeUnderTest* __ret_ptr);
                    }
                    inline ::rust_out::TypeUnderTest TypeUnderTest::pass_by_value() {
                      crubit::Slot<::rust_out::TypeUnderTest> __ret_slot;
                      __crubit_internal::...(__ret_slot.Get());
                      return std::move(__ret_slot).AssumeInitAndTakeValue();
                    }
                    ...
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    ...
                    #[unsafe(no_mangle)]
                    extern "C" fn ...(
                        __self: &mut ::core::mem::MaybeUninit<::rust_out::TypeUnderTest>
                    ) {
                        unsafe { __self.assume_init_drop() };
                    }
                    #[unsafe(no_mangle)]
                    extern "C" fn ...(
                        __ret_slot: &mut ::core::mem::MaybeUninit<::rust_out::TypeUnderTest>
                    ) -> () {
                        __ret_slot.write({ ::rust_out::TypeUnderTest::pass_by_value() });
                    }
                    ...
                }
            );
        });
    }

    #[test]
    fn test_format_item_struct_with_custom_drop_impl_and_no_default_and_clone() {
        let test_src = r#"
                #[derive(Clone)]
                pub struct TypeUnderTest {
                    pub x: i32,
                    pub y: i32,
                }

                impl Drop for TypeUnderTest {
                    fn drop(&mut self) {}
                }

                impl TypeUnderTest {
                    pub fn pass_by_value() -> Self { unimplemented!() }
                }
            "#;
        test_format_item_struct_with_custom_drop_and_no_default_and_clone(test_src);
    }

    #[test]
    fn test_format_item_struct_with_custom_drop_glue_and_no_default_and_clone() {
        let test_src = r#"
                #![allow(dead_code)]

                // `i32` is present to avoid hitting the ZST checks related to (b/258259459)
                #[derive(Clone)]
                struct StructWithCustomDropImpl(i32);

                impl Drop for StructWithCustomDropImpl {
                    fn drop(&mut self) {
                        println!("dropping!");
                    }
                }

                #[derive(Clone)]
                pub struct TypeUnderTest {
                    field: StructWithCustomDropImpl,
                }

                impl TypeUnderTest {
                    pub fn pass_by_value() -> Self { unimplemented!() }
                }
            "#;
        test_format_item_struct_with_custom_drop_and_no_default_and_clone(test_src);
    }

    #[test]
    fn test_format_item_unsupported_struct_with_custom_drop_and_default_and_nonunpin() {
        let test_src = r#"
                #![feature(negative_impls)]

                #[derive(Default)]
                pub struct SomeStruct {
                    pub x: i32,
                    pub y: i32,
                }

                impl !Unpin for SomeStruct {}

                impl Drop for SomeStruct {
                    fn drop(&mut self) {}
                }

                impl SomeStruct {
                    pub fn pass_by_value() -> Self { unimplemented!() }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let move_deleted_msg = "C++ moves are deleted \
                                    because there's no non-destructive implementation available.";
            let pass_by_value_msg = "Error generating bindings for `SomeStruct::pass_by_value` \
                        defined at <crubit_unittests.rs>;l=17: \
                 Can't pass the return type by value without a move constructor";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        public:
                          ...
                          __COMMENT__ "Default::default"
                          SomeStruct();

                          __COMMENT__ "Drop::drop"
                          ~SomeStruct();

                          __COMMENT__ #move_deleted_msg
                          SomeStruct(SomeStruct&&) = delete;
                          SomeStruct& operator=(SomeStruct&&) = delete;
                          ...
                          __COMMENT__ #pass_by_value_msg
                          ...
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    ...
                    namespace __crubit_internal {
                    extern "C" void ...(  // `default` thunk decl
                        ::rust_out::SomeStruct* __ret_ptr);
                    }
                    inline SomeStruct::SomeStruct() {
                      __crubit_internal::...(this);
                    }
                    namespace __crubit_internal {
                    extern "C" void ...(  // `drop` thunk decl
                        ::rust_out::SomeStruct& [[clang::annotate_type("lifetime", "__anon1")]]);
                    }
                    inline SomeStruct::~SomeStruct() {
                      __crubit_internal::...(*this);
                    }
                    ...
                }
            );
            assert_cc_not_matches!(result.cc_details.tokens, quote! { pass_by_value });
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    ...
                    #[unsafe(no_mangle)]
                    extern "C" fn ...(
                        __ret_slot: &mut ::core::mem::MaybeUninit<::rust_out::SomeStruct>
                    ) -> () {
                        __ret_slot.write(
                           { <::rust_out::SomeStruct as ::core::default::Default>::default() }
                        );
                    }
                    #[unsafe(no_mangle)]
                    extern "C" fn ...(
                        __self: &mut ::core::mem::MaybeUninit<::rust_out::SomeStruct>
                    ) {
                        unsafe { __self.assume_init_drop() };
                    }
                    ...
                }
            );
            assert_rs_not_matches!(result.rs_details.tokens, quote! { pass_by_value });
        });
    }

    #[test]
    fn test_format_item_source_loc_macro_rules() {
        let test_src = r#"
            #![allow(dead_code)]

            macro_rules! some_tuple_struct_macro_for_testing_source_loc {
                () => {
                    /// Some doc on SomeTupleStructMacroForTesingSourceLoc.
                    pub struct SomeTupleStructMacroForTesingSourceLoc(i32);
                };
            }

            some_tuple_struct_macro_for_testing_source_loc!();
        "#;
        test_format_item(test_src, "SomeTupleStructMacroForTesingSourceLoc", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let source_loc_comment = " Some doc on SomeTupleStructMacroForTesingSourceLoc.\n\n\
                                      Generated from: <crubit_unittests.rs>;l=7";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #source_loc_comment
                    struct ... SomeTupleStructMacroForTesingSourceLoc final {
                        ...
                    }
                    ...
                },
            );
        });
    }

    #[test]
    fn test_format_item_source_loc_with_no_doc_comment() {
        let test_src = r#"
            #![allow(dead_code)]

            pub struct SomeTupleStructWithNoDocComment(i32);
        "#;
        test_format_item(test_src, "SomeTupleStructWithNoDocComment", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            let comment = "Generated from: <crubit_unittests.rs>;l=4";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    struct ... SomeTupleStructWithNoDocComment final {
                        ...
                    }
                    ...
                },
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_static_value() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub static STATIC_VALUE: i32 = 42;
            "#;
        test_format_item(test_src, "STATIC_VALUE", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Unsupported rustc_hir::hir::ItemKind: static item");
        });
    }

    #[test]
    fn test_format_item_use_normal_type() {
        let test_src = r#"
            pub mod test_mod {
                pub struct S{
                    pub field: i32
                }
            }

            pub use test_mod::S as G;
            "#;
        test_format_item(test_src, "G", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    using G = ::rust_out::test_mod::S;
                }
            );
        });
    }

    #[test]
    fn test_generate_bindings_use_list_items() {
        let test_src = r#"
            pub mod test_mod {
                pub struct X{
                    pub field: i32
                }
                pub struct Y{
                    pub field: i32
                }
            }

            pub use test_mod::{X, Y};
            "#;

        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    using X = ::rust_out::test_mod::X;
                    using Y = ::rust_out::test_mod::Y;
                }
            );
        });
    }

    #[test]
    fn test_generate_bindings_use_glob() {
        let test_src = r#"
            pub mod test_mod {
                pub struct X{
                    pub field: i32
                }
                pub struct Y{
                    pub field: i32
                }
            }

            pub use test_mod::*;
            "#;

        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    using X = ::rust_out::test_mod::X;
                    using Y = ::rust_out::test_mod::Y;
                }
            );
        });
    }

    #[test]
    fn test_format_item_type_alias() {
        let test_src = r#"
                pub type TypeAlias = i32;
            "#;
        test_format_item(test_src, "TypeAlias", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    using TypeAlias = std::int32_t;
                }
            );
        });
    }

    #[test]
    fn test_format_item_type_alias_should_give_underlying_type() {
        let test_src = r#"
                pub type TypeAlias1 = i32;
                pub type TypeAlias2 = TypeAlias1;
            "#;
        test_format_item(test_src, "TypeAlias2", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    using TypeAlias2 = std::int32_t;
                }
            );
        });
    }

    #[test]
    fn test_format_item_private_type_alias_wont_generate_bindings() {
        let test_src = r#"
            #[allow(dead_code)]
            type TypeAlias = i32;
            "#;
        test_format_item(test_src, "TypeAlias", |result| {
            let result = result.unwrap();
            assert!(result.is_none());
        });
    }

    #[test]
    fn test_format_item_pub_type_alias_on_private_type_wont_generate_bindings() {
        let test_src = r#"
            #![allow(private_interfaces)]
            struct SomeStruct;
            pub type TypeAlias = SomeStruct;
            "#;
        test_format_item(test_src, "TypeAlias", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Not a public or a supported reexported type (b/262052635).");
        });
    }

    #[test]
    fn test_format_item_unsupported_generic_type_alias() {
        let test_src = r#"
            pub type TypeAlias<T> = T;
            "#;
        test_format_item(test_src, "TypeAlias", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "The following Rust type is not supported yet: T");
        });
    }

    #[test]
    fn test_format_item_unsupported_type_without_direct_existence() {
        let test_src = r#"
            pub trait Evil {
                type Type;
            }

            const _ : () = {
                pub struct NamelessType;
                impl Evil for i64 {
                    type Type = NamelessType;
                }
            };
            pub type EvilAlias = <i64 as Evil>::Type;
            "#;
        test_format_item(test_src, "EvilAlias", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "The following Rust type is not supported yet: <i64 as Evil>::Type");
        });
    }

    #[test]
    fn test_format_item_type_alias_deprecated() {
        let test_src = r#"
                #[deprecated = "Use `OtherTypeAlias` instead"]
                pub type TypeAlias = i32;
            "#;
        test_format_item(test_src, "TypeAlias", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    using TypeAlias [[deprecated("Use `OtherTypeAlias` instead")]] = std::int32_t;
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_impl_item_const_value() {
        let test_src = r#"
                #![allow(dead_code)]

                pub struct SomeStruct(i32);

                impl SomeStruct {
                    pub const CONST_VALUE: i32 = 42;
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeStruct final {
                        ...
                        static constexpr std::int32_t CONST_VALUE = 42;
                        ...
                    };
                    ...
                }
            );
        });
    }

    #[test]
    fn test_format_item_generate_bindings_for_top_level_type_alias() {
        let test_src = r#"
            #![feature(inherent_associated_types)]
            #![allow(incomplete_features)]
            #![allow(dead_code)]
            pub struct Evil {
                dumb: i32,
            }

            impl Evil {
                pub type Type = i64;
            }
            pub type EvilAlias = Evil::Type;
        "#;
        test_format_item(test_src, "Evil", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_not_matches!(
                main_api.tokens,
                quote! {
                    std::int64_t
                }
            );
        });
    }

    /// `test_format_ret_ty_for_cc_successes` provides test coverage for cases
    /// where `format_ty_for_cc` takes `TypeLocation::FnReturn` and returns
    /// an `Ok(...)`.  Additional testcases are covered by
    /// `test_format_ty_for_cc_successes`.
    #[test]
    fn test_format_ret_ty_for_cc_successes() {
        let testcases = [
            // ( <Rust type>, <expected C++ type> )
            ("bool", "bool"), // TyKind::Bool
            ("()", "void"),
            // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
            // details).
            ("!", "void"),
            (
                "extern \"C\" fn (f32, f32) -> f32",
                "crubit :: type_identity_t < float (float , float) > &",
            ),
        ];
        test_ty(TypeLocation::FnReturn, &testcases, quote! {}, |desc, tcx, ty, expected| {
            let actual = {
                let db = bindings_db_for_tests(tcx);
                let cc_snippet = format_ty_for_cc(&db, ty, TypeLocation::FnReturn).unwrap();
                cc_snippet.tokens.to_string()
            };
            let expected = expected.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual, expected, "{desc}");
        });
    }

    /// `test_format_ty_for_cc_successes` provides test coverage for cases where
    /// `format_ty_for_cc` returns an `Ok(...)`.
    ///
    /// Note that using `std::int8_t` (instead of `::std::int8_t`) has been an
    /// explicit decision. The "Google C++ Style Guide" suggests to "avoid
    /// nested namespaces that match well-known top-level namespaces" and "in
    /// particular, [...] not create any nested std namespaces.".  It
    /// seems desirable if the generated bindings conform to this aspect of the
    /// style guide, because it makes things easier for *users* of these
    /// bindings.
    #[test]
    fn test_format_ty_for_cc_successes() {
        struct FormatCcExpectation {
            expected_tokens: &'static str,
            expected_includes: Vec<&'static str>,
            expected_prereq_def: Option<&'static str>,
            expected_prereq_fwd_decl: Option<&'static str>,
        }

        // Helper macro to create a `FormatCcExpectation`s. Handles all a variation of
        // relevant fields (e.g. expected includes or forward decls).
        macro_rules! case {
            (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*], prereq_def: $expected_prereq_def:expr, prereq_fwd_decl: $expected_prereq_fwd_decl:expr) => {
                (
                    $input_rust_ty,
                    FormatCcExpectation {
                        expected_tokens: $expected_cc_ty,
                        expected_includes: Vec::<&'static str>::from([$($includes),*]),
                        expected_prereq_def: $expected_prereq_def,
                        expected_prereq_fwd_decl: $expected_prereq_fwd_decl,
                    }
            )
            };
            (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr) => {
                case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [], prereq_def: None, prereq_fwd_decl: None)
            };
            (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*]) => {
                case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [$($includes),*], prereq_def: None, prereq_fwd_decl: None)
            };
            (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*], prereq_def: $expected_prereq_def:expr) => {
                case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [$($includes),*], prereq_def: Some($expected_prereq_def), prereq_fwd_decl: None)
            };
            (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*], prereq_fwd_decl: $expected_prereq_fwd_decl:expr) => {
                case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [$($includes),*], prereq_def: None, prereq_fwd_decl: Some($expected_prereq_fwd_decl))
            };
        }

        let testcases = [
            case!(rs: "bool", cc:  "bool"),
            case!(rs: "f32", cc: "float"),
            case!(rs: "f64", cc: "double"),
            // The ffi aliases are special-cased to refer to the C++ fundamental integer types,
            // if the type alias information is not lost (e.g. from generics).
            case!(rs: "std::ffi::c_char", cc:  "char"),
            case!(rs: "::std::ffi::c_char", cc:  "char"),
            case!(rs: "core::ffi::c_char", cc:  "char"),
            case!(rs: "::core::ffi::c_char", cc:  "char"),
            case!(rs: "std::ffi::c_uchar", cc: "unsigned char"),
            case!(rs: "std::ffi::c_longlong", cc: "long long"),
            case!(rs: "c_char", cc:  "char"),
            // Simple pointers/references do not lose the type alias information.
            case!(rs: "*const std::ffi::c_uchar", cc: "unsigned char const *"),
            case!(
                rs: "&'static std::ffi::c_uchar",
                cc: "unsigned char const & [[clang :: annotate_type (\"lifetime\" , \"static\")]]"
            ),
            // Generics lose type alias information.
            case!(rs: "Identity<std::ffi::c_longlong>", cc: "std::int64_t", includes: ["<cstdint>"]),
            case!(rs: "i8", cc: "std::int8_t", includes: ["<cstdint>"]),
            case!(rs: "i16", cc:  "std::int16_t", includes: ["<cstdint>"]),
            case!(rs: "i32", cc:  "std::int32_t", includes: ["<cstdint>"]),
            case!(rs: "i64", cc:  "std::int64_t", includes: ["<cstdint>"]),
            case!(rs: "isize", cc: "std::intptr_t", includes: ["<cstdint>"]),
            case!(rs: "u8", cc: "std::uint8_t", includes: ["<cstdint>"]),
            case!(rs: "u16", cc: "std::uint16_t", includes: ["<cstdint>"]),
            case!(rs: "u32", cc: "std::uint32_t", includes: ["<cstdint>"]),
            case!(rs: "u64", cc: "std::uint64_t", includes: ["<cstdint>"]),
            case!(rs: "usize", cc: "std::uintptr_t", includes: ["<cstdint>"]),
            case!(
                rs: "char",
                cc: "rs_std::rs_char",
                includes: ["<crubit/support/for/tests/rs_std/rs_char.h>"]
            ),
            case!(rs: "SomeStruct", cc: "::rust_out::SomeStruct", includes: [],  prereq_def: "SomeStruct"),
            case!(rs: "SomeEnum", cc: "::rust_out::SomeEnum", includes: [], prereq_def: "SomeEnum"),
            case!(rs: "SomeUnion", cc: "::rust_out::SomeUnion", includes: [], prereq_def: "SomeUnion"),
            case!(rs: "*const i32", cc: "std :: int32_t const *", includes: ["<cstdint>"]),
            case!(rs: "*mut i32", cc: "std :: int32_t *", includes: ["<cstdint>"]),
            case!(
                rs: "&'static i32",
                cc: "std :: int32_t const & [[clang :: annotate_type (\"lifetime\" , \"static\")]]",
                includes: ["<cstdint>"]
            ),
            case!(
                rs: "&'static mut i32",
                cc: "std :: int32_t & [[clang :: annotate_type (\"lifetime\" , \"static\")]]",
                includes: ["<cstdint>"]
            ),
            // Slice pointers:
            case!(
                rs: "*const [i8]",
                cc: "rs_std::SliceRef<const std::int8_t>",
                includes: ["<cstdint>", "<crubit/support/for/tests/rs_std/slice_ref.h>"]
            ),
            case!(
                rs: "*mut [i64]",
                cc: "rs_std::SliceRef<std::int64_t>",
                includes: ["<cstdint>", "<crubit/support/for/tests/rs_std/slice_ref.h>"]
            ),
            case!(
                rs: "*const [c_char]",
                cc: "rs_std::SliceRef<const char>",
                includes: ["<crubit/support/for/tests/rs_std/slice_ref.h>"]
            ),
            case!(
                rs: "*mut [SomeStruct]",
                cc: "rs_std::SliceRef< ::rust_out::SomeStruct>",
                includes: [ "<crubit/support/for/tests/rs_std/slice_ref.h>"],
                prereq_def: "SomeStruct"

            ),
            // `SomeStruct` is a `fwd_decls` prerequisite (not `defs` prerequisite):
            case!(
                rs: "*mut SomeStruct",
                cc: "::rust_out::SomeStruct*",
                includes: [],
                prereq_fwd_decl: "SomeStruct"
            ),
            // Testing propagation of deeper/nested `fwd_decls`:
            case!(
                rs: "*mut *mut SomeStruct",
                cc: ":: rust_out :: SomeStruct * *",
                includes: [],
                prereq_fwd_decl: "SomeStruct"
            ),
            // Testing propagation of `const` / `mut` qualifiers:
            case!(rs: "*mut *const f32", cc: "float const * *"),
            case!(rs: "*const *mut f32", cc: "float * const *"),
            // Rust function pointers are non-nullable, so when function pointers are used as a
            // parameter type (i.e. in `TypeLocation::FnParam`) then we can translate to
            // generate a C++ function *reference*, rather than a C++ function *pointer*.
            case!(
                rs: "extern \"C\" fn (f32, f32) -> f32",
                cc: "crubit :: type_identity_t < float (float , float) > &",
                includes: ["<crubit/support/for/tests/internal/cxx20_backports.h>"]
            ),
            // Unsafe extern "C" function pointers are, to C++, just function pointers.
            case!(
                rs: "unsafe extern \"C\" fn(f32, f32) -> f32",
                cc: "crubit :: type_identity_t < float (float , float) > &",
                includes: ["<crubit/support/for/tests/internal/cxx20_backports.h>"]
            ),
            // Nested function pointer (i.e. `TypeLocation::Other`) means that
            // we need to generate a C++ function *pointer*, rather than a C++
            // function *reference*.
            case!(
                rs: "*const extern \"C\" fn (f32, f32) -> f32",
                cc: "crubit :: type_identity_t < float (float , float) > * const *",
                includes: ["<crubit/support/for/tests/internal/cxx20_backports.h>"]
            ),
            // Extra parens/sugar are expected to be ignored:
            case!(rs: "(bool)", cc: "bool"),
            // References to MaybeUninit:
            case!(
                rs: "*const std::mem::MaybeUninit<i32>",
                cc: "std :: int32_t const *",
                includes: ["<cstdint>"]
            ),
            case!(
                rs: "&mut std::mem::MaybeUninit<i32>",
                cc: "std :: int32_t & [[clang :: annotate_type (\"lifetime\" , \"__anon1\")]]",
                includes: ["<cstdint>"]
            ),
        ];
        let preamble = quote! {
            #![allow(unused_parens)]
            #![feature(register_tool)]
            #![register_tool(__crubit)]

            pub struct SomeStruct {
                pub x: i32,
                pub y: i32,
            }
            pub enum SomeEnum {
                Cartesian{x: f64, y: f64},
                Polar{angle: f64, dist: f64},
            }
            pub union SomeUnion {
                pub x: i32,
                pub y: i32,
            }

            #[allow(unused)]
            type Identity<T> = T;

            pub use core::ffi::c_char;
            // TODO(b/283258442): Correctly handle something like this:
            // pub type MyChar = core::ffi::c_char;
        };
        test_ty(
            TypeLocation::FnParam,
            &testcases,
            preamble,
            |desc,
             tcx,
             ty,
             FormatCcExpectation {
                 expected_tokens,
                 expected_includes,
                 expected_prereq_def,
                 expected_prereq_fwd_decl,
             }| {
                let (actual_tokens, actual_prereqs) = {
                    let db = bindings_db_for_tests(tcx);
                    let s = format_ty_for_cc(&db, ty, TypeLocation::FnParam).unwrap();
                    (s.tokens.to_string(), s.prereqs)
                };
                let (actual_includes, actual_prereq_defs, actual_prereq_fwd_decls) =
                    (actual_prereqs.includes, actual_prereqs.defs, actual_prereqs.fwd_decls);

                let expected_tokens = expected_tokens.parse::<TokenStream>().unwrap().to_string();
                assert_eq!(actual_tokens, expected_tokens, "{desc}");

                assert!(
                    expected_includes.len() == actual_includes.len(),
                    "{desc}: `actual_includes` is unexpectedly not of the same length as `expected_includes`. actual_includes: {actual_includes:#?}; expected_includes: {expected_includes:#?}"
                );

                if expected_includes.len() > 0 {
                    let expected_includes = expected_includes
                        .into_iter()
                        .map(|include| include.parse::<TokenStream>().unwrap())
                        .collect::<Vec<_>>();
                    assert_cc_matches!(
                        format_cc_includes(&actual_includes),
                        quote! { #( __HASH_TOKEN__ include #expected_includes )*}
                    );
                }

                if let Some(expected_prereq_def) = expected_prereq_def {
                    let expected_def_id = find_def_id_by_name(tcx, expected_prereq_def);
                    assert_eq!(1, actual_prereq_defs.len());
                    assert_eq!(expected_def_id, actual_prereq_defs.into_iter().next().unwrap());
                } else {
                    assert!(
                        actual_prereq_defs.is_empty(),
                        "{desc}: `actual_prereq_defs` is unexpectedly non-empty",
                    );
                }

                if let Some(expected_prereq_fwd_decl) = expected_prereq_fwd_decl {
                    let expected_def_id = find_def_id_by_name(tcx, expected_prereq_fwd_decl);
                    assert_eq!(1, actual_prereq_fwd_decls.len());
                    assert_eq!(
                        expected_def_id,
                        actual_prereq_fwd_decls.into_iter().next().unwrap()
                    );
                } else {
                    assert!(
                        actual_prereq_fwd_decls.is_empty(),
                        "{desc}: `actual_prereq_fwd_decls` is unexpectedly non-empty",
                    );
                }
            },
        );
    }

    /// `test_format_ty_for_cc_failures` provides test coverage for cases where
    /// `format_ty_for_cc` returns an `Err(...)`.
    ///
    /// It seems okay to have no test coverage for now for the following types
    /// (which should never be encountered when generating bindings and where
    /// `format_ty_for_cc` should panic):
    /// - TyKind::Closure
    /// - TyKind::Error
    /// - TyKind::FnDef
    /// - TyKind::Infer
    ///
    /// TODO(lukasza): Add test coverage (here and in the "for_rs" flavours)
    /// for:
    /// - TyKind::Bound
    /// - TyKind::Dynamic (`dyn Eq`)
    /// - TyKind::Foreign (`extern type T`)
    /// - https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html:
    ///   TyKind::Generator, TyKind::GeneratorWitness
    /// - TyKind::Param
    /// - TyKind::Placeholder
    #[test]
    fn test_format_ty_for_cc_failures() {
        let testcases = [
            // ( <Rust type>, <expected error message> )
            (
                "()", // Empty TyKind::Tuple
                "`()` / `void` is only supported as a return type (b/254507801)",
            ),
            (
                // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
                // details).
                "!", // TyKind::Never
                "The never type `!` is only supported as a return type (b/254507801)",
            ),
            (
                "(i32, i32)", // Non-empty TyKind::Tuple
                "Tuples are not supported yet: (i32, i32) (b/254099023)",
            ),
            (
                "&'static &'static i32", // TyKind::Ref (nested reference - referent of reference)
                "Failed to format the referent of the reference type `&'static &'static i32`: \
                 Can't format `&'static i32`, because references are only supported \
                 in function parameter types and return types (b/286256327)",
            ),
            (
                "extern \"C\" fn (&i32)", // TyKind::Ref (nested reference - underneath fn ptr)
                "Generic functions are not supported yet (b/259749023)",
            ),
            (
                "[i32; 42]", // TyKind::Array
                "The following Rust type is not supported yet: [i32; 42]",
            ),
            (
                // Check that the failure for slices is about not being supported and not failed
                // asserts about ABI and layout.
                "&'static [i32]", // TyKind::Slice (nested underneath TyKind::Ref)
                "Failed to format the referent of the reference type `&'static [i32]`: \
                 The following Rust type is not supported yet: [i32]",
            ),
            (
                "&'static str", // TyKind::Str (nested underneath TyKind::Ref)
                "Failed to format the referent of the reference type `&'static str`: \
                 The following Rust type is not supported yet: str",
            ),
            (
                "impl Eq", // TyKind::Alias
                "The following Rust type is not supported yet: impl Eq",
            ),
            (
                "fn(i32) -> i32", // TyKind::FnPtr (default ABI = "Rust")
                "Function pointers can't have a thunk: \
                 Any calling convention other than `extern \"C\"` requires a thunk",
            ),
            (
                "extern \"C\" fn (SomeStruct, f32) -> f32",
                "Function pointers can't have a thunk: Type of parameter #0 requires a thunk",
            ),
            (
                "extern \"C\" fn (f32, f32) -> SomeStruct",
                "Function pointers can't have a thunk: Return type requires a thunk",
            ),
            // TODO(b/254094650): Consider mapping this to Clang's (and GCC's) `__int128`
            // or to `absl::in128`.
            ("i128", "C++ doesn't have a standard equivalent of `i128` (b/254094650)"),
            ("u128", "C++ doesn't have a standard equivalent of `u128` (b/254094650)"),
            ("ConstGenericStruct<42>", "Generic types are not supported yet (b/259749095)"),
            ("TypeGenericStruct<u8>", "Generic types are not supported yet (b/259749095)"),
            (
                // This double-checks that TyKind::Adt(..., substs) are present
                // even if the type parameter argument is not explicitly specified
                // (here it comes from the default: `...Struct<T = u8>`).
                "TypeGenericStruct",
                "Generic types are not supported yet (b/259749095)",
            ),
            ("LifetimeGenericStruct<'static>", "Generic types are not supported yet (b/259749095)"),
            (
                "std::cmp::Ordering",
                "Type `std::cmp::Ordering` comes from the `core` crate, \
                 but no `--crate-header` was specified for this crate",
            ),
            ("Option<i8>", "Generic types are not supported yet (b/259749095)"),
            (
                // TODO(b/258261328): Once cross-crate bindings are supported we should try
                // to test them via a test crate that we control (rather than testing via
                // implementation details of the std crate).
                "core::alloc::LayoutError",
                "Type `std::alloc::LayoutError` comes from the `core` crate, but no `--crate-header` was specified for this crate",
            ),
            (
                "*const Option<i8>",
                "Failed to format the pointee \
                 of the pointer type `*const std::option::Option<i8>`: \
                 Generic types are not supported yet (b/259749095)",
            ),
        ];
        let preamble = quote! {
            #![feature(never_type)]

            #[repr(C)]
            pub struct SomeStruct {
                pub x: i32,
                pub y: i32,
            }

            pub struct ConstGenericStruct<const N: usize> {
                pub arr: [u8; N],
            }

            pub struct TypeGenericStruct<T = u8> {
                pub t: T,
            }

            pub struct LifetimeGenericStruct<'a> {
                pub reference: &'a u8,
            }
        };
        test_ty(TypeLocation::FnParam, &testcases, preamble, |desc, tcx, ty, expected_msg| {
            let db = bindings_db_for_tests(tcx);
            let anyhow_err = format_ty_for_cc(&db, ty, TypeLocation::FnParam)
                .expect_err(&format!("Expecting error for: {desc}"));
            let actual_msg = format!("{anyhow_err:#}");
            assert_eq!(&actual_msg, *expected_msg, "{desc}");
        });
    }

    #[test]
    fn test_format_ty_for_rs_successes() {
        // Test coverage for cases where `format_ty_for_rs` returns an `Ok(...)`.
        let testcases = [
            // ( <Rust type>, <expected Rust spelling for ..._cc_api_impl.rs> )
            ("bool", "bool"),
            ("f32", "f32"),
            ("f64", "f64"),
            ("i8", "i8"),
            ("i16", "i16"),
            ("i32", "i32"),
            ("i64", "i64"),
            ("i128", "i128"),
            ("isize", "isize"),
            ("u8", "u8"),
            ("u16", "u16"),
            ("u32", "u32"),
            ("u64", "u64"),
            ("u128", "u128"),
            ("usize", "usize"),
            ("char", "char"),
            ("!", "!"),
            ("()", "()"),
            // ADTs:
            ("SomeStruct", "::rust_out::SomeStruct"),
            ("SomeEnum", "::rust_out::SomeEnum"),
            ("SomeUnion", "::rust_out::SomeUnion"),
            // Type from another crate:
            ("std::cmp::Ordering", "::core::cmp::Ordering"),
            // `const` and `mut` pointers:
            ("*const i32", "*const i32"),
            ("*mut i32", "*mut i32"),
            // References:
            ("&i32", "& '__anon1 i32"),
            ("&mut i32", "& '__anon1 mut i32"),
            ("&'_ i32", "& '__anon1 i32"),
            ("&'static i32", "& 'static i32"),
            // Pointer to an ADT:
            ("*mut SomeStruct", "* mut :: rust_out :: SomeStruct"),
            ("extern \"C\" fn(i32) -> i32", "extern \"C\" fn(i32) -> i32"),
            // Pointer to a Slice:
            ("*mut [i32]", "*mut [i32]"),
            // MaybeUninit:
            ("&'static std::mem::MaybeUninit<i32>", "& 'static std :: mem :: MaybeUninit < i32 >"),
            (
                "&'static mut std::mem::MaybeUninit<i32>",
                "& 'static mut std :: mem :: MaybeUninit < i32 >",
            ),
            ("*const std::mem::MaybeUninit<i32>", "*const std::mem::MaybeUninit<i32>"),
            ("*mut std::mem::MaybeUninit<i32>", "*mut std::mem::MaybeUninit<i32>"),
        ];
        let preamble = quote! {
            #![feature(never_type)]

            pub struct SomeStruct {
                pub x: i32,
                pub y: i32,
            }
            pub enum SomeEnum {
                Cartesian{x: f64, y: f64},
                Polar{angle: f64, dist: f64},
            }
            pub union SomeUnion {
                pub x: i32,
                pub y: i32,
            }
        };
        test_ty(TypeLocation::FnParam, &testcases, preamble, |desc, tcx, ty, expected_tokens| {
            let db = bindings_db_for_tests(tcx);
            let actual_tokens = format_ty_for_rs(&db, ty.mid()).unwrap().to_string();
            let expected_tokens = expected_tokens.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual_tokens, expected_tokens, "{desc}");
        });
    }

    #[test]
    fn test_format_ty_for_rs_failures() {
        // This test provides coverage for cases where `format_ty_for_rs` returns an
        // `Err(...)`.
        let testcases = [
            // ( <Rust type>, <expected error message> )
            (
                "(i32, i32)", // Non-empty TyKind::Tuple
                "Tuples are not supported yet: (i32, i32) (b/254099023)",
            ),
            (
                "[i32; 42]", // TyKind::Array
                "The following Rust type is not supported yet: [i32; 42]",
            ),
            (
                "&'static str", // TyKind::Str (nested underneath TyKind::Ref)
                "Failed to format the referent of the reference type `&'static str`: \
                 The following Rust type is not supported yet: str",
            ),
            (
                "impl Eq", // TyKind::Alias
                "The following Rust type is not supported yet: impl Eq",
            ),
            (
                "Option<i8>", // TyKind::Adt - generic + different crate
                "Generic types are not supported yet (b/259749095)",
            ),
        ];
        let preamble = quote! {};
        test_ty(TypeLocation::FnParam, &testcases, preamble, |desc, tcx, ty, expected_err| {
            let db = bindings_db_for_tests(tcx);
            let anyhow_err =
                format_ty_for_rs(&db, ty.mid()).expect_err(&format!("Expecting error for: {desc}"));
            let actual_err = format!("{anyhow_err:#}");
            assert_eq!(&actual_err, *expected_err, "{desc}");
        });
    }

    #[test]
    fn test_format_namespace_bound_cc_tokens() {
        run_compiler_for_testing("", |tcx| {
            let db = bindings_db_for_tests(tcx);
            let top_level = NamespaceQualifier::new::<&str>([]);
            let m1 = NamespaceQualifier::new(["m1"]);
            let m2 = NamespaceQualifier::new(["m2"]);
            let input = [
                (None, top_level.clone(), quote! { void f0a(); }),
                (None, m1.clone(), quote! { void f1a(); }),
                (None, m1.clone(), quote! { void f1b(); }),
                (None, top_level.clone(), quote! { void f0b(); }),
                (None, top_level.clone(), quote! { void f0c(); }),
                (None, m2.clone(), quote! { void f2a(); }),
                (None, m1.clone(), quote! { void f1c(); }),
                (None, m1.clone(), quote! { void f1d(); }),
            ];
            assert_cc_matches!(
                format_namespace_bound_cc_tokens(&db, input, tcx),
                quote! {
                    void f0a();

                    namespace m1 {
                    void f1a();
                    void f1b();
                    }  // namespace m1

                    void f0b();
                    void f0c();

                    namespace m2 {
                    void f2a();
                    }

                    namespace m1 {
                    void f1c();
                    void f1d();
                    }  // namespace m1
                },
            );
        });
    }

    #[test]
    fn test_must_use_attr_for_fn_no_msg() {
        let test_src = r#"
        #[must_use]
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }"#;

        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    [[nodiscard]] std::int32_t add(std::int32_t x, std::int32_t y);
                }
            )
        })
    }

    #[test]
    fn test_must_use_attr_for_fn_msg() {
        let test_src = r#"
        #[must_use = "hello!"]
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }"#;

        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    [[nodiscard("hello!")]] std::int32_t add(std::int32_t x, std::int32_t y);
                }
            )
        })
    }

    #[test]
    fn test_must_use_attr_for_struct_no_msg() {
        let test_src = r#"
        #[must_use]
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... [[nodiscard]] ... SomeStruct final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_format_item_rename_field_with_conflicting_name() {
        let test_src = r#"
        pub struct X {
            pub a: i32,
            b: i32,
            #[allow(dead_code)]
            c: i32,
        }

        impl X {
            pub fn a(&self) -> i32 {
                self.a
            }
            pub fn b(&self) -> i32 {
                self.b
            }
        }
        "#;

        test_format_item(test_src, "X", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t a_;
                }
            );
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t b_;
                }
            );
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t c;
                }
            );
            // Check that the fields are not renamed in the Rust side.
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    ::core::mem::offset_of!(::rust_out::X, a) == 0
                }
            );
        })
    }

    #[test]
    fn test_must_use_attr_for_struct_msg() {
        let test_src = r#"
        #[must_use = "foo"]
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... [[nodiscard("foo")]] ... SomeStruct final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_must_use_attr_for_enum_no_msg() {
        let test_src = r#"
        #[must_use]
        pub enum SomeEnum {
            A(i32),
            B(u32),
        }"#;

        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... [[nodiscard]] ... SomeEnum final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_must_use_attr_for_enum_msg() {
        let test_src = r#"
        #[must_use = "foo"]
        pub enum SomeEnum {
            A(i32),
            B(u32),
        }"#;

        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... [[nodiscard("foo")]] ... SomeEnum final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_must_use_attr_for_union_no_msg() {
        let test_src = r#"
        #[must_use]
        pub union SomeUnion {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    union ... [[nodiscard]] ... SomeUnion final {
                        ...
                    };
                }
            )
        })
    }
    #[test]
    fn test_must_use_attr_for_union_msg() {
        let test_src = r#"
        #[must_use = "foo"]
        pub union SomeUnion {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    union ... [[nodiscard("foo")]] ... SomeUnion final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_fn_no_args() {
        let test_src = r#"
        #[deprecated]
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }"#;

        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    [[deprecated]] std::int32_t add(std::int32_t x, std::int32_t y);
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_fn_with_message() {
        let test_src = r#"
        #[deprecated = "Use add_i32 instead"]
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }"#;

        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    [[deprecated("Use add_i32 instead")]] std::int32_t add(std::int32_t x, std::int32_t y);
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_fn_with_named_args() {
        let test_src = r#"
        #[deprecated(since = "3.14", note = "Use add_i32 instead")]
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }"#;

        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    [[deprecated("Use add_i32 instead")]] std::int32_t add(std::int32_t x, std::int32_t y);
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_struct_no_args() {
        let test_src = r#"
        #[deprecated]
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... [[deprecated]] ... SomeStruct final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_struct_with_message() {
        let test_src = r#"
        #[deprecated = "Use AnotherStruct instead"]
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... [[deprecated("Use AnotherStruct instead")]] ... SomeStruct final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_struct_with_named_args() {
        let test_src = r#"
        #[deprecated(since = "3.14", note = "Use AnotherStruct instead")]
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... [[deprecated("Use AnotherStruct instead")]] ... SomeStruct final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_union_with_named_args() {
        let test_src = r#"
        #[deprecated(since = "3.14", note = "Use AnotherUnion instead")]
        pub struct SomeUnion {
            pub x: u32,
            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... [[deprecated("Use AnotherUnion instead")]] ... SomeUnion final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_enum_with_named_args() {
        let test_src = r#"
        #[deprecated(since = "3.14", note = "Use AnotherEnum instead")]
        pub enum SomeEnum {
            Integer(i32),
            FloatingPoint(f64),
        }"#;

        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... [[deprecated("Use AnotherEnum instead")]] ... SomeEnum final {
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_struct_fields() {
        let test_src = r#"
        pub struct SomeStruct {
            #[deprecated = "Use `y` instead"]
            pub x: u32,

            pub y: u32,
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... SomeStruct final {
                        ...
                        union {
                            ...
                            [[deprecated("Use `y` instead")]] std::uint32_t x;
                        }
                        ...
                        union {
                            ...
                            std::uint32_t y;
                        }
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_impl_block() {
        let test_src = r#"
        pub struct SomeStruct {
            pub x: u32,
            pub y: u32,
        }

        #[deprecated = "Use AnotherStruct instead"]
        impl SomeStruct {
            pub fn sum(&self) -> u32 {
                self.x + self.y
            }

            pub fn product(&self) -> u32 {
                self.x * self.y
            }
        }"#;

        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    struct ... SomeStruct final {
                        ...
                        ... [[deprecated("Use AnotherStruct instead")]] std::uint32_t sum() const ...
                        ...
                        ... [[deprecated("Use AnotherStruct instead")]] std::uint32_t product() const ...
                        ...
                    };
                }
            )
        })
    }

    #[test]
    fn test_multiple_attributes() {
        let test_src = r#"
        #[must_use = "Must use"]
        #[deprecated = "Deprecated"]
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }"#;

        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    [[nodiscard("Must use")]] [[deprecated("Deprecated")]] std::int32_t add(std::int32_t x, std::int32_t y);
                        ...
                }
            )
        })
    }

    #[test]
    fn test_repr_c_union_fields_impl_clone() {
        let test_src = r#"
        #[repr(C)]
        pub union SomeUnion {
            pub x: u32,
        }

        impl Clone for SomeUnion {
            fn clone(&self) -> SomeUnion {
                return SomeUnion {x: 1}
            }
        }

        const _: () = assert!(std::mem::size_of::<SomeUnion>() == 4);
        const _: () = assert!(std::mem::align_of::<SomeUnion>() == 4);
        "#;

        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    union CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeUnion final {
                        public:
                            ...
                            __COMMENT__ "Clone::clone"
                            SomeUnion(const SomeUnion&);

                            __COMMENT__ "Clone::clone_from"
                            SomeUnion& operator=(const SomeUnion&);
                        ...
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    ...
                    static_assert(std::is_trivially_destructible_v<SomeUnion>);
                    static_assert(std::is_trivially_move_constructible_v<SomeUnion>);
                    static_assert(std::is_trivially_move_assignable_v<SomeUnion>);
                    ...
                    inline SomeUnion::SomeUnion(const SomeUnion& other) {...}
                    inline SomeUnion& SomeUnion::operator=(const SomeUnion& other) {...}
                    ...
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    ...
                    extern "C" fn ... (...) -> () {...({ <::rust_out::SomeUnion as ::core::clone::Clone>::clone(__self...) })...}
                    ...
                    extern "C" fn ... (...) -> () {...<::rust_out::SomeUnion as ::core::clone::Clone>::clone_from(__self, source)...}
                    ...
                }
            );
        })
    }

    #[test]
    fn test_repr_c_union_fields_impl_drop() {
        let test_src = r#"
        #[repr(C)]
        pub union SomeUnion {
            pub x: u32,
        }

        impl Drop for SomeUnion {
            fn drop(&mut self) {
                println!(":)")
            }
        }

        const _: () = assert!(std::mem::size_of::<SomeUnion>() == 4);
        const _: () = assert!(std::mem::align_of::<SomeUnion>() == 4);
        "#;

        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    union CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeUnion final {
                        public:
                            ...
                            __COMMENT__ "Drop::drop"
                            ~SomeUnion();

                            ...
                            SomeUnion(SomeUnion&&) = delete;
                            SomeUnion& operator=(SomeUnion&&) = delete;
                            ...
                        ...
                    };
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    ...
                    inline SomeUnion::~SomeUnion() {...}
                    ...
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    ...
                    extern "C" fn ... (__self: &mut ::core::mem::MaybeUninit<::rust_out::SomeUnion>...) { unsafe { __self.assume_init_drop() }; }
                    ...
                }
            );
        })
    }

    #[test]
    fn test_repr_c_enum_drop() {
        let test_src = r#"
        #[repr(C, i32)]
        pub enum SomeEnum {
            A(i32),
            B{x: u32},
            C,
            D{foo: i32, bar: i32} = 3,
        }

        impl Drop for SomeEnum {
            fn drop(&mut self) {
                println!(":)")
            }
        }

        const _: () = assert!(std::mem::size_of::<SomeEnum>() == 12);
        const _: () = assert!(std::mem::align_of::<SomeEnum>() == 4);
        "#;

        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) ... [[clang::trivial_abi]] SomeEnum final {
                        public:
                            ...
                            __COMMENT__ "Drop::drop"
                            ~SomeEnum();

                            ...
                            SomeEnum(SomeEnum&&) = delete;
                            SomeEnum& operator=(SomeEnum&&) = delete;
                            ...
                        ...
                    };
                }
            );
        })
    }

    #[test]
    fn test_repr_c_enum_clone() {
        let test_src = r#"
        #[repr(C, i32)]
        pub enum SomeEnum {
            A(i32),
            B{x: u32},
            C,
            D{foo: i32, bar: i32} = 3,
        }

        impl Clone for SomeEnum {
            fn clone(&self) -> SomeEnum {
                return SomeEnum::A(1)
            }
        }

        const _: () = assert!(std::mem::size_of::<SomeEnum>() == 12);
        const _: () = assert!(std::mem::align_of::<SomeEnum>() == 4);
        "#;

        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) ... [[clang::trivial_abi]] SomeEnum final {
                        public:
                            ...
                            __COMMENT__ "Clone::clone"
                            SomeEnum(const SomeEnum&);

                            __COMMENT__ "Clone::clone_from"
                            SomeEnum& operator=(const SomeEnum&);
                        ...
                    };
                }
            );
        })
    }

    pub(crate) fn test_ty<TestFn, Expectation>(
        type_location: TypeLocation,
        testcases: &[(&str, Expectation)],
        preamble: TokenStream,
        test_fn: TestFn,
    ) where
        TestFn: for<'tcx> Fn(
                /* testcase_description: */ &str,
                TyCtxt<'tcx>,
                SugaredTy<'tcx>,
                &Expectation,
            ) + Sync,
        Expectation: Sync,
    {
        for (index, (input, expected)) in testcases.iter().enumerate() {
            let desc = format!("test #{index}: test input: `{input}`");
            let input = {
                let ty_tokens: TokenStream = input.parse().unwrap();
                let input = match type_location {
                    TypeLocation::FnReturn => quote! {
                        #preamble
                        pub fn test_function() -> #ty_tokens { unimplemented!() }
                    },
                    TypeLocation::FnParam => quote! {
                        #preamble
                        pub fn test_function(_arg: #ty_tokens) { unimplemented!() }
                    },
                    TypeLocation::Other => unimplemented!(),
                };
                input.to_string()
            };
            run_compiler_for_testing(input, |tcx| {
                let (sig_mid, sig_hir) = crate::generate_function::get_fn_sig(
                    tcx,
                    find_def_id_by_name(tcx, "test_function"),
                );
                let ty = match type_location {
                    TypeLocation::FnReturn => {
                        let rustc_hir::FnRetTy::Return(ty_hir) = sig_hir.output else {
                            unreachable!(
                                "HIR return type should be fully specified, got: {:?}",
                                sig_hir.output
                            );
                        };
                        SugaredTy::new(sig_mid.output(), Some(ty_hir))
                    }
                    TypeLocation::FnParam => {
                        SugaredTy::new(sig_mid.inputs()[0], Some(&sig_hir.inputs[0]))
                    }
                    TypeLocation::Other => unimplemented!(),
                };
                test_fn(&desc, tcx, ty, expected);
            });
        }
    }

    /// Tests invoking `generate_item` on the item with the specified `name`
    /// from the given Rust `source`.  Returns the result of calling
    /// `test_function` with `generate_item`'s result as an argument.
    /// (`test_function` should typically `assert!` that it got the expected
    /// result from `generate_item`.)
    pub(crate) fn test_format_item<F, T>(source: &str, name: &str, test_function: F) -> T
    where
        F: FnOnce(Result<Option<ApiSnippets>, String>) -> T + Send,
        T: Send,
    {
        run_compiler_for_testing(source, |tcx| {
            let def_id = find_def_id_by_name(tcx, name);
            let result = bindings_db_for_tests(tcx).generate_item(def_id);

            // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations says:
            // To print causes as well [...], use the alternate selector “{:#}”.
            let result = result.map_err(|anyhow_err| format!("{anyhow_err:#}"));

            test_function(result)
        })
    }

    /// Tests invoking `generate_item` on the item with the specified `name`
    /// from the given Rust `source`, with the specified features  Returns
    /// the result of calling `test_function` with `generate_item`'s result
    /// as an argument. (`test_function` should typically `assert!` that it
    /// got the expected result from `generate_item`.)
    pub(crate) fn test_format_item_with_features<F, T>(
        source: &str,
        name: &str,
        features: impl Into<flagset::FlagSet<crubit_feature::CrubitFeature>>,
        test_function: F,
    ) -> T
    where
        F: FnOnce(Result<Option<ApiSnippets>, String>) -> T + Send,
        T: Send,
    {
        let features = features.into();
        run_compiler_for_testing(source, |tcx| {
            let def_id = find_def_id_by_name(tcx, name);
            let result = bindings_db_for_tests_with_features(tcx, features).generate_item(def_id);

            // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations says:
            // To print causes as well [...], use the alternate selector “{:#}”.
            let result = result.map_err(|anyhow_err| format!("{anyhow_err:#}"));

            test_function(result)
        })
    }

    pub(crate) fn bindings_db_for_tests_with_features(
        tcx: TyCtxt,
        features: impl Into<flagset::FlagSet<crubit_feature::CrubitFeature>>,
    ) -> Database {
        Database::new(
            tcx,
            /* crubit_support_path_format= */ "<crubit/support/for/tests/{header}>".into(),
            /* default_features= */ Default::default(),
            /* crate_name_to_include_paths= */ Default::default(),
            /* crate_name_to_features= */
            Rc::new(HashMap::from([(Rc::from("self"), features.into())])),
            /* crate_name_to_namespace= */ HashMap::default().into(),
            /* crate_renames= */ HashMap::default().into(),
            /* errors = */ Rc::new(IgnoreErrors),
            /* no_thunk_name_mangling= */ true,
            /* include_guard */ IncludeGuard::PragmaOnce,
        )
    }

    pub(crate) fn bindings_db_for_tests(tcx: TyCtxt) -> Database {
        bindings_db_for_tests_with_features(
            tcx,
            crubit_feature::CrubitFeature::Experimental | crubit_feature::CrubitFeature::Supported,
        )
    }

    /// Tests invoking `generate_bindings` on the given Rust `source`.
    /// Returns the result of calling `test_function` with the generated
    /// bindings as an argument. (`test_function` should typically `assert!`
    /// that it got the expected `GeneratedBindings`.)
    pub(crate) fn test_generated_bindings<F, T>(source: &str, test_function: F) -> T
    where
        F: FnOnce(Result<BindingsTokens>) -> T + Send,
        T: Send,
    {
        run_compiler_for_testing(source, |tcx| {
            test_function(generate_bindings(&bindings_db_for_tests(tcx)))
        })
    }
}
