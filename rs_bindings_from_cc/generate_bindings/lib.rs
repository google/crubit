// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

use arc_anyhow::{Context, Result};
use code_gen_utils::{format_cc_includes, make_rs_ident, CcInclude};
use database::code_snippet::{ApiSnippets, Bindings, BindingsTokens};
use database::db::{BindingsGenerator, Database, ReportFatalError};
use database::rs_snippet::RsTypeKind;
use error_report::{bail, ErrorReporting};
use ffi_types::Environment;
use generate_comment::generate_top_level_comment;
use generate_comment::{generate_comment, generate_doc_comment, generate_unsupported};
use generate_struct_and_union::generate_incomplete_record;
use has_bindings::{has_bindings, HasBindings};
use ir::*;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use rs_type_kind::rs_type_kind;
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use token_stream_printer::{
    cc_tokens_to_formatted_string, rs_tokens_to_formatted_string, RustfmtConfig,
};

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

    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(
        &ir,
        crubit_support_path_format,
        errors,
        fatal_errors,
        environment,
    )?;
    let rs_api = {
        let rustfmt_exe_path = Path::new(rustfmt_exe_path);
        let rustfmt_config_path = if rustfmt_config_path.is_empty() {
            None
        } else {
            Some(Path::new(rustfmt_config_path))
        };
        let rustfmt_config = RustfmtConfig::new(rustfmt_exe_path, rustfmt_config_path);
        rs_tokens_to_formatted_string(rs_api, &rustfmt_config)?
    };
    let rs_api_impl = cc_tokens_to_formatted_string(rs_api_impl, Path::new(clang_format_exe_path))?;

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

fn generate_type_alias(db: &dyn BindingsGenerator, type_alias: &TypeAlias) -> Result<ApiSnippets> {
    // Skip the type alias if it maps to a bridge type.
    let rs_type_kind = RsTypeKind::new_type_alias(db, Rc::new(type_alias.clone()))?;
    if rs_type_kind.is_bridge_type() {
        return Ok(ApiSnippets::default());
    }
    let ident = make_rs_ident(&type_alias.rs_name.identifier);
    let doc_comment = generate_doc_comment(
        type_alias.doc_comment.as_deref(),
        Some(&type_alias.source_loc),
        db.environment(),
    );
    let underlying_type = db
        .rs_type_kind(type_alias.underlying_type.rs_type.clone())
        .with_context(|| format!("Failed to format underlying type for {}", type_alias))?;

    let underlying_type_tokens = underlying_type.to_token_stream(db);
    Ok(quote! {
        #doc_comment
        pub type #ident = #underlying_type_tokens;
    }
    .into())
}

fn generate_global_var(db: &dyn BindingsGenerator, var: &GlobalVar) -> Result<ApiSnippets> {
    let ident = make_rs_ident(&var.rs_name.identifier);
    let type_ = db.rs_type_kind(var.type_.rs_type.clone())?;

    let link_name = if let Some(mangled_name) = &var.mangled_name {
        let mangled_name = &**mangled_name;
        quote! { #[link_name = #mangled_name] }
    } else {
        quote! {}
    };
    let mutness = if !var.type_.cpp_type.is_const { quote!(mut) } else { quote!() };
    let type_tokens = type_.to_token_stream(db);
    Ok(quote! {
        extern "C" {
            #link_name
            pub static #mutness #ident: #type_tokens;
        }
    }
    .into())
}

fn generate_namespace(db: &dyn BindingsGenerator, namespace: &Namespace) -> Result<ApiSnippets> {
    let ir = db.ir();
    let mut items = vec![];
    let mut thunks = vec![];
    let mut cc_details = vec![];
    let mut assertions = vec![];
    let mut features = BTreeSet::new();

    for item_id in namespace.child_item_ids.iter() {
        let item: &Item = ir.find_decl(*item_id).with_context(|| {
            format!("Failed to look up namespace.child_item_ids for {:?}", namespace)
        })?;
        let generated = db.generate_item(item.clone())?;
        items.push(generated.main_api);
        if !generated.thunks.is_empty() {
            thunks.push(generated.thunks);
        }
        if !generated.cc_details.is_empty() {
            cc_details.push(generated.cc_details);
        }
        if !generated.assertions.is_empty() {
            assertions.push(generated.assertions);
        }
        features.extend(generated.features);
    }

    let reopened_namespace_idx = ir.get_reopened_namespace_idx(namespace.id)?;
    // True if this is actually the module with the name `#name`, rather than e.g.
    // `#name_0`, `#name_1`, etc.
    let is_canonical_namespace_module =
        ir.is_last_reopened_namespace(namespace.id, namespace.canonical_namespace_id)?;

    let name = if is_canonical_namespace_module {
        make_rs_ident(&namespace.rs_name.identifier)
    } else {
        make_rs_ident(&format!("{}_{}", &namespace.rs_name.identifier, reopened_namespace_idx))
    };

    let use_stmt_for_previous_namespace = if reopened_namespace_idx == 0 {
        quote! {}
    } else {
        let previous_namespace_ident = make_rs_ident(&format!(
            "{}_{}",
            &namespace.rs_name.identifier,
            reopened_namespace_idx - 1
        ));
        // unused_imports warns a re-export of an empty module. Currently, there is no
        // infra in Crubit to tell if the (generated) module is empty, so we
        // emit `allow(unused_imports)`. TODO(b/308949532): Skip re-export if
        // previous module is empty (transitively).
        quote! {
          __HASH_TOKEN__ [allow(unused_imports)]
          pub use super::#previous_namespace_ident::*; __NEWLINE__ __NEWLINE__
        }
    };
    let use_stmt_for_inline_namespace = if namespace.is_inline && is_canonical_namespace_module {
        // TODO(b/308949532): Skip re-export if the canonical module is empty
        // (transitively).
        quote! {
          __HASH_TOKEN__ [allow(unused_imports)]
          pub use #name::*; __NEWLINE__
        }
    } else {
        quote! {}
    };

    let namespace_tokens = quote! {
        pub mod #name {
            #use_stmt_for_previous_namespace

            #( #items __NEWLINE__ __NEWLINE__ )*
        }
        __NEWLINE__
        #use_stmt_for_inline_namespace
    };

    Ok(ApiSnippets {
        main_api: namespace_tokens,
        features,
        thunks: quote! { #( #thunks )* },
        cc_details: quote! { #( #cc_details )* },
        assertions: quote! { #( #assertions )* },
        ..Default::default()
    })
}

/// Returns generated bindings for an item, or `Err` if bindings generation
/// failed in such a way as to make the generated bindings as a whole invalid.
fn generate_item(db: &dyn BindingsGenerator, item: Item) -> Result<ApiSnippets> {
    let err = match generate_item_impl(db, &item) {
        Ok(generated) => return Ok(generated),
        Err(err) => err,
    };

    // We didn't guarantee that bindings would exist, so it is not invalid to
    // write down the error but continue.
    let unsupported_item = match item {
        Item::Enum(enum_) => {
            // For now, we special case on enums because they previously reported their own errors from generate_enum and it has more information than the general case.
            let unsupported_item_path = UnsupportedItemPath {
                ident: UnqualifiedIdentifier::Identifier(enum_.cc_name.clone()),
                enclosing_item_id: enum_.enclosing_item_id,
            };
            UnsupportedItem::new_with_cause(db.ir(), &enum_, Some(unsupported_item_path), err)
        }
        _ => {
            if has_bindings(db, &item) == HasBindings::Yes {
                return Err(err);
            }
            // FIXME(cramertj): get paths here in more cases. It may be that
            // `generate_item_impl` failed in such a way that the path is still available.
            UnsupportedItem::new_with_cause(db.ir(), &item, /* path= */ None, err)
        }
    };

    Ok(generate_unsupported(db, &unsupported_item))
}

/// The implementation of generate_item, without the error recovery logic.
///
/// Returns Err if bindings could not be generated for this item.
fn generate_item_impl(db: &dyn BindingsGenerator, item: &Item) -> Result<ApiSnippets> {
    let ir = db.ir();
    if let Some(owning_target) = item.owning_target() {
        if !ir.is_current_target(owning_target) {
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
                if db.overloaded_funcs().contains(&generated_function.id) {
                    bail!("Cannot generate bindings for overloaded function")
                } else {
                    (*generated_function.snippets).clone()
                }
            }
        },
        Item::IncompleteRecord(incomplete_record) => {
            generate_incomplete_record(db, incomplete_record)?
        }
        Item::Record(record) => db.generate_record(Rc::clone(record))?,
        Item::Enum(enum_) => db.generate_enum(Rc::clone(enum_))?,
        Item::GlobalVar(var) => generate_global_var(db, var)?,
        Item::TypeAlias(type_alias) => generate_type_alias(db, type_alias)?,
        Item::UnsupportedItem(unsupported) => generate_unsupported(db, unsupported),
        Item::Comment(comment) => generate_comment(comment)?,
        Item::Namespace(namespace) => generate_namespace(db, namespace)?,
        Item::UseMod(use_mod) => {
            let UseMod { path, mod_name, .. } = &**use_mod;
            let mod_name = make_rs_ident(&mod_name.identifier);
            // TODO(b/308949532): Skip re-export if the module being used is empty
            // (transitively).
            quote! {
                #[path = #path]
                mod #mod_name;
                __HASH_TOKEN__ [allow(unused_imports)]
                pub use #mod_name::*;
            }
            .into()
        }
        Item::TypeMapOverride(type_override) => {
            let rs_type = RsTypeKind::new_type_map_override(db, type_override)?;
            let disable_comment = format!(
                "Type bindings for {cpp_type} suppressed due to being mapped to \
                    an existing Rust type ({rs_type})",
                cpp_type = type_override.debug_name(&ir),
                rs_type = rs_type.display(db),
            );
            let assertions = if let Some(size_align) = &type_override.size_align {
                generate_struct_and_union::rs_size_align_assertions(
                    rs_type.to_token_stream(db),
                    size_align,
                )
            } else {
                quote! {}
            };

            ApiSnippets {
                main_api: quote! {
                    __COMMENT__ #disable_comment
                },
                assertions,
                ..Default::default()
            }
        }
    };

    // Suppress bindings at the last minute, to collect other errors first.
    if let HasBindings::No(reason) = has_bindings(db, item) {
        return Err(reason.into());
    }

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
        is_rs_type_kind_unsafe,
        generate_enum::generate_enum,
        generate_item,
        generate_struct_and_union::generate_record,
        rs_type_kind,
        generate_function::generate_function,
        generate_function::overloaded_funcs,
        generate_function::is_record_clonable,
        generate_function::get_binding,
        generate_struct_and_union::collect_unqualified_member_functions,
    )
}

/// Returns the Rust code implementing bindings, plus any auxiliary C++ code
/// needed to support it.
//
/// Public for use in `generate_bindings_tokens_for_test`.
pub fn generate_bindings_tokens(
    ir: &IR,
    crubit_support_path_format: &str,
    errors: &dyn ErrorReporting,
    fatal_errors: &dyn ReportFatalError,
    environment: Environment,
) -> Result<BindingsTokens> {
    let db = new_database(ir, errors, fatal_errors, environment);
    let mut items = vec![];
    let mut thunks = vec![];
    let mut cc_details = vec![
        generate_rs_api_impl_includes(&db, crubit_support_path_format)?,
        quote! {
            __HASH_TOKEN__ pragma clang diagnostic push __NEWLINE__
            // Disable Clang thread-safety-analysis warnings that would otherwise
            // complain about thunks that call mutex locking functions in an unpaired way.
            __HASH_TOKEN__ pragma clang diagnostic ignored "-Wthread-safety-analysis" __NEWLINE__
        },
    ];
    let mut assertions = vec![];

    let mut features = BTreeSet::new();

    // For #![rustfmt::skip].
    features.insert(make_rs_ident("custom_inner_attributes"));
    // For the `vector` in `cc_std`.
    features.insert(make_rs_ident("allocator_api"));
    features.insert(make_rs_ident("cfg_sanitize"));

    for top_level_item_id in ir.top_level_item_ids() {
        let item: &Item =
            ir.find_decl(*top_level_item_id).context("Failed to look up ir.top_level_item_ids")?;
        let generated = db.generate_item(item.clone())?;
        items.push(generated.main_api);
        if !generated.thunks.is_empty() {
            thunks.push(generated.thunks);
        }
        if !generated.assertions.is_empty() {
            assertions.push(generated.assertions);
        }
        if !generated.cc_details.is_empty() {
            cc_details.push(generated.cc_details);
        }
        features.extend(generated.features);
    }

    cc_details.push(quote! {
        __NEWLINE__
        __HASH_TOKEN__ pragma clang diagnostic pop __NEWLINE__
        // To satisfy http://cs/symbol:devtools.metadata.Presubmit.CheckTerminatingNewline check.
        __NEWLINE__
    });

    let mod_detail = if thunks.is_empty() {
        quote! {}
    } else {
        quote! {
            mod detail {
                #[allow(unused_imports)]
                use super::*;
                unsafe extern "C" {
                    #( #thunks )*
                }
            }
        }
    };

    let features = if features.is_empty() {
        quote! {}
    } else {
        quote! {
            #![feature( #(#features),* )]  __NEWLINE__
            #![allow(stable_features)]
        }
    };

    let assertions = if assertions.is_empty() {
        quote! {}
    } else {
        quote! {
            const _: () = { __NEWLINE__
                #( #assertions __NEWLINE__ __NEWLINE__ )*
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
            #![allow(dead_code)] __NEWLINE__

            #![deny(warnings)] __NEWLINE__ __NEWLINE__

            #( #items __NEWLINE__ __NEWLINE__ )*

            #mod_detail __NEWLINE__ __NEWLINE__

            #assertions
        },
        rs_api_impl: quote! {#(#cc_details  __NEWLINE__ __NEWLINE__ )*},
    })
}

fn is_rs_type_kind_unsafe(db: &dyn BindingsGenerator, rs_type_kind: RsTypeKind) -> Result<bool> {
    match rs_type_kind {
        RsTypeKind::Pointer { .. } => Ok(true),
        RsTypeKind::Reference { referent: t, .. }
        | RsTypeKind::RvalueReference { referent: t, .. }
        | RsTypeKind::TypeAlias { underlying_type: t, .. }
        | RsTypeKind::Slice(t)
        | RsTypeKind::Option(t) => db.is_rs_type_kind_unsafe(t.as_ref().clone()),
        RsTypeKind::FuncPtr { return_type, param_types, .. } => {
            // Easier to do this imperatively when Result is involved...
            if db.is_rs_type_kind_unsafe(return_type.as_ref().clone())? {
                return Ok(true);
            }
            for param_type in param_types.iter().cloned() {
                if db.is_rs_type_kind_unsafe(param_type)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        RsTypeKind::IncompleteRecord { .. } => {
            // TODO(b/390474240): Add a way to mark a forward declaration as being an unsafe
            // type.
            Ok(false)
        }
        RsTypeKind::Enum { .. }
        | RsTypeKind::Primitive(..)
        | RsTypeKind::TypeMapOverride { .. } => Ok(false),
        // TODO(b/390621592): Should bridge types just delegate to the underlying type?
        RsTypeKind::BridgeType { original_type: record, .. }
        | RsTypeKind::Record { record, .. } => {
            if record.is_unsafe_type {
                return Ok(true);
            }
            if record.record_type == RecordType::Union {
                return Ok(true);
            }
            for field in &record.fields {
                if field.access != AccessSpecifier::Public {
                    continue;
                }
                let Ok(mapped_type) = &field.type_ else {
                    continue;
                };
                let field_rs_type_kind = db.rs_type_kind(mapped_type.rs_type.clone())?;
                if db.is_rs_type_kind_unsafe(field_rs_type_kind)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    }
}

fn generate_rs_api_impl_includes(
    db: &Database,
    crubit_support_path_format: &str,
) -> Result<TokenStream> {
    let ir = db.ir();

    let mut internal_includes = BTreeSet::new();
    internal_includes.insert(CcInclude::memory()); // ubiquitous.
    if ir.records().next().is_some() {
        internal_includes.insert(CcInclude::cstddef());
        internal_includes.insert(CcInclude::SupportLibHeader(
            crubit_support_path_format.into(),
            "internal/sizeof.h".into(),
        ));
    };

    for record in ir.records() {
        if record.bridge_type_info.is_some() {
            internal_includes.insert(CcInclude::SupportLibHeader(
                crubit_support_path_format.into(),
                "internal/lazy_init.h".into(),
            ));
        }
    }

    for type_alias in ir.type_aliases() {
        if let Ok(RsTypeKind::BridgeType { .. }) =
            RsTypeKind::new_type_alias(db, type_alias.clone())
        {
            internal_includes.insert(CcInclude::SupportLibHeader(
                crubit_support_path_format.into(),
                "internal/lazy_init.h".into(),
            ));
        }
    }

    for crubit_header in ["internal/cxx20_backports.h", "internal/offsetof.h"] {
        internal_includes.insert(CcInclude::SupportLibHeader(
            crubit_support_path_format.into(),
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

    Ok(quote! {
        #internal_includes
        __NEWLINE__
        __COMMENT__ "Public headers of the C++ library being wrapped."
        #( #ir_includes )* __NEWLINE__
    })
}
