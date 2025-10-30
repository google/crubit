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
    crubit_abi_type_from_ty, ensure_ty_is_pointer_like, format_cc_ident, format_cc_ident_symbol,
    format_param_types_for_cc, format_region_as_cc_lifetime, format_ret_ty_for_cc,
    format_top_level_ns_for_crate, is_bridged_type, BridgedBuiltin, BridgedType,
    BridgedTypeConversionInfo,
};
use crate::generate_function::{generate_function, must_use_attr_of};
use crate::generate_function_thunk::{generate_trait_thunks, TraitThunks};
use crate::generate_struct_and_union::{
    from_trait_impls_by_argument, generate_adt, generate_adt_core, scalar_value_to_string,
};
use arc_anyhow::{Context, Error, Result};
use code_gen_utils::{format_cc_includes, CcConstQualifier, CcInclude, NamespaceQualifier};
use database::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet, ExternCDecl, RsSnippet};
use database::{
    AdtCoreBindings, BindingsGenerator, ExportedPath, FineGrainedFeature, FullyQualifiedName,
    PublicPaths, SugaredTy, TypeLocation,
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
use rustc_hir::{Item, ItemKind, Node};
use rustc_middle::dep_graph::DepContext;
use rustc_middle::metadata::ModChild;
use rustc_middle::mir::ConstValue;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::def_id::{CrateNum, DefId, LOCAL_CRATE};
use rustc_span::symbol::{sym, Symbol};
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
    crubit_support_path_format: dyn_format::Format<1>,
    crubit_debug_path_format: Option<dyn_format::Format<2>>,
    default_features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    crate_name_to_include_paths: Rc<HashMap<Rc<str>, Vec<CcInclude>>>,
    crate_name_to_features: Rc<HashMap<Rc<str>, flagset::FlagSet<crubit_feature::CrubitFeature>>>,
    crate_name_to_namespace: Rc<HashMap<Rc<str>, Rc<str>>>,
    crate_renames: Rc<HashMap<Rc<str>, Rc<str>>>,
    errors: Rc<dyn ErrorReporting>,
    fatal_errors: Rc<dyn ReportFatalError>,
    no_thunk_name_mangling: bool,
    h_out_include_guard: IncludeGuard,
) -> Database<'db> {
    Database::new(
        tcx,
        source_crate_name,
        crubit_support_path_format,
        crubit_debug_path_format,
        default_features,
        crate_name_to_include_paths,
        crate_name_to_features,
        crate_name_to_namespace,
        crate_renames,
        errors,
        fatal_errors,
        no_thunk_name_mangling,
        h_out_include_guard,
        source_crate_num,
        support_header,
        repr_attrs_from_db,
        symbol_canonical_name,
        public_paths_by_def_id,
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
        from_trait_impls_by_argument,
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

    let cc_api_impl = quote! {
        #top_comment

        #![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)] __NEWLINE__

        // `rust_builtin_type_abi_assumptions.md` documents why the generated
        // bindings need to relax the `improper_ctypes_definitions` warning
        // for `char` (and possibly for other built-in types in the future).
        #![allow(improper_ctypes_definitions)] __NEWLINE__
        #![deny(warnings)]  __NEWLINE__

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
    // TODO: b/455882065 - Figure out the correct way to handle top level aliases of deprecated
    // definitions and remove this workaround.
    if !attributes.is_empty() && !namespaces.is_empty() {
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

/// Implementation of `BindingsGenerator::public_paths_by_def_id`.
fn public_paths_by_def_id(
    db: &dyn BindingsGenerator<'_>,
    crate_num: CrateNum,
) -> HashMap<DefId, PublicPaths> {
    /// This is retooled logic from rustc's `visible_parent_map` function. Except where that only
    /// selects the shortest visible path, we track all paths and defer selecting the correct one
    /// to callers.
    use rustc_middle::metadata::ModChild;
    use rustc_span::kw;
    use std::collections::vec_deque::VecDeque;

    let tcx = db.tcx();
    let mut visible_parent_map = HashMap::default();

    let bfs_queue = &mut VecDeque::new();
    let mut module_seen_set = HashSet::new();

    struct ModLikeDef {
        path_so_far: Vec<Symbol>,
        is_doc_hidden: bool,
        def_id: DefId,
    }

    bfs_queue.push_back(ModLikeDef {
        path_so_far: vec![],
        is_doc_hidden: false,
        def_id: crate_num.as_def_id(),
    });

    let mut add_child = |bfs_queue: &mut VecDeque<_>,
                         child: &ModChild,
                         mut parent: Vec<Symbol>,
                         is_doc_hidden: bool| {
        // Underscores do not create paths that are valid to spell, so we can exclude them from our
        // traversal.
        if !child.vis.is_public() || child.ident.name == kw::Underscore {
            return;
        }

        let Res::Def(def_kind, mut def_id) = child.res else {
            return;
        };

        // Don't include definitions that are within an `extern` block. These are foreign symbols
        // getting linked into rust, so we do not want to emit bindings for them. Expectation is
        // downstream consumers can link in the C symbols themselves.
        use rustc_hir::definitions::DefPathData;
        if tcx.def_path(def_id).data.iter().any(|segment| segment.data == DefPathData::ForeignMod) {
            return;
        }

        if matches!(def_kind, DefKind::Ctor(_, _)) {
            // We don't need to track paths for constructors. They will get handled by their
            // enclosing parent type.
            return;
        }
        if let Some(stability) = tcx.lookup_stability(def_id) {
            if stability.is_unstable() {
                return;
            }
        }

        // Map type aliases to their underlying type.
        let mut type_alias_def_id = None;
        if def_kind == DefKind::TyAlias {
            let underlying_type = tcx.type_of(def_id).instantiate_identity();
            if let crate::ty::TyKind::Adt(def, _) = underlying_type.kind() {
                type_alias_def_id = Some(def_id);
                def_id = def.did();
            }
        }

        let path = ExportedPath {
            path: parent.clone(),
            name: child.ident.name,
            type_alias_def_id,
            is_doc_hidden,
        };
        use std::collections::hash_map::Entry;
        match visible_parent_map.entry(def_id) {
            Entry::Vacant(vacant) => {
                vacant.insert(PublicPaths::new(path));
            }
            Entry::Occupied(mut occupied) => occupied.get_mut().insert(path),
        }
        if child.res.mod_def_id().is_some() && module_seen_set.insert(def_id) {
            parent.push(child.ident.name);
            bfs_queue.push_back(ModLikeDef { path_so_far: parent, is_doc_hidden, def_id });
        }
    };

    while let Some(mod_like) = bfs_queue.pop_front() {
        let module_children = module_children(tcx, mod_like.def_id);
        let is_doc_hidden = mod_like.is_doc_hidden || tcx.is_doc_hidden(mod_like.def_id);
        for child in module_children.iter() {
            add_child(bfs_queue, child, mod_like.path_so_far.clone(), is_doc_hidden);
        }
    }

    visible_parent_map
}

fn module_children(tcx: TyCtxt<'_>, parent: DefId) -> &[ModChild] {
    match parent.as_local() {
        None => tcx.module_children(parent),
        // Local `module_children` does not use the query due to perf impacts.
        Some(local_def_id) => tcx.module_children_local(local_def_id),
    }
}

fn resolve_if_use(db: &dyn BindingsGenerator<'_>, def_id: DefId) -> Option<DefId> {
    let tcx = db.tcx();
    let DefKind::Use = tcx.def_kind(def_id) else {
        return None;
    };

    let parent = tcx.opt_parent(def_id).expect("Expected use to have a parent");
    let module_children = module_children(tcx, parent);
    for child in module_children.iter() {
        if child.reexport_chain.first().and_then(|reexport| reexport.id()) == Some(def_id) {
            return child.res.opt_def_id();
        }
    }
    None
}

/// Implementation of `BindingsGenerator::symbol_canonical_name`.
fn symbol_canonical_name(
    db: &dyn BindingsGenerator<'_>,
    def_id: DefId,
) -> Option<FullyQualifiedName> {
    let tcx = db.tcx();

    // TODO: b/433286909 - We shouldn't pass DefKind::Use to this method and instead should keep what our use
    // is pointing at alongside the use as we generate_items and pass that when we want to determine
    // canonical name.
    let def_id = resolve_if_use(db, def_id).unwrap_or(def_id);

    let (full_path_strs, item_name, type_alias_def_id) = {
        // If our definition is at a path that can't be spelled, we have to pick a path from our
        // aliases.
        let paths = db.public_paths_by_def_id(def_id.krate);

        // If our definition has no public spellings, we can't give it a canonical name.
        let paths = paths.get(&def_id)?;

        // Select a canonical path for this symbol from available paths.
        // Our paths are kept in sorted order, so the canonical path will be the first one.
        let canonical_path = paths.canonical();

        // If the use is in the same scope as the canonical path, we want to use the original
        // name of the symbol, not the alias.
        (
            canonical_path.path.iter().map(|s| Rc::<str>::from(s.as_str())).collect::<Vec<_>>(),
            canonical_path.name,
            canonical_path.type_alias_def_id,
        )
    };

    let rs_name = Some(item_name);

    let krate = tcx.crate_name(def_id.krate);
    if krate.as_str() == "polars_plan" && matches!(item_name.as_str(), "date_range" | "time_range")
    {
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
    let cpp_ns_path = NamespaceQualifier::new(full_path_strs);

    // If our canonical definition is a type alias, use the attributes on that type alias, if available, otherwise use the attributes on the underlying definition.
    let attributes = type_alias_def_id
        .and_then(|id| {
            use crubit_attr::CrubitAttrs;
            let attrs = crubit_attr::get_attrs(tcx, id).unwrap();
            (attrs != CrubitAttrs::default()).then_some(attrs)
        })
        .unwrap_or_else(|| crubit_attr::get_attrs(tcx, def_id).unwrap());
    let cpp_type = attributes.cpp_type;
    let cpp_top_level_ns = format_top_level_ns_for_crate(db, def_id.krate);
    let cpp_name = attributes.cpp_name.map(|s| Symbol::intern(s.as_str())).or_else(|| {
        // If the rs_name is going to be used for the cpp_name, then we need to unkeyword it.
        // This prevents silly Rust names like "reinterpret_cast" from trying to be named
        // "reinterpret_cast" in C++, which would be an error.
        // If the user has opted in to one of these reserved names by setting cpp_name, however,
        // we should _not_ implicitly change it, and should instead given them an error.
        // Hence, this unkeywording behavior only happens in the case where we implicitly
        // delegate to the Rust name.
        rs_name.map(|rs_name| {
            Symbol::intern(code_gen_utils::unkeyword_cpp_ident(rs_name.as_str()).as_ref())
        })
    });
    Some(FullyQualifiedName {
        cpp_name,
        cpp_ns_path,
        rs_name,
        krate,
        cpp_top_level_ns,
        rs_mod_path,
        cpp_type,
    })
}

/// Checks whether a definition matches a specific qualified name by matching it's definition path
/// against `name`. Name must include the crate in it's path.
fn matches_qualified_name(db: &dyn BindingsGenerator<'_>, item_did: DefId, name: &[&str]) -> bool {
    let tcx = db.tcx();
    let path = tcx.def_path(item_did);
    if path.data.len() + 1 != name.len() {
        return false;
    }
    // This will always return false for anonymous path data because the caller won't be able to
    // specify the `disambiguator` that gets inserted into the symbol. That's fine. It is expected
    // this function will only be called to check for non-anonymous paths.
    [tcx.crate_name(path.krate)]
        .into_iter()
        .chain(path.data.into_iter().map(|seg| seg.as_sym(/*verbose=*/ false)))
        .zip(name.iter().map(|s| Symbol::intern(s)))
        .all(|(sym, expected)| sym == expected)
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

    // If our definition does not support attributes, get_all_attrs will panic, so we check
    // beforehand and return None.
    if !crubit_attr::supports_attrs(tcx.def_kind(def_id)) {
        return None;
    }

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

fn generate_using(
    db: &dyn BindingsGenerator<'_>,
    using_name: &Symbol,
    def_id: DefId,
) -> Result<CcSnippet> {
    let tcx = db.tcx();
    match tcx.def_kind(def_id) {
        DefKind::Fn => {
            // TODO(b/350772554): Support exporting private functions.
            let mut prereqs = match db.generate_function(def_id) {
                Ok(snippet) => snippet.main_api.prereqs,
                Err(err) => {
                    bail!("Unable to `use` function whose bindings failed: {err:?}");
                }
            };
            let fully_qualified_fn_name = FullyQualifiedName::new(db, def_id);
            let formatted_fully_qualified_fn_name = fully_qualified_fn_name.format_for_cc(db)?;
            let main_api_fn_name =
                format_cc_ident(db, fully_qualified_fn_name.cpp_name.unwrap().as_str())
                    .context("Error formatting function name")?;
            let using_name =
                format_cc_ident(db, using_name.as_str()).context("Error formatting using name")?;

            prereqs.defs.insert(def_id);
            let tokens = if format!("{}", using_name) == format!("{}", main_api_fn_name) {
                quote! { using #formatted_fully_qualified_fn_name; }
            } else {
                // TODO(b/350772554): Support function alias.
                bail!("Unsupported function alias");
            };
            Ok(CcSnippet { prereqs, tokens })
        }
        DefKind::Struct | DefKind::Enum => {
            // This points directly to a type definition, not an alias or compound data
            // type, so we can drop the hir type.
            let use_type = SugaredTy::missing_hir(tcx.type_of(def_id).instantiate_identity());
            create_type_alias(db, def_id, using_name.as_str(), use_type)
        }
        DefKind::TyAlias => generate_type_alias(db, def_id, using_name.as_str()),
        _ => {
            bail!("Unsupported use statement that refers to this type of the entity: {:#?}", def_id)
        }
    }
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
) -> Result<CcSnippet> {
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
) -> Result<CcSnippet> {
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

    Ok(CcSnippet { prereqs: main_api_prereqs, tokens })
}

fn is_public_or_supported_export(db: &dyn BindingsGenerator<'_>, def_id: DefId) -> bool {
    (def_id.krate != db.source_crate_num())
        || is_directly_public(db.tcx(), def_id)
        // If we have a private symbol in this crate that is publicly re-exported, then it is supported.
        || (is_exported(db.tcx(), def_id) && db.symbol_canonical_name(def_id).is_some())
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
        } = generate_trait_thunks(db, trait_id, &[], &core)?;

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
        } = generate_trait_thunks(db, trait_id, &[], &core)?;
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

fn generate_source_location(db: &dyn BindingsGenerator, def_id: DefId) -> String {
    let tcx = db.tcx();
    let def_span = tcx.def_span(def_id);
    let rustc_span::FileLines { file, lines } =
        match tcx.sess().source_map().span_to_lines(def_span) {
            Ok(filelines) => filelines,
            Err(_) => return "unknown location".to_string(),
        };
    let file_name = file.name.prefer_local().to_string();
    // Note: line_index starts at 0, while most everything else starts indexing at 1.
    let line_number = (lines[0].line_index + 1).to_string();
    if let Some(path_format) = db.crubit_debug_path_format() {
        if file.name.is_real() {
            return path_format.format(&[file_name.as_str(), line_number.as_str()]);
        }
    }
    format!("{file_name};l={line_number}")
}

/// Formats the doc comment (if any) associated with the item identified by
/// `local_def_id`, and appends the source location at which the item is
/// defined.
fn generate_doc_comment(db: &dyn BindingsGenerator, def_id: DefId) -> TokenStream {
    let doc_comment = db
        .tcx()
        .get_all_attrs(def_id)
        .iter()
        .filter_map(|attr| attr.doc_str())
        .map(|symbol| symbol.to_string())
        .chain(once(format!("Generated from: {}", generate_source_location(db, def_id))))
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
    let should_generate = match resolve_if_use(db, def_id) {
        Some(res_def_id) => {
            // Only generate a binding if the use is public and points as something exported.
            is_directly_public(tcx, def_id) && is_public_or_supported_export(db, res_def_id)
        }
        None => is_public_or_supported_export(db, def_id),
    };
    if !should_generate {
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
        DefKind::TyAlias => generate_type_alias(db, def_id, tcx.item_name(def_id).as_str())
            .map(|snippets| Some(snippets.into_main_api())),
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
) -> CcSnippet {
    let tcx = db.tcx();
    db.errors().report(&err);
    let source_loc = generate_source_location(db, def_id);
    let name = tcx.def_path_str(def_id);

    // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
    // says: To print causes as well [...], use the alternate selector “{:#}”.
    let msg = format!("Error generating bindings for `{name}` defined at {source_loc}: {err:#}");
    CcSnippet::new(quote! { __NEWLINE__ __NEWLINE__ __COMMENT__ #msg __NEWLINE__ })
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

struct FormattedItem {
    def_id: DefId,
    snippets: Option<ApiSnippets>,
    aliases: Vec<ExportedPath>,
}

fn formatted_items_in_crate(db: &dyn BindingsGenerator<'_>) -> impl Iterator<Item = FormattedItem> {
    let tcx = db.tcx();
    let defs_in_crate = db.public_paths_by_def_id(db.source_crate_num());
    defs_in_crate
        .into_iter()
        .filter_map(|(def_id, paths)| {
            let mut snippets = None;
            let aliases = if def_id.krate == db.source_crate_num() {
                // We only want to call `generate_item` on DefIds from our source crate. External
                // crate DefIds might appear in this map if our crate re-exports them, but we don't
                // want to regenerate those definitions.
                let api_snippets = db.generate_item(def_id).unwrap_or_else(|err| {
                    Some(generate_unsupported_def(db, def_id, err).into_main_api())
                })?;
                snippets = Some(api_snippets);
                let (_, aliases) = paths.into_canonical_and_aliases();
                aliases
            } else {
                paths.into_extern_aliases()
            };
            Some(FormattedItem { def_id, snippets, aliases })
        })
        .sorted_by_def_with(tcx, |item| item.def_id)
}

/// Formats all public items from the Rust crate being compiled.
fn generate_crate(db: &Database) -> Result<BindingsTokens> {
    struct CcDetails {
        def_id: DefId,
        namespace: NamespaceQualifier,
        tokens: TokenStream,
    }
    impl CcDetails {
        fn new(def_id: DefId, namespace: NamespaceQualifier, tokens: TokenStream) -> Self {
            Self { def_id, namespace, tokens }
        }
    }
    let mut cc_details_prereqs = CcPrerequisites::default();
    let mut cc_details: Vec<CcDetails> = vec![];
    let mut cc_api_impl = TokenStream::default();
    let mut extern_c_decls = BTreeSet::new();
    let mut main_apis = HashMap::<DefId, CcSnippet>::new();
    for item in formatted_items_in_crate(db) {
        let def_id = item.def_id;
        // We delay handling aliases until after sorting by def id to ensure they are emitted in a
        // deterministic order, same as the main api definitions. This is important for caching and
        // testing.
        for alias in item.aliases.iter() {
            let using_snippets = generate_using(db, &alias.name, def_id)
                .unwrap_or_else(|err| generate_unsupported_def(db, def_id, err));
            cc_details.push(CcDetails::new(
                // `def_id` gets used for coalescing namespaces in
                // `format_namespace_bound_cc_tokens`, so we want to carry it alongside our alias
                // even though we technically no longer need it.
                def_id,
                NamespaceQualifier::from(alias),
                using_snippets.into_tokens(&mut cc_details_prereqs),
            ));
        }
        let Some(api_snippets) = item.snippets else {
            continue;
        };
        let old_item = main_apis.insert(def_id, api_snippets.main_api);
        assert!(old_item.is_none(), "Duplicated key: {def_id:?}");

        // `cc_details` don't participate in the toposort, because
        // `CcPrerequisites::defs` always use `main_api` as the predecessor
        // - `chain`ing `cc_details` after `ordered_main_apis` trivially
        // meets the prerequisites.
        cc_details.push(CcDetails::new(
            def_id,
            FullyQualifiedName::new(db, def_id).cpp_ns_path,
            api_snippets.cc_details.into_tokens(&mut cc_details_prereqs),
        ));
        cc_api_impl.extend(api_snippets.rs_details.into_tokens(&mut extern_c_decls));
    }

    // Find the order of `main_apis` that 1) meets the requirements of
    // `CcPrerequisites::defs` and 2) makes a best effort attempt to keep the
    // `main_apis` in the same order as the source order of the Rust APIs.
    let tcx = db.tcx();
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
            .map(|(def_id, tokens)| {
                (tcx.opt_parent(def_id), FullyQualifiedName::new(db, def_id).cpp_ns_path, tokens)
            })
            .chain(
                cc_details.into_iter().map(|details| {
                    (tcx.opt_parent(details.def_id), details.namespace, details.tokens)
                }),
            )
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
