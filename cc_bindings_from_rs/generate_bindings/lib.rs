// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(rustc_private)]
#![feature(cfg_accessible)]
#![deny(rustc::internal)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

extern crate rustc_abi;
extern crate rustc_ast;
extern crate rustc_attr_parsing;
extern crate rustc_hir;
extern crate rustc_infer;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;
extern crate rustc_type_ir;

pub mod format_type;
pub mod generate_function;
mod generate_function_thunk;
mod generate_struct_and_union;

use crate::format_type::{
    create_canonical_name_from_foreign_path, crubit_abi_type_from_ty, ensure_ty_is_pointer_like,
    format_cc_ident, format_cc_ident_symbol, format_param_types_for_cc,
    format_region_as_cc_lifetime, format_ret_ty_for_cc, format_top_level_ns_for_crate,
    is_bridged_type, BridgedBuiltin, BridgedType, BridgedTypeConversionInfo,
};
use crate::generate_function::{generate_function, must_use_attr_of};
use crate::generate_function_thunk::{generate_trait_thunks, TraitThunks};
use crate::generate_struct_and_union::{generate_adt, generate_adt_core, scalar_value_to_string};
use arc_anyhow::{Context, Error, Result};
use code_gen_utils::{format_cc_includes, CcConstQualifier, CcInclude, NamespaceQualifier};
use database::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet, ExternCDecl, RsSnippet};
use database::{
    AdtCoreBindings, BindingsGenerator, FineGrainedFeature, FullyQualifiedName, SugaredTy,
    TypeLocation,
};
pub use database::{Database, IncludeGuard};
use error_report::{anyhow, bail, ErrorReporting, ReportFatalError};
use itertools::Itertools;
use proc_macro2::TokenStream;
use query_compiler::{
    does_type_implement_trait, get_layout, get_scalar_int_type, get_tag_size_with_padding,
    is_c_abi_compatible_by_value, is_copy, is_directly_public, is_exported,
    liberate_and_deanonymize_late_bound_regions, post_analysis_typing_env, repr_attrs,
};
use quote::{format_ident, quote};
use rustc_abi::{AddressSpace, BackendRepr, Integer, Primitive, Scalar};
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{self as hir, Item, ItemKind, Node, UseKind, UsePath};
use rustc_middle::dep_graph::DepContext;
use rustc_middle::mir::ConstValue;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::def_id::{CrateNum, DefId, LocalDefId, LOCAL_CRATE};
use rustc_span::symbol::{sym, Ident, Symbol};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::iter::once;
use std::rc::Rc;

/// Implementation of `BindingsGenerator::support_header`.
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

/// Wrap `repr_attrs` for use as a database function.
fn repr_attrs_from_db(
    db: &dyn BindingsGenerator<'_>,
    def_id: DefId,
) -> Rc<[rustc_hir::attrs::ReprAttr]> {
    repr_attrs(db.tcx(), def_id)
}

fn source_crate_num(db: &dyn BindingsGenerator<'_>) -> CrateNum {
    let Some(source_crate_name) = db.source_crate_name() else {
        return LOCAL_CRATE;
    };
    let source_crate_name = Symbol::intern(&*source_crate_name);
    let tcx = db.tcx();
    let Some(crate_num) = tcx
        .used_crates(())
        .iter()
        .copied()
        .find(|&crate_num| tcx.crate_name(crate_num) == source_crate_name)
    else {
        db.fatal_errors()
            .report(&format!("Failed to resolve source crate name: `{source_crate_name}`"));
        return LOCAL_CRATE;
    };
    crate_num
}

pub fn new_database<'db>(
    tcx: TyCtxt<'db>,
    source_crate_name: Option<Rc<str>>,
    crubit_support_path_format: Rc<str>,
    default_features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    crate_name_to_include_paths: Rc<HashMap<Rc<str>, Vec<CcInclude>>>,
    crate_name_to_features: Rc<HashMap<Rc<str>, flagset::FlagSet<crubit_feature::CrubitFeature>>>,
    crate_name_to_namespace: Rc<HashMap<Rc<str>, Rc<str>>>,
    crate_renames: Rc<HashMap<Rc<str>, Rc<str>>>,
    errors: Rc<dyn ErrorReporting>,
    fatal_errors: Rc<dyn ReportFatalError>,
    no_thunk_name_mangling: bool,
    h_out_include_guard: IncludeGuard,
    show_warnings: bool,
) -> Database<'db> {
    Database::new(
        tcx,
        source_crate_name,
        crubit_support_path_format,
        default_features,
        crate_name_to_include_paths,
        crate_name_to_features,
        crate_name_to_namespace,
        crate_renames,
        errors,
        fatal_errors,
        no_thunk_name_mangling,
        h_out_include_guard,
        show_warnings,
        source_crate_num,
        support_header,
        repr_attrs_from_db,
        reexported_symbol_canonical_name_mapping,
        format_cc_ident_symbol,
        format_top_level_ns_for_crate,
        format_type::format_ty_for_cc,
        format_type::format_ty_for_rs,
        generate_default_ctor,
        generate_copy_ctor_and_assignment_operator,
        generate_move_ctor_and_assignment_operator,
        generate_item,
        generate_function,
        generate_adt_core,
        crubit_abi_type_from_ty,
    )
}

pub fn generate_bindings(db: &Database) -> Result<BindingsTokens> {
    let tcx = db.tcx();

    let top_comment = {
        let source_crate_num = db.source_crate_num();
        let crate_name = tcx.crate_name(source_crate_num);
        let crubit_features = {
            let mut crubit_features: Vec<&str> = crate_features(db, source_crate_num)
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

    let allow_warnings = if !db.show_warnings() {
        quote! { #![allow(warnings)] }
    } else {
        quote! {}
    };

    let cc_api_impl = quote! {
        #top_comment

        #allow_warnings

        #![allow(unused_unsafe)]

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
    mut tokens: TokenStream,
    attributes: Vec<TokenStream>,
) -> Result<TokenStream> {
    let mut namespaces = ns.parts().map(|s| format_cc_ident(db, s)).collect::<Result<Vec<_>>>()?;

    // Nested namespace syntax does not accept attributes (see b/445613694), so we have to split out
    // the with-attribute decl to contain only the trailing namespace.
    if !attributes.is_empty() {
        let innermost_namespace = namespaces
            .pop()
            .expect("there should be at least one namespace if there are attributes");
        tokens = quote! {
            __NEWLINE__
            namespace #(#attributes)* #innermost_namespace { __NEWLINE__
                #tokens __NEWLINE__
            } __NEWLINE__
        };
    }

    if !namespaces.is_empty() {
        tokens = quote! {
            __NEWLINE__
            namespace #(#namespaces)::* { __NEWLINE__
                #tokens __NEWLINE__
            } __NEWLINE__
        };
    }
    Ok(tokens)
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
    // TODO: b/433286909 - Support adding aliases to the name map even if they aren't used by the
    // local crate.
    tcx.hir_visit_all_item_likes_in_crate(&mut visitor);

    visitor.symbols
}

#[derive(Debug)]
struct AliasInfo {
    using_name: String,
    local_def_id: LocalDefId,
    type_def_id: DefId,
    def_kind: DefKind,
}

fn create_canonical_name(
    db: &dyn BindingsGenerator<'_>,
    name_map: &HashMap<DefId, FullyQualifiedName>,
    alias_info: &AliasInfo,
) -> Option<FullyQualifiedName> {
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
        con_name.rs_mod_path.parts().cloned().collect()
    } else {
        let mut full_path = tcx.def_path(def_id).data; // mod_path + name
        full_path.pop().expect("At least the use exists");
        full_path
            .into_iter()
            .filter_map(|p| p.data.get_opt_name())
            .map(|s| Rc::<str>::from(s.as_str()))
            .collect()
    };

    if krate.as_str() == "polars_plan" && matches!(rs_name.as_str(), "date_range" | "time_range") {
        // Short-circuit `polars_plan::dsl::{date_range, time_range}`.
        //
        // These two paths are the result of a pathological chain of ambiguous reexports that is
        // not supported by the compiler and must not be considered.
        //
        // See https://github.com/rust-lang/rust/issues/144333 for details.
        let path_strs: Vec<&str> = full_path_strs.iter().map(|x| &**x).collect();
        if matches!(&*path_strs, ["dsl"]) {
            return None;
        }
    }

    let rs_mod_path = NamespaceQualifier::new(full_path_strs.clone());

    let cpp_ns_path =
        NamespaceQualifier::new(full_path_strs.into_iter().chain([Rc::from("__crubit_internal")]));
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
}

/// Implementation of `BindingsGenerator::reexported_symbol_canonical_name_mapping`.
// TODO(b/350772554): Don't generate bindings for ambiguous symbols.
fn reexported_symbol_canonical_name_mapping(
    db: &dyn BindingsGenerator<'_>,
) -> HashMap<DefId, FullyQualifiedName> {
    let tcx = db.tcx();
    let mut name_map: HashMap<DefId, FullyQualifiedName> =
        symbols_from_extern_crate(db).into_iter().collect();

    let aliases = tcx
        .hir_free_items()
        .filter_map(|item_id| {
            let local_def_id: LocalDefId = item_id.owner_id.def_id;
            let Item { kind: kind @ ItemKind::Use(use_path, use_kind), .. } =
                tcx.hir_expect_item(local_def_id)
            else {
                return None;
            };
            let ident_str = &kind.ident().map_or("".to_owned(), |ident| ident.as_str().to_owned());
            // TODO(b/350772554): Preserve the errors.
            collect_alias_from_use(db, ident_str, use_path, use_kind).ok().map(|aliases| {
                aliases.into_iter().map(move |(using_name, type_def_id, def_kind)| AliasInfo {
                    using_name,
                    local_def_id,
                    type_def_id,
                    def_kind,
                })
            })
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
        if let Some(canonical_name) = create_canonical_name(db, &name_map, &alias_info) {
            name_map.insert(alias_info.type_def_id, canonical_name);
        }
    }

    name_map
}

/// Checks whether a definition matches a specific qualified name.
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
pub enum CcType {
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

/// Returns the C++ must_use tag for the item identified by `def_id`, or None if there is no such
/// tag.
fn generate_must_use_tag(tcx: TyCtxt, def_id: DefId) -> Option<TokenStream> {
    if let Some(must_use_attr) = must_use_attr_of(tcx, def_id) {
        let cc_must_use_tag = match must_use_attr.reason {
            None => quote! {[[nodiscard]]},
            Some(reason) => {
                let reason = reason.as_str();
                quote! {[[nodiscard(#reason)]]}
            }
        };
        return Some(cc_must_use_tag);
    }
    None
}

/// Returns the C++ deprecated tag for the item identified by `def_id`, if it is
/// deprecated. Otherwise, returns None.
fn generate_deprecated_tag(tcx: TyCtxt, def_id: DefId) -> Option<TokenStream> {
    use rustc_hir::attrs::AttributeKind;
    use rustc_hir::find_attr;

    if let Some((deprecation, _span)) = find_attr!(tcx.get_all_attrs(def_id), AttributeKind::Deprecation{deprecation, span} => (*deprecation, *span))
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
            if !def_id.is_local() {
                bail!("`use` of external functions is not yet supported");
            }
            // TODO(b/350772554): Support exporting private functions.
            match db.generate_function(def_id) {
                Ok(snippet) => {
                    prereqs = snippet.main_api.prereqs;
                }
                Err(err) => {
                    bail!("Unable to `use` function whose bindings failed: {err:?}");
                }
            }
            let fully_qualified_fn_name = FullyQualifiedName::new(db, def_id);
            let formatted_fully_qualified_fn_name = fully_qualified_fn_name.format_for_cc(db)?;
            let main_api_fn_name =
                format_cc_ident(db, fully_qualified_fn_name.cpp_name.unwrap().as_str())
                    .context("Error formatting function name")?;
            let using_name =
                format_cc_ident(db, using_name).context("Error formatting using name")?;

            prereqs.defs.insert(def_id);
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
        DefKind::TyAlias => generate_type_alias(db, def_id, using_name),
        _ => {
            bail!("Unsupported use statement that refers to this type of the entity: {:#?}", def_id)
        }
    }
}

fn debug_print_use_path(use_path: &UsePath) -> String {
    use_path.segments.iter().map(|segment| segment.ident.as_str()).collect::<Vec<&str>>().join("::")
}

fn use_path_as_single_res(use_path: &UsePath) -> Result<Res> {
    // TODO(b/350772554): Support multiple items with the same name in `use`
    // statements.`
    let mut present_items: Vec<Res> =
        use_path.res.present_items().filter(|res| !matches!(res, Res::Err)).collect();

    // Filter `Ctor` functions with the same names as their types.
    if present_items.len() > 1 {
        present_items.retain(|item| !matches!(item, Res::Def(DefKind::Ctor(..), _)))
    }

    if present_items.is_empty() {
        bail!("`use` path `{}` resolved to no non-error items", debug_print_use_path(use_path))
    }

    if present_items.len() > 1 {
        bail!(
            "`use` path `{}` resolved to multiple items with the same name: {:?}",
            debug_print_use_path(use_path),
            use_path.res.present_items().collect::<Vec<_>>()
        );
    }

    Ok(present_items.into_iter().next().unwrap())
}

struct DefInfo {
    ident: Ident,
    def_id: DefId,
    def_kind: DefKind,
}

enum PublicOnly {
    Yes,
    No,
}

/// Returns all public definitions in the given module.
fn defs_in_mod<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    public_only: PublicOnly,
) -> impl Iterator<Item = DefInfo> + use<'tcx> {
    let module_children = match def_id.as_local() {
        None => tcx.module_children(def_id),
        // Local `module_children` does not use the query due to perf impacts.
        Some(local_def_id) => tcx.module_children_local(local_def_id),
    };
    module_children.iter().filter_map(move |mod_child| {
        if matches!(public_only, PublicOnly::Yes) && !mod_child.vis.is_public() {
            return None;
        }

        let hir::def::Res::Def(mut item_def_kind, mut item_def_id) = mod_child.res else {
            // TODO(b/350772554): Support PrimTy.
            return None;
        };

        // For re-exported items, we want to return the ID of the re-export itself, not the ID of
        // the re-export target.
        if let Some(&reexport) = mod_child.reexport_chain.first() {
            if let Some(reexport_id) = reexport.id() {
                item_def_id = reexport_id;
                item_def_kind = tcx.def_kind(reexport_id);
            }
        }

        if let Some(stability) = tcx.lookup_stability(item_def_id) {
            if stability.is_unstable() {
                return None;
            }
        }

        // Omit tuple Ctors functions as top-level C++ items. They are instead emitted only as part
        // of the type with the same name.
        if matches!(item_def_kind, DefKind::Ctor(..)) {
            return None;
        }

        Some(DefInfo { ident: mod_child.ident, def_id: item_def_id, def_kind: item_def_kind })
    })
}

/// Returns all public items of a bindable kind (fns, structs, enums, using statements) in the given
/// module and its nested modules.
fn defs_in_mod_recursive(tcx: TyCtxt, def_id: DefId) -> Vec<DefInfo> {
    let mut items = vec![];
    // List of child modules to visit.
    let mut mods_to_visit = vec![def_id];
    let mut visited = HashSet::new();
    while let Some(mod_id) = mods_to_visit.pop() {
        items.extend(
            defs_in_mod(tcx, mod_id, PublicOnly::No)
                .filter(|info| visited.insert(info.def_id))
                .inspect(|def_info| {
                    if matches!(def_info.def_kind, DefKind::Mod) {
                        mods_to_visit.push(def_info.def_id);
                    }
                }),
        );
    }
    items
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
    let res = use_path_as_single_res(use_path)?;
    // TODO(b/350772554): Support PrimTy.
    let Res::Def(def_kind, def_id) = res else {
        bail!(
            "`use` statement `{}` refers to unsupported definition kind: {:#?}",
            debug_print_use_path(use_path),
            res
        );
    };
    // TODO(b/350772554): Support `use some_module`.
    if def_kind == DefKind::Mod && !matches!(use_kind, UseKind::Glob) {
        bail!("`use` of a module (`{}`) is not yet supported", debug_print_use_path(use_path))
    }

    let mut aliases = vec![];
    if def_kind == DefKind::Mod {
        for DefInfo { ident, def_id: item_def_id, def_kind: item_def_kind } in
            defs_in_mod(db.tcx(), def_id, PublicOnly::Yes)
        {
            // TODO(b/350772554): Support export Enum fields.
            if !ident.name.is_empty() {
                aliases.push((ident.name.to_string(), item_def_id, item_def_kind));
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

fn generate_const(db: &dyn BindingsGenerator<'_>, def_id: DefId) -> Result<ApiSnippets> {
    let tcx = db.tcx();
    let unsupported_node_item_msg = "Called `generate_const` with a `rustc_hir::Node` that is not a `Node::Item` or `Node::ImplItem`";
    let ty = tcx.type_of(def_id).instantiate_identity();
    let hir_ty = def_id.as_local().map(|local_def_id| {
        let hir_node = tcx.hir_node_by_def_id(local_def_id);
        match hir_node {
            Node::Item(item) => item.expect_const().2,
            Node::ImplItem(item) => item.expect_const().0,
            _ => panic!("{}", unsupported_node_item_msg),
        }
    });
    let rust_type = SugaredTy::new(ty, hir_ty);
    let cc_type_snippet = db.format_ty_for_cc(rust_type, TypeLocation::Const)?;

    let cc_type = cc_type_snippet.tokens;
    let cc_name = format_cc_ident(db, tcx.item_name(def_id).as_str())?;

    // Note that `&str` constants may appear as either `ConstValue::Slice` or
    // `ConstValue::Indirect`.
    let const_value: ConstValue = tcx.const_eval_poly(def_id).unwrap();
    let cc_value = match const_value {
        ConstValue::Scalar(scalar) => scalar_value_to_string(tcx, scalar, *ty.kind()),
        ConstValue::ZeroSized => bail!("const of type `{rust_type}` cannot be generated as zero-sized consts are not supported in C++."),
        ConstValue::Slice { .. } | ConstValue::Indirect { .. } => {
            let string_literal = match ty.kind() {
                ty::TyKind::Ref(_region, referent_ty, mutability)
                    if matches!(referent_ty.kind(), ty::TyKind::Str) =>
                {
                    if mutability.is_mut() {
                        panic!("Unexpected mutable reference in a constant of type `{rust_type}`")
                    }
                    if let Some(slice) = const_value.try_get_slice_bytes_for_diagnostics(tcx) {
                        let str_data = std::str::from_utf8(slice).unwrap();
                        Some(quote! { rs_std::StrRef(#str_data) }.to_string())
                    } else { None }
                }
                _ => None
            };
            string_literal.ok_or_else(|| {
                anyhow!("const of type `{rust_type}` cannot be generated as only scalar consts are supported.")
            })
        }
    }?
    .parse::<TokenStream>()
    .unwrap();

    Ok(ApiSnippets {
        main_api: CcSnippet {
            tokens: quote! {
                static constexpr #cc_type #cc_name = #cc_value;
            },
            ..cc_type_snippet
        },
        cc_details: CcSnippet::default(),
        rs_details: RsSnippet::default(),
    })
}

fn generate_type_alias(
    db: &dyn BindingsGenerator<'_>,
    def_id: DefId,
    using_name: &str,
) -> Result<ApiSnippets> {
    let tcx = db.tcx();
    let hir_ty = def_id.as_local().map(|local_def_id| {
        let Item { kind: ItemKind::TyAlias(_, _, hir_ty, ..), .. } =
            tcx.hir_expect_item(local_def_id)
        else {
            panic!("called generate_type_alias on a non-type-alias");
        };
        *hir_ty
    });
    let alias_type = SugaredTy::new(tcx.type_of(def_id).instantiate_identity(), hir_ty);
    create_type_alias(db, def_id, using_name, alias_type)
}

fn create_type_alias<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    def_id: DefId,
    alias_name: &str,
    alias_type: SugaredTy<'tcx>,
) -> Result<ApiSnippets> {
    let cc_bindings = db.format_ty_for_cc(alias_type, TypeLocation::Other)?;
    let mut main_api_prereqs = CcPrerequisites::default();
    let actual_type_name = cc_bindings.into_tokens(&mut main_api_prereqs);

    let alias_name = format_cc_ident(db, alias_name).context("Error formatting type alias name")?;

    let fully_qualified_name = database::FullyQualifiedName::new(db, def_id);
    let rs_type = format!("{}", fully_qualified_name.format_for_rs());

    main_api_prereqs.includes.insert(db.support_header("annotations_internal.h"));
    let mut attributes = vec![quote! {CRUBIT_INTERNAL_RUST_TYPE(#rs_type)}];
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

/// Implementation of `BindingsGenerator::generate_default_ctor`.
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

/// Implementation of `BindingsGenerator::generate_copy_ctor_and_assignment_operator`.
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

        if is_copy(tcx, core.def_id, core.self_ty) {
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

/// Implementation of `BindingsGenerator::generate_move_ctor_and_assignment_operator`.
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
        if generate_struct_and_union::adt_core_bindings_needs_drop(&core, tcx) {
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
fn generate_fwd_decl(db: &Database<'_>, def_id: DefId) -> TokenStream {
    // `generate_fwd_decl` should only be called for items from
    // `CcPrerequisites::fwd_decls` and `fwd_decls` should only contain ADTs
    // that `generate_adt_core` succeeds for.
    let core_bindings = db
        .generate_adt_core(def_id)
        .expect("`generate_fwd_decl` should only be called if `generate_adt_core` succeeded");
    let AdtCoreBindings { keyword, cc_short_name, .. } = &*core_bindings;

    quote! { #keyword #cc_short_name; }
}

fn generate_source_location(tcx: TyCtxt, def_id: DefId) -> String {
    let def_span = tcx.def_span(def_id);
    let rustc_span::FileLines { file, lines } =
        match tcx.sess().source_map().span_to_lines(def_span) {
            Ok(filelines) => filelines,
            Err(_) => return "unknown location".to_string(),
        };
    let file_name = file.name.prefer_local().to_string();
    // Note: line_index starts at 0, while CodeSearch starts indexing at 1.
    let line_number = lines[0].line_index + 1;
    format!("{file_name};l={line_number}")
}

/// Formats the doc comment (if any) associated with the item identified by
/// `local_def_id`, and appends the source location at which the item is
/// defined.
fn generate_doc_comment(tcx: TyCtxt, def_id: DefId) -> TokenStream {
    let doc_comment = tcx
        .get_all_attrs(def_id)
        .iter()
        .filter_map(|attr| attr.doc_str())
        .map(|symbol| symbol.to_string())
        .chain(once(format!("Generated from: {}", generate_source_location(tcx, def_id))))
        .join("\n\n");
    quote! { __COMMENT__ #doc_comment}
}

/// Returns the name of the item identified by `def_id`, or "<unknown>" if
/// the item can't be identified.
fn item_name(db: &dyn BindingsGenerator<'_>, def_id: DefId) -> Symbol {
    db.tcx().opt_item_name(def_id).unwrap_or_else(|| Symbol::intern("<unknown>"))
}

/// Implementation of `BindingsGenerator::generate_item`.
fn generate_item(db: &dyn BindingsGenerator<'_>, def_id: DefId) -> Result<Option<ApiSnippets>> {
    let tcx = db.tcx();
    let generated = generate_item_impl(db, def_id);
    let attributes = crubit_attr::get_attrs(tcx, def_id).unwrap();
    if attributes.must_bind {
        if let Err(e) = &generated {
            let item_name = item_name(db, def_id);
            let must_bind_message = format!(
                "Failed to generate bindings for `{item_name}`:\n\
                {e:?}\n\
                This is a hard error because `{item_name}` was annotated with \
                `#[crubit_annotate::must_bind]`"
            );
            db.fatal_errors().report(&must_bind_message);
        }
    }
    generated
}

// A helper for `generate_item`.
// The wrapper is used to ensure that the `must_bind` annotation is enforced.
fn generate_item_impl(
    db: &dyn BindingsGenerator<'_>,
    def_id: DefId,
) -> Result<Option<ApiSnippets>> {
    let tcx = db.tcx();

    // TODO(b/350772554): Support `use` mod.
    if !is_public_or_supported_export(db, def_id) {
        return Ok(None);
    }

    let item = match tcx.def_kind(def_id) {
        DefKind::Struct | DefKind::Enum | DefKind::Union => {
            let attributes = crubit_attr::get_attrs(tcx, def_id).unwrap();

            let has_composable_bridging_attrs = matches!(
                attributes.get_bridging_attrs()?,
                Some(crubit_attr::BridgingAttrs::Composable { .. })
            );

            if !has_composable_bridging_attrs
                && BridgedBuiltin::new(db, tcx.adt_def(def_id)).is_none()
                && query_compiler::has_non_lifetime_generics(tcx, def_id)
            {
                bail!("Generic types are not supported yet (b/259749095)");
            }

            if let Some(cpp_type) = attributes.cpp_type {
                let item_name = tcx.def_path_str(def_id);
                bail!(
                    "Type bindings for {item_name} suppressed due to being mapped to \
                            an existing C++ type ({cpp_type})"
                );
            }
            db.generate_adt_core(def_id).map(|core| Some(generate_adt(db, core)))
        }
        DefKind::Fn => db.generate_function(def_id).map(Some),
        DefKind::TyAlias => {
            generate_type_alias(db, def_id, tcx.item_name(def_id).as_str()).map(Some)
        }
        DefKind::Use => {
            let Some(local_def_id) = def_id.as_local() else {
                bail!("Generated bindings for `use` from remote crate not yet supported.");
            };
            let Item { kind: kind @ ItemKind::Use(use_path, use_kind), .. } =
                tcx.hir_expect_item(local_def_id)
            else {
                panic!("Use item not found");
            };
            let ident_str = &kind.ident().map_or("".to_owned(), |ident| ident.as_str().to_owned());
            generate_use(db, ident_str, use_path, use_kind).map(Some)
        }
        DefKind::Const => generate_const(db, def_id).map(Some),
        DefKind::Impl { .. } => Ok(None), // Handled by `generate_adt`
        DefKind::Mod => Ok(None),         // Handled by `generate_crate`
        kind => bail!("Unsupported rustc_hir::hir::DefKind: {kind:?}"),
    };

    if let Ok(Some(item)) = item {
        Ok(Some(item.resolve_feature_requirements(crate_features(db, db.source_crate_num()))?))
    } else {
        item
    }
}

/// Formats a C++ comment explaining why no bindings have been generated for
/// `local_def_id`.
fn generate_unsupported_def(
    db: &dyn BindingsGenerator<'_>,
    def_id: DefId,
    err: Error,
) -> ApiSnippets {
    let tcx = db.tcx();
    db.errors().report(&err);
    let source_loc = generate_source_location(tcx, def_id);
    let name = tcx.def_path_str(def_id);

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
pub fn format_namespace_bound_cc_tokens(
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
                let name = ns.parts().join("::");
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

/// Compares two `DefId` s
pub(crate) fn stable_def_id_cmp<'tcx>(tcx: TyCtxt<'tcx>, lhs_id: DefId, rhs_id: DefId) -> Ordering {
    let lhs_span = tcx.def_span(lhs_id);
    let rhs_span = tcx.def_span(rhs_id);
    if lhs_span.source_equal(rhs_span) {
        let lhs_def_path_hash = tcx.def_path_hash(lhs_id);
        let rhs_def_path_hash = tcx.def_path_hash(rhs_id);
        lhs_def_path_hash.cmp(&rhs_def_path_hash)
    } else {
        lhs_span.cmp(&rhs_span)
    }
}

pub(crate) trait SortedByDef: Iterator + Sized {
    fn sorted_by_def<'tcx>(self, tcx: TyCtxt<'tcx>) -> std::vec::IntoIter<Self::Item>
    where
        Self::Item: Copy + Into<DefId>,
    {
        self.sorted_by_def_with(tcx, |item| (*item).into())
    }

    fn sorted_by_def_with<'tcx>(
        self,
        tcx: TyCtxt<'tcx>,
        mut item_to_def_id: impl FnMut(&Self::Item) -> DefId,
    ) -> std::vec::IntoIter<Self::Item> {
        self.sorted_unstable_by(|lhs, rhs| {
            stable_def_id_cmp(tcx, item_to_def_id(lhs), item_to_def_id(rhs))
        })
    }
}
impl<T: Iterator + Sized> SortedByDef for T {}

/// Formats all public items from the Rust crate being compiled.
fn generate_crate(db: &Database) -> Result<BindingsTokens> {
    let tcx = db.tcx();
    let mut cc_details_prereqs = CcPrerequisites::default();
    let mut cc_details: Vec<(DefId, TokenStream)> = vec![];
    let mut cc_api_impl = TokenStream::default();
    let mut extern_c_decls = BTreeSet::new();
    let mut main_apis = HashMap::<DefId, CcSnippet>::new();

    let defs_in_crate = defs_in_mod_recursive(tcx, db.source_crate_num().as_def_id());
    let formatted_items = defs_in_crate
        .into_iter()
        .filter_map(|def_info| {
            let def_id = def_info.def_id;
            db.generate_item(def_id)
                .unwrap_or_else(|err| Some(generate_unsupported_def(db, def_id, err)))
                .map(|api_snippets| (def_id, api_snippets))
        })
        .sorted_by_def_with(tcx, |&(id, _)| id.into());

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
                let predecessors = main_api.prereqs.defs.iter().copied().filter(|pre|
                        // Only consider `pre`s that we're currently generating APIs for.
                        main_apis.contains_key(pre));
                predecessors.map(move |predecessor| toposort::Dependency { predecessor, successor })
            });
            toposort::toposort(nodes, deps, move |&lhs_id, &rhs_id| {
                stable_def_id_cmp(tcx, lhs_id, rhs_id)
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
        let mut already_declared: HashSet<DefId> = HashSet::new();
        let mut fwd_decls: HashSet<DefId> = HashSet::new();
        let mut includes = cc_details_prereqs.includes;
        let mut ordered_main_apis: Vec<(DefId, TokenStream)> = Vec::new();
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
            .sorted_by_def(tcx)
            .map(|local_def_id| (local_def_id, generate_fwd_decl(db, local_def_id)));

        // The first item of the tuple here is the DefId of the namespace.
        let ordered_cc: Vec<(Option<DefId>, NamespaceQualifier, TokenStream)> = fwd_decls
            .into_iter()
            .chain(ordered_main_apis)
            .chain(cc_details)
            .map(|(def_id, tokens)| {
                let ns_def_id = tcx.opt_parent(def_id);
                let mod_path = FullyQualifiedName::new(db, def_id).cpp_ns_path;
                (ns_def_id, mod_path, tokens)
            })
            .collect_vec();

        (includes, ordered_cc)
    };

    // Generate top-level elements of the C++ header file.
    let cc_api = {
        let cpp_top_level_ns = format_top_level_ns_for_crate(db, db.source_crate_num())
            .iter()
            .map(|ns| db.format_cc_ident(*ns))
            .collect::<Result<Vec<_>>>()?;

        let includes = format_cc_includes(&includes);
        let ordered_cc = format_namespace_bound_cc_tokens(db, ordered_cc, tcx);
        quote! {
            #includes
            __NEWLINE__ __NEWLINE__
            namespace #(#cpp_top_level_ns)::* {
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
