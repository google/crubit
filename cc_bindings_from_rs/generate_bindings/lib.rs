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
    adt_needs_bindings, cpp_enum_cpp_underlying_type, from_trait_impls_by_argument, generate_adt,
    generate_adt_core, generate_associated_item, scalar_value_to_string,
};
use arc_anyhow::{Context, Error, Result};
use code_gen_utils::{format_cc_includes, CcConstQualifier, CcInclude, NamespaceQualifier};
use database::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet, ExternCDecl, RsSnippet};
use database::{
    AdtCoreBindings, BindingsGenerator, ExportedPath, FineGrainedFeature, FullyQualifiedName,
    NoMoveOrAssign, PublicPaths, SugaredTy, TypeLocation, UnqualifiedName,
};
pub use database::{Database, IncludeGuard};
use error_report::{anyhow, bail, ErrorReporting, ReportFatalError};
use itertools::Itertools;
use proc_macro2::TokenStream;
use query_compiler::{
    does_type_implement_trait, get_layout, get_scalar_int_type, get_tag_size_with_padding,
    is_c_abi_compatible_by_value, is_copy, liberate_and_deanonymize_late_bound_regions,
    post_analysis_typing_env, repr_attrs,
};
use quote::{format_ident, quote};
use rustc_abi::{AddressSpace, BackendRepr, Integer, Primitive, Scalar};
use rustc_hir::def::{DefKind, Res};
use rustc_middle::dep_graph::DepContext;
use rustc_middle::metadata::{ModChild, Reexport};
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
    let metadata_block = if db.kythe_annotations() {
        quote! {
            __HASH_TOKEN__ ifdef KYTHE_IS_RUNNING __NEWLINE__
            __HASH_TOKEN__ pragma kythe_inline_metadata "This file contains Kythe metadata."
            __NEWLINE__
            __HASH_TOKEN__ endif __NEWLINE__
        }
    } else {
        TokenStream::new()
    };
    match db.h_out_include_guard() {
        IncludeGuard::PragmaOnce => Ok(quote! {
            __HASH_TOKEN__ pragma once __NEWLINE__
            __NEWLINE__
            #metadata_block

            #cc_api
        }),
        IncludeGuard::Guard(include_guard_str) => {
            let include_guard = format_cc_ident(db, include_guard_str.as_str())?;
            Ok(quote! {
                __HASH_TOKEN__ ifndef #include_guard __NEWLINE__
                __HASH_TOKEN__ define #include_guard __NEWLINE__
                __NEWLINE__
                #metadata_block

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
    // This is a temporary workaround while migrating to the rmeta interface. Our old implementation
    // breaks with some rmeta files, notably proto files, due to crate renaming behavior. But our
    // new implementation relies on assuming our source is the placeholder file provided by
    // compilation.
    if db.enable_rmeta_interface() {
        let tcx = db.tcx();
        // We know statically this will be the fake input
        // ```
        // extern crate <source-crate-name>;
        // fn main() {}
        // ```
        // And we use that fact here to look up the def id of `extern crate <source-crate-name>`.
        let mod_children = tcx.module_children_local(LOCAL_CRATE.as_def_id().expect_local());
        mod_children
            .iter()
            .find_map(|mod_child| {
                if db
                    .source_crate_name()
                    .is_some_and(|name| name.as_ref() == mod_child.ident.as_str())
                {
                    use rustc_middle::metadata::Reexport;
                    mod_child.reexport_chain.first().and_then(|reexport| match reexport {
                        Reexport::ExternCrate(def_id) => def_id.as_local(),
                        _ => None,
                    })
                } else {
                    None
                }
            })
            .and_then(|def_id| tcx.resolutions(()).extern_crate_map.get(&def_id))
            .cloned()
            .unwrap_or(LOCAL_CRATE)
    } else {
        let Some(source_crate_name) = db.source_crate_name() else {
            return LOCAL_CRATE;
        };
        let source_crate_name = Symbol::intern(source_crate_name.as_ref());
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
}

pub fn new_database<'db>(
    tcx: TyCtxt<'db>,
    source_crate_name: Option<Rc<str>>,
    crubit_support_path_format: dyn_format::Format<1>,
    crubit_debug_path_format: Option<dyn_format::Format<2>>,
    default_features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    enable_hir_types: bool,
    kythe_annotations: bool,
    enable_rmeta_interface: bool,
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
        enable_hir_types,
        kythe_annotations,
        enable_rmeta_interface,
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
        supported_traits,
        symbol_unqualified_name,
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
        adt_needs_bindings,
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
    let cc_api = quote! {
        __HASH_TOKEN__ pragma clang diagnostic push __NEWLINE__
        __HASH_TOKEN__ pragma clang diagnostic ignored "-Wreturn-type-c-linkage" __NEWLINE__

        #cc_api

        __HASH_TOKEN__ pragma clang diagnostic pop __NEWLINE__
    };
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
    let features = if krate == db.source_crate_num() {
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
            // SIMD primitives often have name collisions with SIMD primitives in C++. The C++
            // primitives are macros, so namespacing does not prevent collision. We expect people
            // will not need bindings to these primitives, so we exclude them to prevent the
            // collision.
            if ["simd_arch", "simd_x86"].contains(&stability.feature.as_str()) {
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
        // TODO: b/459865403 - Support bindings for `pub extern crate core`.
        let is_extern_crate_core = |child: &ModChild| {
            child
                .reexport_chain
                .first()
                .is_some_and(|reexport| matches!(reexport, Reexport::ExternCrate(_)))
                && tcx.opt_item_name(def_id).is_some_and(|name| name.as_str() == "core")
        };
        if child.res.mod_def_id().is_some()
            && module_seen_set.insert(def_id)
            && !is_extern_crate_core(child)
        {
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

fn symbol_unqualified_name(
    db: &dyn BindingsGenerator<'_>,
    def_id: DefId,
) -> Option<UnqualifiedName> {
    let tcx = db.tcx();
    let item_name = db
        .public_paths_by_def_id(def_id.krate)
        .get(&def_id)
        .map(|path| path.canonical().name)
        .or_else(|| tcx.opt_item_name(def_id))?;
    let rs_name = item_name;
    let attributes = crubit_attr::get_attrs(tcx, def_id)
        .unwrap_or_else(|_| panic!("Expected crubit_attrs on {def_id:?}"));
    let cpp_name = attributes.cpp_name.map(|s| Symbol::intern(s.as_str())).unwrap_or_else(|| {
        // If the rs_name is going to be used for the cpp_name, then we need to unkeyword it.
        // This prevents silly Rust names like "reinterpret_cast" from trying to be named
        // "reinterpret_cast" in C++, which would be an error.
        // If the user has opted in to one of these reserved names by setting cpp_name, however,
        // we should _not_ implicitly change it, and should instead given them an error.
        // Hence, this unkeywording behavior only happens in the case where we implicitly
        // delegate to the Rust name.
        Symbol::intern(code_gen_utils::unkeyword_cpp_ident(rs_name.as_str()).as_ref())
    });
    let cpp_type = attributes.cpp_type;
    Some(UnqualifiedName { cpp_name, rs_name, cpp_type })
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

    let (full_path_strs, type_alias_def_id) = {
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
            canonical_path.type_alias_def_id,
        )
    };

    let unqualified = type_alias_def_id
        .and_then(|def_id| {
            use crubit_attr::CrubitAttrs;
            let attrs = crubit_attr::get_attrs(tcx, def_id).unwrap();
            if attrs == CrubitAttrs::default() {
                None
            } else {
                db.symbol_unqualified_name(def_id)
            }
        })
        .or_else(|| db.symbol_unqualified_name(def_id))?;

    // `crate_name` gets the crate name written out in the rmeta file, which is not always the name
    // we want to spell out in our generated bindings. Proto targets, for example, rename their crate
    // to include the `_rust_proto` suffix, but the rmeta file contains the unsuffixed crate name.
    // If we're naming a symbol from our source crate, use the source crate name as the krate name
    // to resolve any renaming issues.
    let mut krate = (def_id.krate == db.source_crate_num())
        .then_some(())
        .and_then(|_| db.source_crate_name())
        .map(|source_crate_name| Symbol::intern(source_crate_name.as_ref()))
        .unwrap_or_else(|| tcx.crate_name(def_id.krate));

    // TODO: b/475830072 - Replace with a less brittle solution.
    if krate.as_str() == "alloc" {
        krate = Symbol::intern("std");
    }
    if krate.as_str() == "polars_plan"
        && matches!(unqualified.rs_name.as_str(), "date_range" | "time_range")
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
    let cpp_top_level_ns = format_top_level_ns_for_crate(db, def_id.krate);

    Some(FullyQualifiedName { krate, cpp_top_level_ns, cpp_ns_path, rs_mod_path, unqualified })
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
            let mut prereqs = match db.generate_function(def_id) {
                Ok(snippet) => snippet.main_api.prereqs,
                Err(err) => {
                    bail!("Unable to `use` function whose bindings failed: {err:?}");
                }
            };
            let fully_qualified_fn_name = db
                .symbol_canonical_name(def_id)
                .unwrap_or_else(|| panic!("Failed to get canonical name for {:?}", def_id));
            let formatted_fully_qualified_fn_name = fully_qualified_fn_name.format_for_cc(db)?;
            let main_api_fn_name =
                format_cc_ident(db, fully_qualified_fn_name.unqualified.cpp_name.as_str())
                    .context("Error formatting function name")?;
            let using_name_ident =
                format_cc_ident(db, using_name.as_str()).context("Error formatting using name")?;

            prereqs.defs.insert(def_id);
            let tokens = if using_name_ident.to_string() == main_api_fn_name.to_string() {
                quote! { using #formatted_fully_qualified_fn_name; }
            } else {
                quote! { constexpr auto #using_name_ident = #formatted_fully_qualified_fn_name; }
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
    // TODO: b/457843120 - Remove this workaround once we can properly support float constants.
    let unsupported_consts = [
        ["core", "f32", "INFINITY"],
        ["core", "f64", "INFINITY"],
        ["core", "f32", "NEG_INFINITY"],
        ["core", "f64", "NEG_INFINITY"],
        ["core", "f32", "NAN"],
        ["core", "f64", "NAN"],
    ];
    if unsupported_consts.iter().any(|const_path| matches_qualified_name(db, def_id, const_path)) {
        bail!(
            "Cannot generate bindings to unsupported constant: {}",
            tcx.item_name(def_id).as_str()
        )
    }
    let ty = tcx.type_of(def_id).instantiate_identity();
    let rust_type = SugaredTy::missing_hir(ty);
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

// Implementation of `BindingsGenerator::supported_traits`.
fn supported_traits(db: &dyn BindingsGenerator<'_>) -> Rc<[DefId]> {
    let tcx = db.tcx();
    let traits = tcx
        .visible_traits()
        .filter(|trait_id| {
            let crate_name = tcx.crate_name(trait_id.krate);
            // TODO: b/269294366 - Support traits in stdlib once we can generate bindings for the
            // stdlib that can be depended on.
            let not_in_stdlib = crate_name.as_str() != "std"
                && crate_name.as_str() != "core"
                && crate_name.as_str() != "alloc";

            let generics = tcx.generics_of(*trait_id);
            // TODO: b/259749095 - Support generics in Traits.
            // Traits will have a single parameter for the self type which is allowed.
            let no_generic_args = (generics.has_self
                && generics.own_params.iter().filter(|param| param.kind.is_ty_or_const()).count()
                    == 1)
                || !generics.requires_monomorphization(tcx);

            let is_exposed_trait = db.symbol_canonical_name(*trait_id).is_some();
            // We might want to explicitly omit certain marker traits here that are already handled by the bindings for structs/enums (Copy, Clone, Default, etc.).
            // Unless, we think there's value in exposing them explicitly as traits.
            not_in_stdlib && no_generic_args && is_exposed_trait
        })
        .collect::<Vec<DefId>>()
        .into_boxed_slice();
    Rc::from(traits)
}

fn generate_trait(
    db: &dyn BindingsGenerator<'_>,
    trait_id: DefId,
) -> arc_anyhow::Result<ApiSnippets> {
    if !db.supported_traits().contains(&trait_id) {
        bail!("Trait is not yet supported")
    }

    let canonical_name = db
        .symbol_canonical_name(trait_id)
        .expect("generate_trait was unexpectedly called on an item without a canonical name");

    let doc_comment = generate_doc_comment(db, trait_id);
    let trait_name = format_cc_ident(db, canonical_name.unqualified.cpp_name.as_str())?;
    let rs_type = canonical_name.format_for_rs().to_string();
    let attributes = vec![quote! {CRUBIT_INTERNAL_RUST_TYPE(#rs_type)}];

    let main_api = CcSnippet::with_include(
        quote! {
            __NEWLINE__ #doc_comment
            struct #(#attributes)* #trait_name {
                template <typename T>
                using impl = rs_std::impl<T, #trait_name>;
            };
            __NEWLINE__
        },
        db.support_header("rs_std/traits.h"),
    );
    Ok(ApiSnippets { main_api, ..Default::default() })
}

fn generate_type_alias(
    db: &dyn BindingsGenerator<'_>,
    def_id: DefId,
    using_name: &str,
) -> Result<CcSnippet> {
    let tcx = db.tcx();
    let mir_ty = tcx.type_of(def_id).instantiate_identity();
    let alias_type = SugaredTy::missing_hir(mir_ty);
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

    let fully_qualified_name = db
        .symbol_canonical_name(def_id)
        .ok_or_else(|| anyhow!("Failed to get canonical name for {:?}", def_id))?;
    let rs_type = format!("{}", fully_qualified_name.format_for_rs());

    main_api_prereqs.includes.insert(db.support_header("annotations_internal.h"));
    let mut attributes = vec![quote! {CRUBIT_INTERNAL_RUST_TYPE(#rs_type)}];
    if let Some(cc_deprecated_tag) = generate_deprecated_tag(db.tcx(), def_id) {
        attributes.push(cc_deprecated_tag);
    }

    let tokens = quote! {using #alias_name #(#attributes)* = #actual_type_name;};

    Ok(CcSnippet { prereqs: main_api_prereqs, tokens })
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

            // This might be the case for `#[repr(transparent)]` types.
            // TODO: b/459482188 - This is ultimately dependent on the return ABI of the thunk and
            // should be cetnralized with the other callsites that depend on return type ABI.
            let ctor_impl = if is_c_abi_compatible_by_value(tcx, core.self_ty) {
                quote! {
                    inline #cc_struct_name::#cc_struct_name() {
                       *this = __crubit_internal::#thunk_name();
                    }
                }
            } else {
                quote! {
                    inline #cc_struct_name::#cc_struct_name() {
                        __crubit_internal::#thunk_name(this);
                    }
                }
            };
            let tokens = quote! {
                #cc_thunk_decls
                #ctor_impl
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
#[allow(clippy::result_large_err)]
fn generate_move_ctor_and_assignment_operator<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    core: Rc<AdtCoreBindings<'tcx>>,
) -> Result<ApiSnippets, NoMoveOrAssign> {
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
                    "C++ move operations are unavailable for this type. See \
                    http://crubit.rs/rust/movable_types for an explanation of Rust types that are C++ \
                    movable."
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
        NoMoveOrAssign {
            err,
            explicitly_deleted: ApiSnippets {
                main_api: CcSnippet::new(quote! {
                    __NEWLINE__ __COMMENT__ #msg
                    #adt_cc_name(#adt_cc_name&&) = delete;  __NEWLINE__
                    #adt_cc_name& operator=(#adt_cc_name&&) = delete;
                }),
                ..Default::default()
            },
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

    // If we're forward declaring a C++ enum, we need to include the underlying type in the forward
    // declaration. Otherwise, it will default to `int` and cause a compilation error.
    let tcx = db.tcx();
    let crubit_attrs = crubit_attr::get_attrs(tcx, core_bindings.def_id).unwrap_or_default();
    if crubit_attrs.cpp_enum.is_some() {
        let cpp_enum_cpp_underlying_type_snippet = cpp_enum_cpp_underlying_type(db, def_id)
            .expect("`generate_fwd_decl` should only be called if we successfully generated an enum for this type");
        let cpp_enum_cpp_underlying_type = cpp_enum_cpp_underlying_type_snippet.tokens;
        return quote! { #keyword #cc_short_name : #cpp_enum_cpp_underlying_type; };
    }

    quote! { #keyword #cc_short_name; }
}

fn generate_kythe_doc_comment(
    db: &dyn BindingsGenerator,
    def_id: DefId,
    doc_comment: String,
) -> TokenStream {
    // Always emit a new capture tag so we can unconditionally emit capture brackets around
    // identifiers. (Otherwise, we might accidentally associate a capture range with the wrong
    // capture tag; it's fine to emit capture tags that never capture anything.)
    let tcx = db.tcx();
    let def_span = tcx.def_ident_span(def_id).unwrap_or_else(|| tcx.def_span(def_id));
    #[rustversion::before(2025-12-14)]
    let file_name = tcx.sess().source_map().span_to_filename(def_span).prefer_local().to_string();
    #[rustversion::since(2025-12-14)]
    let file_name = tcx
        .sess()
        .source_map()
        .span_to_filename(def_span)
        .prefer_local_unconditionally()
        .to_string();
    let start = def_span.lo().0.to_string();
    let end = def_span.hi().0.to_string();
    quote! { __CAPTURE_TAG__ #file_name #start #end __COMMENT__ #doc_comment}
}

fn generate_source_location(db: &dyn BindingsGenerator, def_id: DefId) -> String {
    let tcx = db.tcx();
    let def_span = tcx.def_span(def_id);
    let rustc_span::FileLines { file, lines } =
        match tcx.sess().source_map().span_to_lines(def_span) {
            Ok(filelines) => filelines,
            Err(_) => return "unknown location".to_string(),
        };
    #[rustversion::before(2025-12-14)]
    let file_name = file.name.prefer_local().to_string();
    #[rustversion::since(2025-12-14)]
    let file_name = file.name.prefer_local_unconditionally().to_string();
    // Virtual paths will have a "./" prefix that we don't want to display.
    let file_name = file_name.strip_prefix("./").unwrap_or(file_name.as_str());

    // Note: line_index starts at 0, while most everything else starts indexing at 1.
    let line_number = (lines[0].line_index + 1).to_string();
    if let Some(path_format) = db.crubit_debug_path_format() {
        if file.name.is_real() {
            return path_format.format(&[file_name, line_number.as_str()]);
        }
    }
    format!("{file_name};l={line_number}")
}

/// Formats the doc comment (if any) associated with the item identified by
/// `local_def_id`, and appends the source location at which the item is
/// defined.
fn generate_doc_comment(db: &dyn BindingsGenerator, def_id: DefId) -> TokenStream {
    let mut docs = db
        .tcx()
        .get_all_attrs(def_id)
        .iter()
        .filter_map(|attr| attr.doc_str())
        .map(|symbol| symbol.to_string())
        .peekable();
    let leading_newline = if docs.peek().is_none() { "" } else { "\n" };
    let doc_comment = docs
        .chain(once(format!(
            "{}Generated from: {}",
            leading_newline,
            generate_source_location(db, def_id)
        )))
        .join("\n");
    if db.kythe_annotations() {
        generate_kythe_doc_comment(db, def_id, doc_comment)
    } else {
        quote! { __COMMENT__ #doc_comment}
    }
}

/// Returns the name of the item identified by `def_id`, or "<unknown>" if
/// the item can't be identified.
fn item_name(db: &dyn BindingsGenerator<'_>, def_id: DefId) -> Symbol {
    db.tcx().opt_item_name(def_id).unwrap_or_else(|| Symbol::intern("<unknown>"))
}

fn item_name_for_error_report(
    db: &dyn BindingsGenerator<'_>,
    def_id: DefId,
) -> error_report::ItemName {
    let name = format!(
        "{}::{}",
        db.tcx().crate_name(db.source_crate_num()),
        db.tcx().def_path_str(def_id)
    )
    .into();
    let id = ((def_id.index.as_u32() as u64) << 32) | def_id.krate.as_u32() as u64;
    error_report::ItemName { name, id }
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

#[macro_export]
macro_rules! error_scope {
    ($db:expr, $def_id:expr) => {
        let db = $db;
        let errors = db.errors();
        let _error_scope =
            error_report::ItemScope::new(&*errors, $crate::item_name_for_error_report(db, $def_id));
    };
}

// A helper for `generate_item`.
// The wrapper is used to ensure that the `must_bind` annotation is enforced.
fn generate_item_impl(
    db: &dyn BindingsGenerator<'_>,
    def_id: DefId,
) -> Result<Option<ApiSnippets>> {
    let tcx = db.tcx();
    if db.symbol_canonical_name(def_id).is_none() {
        return Ok(None);
    };
    let item = match tcx.def_kind(def_id) {
        DefKind::Struct | DefKind::Enum | DefKind::Union => {
            db.adt_needs_bindings(def_id).map(|core| Some(generate_adt(db, core)))
        }
        DefKind::Fn => db.generate_function(def_id).map(Some),
        DefKind::TyAlias => generate_type_alias(db, def_id, tcx.item_name(def_id).as_str())
            .map(|snippets| Some(snippets.into_main_api())),
        DefKind::Const => generate_const(db, def_id).map(Some),
        DefKind::Trait => generate_trait(db, def_id).map(Some),
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
    db.errors().assert_in_item(item_name_for_error_report(db, def_id));
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
    let lhs_def_path_hash = tcx.def_path_str(lhs_id);
    let rhs_def_path_hash = tcx.def_path_str(rhs_id);
    lhs_def_path_hash.cmp(&rhs_def_path_hash).then_with(|| {
        let lhs_def_hash = tcx.def_path_hash(lhs_id);
        let rhs_def_hash = tcx.def_path_hash(rhs_id);
        lhs_def_hash.cmp(&rhs_def_hash)
    })
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

/// Generate bindings to supported trait implementations. An implementation is supported if both
/// its trait and implementing type receive bindings.
fn generate_trait_impls<'a, 'b>(
    db: &'a dyn BindingsGenerator<'b>,
) -> impl Iterator<Item = ApiSnippets> + use<'a, 'b> {
    let tcx = db.tcx();
    let supported_traits: Vec<DefId> = db.supported_traits().iter().copied().collect();
    // TyCtxt makes it easy to get all the implementations of a trait, but there isn't an easy way
    // to say give me all the trait implementations for this type. This is by design. The compiler
    // lazily determines conformance to traits as needed for types and never computes every trait
    // for a type in a single data structure.
    //
    // We, however, want every implementation for a supported type, so we can emit bindings to them.
    // We achieve this by walking every supported trait, walking every implementation of that trait,
    // and paring down to the implementations that receive bindings.
    //
    // A serendipitous side effect of this approach is that our implementations are emitted as a
    // single list containing just implementations. We want to emit all of our implementations in a
    // separate portion of our header from the rest of our bindings. Our impls become template
    // specializaitons, which are required to be in an enclosing namespace of the template they
    // specialize. This prevents them from living in the same namespace as our other bindings, as
    // they may implement a trait that is not enclosed by that namespace.
    supported_traits
        .into_iter()
        .flat_map(move |trait_def_id| {
            use rustc_middle::ty::fast_reject::SimplifiedType;
            tcx.trait_impls_of(trait_def_id)
                .non_blanket_impls()
                .into_iter()
                .filter_map(move |(simple_ty, impl_def_ids)| match simple_ty {
                    SimplifiedType::Adt(did) => {
                        // Only bind implementations for supported ADTs.
                        if db.adt_needs_bindings(*did).is_err() {
                            return None;
                        }
                        let crate_name = tcx.crate_name(did.krate);
                        // TODO: b/391443811 - Add support for implementations of stdlib types once
                        // we have headers that can be included for those types.
                        if ["std", "core", "alloc"].contains(&crate_name.as_str()) {
                            return None;
                        }
                        let adt_cc_name = db.symbol_canonical_name(*did)?.format_for_cc(db).ok()?;
                        Some((adt_cc_name, trait_def_id, impl_def_ids))
                    }
                    // TODO: b/457803426 - Support trait implementations for non-adt types.
                    _ => None,
                })
                .flat_map(move |(adt_cc_name, trait_def_id, impl_def_ids)| {
                    impl_def_ids
                        .iter()
                        // TODO: b/458768435 - This is technically suboptimal. We could instead only
                        // query for the impls from this crate, but the logic is complicated by
                        // supporting LOCAL_CRATE. Revisit once we've migrated to rmetas.
                        .filter(|impl_def_id| impl_def_id.krate == db.source_crate_num())
                        .map(move |impl_def_id| (adt_cc_name.clone(), trait_def_id, impl_def_id))
                })
        })
        .map(move |(adt_cc_name, trait_def_id, impl_def_id)| {
            let canonical_name = db.symbol_canonical_name(trait_def_id).expect(
                "symbol_canonical_name was unexpectedly called on a trait without a canonical name",
            );
            let trait_name = canonical_name.format_for_cc(db).map_err(|err| (impl_def_id, err))?;
            let mut prereqs = CcPrerequisites::default();
            if trait_def_id.krate == db.source_crate_num() {
                prereqs.defs.insert(trait_def_id);
            } else {
                let other_crate_name = tcx.crate_name(trait_def_id.krate);
                let crate_name_to_include_paths = db.crate_name_to_include_paths();
                let includes = crate_name_to_include_paths
                    .get(other_crate_name.as_str())
                    .ok_or_else(|| {
                        let trait_name = tcx.def_path_str(trait_def_id);
                        (
                            impl_def_id,
                            anyhow!(
                                "Trait `{trait_name}` comes from the `{other_crate_name}` crate, \
                                but no `--crate-header` was specified for this crate"
                            ),
                        )
                    })?;
                prereqs.includes.extend(includes.iter().cloned());
            }

            let mut member_function_names = HashSet::new();
            let assoc_items: ApiSnippets = tcx
                .associated_items(impl_def_id)
                .in_definition_order()
                .flat_map(|assoc_item| {
                    generate_associated_item(db, assoc_item, &mut member_function_names)
                })
                .collect();

            let main_api = assoc_items.main_api.into_tokens(&mut prereqs);
            prereqs.includes.insert(db.support_header("rs_std/traits.h"));

            Ok(ApiSnippets {
                main_api: CcSnippet {
                    tokens: quote! {
                        __NEWLINE__
                        template<>
                        struct rs_std::impl<#adt_cc_name, #trait_name> {
                            static constexpr bool kIsImplemented = true;

                            #main_api
                        };
                        __NEWLINE__
                    },
                    prereqs,
                },
                cc_details: assoc_items.cc_details,
                rs_details: assoc_items.rs_details,
            })
        })
        .map(|results_snippets| {
            results_snippets.unwrap_or_else(|(def_id, err)| {
                generate_unsupported_def(db, *def_id, err).into_main_api()
            })
        })
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
                error_scope!(db, def_id);
                let api_snippets = db.generate_item(def_id).transpose()?;
                let (api_snippets, aliases) = api_snippets.map_or_else(
                    |err| (generate_unsupported_def(db, def_id, err).into_main_api(), vec![]),
                    |api_snippets| {
                        // Only generate aliases if we generate a definition.
                        let (_, aliases) = paths.into_canonical_and_aliases();
                        (api_snippets, aliases)
                    },
                );
                snippets = Some(api_snippets);
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
        error_scope!(db, def_id);
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
            db.symbol_canonical_name(def_id)
                .unwrap_or_else(|| panic!("Exported item {def_id:?} should have a canonical name"))
                .cpp_ns_path,
            api_snippets.cc_details.into_tokens(&mut cc_details_prereqs),
        ));
        cc_api_impl.extend(api_snippets.rs_details.into_tokens(&mut extern_c_decls));
    }

    // Because trait implementations are template specialization, they can't live in the top-level
    // namespace generated for our other definitions. Template specializations must live in an
    // enclosing namespace of the template they specialize. For this reason, we put our specializations in the top level namespace. The remainder of our implementation code, should be handled like normal.
    // We append our cc_details and rs_details here, so that they get processed like normal, but save our main_api to be specially placed in the top level namespace.
    let mut impls_cc_details = vec![];
    let impls = generate_trait_impls(db)
        .map(|snippets| {
            impls_cc_details.push(snippets.cc_details.clone().into_tokens(&mut cc_details_prereqs));
            cc_api_impl.extend(snippets.rs_details.clone().into_tokens(&mut extern_c_decls));
            snippets.main_api.clone().into_tokens(&mut cc_details_prereqs)
        })
        .collect::<Vec<_>>();
    let impls_tokens = if impls.is_empty() {
        // Exclude leading newline for an empty impls list.
        quote! {}
    } else {
        quote! {
            __NEWLINE__
            #(#impls)__NEWLINE__*
        }
    };

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
        let ordered_cc: Vec<(Option<DefId>, NamespaceQualifier, TokenStream)> =
            fwd_decls
                .into_iter()
                .chain(ordered_main_apis)
                .map(|(def_id, tokens)| {
                    (
                        tcx.opt_parent(def_id),
                        db.symbol_canonical_name(def_id)
                            .unwrap_or_else(|| {
                                panic!("Exported item {def_id:?} should have a canonical name")
                            })
                            .cpp_ns_path,
                        tokens,
                    )
                })
                .chain(cc_details.into_iter().map(|details| {
                    (tcx.opt_parent(details.def_id), details.namespace, details.tokens)
                }))
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
            #impls_tokens
            #(#impls_cc_details)__NEWLINE__*
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
