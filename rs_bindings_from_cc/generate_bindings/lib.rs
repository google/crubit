// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

use arc_anyhow::{anyhow, ensure, Context, Result};
use code_gen_utils::{format_cc_includes, is_cpp_reserved_keyword, make_rs_ident, CcInclude};
use cpp_type_name::format_cpp_type_with_references;
use crubit_abi_type::{CrubitAbiType, CrubitAbiTypeToRustExprTokens, FullyQualifiedPath};
use database::code_snippet::{
    self, ApiSnippets, Bindings, BindingsTokens, CppDetails, CppIncludes, Feature, GeneratedItem,
};
use database::db::{self, BindingsGenerator, CodegenFunctions, Database};
use database::rs_snippet::{
    BridgeRsTypeKind, Callable, FnTrait, Mutability, RsTypeKind, RustPtrKind,
};
use error_report::{bail, ErrorReporting, ReportFatalError};
use ffi_types::Environment;
use generate_comment::generate_top_level_comment;
use generate_comment::{generate_comment, generate_doc_comment, generate_unsupported};
use generate_struct_and_union::generate_incomplete_record;
use heck::ToSnakeCase;
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use rs_type_kind::rs_type_kind_with_lifetime_elision;
use std::collections::{BTreeSet, HashMap};
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use token_stream_printer::{
    cc_tokens_to_formatted_string, rs_tokens_to_formatted_string, RustfmtConfig,
};

use dyn_format::Format;

mod generate_dyn_callable;

/// Deserializes IR from `json` and generates bindings source code.
pub fn generate_bindings(
    json: &[u8],
    crubit_support_path_format: &str,
    clang_format_exe_path: &OsStr,
    rustfmt_exe_path: &OsStr,
    rustfmt_config_path: &OsStr,
    errors: &dyn ErrorReporting,
    fatal_errors: &dyn ReportFatalError,
    environment: Environment,
) -> Result<Bindings> {
    let ir = deserialize_ir(json).with_context(|| {
        let ir_string = String::from_utf8_lossy(json);
        format!("Failed to deserialize IR:\n{}", ir_string)
    })?;

    let crubit_support_path_format =
        Format::parse_with_metavars(crubit_support_path_format, &["header"])?;

    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(
        &ir,
        crubit_support_path_format,
        errors,
        fatal_errors,
        environment,
    )?;
    let rs_api = {
        let rustfmt_exe_path =
            if rustfmt_exe_path.is_empty() { None } else { Some(Path::new(rustfmt_exe_path)) };
        let rustfmt_config_path = if rustfmt_config_path.is_empty() {
            None
        } else {
            Some(Path::new(rustfmt_config_path))
        };
        let rustfmt_config =
            rustfmt_exe_path.map(|path| RustfmtConfig::new(path, rustfmt_config_path));
        rs_tokens_to_formatted_string(rs_api, rustfmt_config.as_ref())?
    };
    let rs_api_impl = {
        let clang_format_exe_path = if clang_format_exe_path.is_empty() {
            None
        } else {
            Some(Path::new(clang_format_exe_path))
        };
        cc_tokens_to_formatted_string(rs_api_impl, clang_format_exe_path)?
    };

    let top_level_comment = generate_top_level_comment(&ir, environment);
    // TODO(lukasza): Try to remove `#![rustfmt:skip]` - in theory it shouldn't
    // be needed when `@generated` comment/keyword is present...
    let rs_api = format!(
        "{top_level_comment}\n\
        #![rustfmt::skip]\n\
        {rs_api}"
    );

    let rs_api_impl = format!(
        "{top_level_comment}\n\
        {rs_api_impl}"
    );

    Ok(Bindings { rs_api, rs_api_impl })
}

fn generate_type_alias(
    db: &dyn BindingsGenerator,
    type_alias: Rc<TypeAlias>,
) -> Result<ApiSnippets> {
    // Skip the type alias if it maps to a bridge type.
    let rs_type_kind = db.rs_type_kind((&*type_alias).into())?;
    let generated_item = if rs_type_kind.unalias().is_bridge_type() {
        let disable_comment = format!(
            "Type alias for {cpp_type} suppressed due to being a bridge type",
            cpp_type = type_alias.debug_name(db.ir()),
        );
        GeneratedItem::Comment { message: disable_comment.into() }
    } else {
        let underlying_type = db
            .rs_type_kind(type_alias.underlying_type.clone())
            .with_context(|| format!("Failed to format underlying type for {type_alias}"))?;

        // If this type alias refers to a record with nested types,
        // we need to also re-export the generated module.
        let mut underlying_nested_module_path = None;
        if let RsTypeKind::Record { record, crate_path, .. } = &underlying_type {
            if generate_struct_and_union::child_items(record, db)
                .any(|child_item| child_item.is_nested)
            {
                let underlying_nested_module_name =
                    make_rs_ident(&record.rs_name.identifier.to_snake_case());
                underlying_nested_module_path =
                    Some(quote! { #crate_path #underlying_nested_module_name });
            }
        }

        GeneratedItem::TypeAlias {
            doc_comment: generate_doc_comment(
                type_alias.doc_comment.as_deref(),
                Some(&type_alias.source_loc),
                db.environment(),
            ),
            visibility: db::type_visibility(db, &type_alias.owning_target, rs_type_kind)
                .unwrap_or_default(),
            ident: make_rs_ident(&type_alias.rs_name.identifier),
            underlying_type: underlying_type.to_token_stream(db),
            underlying_nested_module_path,
        }
    };
    Ok(ApiSnippets {
        generated_items: HashMap::from([(type_alias.id, generated_item)]),
        ..Default::default()
    })
}

fn generate_global_var(db: &dyn BindingsGenerator, var: Rc<GlobalVar>) -> Result<ApiSnippets> {
    let type_ = db.rs_type_kind(var.type_.clone())?;

    Ok(ApiSnippets {
        generated_items: HashMap::from([(
            var.id,
            GeneratedItem::GlobalVar {
                link_name: var.mangled_name.clone(),
                is_mut: !var.type_.is_const,
                ident: make_rs_ident(&var.rs_name.identifier),
                type_tokens: type_.to_token_stream(db),
                visibility: db::type_visibility(db, &var.owning_target, type_).unwrap_or_default(),
            },
        )]),
        ..Default::default()
    })
}

fn generate_namespace(db: &dyn BindingsGenerator, namespace: Rc<Namespace>) -> Result<ApiSnippets> {
    let ir = db.ir();
    let mut api_snippets = ApiSnippets::default();

    for &item_id in &namespace.child_item_ids {
        let item = ir.find_untyped_decl(item_id);
        api_snippets.append(db.generate_item(item.clone())?);
    }

    api_snippets.generated_items.insert(namespace.id, GeneratedItem::NonCanonicalNamespace);
    api_snippets.generated_items.insert(
        namespace.canonical_namespace_id,
        GeneratedItem::CanonicalNamespace { items: namespace.child_item_ids.to_vec() },
    );
    Ok(api_snippets)
}

/// Implementation of `BindingsGenerator::generate_item`.
fn generate_item(db: &dyn BindingsGenerator, item: Item) -> Result<ApiSnippets> {
    let _scope = item.error_scope(db.ir(), db.errors());
    let err = match generate_item_impl(db, &item) {
        Ok(generated) => return Ok(generated),
        Err(err) => err,
    };

    if db.has_bindings(item.clone()).is_ok() && !matches!(item, Item::Func(_)) {
        return Err(err);
    }
    // We didn't guarantee that bindings would exist, so it is not invalid to
    // write down the error but continue.

    // FIXME(cramertj): get paths here in more cases. It may be that
    // `generate_item_impl` failed in such a way that the path is still available.
    let unsupported_item =
        UnsupportedItem::new_with_cause(db.ir(), &item, /* path= */ None, err);

    Ok(generate_unsupported(db, unsupported_item.into()))
}

/// The implementation of generate_item, without the error recovery logic.
///
/// Returns Err if bindings could not be generated for this item.
fn generate_item_impl(db: &dyn BindingsGenerator, item: &Item) -> Result<ApiSnippets> {
    let ir = db.ir();
    if let Some(owning_target) = item.owning_target() {
        if !ir.is_current_target(&owning_target) {
            return Ok(ApiSnippets::default());
        }
    }
    let generated_item = match item {
        Item::Func(func) => match db.generate_function(func.clone(), None)? {
            None => ApiSnippets::default(),
            Some(generated_function) => {
                if let Err(e) = &generated_function.status {
                    // Add any non-fatal errors to the error report.
                    // These won't result in an UnsupportedItem since we *did* generate an
                    // uncallable function item.
                    db.errors().report(e);
                }
                if db.is_ambiguous_function(&generated_function.id, func.id) {
                    bail!("Cannot generate bindings for overloaded function")
                } else {
                    (*generated_function.snippets).clone()
                }
            }
        },
        Item::IncompleteRecord(incomplete_record) => {
            generate_incomplete_record(db, incomplete_record.clone())?
        }
        Item::Record(record) => db.generate_record(record.clone())?,
        Item::Enum(enum_) => db.generate_enum(enum_.clone())?,
        Item::GlobalVar(var) => generate_global_var(db, var.clone())?,
        Item::TypeAlias(type_alias) => generate_type_alias(db, type_alias.clone())?,
        Item::UnsupportedItem(unsupported) => generate_unsupported(db, unsupported.clone()),
        Item::Comment(comment) => generate_comment(comment.clone()),
        Item::Namespace(namespace) => generate_namespace(db, namespace.clone())?,
        Item::UseMod(use_mod) => {
            let UseMod { path, mod_name, .. } = &**use_mod;
            let mod_name = make_rs_ident(&mod_name.identifier);
            // TODO(b/308949532): Skip re-export if the module being used is empty
            // (transitively).
            ApiSnippets {
                generated_items: HashMap::from([(
                    use_mod.id,
                    GeneratedItem::UseMod { path: path.clone(), mod_name },
                )]),
                ..Default::default()
            }
        }
        Item::ExistingRustType(existing_rust_type) => {
            let rs_type_kind = db.rs_type_kind((&**existing_rust_type).into())?;
            let disable_comment = format!(
                "Type bindings for {cpp_type} suppressed due to being mapped to \
                    an existing Rust type ({rs_type_kind})",
                cpp_type = existing_rust_type.debug_name(&ir),
                rs_type_kind = rs_type_kind.display(db),
            );
            let assertions = existing_rust_type
                .size_align
                .as_ref()
                .map(|size_align| {
                    generate_struct_and_union::rs_size_align_assertions(
                        rs_type_kind.to_token_stream(db),
                        size_align,
                    )
                })
                .into_iter()
                .collect_vec();

            ApiSnippets {
                generated_items: HashMap::from([(
                    existing_rust_type.id,
                    GeneratedItem::Comment { message: disable_comment.into() },
                )]),
                assertions,
                ..Default::default()
            }
        }
    };

    // Suppress bindings at the last minute, to collect other errors first.
    let _ = db.has_bindings(item.clone())?;

    Ok(generated_item)
}

/// Creates a new database. Public for testing.
pub fn new_database<'db>(
    ir: &'db IR,
    errors: &'db dyn ErrorReporting,
    fatal_errors: &'db dyn ReportFatalError,
    environment: Environment,
) -> Database<'db> {
    Database::new(
        ir,
        errors,
        fatal_errors,
        environment,
        CodegenFunctions {
            generate_enum: generate_enum::generate_enum,
            generate_item,
            generate_record: generate_struct_and_union::generate_record,
        },
        is_rs_type_kind_unsafe,
        has_bindings::has_bindings,
        rs_type_kind_with_lifetime_elision,
        generate_function::generate_function,
        generate_function::overload_sets,
        generate_function::is_record_clonable,
        generate_function::get_binding,
        generate_struct_and_union::collect_unqualified_member_functions,
        crubit_abi_type,
        has_bindings::type_target_restriction,
        has_bindings::resolve_type_names,
    )
}

/// Returns the Rust code implementing bindings, plus any auxiliary C++ code
/// needed to support it.
//
/// Public for use in `generate_bindings_tokens_for_test`.
pub fn generate_bindings_tokens(
    ir: &IR,
    crubit_support_path_format: Format<1>,
    errors: &dyn ErrorReporting,
    fatal_errors: &dyn ReportFatalError,
    environment: Environment,
) -> Result<BindingsTokens> {
    let db = new_database(ir, errors, fatal_errors, environment);
    let mut snippets = ApiSnippets::default();

    // For #![rustfmt::skip].
    snippets.features |= Feature::custom_inner_attributes;
    // For the `vector` in `cc_std`.
    snippets.features |= Feature::allocator_api;
    snippets.features |= Feature::cfg_sanitize;

    for top_level_item_id in ir.top_level_item_ids() {
        let item = ir.find_untyped_decl(*top_level_item_id);
        snippets.append(db.generate_item(item.clone())?);
    }

    // Since Idents are not free, we reuse them across records.
    let mut param_idents_buffer = vec![];
    let (dyn_callable_cpp_decls, dyn_callable_rust_impls): (Vec<TokenStream>, Vec<TokenStream>) =
        ir.records()
            .filter_map(|record| {
                // Find records that are template instantiations of `rs_std::DynCallable`.
                let Ok(Some(BridgeRsTypeKind::DynCallable(dyn_callable))) =
                    BridgeRsTypeKind::new(record, &db)
                else {
                    return None;
                };

                // The parameters shall be named `param_0`, `param_1`, etc.
                // These names can be reused across different dyn callables, so we reuse the same vec
                // and just grow it when we need more Idents than it currently contains.
                while dyn_callable.param_types.len() > param_idents_buffer.len() {
                    param_idents_buffer.push(format_ident!("param_{}", param_idents_buffer.len()));
                }
                // Only take as many filled in names as we need.
                let param_idents = &param_idents_buffer[..dyn_callable.param_types.len()];

                // If generate_dyn_callable_cpp_thunk fails, skip. We don't need to generate a nice
                // error because whoever uses this will also fail and generate an error at the relevant
                // site.
                let dyn_callable_cpp_decl =
                    generate_dyn_callable_cpp_thunk(&db, &dyn_callable, param_idents)?;
                let dyn_callable_rust_impl =
                    generate_dyn_callable_rust_thunk_impl(&db, dyn_callable.clone(), param_idents)?;

                Some((dyn_callable_cpp_decl, dyn_callable_rust_impl))
            })
            .unzip();

    let has_callables = !dyn_callable_rust_impls.is_empty();

    // Callables use `Box<dyn F>`.
    let extern_crate_alloc =
        has_callables.then(|| quote! { extern crate alloc; __NEWLINE__ __NEWLINE__  });

    // when we go through the main_api, we want to go through one at a time.
    // if the parent is none, we're responsible.
    // each thing needs to go through all its children.
    let ApiSnippets {
        generated_items,
        thunks,
        assertions,
        cc_details,
        features,
        member_functions: _,
    } = snippets;
    let main_api = code_snippet::generated_items_to_token_stream(
        &generated_items,
        ir,
        ir.top_level_item_ids(),
    );

    let cc_details = CppDetails {
        includes: generate_rs_api_impl_includes(&db, crubit_support_path_format, has_callables),
        dyn_callable_cpp_decls,
        thunks: cc_details,
    };

    let thunks = if thunks.is_empty() {
        None
    } else {
        Some(quote! {
            unsafe extern "C" {
                #( #thunks )*
            }
        })
    };

    let dyn_callable_rust_impls = if dyn_callable_rust_impls.is_empty() {
        None
    } else {
        Some(quote! { #( #dyn_callable_rust_impls )* })
    };

    let mod_detail = if thunks.is_none() && dyn_callable_rust_impls.is_none() {
        None
    } else {
        Some(quote! {
            mod detail {
                #[allow(unused_imports)]
                use super::*;
                #thunks
                __NEWLINE__
                #dyn_callable_rust_impls
            }
        })
    };

    let features = if features.is_empty() {
        quote! {}
    } else {
        let feature_iter = features.into_iter();
        quote! {
            #![feature( #(#feature_iter),* )]  __NEWLINE__
            #![allow(stable_features)]
        }
    };

    let assertions = if assertions.is_empty() {
        quote! {}
    } else {
        quote! {
            const _: () = { __NEWLINE__
                #( #assertions __NEWLINE__ )*
            }; __NEWLINE__
        }
    };

    Ok(BindingsTokens {
        rs_api: quote! {
            #features __NEWLINE__
            #![no_std] __NEWLINE__

            // `rust_builtin_type_abi_assumptions.md` documents why the generated
            // bindings need to relax the `improper_ctypes_definitions` warning
            // for `char` (and possibly for other built-in types in the future).
            #![allow(improper_ctypes)] __NEWLINE__

            // C++ names don't follow Rust guidelines:
            #![allow(nonstandard_style)] __NEWLINE__

            // Parts of our generated code are sometimes considered dead
            // (b/349776381).
            #![allow(dead_code, unused_mut)] __NEWLINE__
            #![deny(warnings)] __NEWLINE__ __NEWLINE__

            #extern_crate_alloc

            #main_api

            #mod_detail __NEWLINE__ __NEWLINE__

            #assertions
        },
        rs_api_impl: cc_details.into_token_stream(),
    })
}

/// Implementation of `BindingsGenerator::is_rs_type_kind_unsafe`.
fn is_rs_type_kind_unsafe(db: &dyn BindingsGenerator, rs_type_kind: RsTypeKind) -> bool {
    match rs_type_kind {
        RsTypeKind::Error { .. } => true,
        RsTypeKind::Pointer { .. } => true,
        RsTypeKind::Reference { referent: t, .. }
        | RsTypeKind::RvalueReference { referent: t, .. }
        | RsTypeKind::TypeAlias { underlying_type: t, .. } => {
            db.is_rs_type_kind_unsafe(t.as_ref().clone())
        }
        RsTypeKind::FuncPtr { return_type, param_types, .. } => {
            db.is_rs_type_kind_unsafe(return_type.as_ref().clone())
                || param_types
                    .iter()
                    .cloned()
                    .any(|param_type| db.is_rs_type_kind_unsafe(param_type))
        }
        RsTypeKind::IncompleteRecord { .. } => {
            // TODO(b/390474240): Add a way to mark a forward declaration as being an unsafe
            // type.
            false
        }
        RsTypeKind::Enum { .. }
        | RsTypeKind::Primitive(..)
        | RsTypeKind::ExistingRustType { .. } => false,
        RsTypeKind::BridgeType { bridge_type, original_type } => match bridge_type {
            // TODO(b/390621592): Should bridge types just delegate to the underlying type?
            BridgeRsTypeKind::BridgeVoidConverters { .. }
            | BridgeRsTypeKind::Bridge { .. }
            | BridgeRsTypeKind::ProtoMessageBridge { .. } => is_record_unsafe(db, &original_type),
            BridgeRsTypeKind::StdOptional(t) => db.is_rs_type_kind_unsafe(t.as_ref().clone()),
            BridgeRsTypeKind::StdPair(t1, t2) => {
                db.is_rs_type_kind_unsafe(t1.as_ref().clone())
                    || db.is_rs_type_kind_unsafe(t2.as_ref().clone())
            }
            BridgeRsTypeKind::StdString { .. } => false,
            BridgeRsTypeKind::DynCallable(dyn_callable) => {
                db.is_rs_type_kind_unsafe(dyn_callable.return_type.as_ref().clone())
                    || dyn_callable
                        .param_types
                        .iter()
                        .cloned()
                        .any(|param_type| db.is_rs_type_kind_unsafe(param_type))
            }
        },
        RsTypeKind::Record { record, .. } => is_record_unsafe(db, &record),
        RsTypeKind::C9Co { result_type, .. } => {
            // A Co<T> logically produces a T, so it is unsafe iff T is unsafe.
            db.is_rs_type_kind_unsafe(result_type.as_ref().clone())
        }
    }
}

/// Helper function for `is_rs_type_kind_unsafe`.
/// Returns true if the record is unsafe, or if it transitively contains a public field of
/// an unsafe type.
fn is_record_unsafe(db: &dyn BindingsGenerator, record: &Record) -> bool {
    if record.is_unsafe_type {
        return true;
    }

    if record.record_type == RecordType::Union {
        return true;
    }
    for field in &record.fields {
        if field.access != AccessSpecifier::Public {
            continue;
        }
        let Ok(cpp_type) = &field.type_ else {
            // If we can't get the CcType for a public field, we assume it's unsafe.
            return true;
        };
        let Ok(field_rs_type_kind) = db.rs_type_kind(cpp_type.clone()) else {
            // If we can't get the RsTypeKind for a public field, we assume it's unsafe.
            return true;
        };
        if db.is_rs_type_kind_unsafe(field_rs_type_kind) {
            return true;
        }
    }
    false
}

fn generate_rs_api_impl_includes(
    db: &Database,
    crubit_support_path_format: Format<1>,
    has_callables: bool,
) -> CppIncludes {
    let ir = db.ir();

    let mut internal_includes = BTreeSet::new();
    internal_includes.insert(CcInclude::memory()); // ubiquitous.
    if ir.records().next().is_some() {
        internal_includes.insert(CcInclude::cstddef());
        internal_includes.insert(CcInclude::SupportLibHeader(
            crubit_support_path_format.clone(),
            "internal/sizeof.h".into(),
        ));
    };

    if has_callables {
        internal_includes.insert(CcInclude::SupportLibHeader(
            crubit_support_path_format.clone(),
            "rs_std/dyn_callable.h".into(),
        ));
    }

    for record in ir.records() {
        // Err means that this bridge type has some issues. For the purpose of generating includes,
        // we can ignore it.
        if let Ok(Some(bridge_type)) = BridgeRsTypeKind::new(record, db) {
            if bridge_type.is_void_converters_bridge_type() {
                internal_includes.insert(CcInclude::SupportLibHeader(
                    crubit_support_path_format.clone(),
                    "internal/lazy_init.h".into(),
                ));
            } else {
                internal_includes.insert(CcInclude::SupportLibHeader(
                    crubit_support_path_format.clone(),
                    "bridge.h".into(),
                ));
                internal_includes.insert(CcInclude::SupportLibHeader(
                    crubit_support_path_format.clone(),
                    "internal/slot.h".into(),
                ));
            }
        }

        if let Ok(rs_type_kind) = db.rs_type_kind((&**record).into()) {
            if rs_type_kind.as_c9_co().is_some() {
                let includes = [
                    "util/c9/internal/rust/co_vtable.h",
                    "util/c9/internal/rust/destroy_coroutine_frame_from_rust.h",
                    "util/c9/internal/rust/start_coroutine_from_rust.h",
                    "util/c9/internal/pass_key.h",
                ];

                for file in includes {
                    internal_includes.insert(CcInclude::user_header(file.into()));
                }
            }
        };
    }

    for type_alias in ir.type_aliases() {
        let Ok(rs_type_kind) = db.rs_type_kind((&**type_alias).into()) else {
            continue;
        };

        if let RsTypeKind::BridgeType { bridge_type, .. } = rs_type_kind.unalias() {
            if bridge_type.is_void_converters_bridge_type() {
                internal_includes.insert(CcInclude::SupportLibHeader(
                    crubit_support_path_format.clone(),
                    "internal/lazy_init.h".into(),
                ));
            } else {
                internal_includes.insert(CcInclude::SupportLibHeader(
                    crubit_support_path_format.clone(),
                    "bridge.h".into(),
                ));
                internal_includes.insert(CcInclude::SupportLibHeader(
                    crubit_support_path_format.clone(),
                    "internal/slot.h".into(),
                ));
            }
        }
    }

    for crubit_header in ["internal/cxx20_backports.h", "internal/offsetof.h"] {
        internal_includes.insert(CcInclude::SupportLibHeader(
            crubit_support_path_format.clone(),
            crubit_header.into(),
        ));
    }
    let internal_includes = format_cc_includes(&internal_includes);

    // In order to generate C++ thunk in all the cases Clang needs to be able to
    // access declarations from public headers of the C++ library.  We don't
    // process these includes via `format_cc_includes` to preserve their
    // original order (some libraries require certain headers to be included
    // first - e.g. `config.h`).
    let ir_includes =
        ir.public_headers().map(|hdr| CcInclude::user_header(hdr.name.clone())).collect_vec();

    CppIncludes { internal_includes, ir_includes }
}

fn make_transmute_abi_type_from_item(
    item: &impl GenericItem,
    rs_name: &str,
    cc_name: &str,
    db: &dyn BindingsGenerator,
) -> Result<CrubitAbiType> {
    // Rust names are of the form ":: tuples_golden :: NontrivialDrop"
    let mut rust_path = rs_name;
    let mut start_with_colon2 = false;
    if let Some(strip_universal_qualifier) = rust_path.strip_prefix(":: ") {
        start_with_colon2 = true;
        rust_path = strip_universal_qualifier;
    }
    let rust_type = FullyQualifiedPath {
        start_with_colon2,
        parts: rust_path
            .split("::")
            .map(|ident| {
                syn::parse_str::<Ident>(ident.trim()).map_err(|_| {
                    anyhow!(
                        "The type `{ident}` does not parse as an identifier. \
                        This may be because it contains template parameters, and \
                        bridging such types by value is not yet supported."
                    )
                })
            })
            .collect::<Result<Rc<[Ident]>>>()?,
    };

    let cpp_type = make_cpp_type_from_item(item, &cc_name.split("::").collect::<Vec<&str>>(), db)?
        .to_token_stream();

    Ok(CrubitAbiType::Transmute { rust_type, cpp_type })
}

/// Implementation of `BindingsGenerator::crubit_abi_type`.
fn crubit_abi_type(db: &dyn BindingsGenerator, rs_type_kind: RsTypeKind) -> Result<CrubitAbiType> {
    match rs_type_kind {
        RsTypeKind::Error { error, .. } => {
            bail!("Type has an error and cannot be bridged: {error}")
        }
        RsTypeKind::TypeAlias { underlying_type, .. } => {
            // We don't actually _have_ to expand the type alias here
            db.crubit_abi_type(underlying_type.as_ref().clone())
        }
        RsTypeKind::Pointer { pointee, kind, mutability } => {
            let rust_tokens = pointee.to_token_stream(db);
            let cpp_tokens = format_cpp_type_with_references(&pointee, db.ir())?;

            Ok(CrubitAbiType::Ptr {
                is_const: mutability == Mutability::Const,
                is_rust_slice: kind == RustPtrKind::Slice,
                rust_type: rust_tokens,
                cpp_type: cpp_tokens,
            })
        }
        RsTypeKind::Enum { enum_, .. } => make_transmute_abi_type_from_item(
            enum_.as_ref(),
            enum_.rs_name.identifier.as_ref(),
            enum_.cc_name.identifier.as_ref(),
            db,
        ),
        RsTypeKind::ExistingRustType(existing_rust_type) => make_transmute_abi_type_from_item(
            existing_rust_type.as_ref(),
            existing_rust_type.rs_name.as_ref(),
            existing_rust_type.cc_name.as_ref(),
            db,
        ),
        RsTypeKind::Primitive(primitive) => Ok(match primitive {
            Primitive::Bool => CrubitAbiType::transmute("bool", "bool"),
            Primitive::Void => bail!("values of type `void` cannot be bridged by value"),
            Primitive::Float => CrubitAbiType::transmute("f32", "float"),
            Primitive::Double => CrubitAbiType::transmute("f64", "double"),
            Primitive::Char => CrubitAbiType::transmute("::core::ffi::c_char", "char"),
            Primitive::SignedChar => CrubitAbiType::SignedChar,
            Primitive::UnsignedChar => CrubitAbiType::UnsignedChar,
            Primitive::Short => CrubitAbiType::transmute("::core::ffi::c_short", "short"),
            Primitive::Int => CrubitAbiType::transmute("::core::ffi::c_int", "int"),
            Primitive::Long => CrubitAbiType::transmute("::core::ffi::c_long", "long"),
            Primitive::LongLong => CrubitAbiType::LongLong,
            Primitive::UnsignedShort => CrubitAbiType::UnsignedShort,
            Primitive::UnsignedInt => CrubitAbiType::UnsignedInt,
            Primitive::UnsignedLong => CrubitAbiType::UnsignedLong,
            Primitive::UnsignedLongLong => CrubitAbiType::UnsignedLongLong,
            Primitive::Char16T => CrubitAbiType::transmute("u16", "char16_t"),
            Primitive::Char32T => CrubitAbiType::transmute("u32", "char32_t"),
            Primitive::PtrdiffT => CrubitAbiType::transmute("isize", "ptrdiff_t"),
            Primitive::IntptrT => CrubitAbiType::transmute("isize", "intptr_t"),
            Primitive::StdPtrdiffT => CrubitAbiType::transmute("isize", "std::ptrdiff_t"),
            Primitive::StdIntptrT => CrubitAbiType::transmute("isize", "std::intptr_t"),
            Primitive::SizeT => CrubitAbiType::transmute("usize", "size_t"),
            Primitive::UintptrT => CrubitAbiType::transmute("usize", "uintptr_t"),
            Primitive::StdSizeT => CrubitAbiType::transmute("usize", "std::size_t"),
            Primitive::StdUintptrT => CrubitAbiType::transmute("usize", "std::uintptr_t"),
            Primitive::Int8T => CrubitAbiType::transmute("i8", "int8_t"),
            Primitive::Int16T => CrubitAbiType::transmute("i16", "int16_t"),
            Primitive::Int32T => CrubitAbiType::transmute("i32", "int32_t"),
            Primitive::Int64T => CrubitAbiType::transmute("i64", "int64_t"),
            Primitive::StdInt8T => CrubitAbiType::transmute("i8", "std::int8_t"),
            Primitive::StdInt16T => CrubitAbiType::transmute("i16", "std::int16_t"),
            Primitive::StdInt32T => CrubitAbiType::transmute("i32", "std::int32_t"),
            Primitive::StdInt64T => CrubitAbiType::transmute("i64", "std::int64_t"),
            Primitive::Uint8T => CrubitAbiType::transmute("u8", "uint8_t"),
            Primitive::Uint16T => CrubitAbiType::transmute("u16", "uint16_t"),
            Primitive::Uint32T => CrubitAbiType::transmute("u32", "uint32_t"),
            Primitive::Uint64T => CrubitAbiType::transmute("u64", "uint64_t"),
            Primitive::StdUint8T => CrubitAbiType::transmute("u8", "std::uint8_t"),
            Primitive::StdUint16T => CrubitAbiType::transmute("u16", "std::uint16_t"),
            Primitive::StdUint32T => CrubitAbiType::transmute("u32", "std::uint32_t"),
            Primitive::StdUint64T => CrubitAbiType::transmute("u64", "std::uint64_t"),
        }),
        RsTypeKind::BridgeType { bridge_type, original_type } => match bridge_type {
            BridgeRsTypeKind::BridgeVoidConverters { .. } => {
                bail!("Void pointer bridge types are not allowed within composable bridging")
            }
            BridgeRsTypeKind::ProtoMessageBridge { .. } => {
                let target =
                    original_type.defining_target().unwrap_or(&original_type.owning_target);
                let rust_abi_path =
                    make_rust_abi_path_from_str("ProtoMessageRustBridge", db.ir(), target);

                let cpp_namespace_qualifier = db.ir().namespace_qualifier(original_type.as_ref());

                // Rust message types are exported to crate root, but we need the full namespace for the C++ ABI.
                let merged_cpp_abi_path = cpp_namespace_qualifier.parts().join("::")
                    + "::"
                    + original_type.cc_name.identifier.as_ref();

                Ok(CrubitAbiType::ProtoMessage {
                    proto_message_rust_bridge: rust_abi_path,
                    rust_proto_path: make_rust_abi_path_from_str(
                        original_type.rs_name.identifier.as_ref(),
                        db.ir(),
                        target,
                    ),
                    cpp_proto_path: make_cpp_abi_path_from_str(&merged_cpp_abi_path)?,
                })
            }
            BridgeRsTypeKind::Bridge { abi_rust, abi_cpp, generic_types, .. } => {
                let target =
                    original_type.defining_target().unwrap_or(&original_type.owning_target);
                let rust_abi_path = make_rust_abi_path_from_str(&abi_rust, db.ir(), target);

                let cpp_abi_path = make_cpp_abi_path_from_str(&abi_cpp)?;

                let type_args = generic_types
                    .iter()
                    .map(|t: &RsTypeKind| db.crubit_abi_type(t.clone()))
                    .collect::<Result<Rc<[CrubitAbiType]>>>()?;

                Ok(CrubitAbiType::Type { rust_abi_path, cpp_abi_path, type_args })
            }
            BridgeRsTypeKind::StdOptional(inner) => {
                let inner_abi = db.crubit_abi_type(inner.as_ref().clone())?;
                Ok(CrubitAbiType::option(inner_abi))
            }
            BridgeRsTypeKind::StdPair(first, second) => {
                let first_abi = db.crubit_abi_type(first.as_ref().clone())?;
                let second_abi = db.crubit_abi_type(second.as_ref().clone())?;
                Ok(CrubitAbiType::Pair(Rc::from(first_abi), Rc::from(second_abi)))
            }
            BridgeRsTypeKind::StdString { in_cc_std } => Ok(CrubitAbiType::StdString { in_cc_std }),
            BridgeRsTypeKind::DynCallable(dyn_callable) => {
                generate_dyn_callable::dyn_callable_crubit_abi_type(db, &dyn_callable)
            }
        },
        RsTypeKind::Record { record, crate_path, .. } => {
            ensure!(
                record.is_unpin(),
                "Type `{}` must be Rust-movable in order to memcpy through a bridge buffer. See crubit.rs/cpp/classes_and_structs#rust_movable",
                record.cc_name
            );

            let rust_type = crate_path
                .to_fully_qualified_path(make_rs_ident(record.rs_name.identifier.as_ref()));

            // This inlines the logic of code_gen_utils::format_cc_ident and joins the namespace parts,
            // except that it creates an Ident instead of a TokenStream.
            code_gen_utils::check_valid_cc_name(&record.cc_name.identifier)
                .expect("IR should only contain valid C++ types");

            // TODO(okabayashi): File a bug for generalizing "canonical insts".
            let cc_name = record.cc_name.identifier.as_ref();
            let cc_name_parts = if cc_name == "std::basic_string_view<char, std::char_traits<char>>"
            {
                // In the C++ TransmuteAbi, we spell string_view as `std::string_view`.
                // In theory we should let Crubit spell the C++ type as
                // `std::basic_string_view<char, std::char_traits<char>>`, but `FullyQualifiedPath`
                // does not support template arguments right now. It's also the case that since
                // Crubit doesn't support templates in general right now, it doesn't make sense to
                // support template arguments in `FullyQualifiedPath` yet.
                &["std", "string_view"][..]
            } else {
                &[cc_name][..]
            };
            let cpp_type =
                make_cpp_type_from_item(record.as_ref(), cc_name_parts, db)?.to_token_stream();

            Ok(CrubitAbiType::Transmute { rust_type, cpp_type })
        }
        _ => bail!("Unsupported RsTypeKind: {}", rs_type_kind.display(db)),
    }
}

/// Generates a unique C++ declaration of an extern "C" function.
///
/// `None` is returned if there is issue generating the thunk. The specific error is not reported
/// because it will be reported elsewhere.
fn generate_dyn_callable_cpp_thunk(
    db: &dyn BindingsGenerator,
    dyn_callable: &Callable,
    param_idents: &[Ident],
) -> Option<TokenStream> {
    assert!(
        param_idents.len() == dyn_callable.param_types.len(),
        "param_idents and param_types should have the same length, this is a Crubit bug."
    );
    let param_types = dyn_callable
        .param_types
        .iter()
        .map(|param_type| -> Option<TokenStream> {
            if param_type.is_c_abi_compatible_by_value() {
                cpp_type_name::format_cpp_type(param_type, db.ir()).ok()
            } else if param_type.is_crubit_abi_bridge_type() {
                Some(quote! { unsigned char* })
            } else {
                // For the layout compatible types, we take a pointer and then ptr::read the
                // contents into Rust.
                let param_type_tokens = cpp_type_name::format_cpp_type(param_type, db.ir()).ok()?;
                Some(quote! { #param_type_tokens* })
            }
        })
        .collect::<Option<Vec<TokenStream>>>()?;

    let out_param_ident;
    let out_param_type;
    let decl_return_type_tokens;
    if dyn_callable.return_type.is_void() {
        out_param_ident = None;
        out_param_type = None;
        decl_return_type_tokens = quote! { void };
    } else if dyn_callable.return_type.is_c_abi_compatible_by_value() {
        out_param_ident = None;
        out_param_type = None;
        decl_return_type_tokens =
            cpp_type_name::format_cpp_type(&dyn_callable.return_type, db.ir()).ok()?;
    } else if dyn_callable.return_type.is_crubit_abi_bridge_type() {
        out_param_ident = Some(format_ident!("out"));
        out_param_type = Some(quote! { unsigned char* });
        decl_return_type_tokens = quote! { void };
    } else {
        let return_type_tokens =
            cpp_type_name::format_cpp_type(&dyn_callable.return_type, db.ir()).ok()?;
        out_param_ident = Some(format_ident!("out"));
        out_param_type = Some(quote! { #return_type_tokens* });
        decl_return_type_tokens = quote! { void };
    };

    let param_idents = param_idents.iter().chain(out_param_ident.as_ref());
    let param_types = param_types.iter().chain(out_param_type.as_ref());
    let thunk_ident = &dyn_callable.thunk_ident;

    Some(quote! {
        extern "C" #decl_return_type_tokens #thunk_ident(
            ::rs_std::internal_dyn_callable::TypeErasedState* state
            #(
                , #param_types #param_idents
            )*
        );
    })
}

/// Generates a unique Rust definition of an extern "C" function.
///
/// This roughly has the form:
/// ```rust
/// #[unsafe(no_mangle)]
/// unsafe extern "C" fn some_mangled_name(
///     f: *mut ::alloc::boxed::Box<dyn Fn()>,
/// ) {
///     // do argument conversions, invoke the callable, and convert the result.
/// }
/// ```
///
/// This definition is responsible for actually calling the `dyn Fn()` object, and is exposed to C++
/// by the thunk generated by `generate_dyn_callable_cpp_thunk`.
///
/// `None` is returned if there is issue generating the definition. The specific error is not
/// reported because it will be reported elsewhere.
fn generate_dyn_callable_rust_thunk_impl(
    db: &dyn BindingsGenerator,
    dyn_callable: Rc<Callable>,
    param_idents: &[Ident],
) -> Option<TokenStream> {
    assert!(
        param_idents.len() == dyn_callable.param_types.len(),
        "param_idents and param_types should have the same length, this is a Crubit bug."
    );
    let mut ffi_to_rust_transforms = quote! {};

    let param_types_tokens = param_idents
        .iter()
        .zip(dyn_callable.param_types.iter())
        .map(|(ident, ty)| -> Option<TokenStream> {
            if ty.is_crubit_abi_bridge_type() {
                let crubit_abi_type = db.crubit_abi_type(ty.clone()).ok()?;
                let crubit_abi_type_expr_tokens = CrubitAbiTypeToRustExprTokens(&crubit_abi_type);
                ffi_to_rust_transforms.extend(quote! {
                    let #ident = ::bridge_rust::internal::decode(#crubit_abi_type_expr_tokens, #ident);
                });
                Some(quote! { *mut ::core::ffi::c_uchar })
            } else if ty.is_c_abi_compatible_by_value() {
                Some(ty.to_token_stream(db))
            } else {
                ffi_to_rust_transforms.extend(quote! {
                    let #ident = ::core::ptr::read(#ident);
                });
                let ty_tokens = ty.to_token_stream(db);
                Some(quote! { *mut #ty_tokens })
            }
        })
        .collect::<Option<Vec<TokenStream>>>()?;

    let unwrapper = match dyn_callable.fn_trait {
        FnTrait::Fn => quote! { &*f },
        FnTrait::FnMut => quote! { &mut *f },
        FnTrait::FnOnce => quote! {
            // SAFETY: For FnOnce, DynCallable ensures that the invoker (where this read occurs) is
            // replaced after the first call, ensuring that this happens at most once.
            ::core::ptr::read(f)
        },
    };
    let mut invoke_rust_and_return_to_ffi = quote! {
        (unsafe { #unwrapper })(#(#param_idents),*)
    };

    let return_type_fragment;
    let out_param_ident;
    let out_param_type;
    if dyn_callable.return_type.is_void() {
        // Put a semicolon at the end to clarify that we do not return anything.
        invoke_rust_and_return_to_ffi = quote! {
            #invoke_rust_and_return_to_ffi;
        };

        return_type_fragment = None;
        out_param_ident = None;
        out_param_type = None;
    } else if dyn_callable.return_type.is_c_abi_compatible_by_value() {
        let ffi_return_type = dyn_callable.return_type.to_token_stream(db);
        return_type_fragment = Some(quote! { -> #ffi_return_type });
        out_param_ident = None;
        out_param_type = None;
    } else if dyn_callable.return_type.is_crubit_abi_bridge_type() {
        let crubit_abi_type = db.crubit_abi_type(dyn_callable.return_type.as_ref().clone()).ok()?;
        let crubit_abi_type_expr_tokens = CrubitAbiTypeToRustExprTokens(&crubit_abi_type);
        let bridge_buffer_ident = format_ident!("bridge_buffer");
        invoke_rust_and_return_to_ffi = quote! {
            ::bridge_rust::internal::encode(
                #crubit_abi_type_expr_tokens,
                #bridge_buffer_ident,
                #invoke_rust_and_return_to_ffi
            );
        };

        return_type_fragment = None;
        out_param_ident = Some(bridge_buffer_ident);
        out_param_type = Some(quote! { *mut ::core::ffi::c_uchar });
    } else {
        let out_ident = format_ident!("out");
        invoke_rust_and_return_to_ffi = quote! {
            match #invoke_rust_and_return_to_ffi {
                result => unsafe {
                    ::core::ptr::write(#out_ident, result);
                }
            }
        };

        let ffi_return_type = dyn_callable.return_type.to_token_stream(db);
        return_type_fragment = None;
        out_param_ident = Some(out_ident);
        out_param_type = Some(quote! { *mut #ffi_return_type });
    }

    let param_idents = param_idents.iter().chain(out_param_ident.as_ref());
    let param_types_tokens = param_types_tokens.iter().chain(out_param_type.as_ref());
    let dyn_fn_spelling = dyn_callable.dyn_fn_spelling(db);
    let thunk_ident = &dyn_callable.thunk_ident;

    Some(quote! {
        #[unsafe(no_mangle)]
        unsafe extern "C" fn #thunk_ident(
            f: *mut ::alloc::boxed::Box<#dyn_fn_spelling>,
            #(
                #param_idents: #param_types_tokens,
            )*
        ) #return_type_fragment {
            #ffi_to_rust_transforms

            #invoke_rust_and_return_to_ffi
        }
    })
}

/// Parses the given Rust path into a [`FullyQualifiedPath`].
/// * if the path is fully qualified, it stays unchanged.
/// * else, if it is the current target, it is prepended with "crate".
/// * else, it is prepended with the "::" and the crate name.
fn make_rust_abi_path_from_str(
    mut rust_path: &str,
    ir: &IR,
    target: &BazelLabel,
) -> FullyQualifiedPath {
    let mut start_with_colon2 = strip_leading_colon2(&mut rust_path);

    let prefix = if start_with_colon2 {
        None
    } else if ir.is_current_target(target) {
        Some(Ident::new("crate", proc_macro2::Span::call_site()))
    } else {
        start_with_colon2 = true;
        Some(make_rs_ident(target.target_name()))
    };

    FullyQualifiedPath {
        start_with_colon2,
        parts: prefix
            .into_iter()
            .chain(rust_path.split("::").map(make_rs_ident))
            .collect::<Rc<[Ident]>>(),
    }
}

/// Parses the given C++ path into a [`FullyQualifiedPath`].
fn make_cpp_abi_path_from_str(mut cpp_path: &str) -> Result<FullyQualifiedPath> {
    let start_with_colon2 = strip_leading_colon2(&mut cpp_path);
    Ok(FullyQualifiedPath {
        start_with_colon2,
        parts: cpp_path
            .split("::")
            .map(|part| {
                ensure!(!part.is_empty(), "cpp path has an empty part: {cpp_path}");
                ensure!(
                    !is_cpp_reserved_keyword(part),
                    "cpp path has a reserved keyword: {cpp_path}"
                );
                // Can't reuse machinery in code_gen_utils because that returns
                // a TokenStream. We _need_ an Ident because it implements Hash.
                syn::parse_str::<Ident>(part)
                    .map_err(|err| anyhow!("Can't format `{part}` as a C++ identifier: {err}"))
            })
            .collect::<Result<Rc<[Ident]>>>()?,
    })
}

/// Strips the leading `::` from the given path if it exists.
///
/// Returns true if the path was modified, false otherwise.
fn strip_leading_colon2(path: &mut &str) -> bool {
    if let Some(stripped) = path.strip_prefix("::") {
        *path = stripped;
        true
    } else {
        false
    }
}

/// Only to be used in a `CrubitAbiType::Transmute` context.
fn make_cpp_type_from_item(
    item: &impl GenericItem,
    cc_name_parts: &[&str],
    db: &dyn BindingsGenerator,
) -> Result<FullyQualifiedPath> {
    let namespace_qualifier = db.ir().namespace_qualifier(item);
    let parts = namespace_qualifier
        .parts()
        .map(AsRef::as_ref)
        .chain(cc_name_parts.iter().copied())
        .map(|ident| {
            syn::parse_str::<Ident>(ident).map_err(|_| {
                anyhow!(
                    "The type `{ident}` does not parse as an identifier. \
            This may be because it contains template parameters, and \
            bridging such types by value is not yet supported."
                )
            })
        })
        .collect::<Result<Rc<[Ident]>>>()?;

    Ok(FullyQualifiedPath { start_with_colon2: true, parts })
}
