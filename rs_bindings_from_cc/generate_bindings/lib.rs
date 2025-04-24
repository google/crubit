// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

use arc_anyhow::{anyhow, ensure, Context, Result};
use code_gen_utils::{format_cc_includes, is_cpp_reserved_keyword, make_rs_ident, CcInclude};
use crubit_abi_type::{CrubitAbiType, FullyQualifiedPath};
use database::code_snippet::{ApiSnippets, Bindings, BindingsTokens};
use database::db::{BindingsGenerator, Database};
use database::rs_snippet::{BridgeRsTypeKind, RsTypeKind};
use error_report::{bail, ErrorReporting, ReportFatalError};
use ffi_types::Environment;
use generate_comment::generate_top_level_comment;
use generate_comment::{generate_comment, generate_doc_comment, generate_unsupported};
use generate_struct_and_union::generate_incomplete_record;
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
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

fn generate_type_alias(
    db: &dyn BindingsGenerator,
    type_alias: Rc<TypeAlias>,
) -> Result<ApiSnippets> {
    // Skip the type alias if it maps to a bridge type.
    let rs_type_kind = db.rs_type_kind((&*type_alias).into())?;
    if rs_type_kind.unalias().is_bridge_type() {
        let disable_comment = format!(
            "Type alias for {cpp_type} suppressed due to being a bridge type",
            cpp_type = type_alias.debug_name(db.ir()),
        );
        return Ok(ApiSnippets {
            main_api: quote! {
                __COMMENT__ #disable_comment
            },
            ..Default::default()
        });
    }
    let ident = make_rs_ident(&type_alias.rs_name.identifier);
    let doc_comment = generate_doc_comment(
        type_alias.doc_comment.as_deref(),
        Some(&type_alias.source_loc),
        db.environment(),
    );
    let underlying_type = db
        .rs_type_kind(type_alias.underlying_type.clone())
        .with_context(|| format!("Failed to format underlying type for {}", type_alias))?;

    let underlying_type_tokens = underlying_type.to_token_stream(db);
    Ok(quote! {
        #doc_comment
        pub type #ident = #underlying_type_tokens;
    }
    .into())
}

fn generate_global_var(db: &dyn BindingsGenerator, var: Rc<GlobalVar>) -> Result<ApiSnippets> {
    let ident = make_rs_ident(&var.rs_name.identifier);
    let type_ = db.rs_type_kind(var.type_.clone())?;

    let link_name = if let Some(mangled_name) = &var.mangled_name {
        let mangled_name = &**mangled_name;
        quote! { #[link_name = #mangled_name] }
    } else {
        quote! {}
    };
    let mutness = if !var.type_.is_const { quote!(mut) } else { quote!() };
    let type_tokens = type_.to_token_stream(db);
    Ok(quote! {
        extern "C" {
            #link_name
            pub static #mutness #ident: #type_tokens;
        }
    }
    .into())
}

fn generate_namespace(db: &dyn BindingsGenerator, namespace: Rc<Namespace>) -> Result<ApiSnippets> {
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

/// Implementation of `BindingsGenerator::generate_item`.
fn generate_item(db: &dyn BindingsGenerator, item: Item) -> Result<ApiSnippets> {
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
                if db.overloaded_funcs().contains(&generated_function.id) {
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
        Item::Comment(comment) => generate_comment(comment.clone())?,
        Item::Namespace(namespace) => generate_namespace(db, namespace.clone())?,
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
            let rs_type_kind = db.rs_type_kind((&**type_override).into())?;
            let disable_comment = format!(
                "Type bindings for {cpp_type} suppressed due to being mapped to \
                    an existing Rust type ({rs_type_kind})",
                cpp_type = type_override.debug_name(&ir),
                rs_type_kind = rs_type_kind.display(db),
            );
            let assertions = if let Some(size_align) = &type_override.size_align {
                generate_struct_and_union::rs_size_align_assertions(
                    rs_type_kind.to_token_stream(db),
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
        is_rs_type_kind_unsafe,
        has_bindings::has_bindings,
        generate_enum::generate_enum,
        generate_item,
        generate_struct_and_union::generate_record,
        rs_type_kind,
        generate_function::generate_function,
        generate_function::overloaded_funcs,
        generate_function::is_record_clonable,
        generate_function::get_binding,
        generate_struct_and_union::collect_unqualified_member_functions,
        crubit_abi_type,
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
        generate_rs_api_impl_includes(&db, crubit_support_path_format),
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
        let item: &Item = ir.find_untyped_decl(*top_level_item_id);
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

/// Implementation of `BindingsGenerator::is_rs_type_kind_unsafe`.
fn is_rs_type_kind_unsafe(db: &dyn BindingsGenerator, rs_type_kind: RsTypeKind) -> bool {
    match rs_type_kind {
        RsTypeKind::Pointer { .. } => true,
        RsTypeKind::Reference { referent: t, .. }
        | RsTypeKind::RvalueReference { referent: t, .. }
        | RsTypeKind::TypeAlias { underlying_type: t, .. }
        | RsTypeKind::Slice(t) => db.is_rs_type_kind_unsafe(t.as_ref().clone()),
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
        | RsTypeKind::TypeMapOverride { .. } => false,
        RsTypeKind::BridgeType { bridge_type, original_type } => match bridge_type {
            // TODO(b/390621592): Should bridge types just delegate to the underlying type?
            BridgeRsTypeKind::BridgeVoidConverters { .. } | BridgeRsTypeKind::Bridge { .. } => {
                is_record_unsafe(db, &original_type)
            }
            BridgeRsTypeKind::StdOptional(t) => db.is_rs_type_kind_unsafe(t.as_ref().clone()),
            BridgeRsTypeKind::StdPair(t1, t2) => {
                db.is_rs_type_kind_unsafe(t1.as_ref().clone())
                    || db.is_rs_type_kind_unsafe(t2.as_ref().clone())
            }
            BridgeRsTypeKind::StdString { .. } => false,
        },
        RsTypeKind::Record { record, .. } => is_record_unsafe(db, &record),
    }
}

/// Helper function for `is_rs_type_kind_unsafe`.
/// Returns true if the record is unsafe, or if it transitively contains a public field of
/// an unsafe type.
fn is_record_unsafe(db: &dyn BindingsGenerator, record: &Record) -> bool {
    if record.is_unsafe_type {
        return true;
    }

    if !db
        .ir()
        .target_crubit_features(&record.owning_target)
        .contains(crubit_feature::CrubitFeature::UnsafeTypes)
    {
        return false;
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

fn generate_rs_api_impl_includes(db: &Database, crubit_support_path_format: &str) -> TokenStream {
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
        // Err means that this bridge type has some issues. For the purpose of generating includes,
        // we can ignore it.
        if let Ok(Some(bridge_type)) = BridgeRsTypeKind::new(record, db) {
            internal_includes.insert(CcInclude::SupportLibHeader(
                crubit_support_path_format.into(),
                if bridge_type.is_void_converters_bridge_type() {
                    "internal/lazy_init.h".into()
                } else {
                    "bridge.h".into()
                },
            ));
        }
    }

    for type_alias in ir.type_aliases() {
        let Ok(rs_type_kind) = db.rs_type_kind((&**type_alias).into()) else {
            continue;
        };

        if let RsTypeKind::BridgeType { bridge_type, .. } = rs_type_kind.unalias() {
            internal_includes.insert(CcInclude::SupportLibHeader(
                crubit_support_path_format.into(),
                if bridge_type.is_void_converters_bridge_type() {
                    "internal/lazy_init.h".into()
                } else {
                    "bridge.h".into()
                },
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

    quote! {
        #internal_includes
        __NEWLINE__
        __COMMENT__ "Public headers of the C++ library being wrapped."
        #( #ir_includes )* __NEWLINE__
    }
}

/// Implementation of `BindingsGenerator::crubit_abi_type`.
fn crubit_abi_type(db: &dyn BindingsGenerator, rs_type_kind: RsTypeKind) -> Result<CrubitAbiType> {
    match rs_type_kind {
        RsTypeKind::TypeAlias { underlying_type, .. } => {
            // We don't actually _have_ to expand the type alias here
            db.crubit_abi_type(underlying_type.as_ref().clone())
        }
        RsTypeKind::Slice(_) => bail!("RsTypeKind::Slice is not supported yet"),
        RsTypeKind::Enum { .. } => bail!("RsTypeKind::Enum is not supported yet"),
        RsTypeKind::TypeMapOverride { .. } => {
            bail!("RsTypeKind::TypeMapOverride is not supported yet")
        }
        RsTypeKind::Primitive(primitive) => {
            let inner = match primitive {
                Primitive::Bool => CrubitAbiType::new("bool", "bool"),
                Primitive::Void => bail!("values of type `void` cannot be bridged by value"),
                Primitive::Float => CrubitAbiType::new("f32", "float"),
                Primitive::Double => CrubitAbiType::new("f64", "double"),
                Primitive::Char => CrubitAbiType::new("::core::ffi::c_char", "char"),
                Primitive::SignedChar => CrubitAbiType::SignedChar,
                Primitive::UnsignedChar => CrubitAbiType::UnsignedChar,
                Primitive::Short => CrubitAbiType::new("::core::ffi::c_short", "short"),
                Primitive::Int => CrubitAbiType::new("::core::ffi::c_int", "int"),
                Primitive::Long => CrubitAbiType::new("::core::ffi::c_long", "long"),
                Primitive::LongLong => CrubitAbiType::LongLong,
                Primitive::UnsignedShort => CrubitAbiType::UnsignedShort,
                Primitive::UnsignedInt => CrubitAbiType::UnsignedInt,
                Primitive::UnsignedLong => CrubitAbiType::UnsignedLong,
                Primitive::UnsignedLongLong => CrubitAbiType::UnsignedLongLong,
                Primitive::Char16T => CrubitAbiType::new("u16", "char16_t"),
                Primitive::Char32T => CrubitAbiType::new("u32", "char32_t"),
                Primitive::PtrdiffT => CrubitAbiType::new("isize", "ptrdiff_t"),
                Primitive::IntptrT => CrubitAbiType::new("isize", "intptr_t"),
                Primitive::StdPtrdiffT => CrubitAbiType::new("isize", "std::ptrdiff_t"),
                Primitive::StdIntptrT => CrubitAbiType::new("isize", "std::intptr_t"),
                Primitive::SizeT => CrubitAbiType::new("usize", "size_t"),
                Primitive::UintptrT => CrubitAbiType::new("usize", "uintptr_t"),
                Primitive::StdSizeT => CrubitAbiType::new("usize", "std::size_t"),
                Primitive::StdUintptrT => CrubitAbiType::new("usize", "std::uintptr_t"),
                Primitive::Int8T => CrubitAbiType::new("i8", "int8_t"),
                Primitive::Int16T => CrubitAbiType::new("i16", "int16_t"),
                Primitive::Int32T => CrubitAbiType::new("i32", "int32_t"),
                Primitive::Int64T => CrubitAbiType::new("i64", "int64_t"),
                Primitive::StdInt8T => CrubitAbiType::new("i8", "std::int8_t"),
                Primitive::StdInt16T => CrubitAbiType::new("i16", "std::int16_t"),
                Primitive::StdInt32T => CrubitAbiType::new("i32", "std::int32_t"),
                Primitive::StdInt64T => CrubitAbiType::new("i64", "std::int64_t"),
                Primitive::Uint8T => CrubitAbiType::new("u8", "uint8_t"),
                Primitive::Uint16T => CrubitAbiType::new("u16", "uint16_t"),
                Primitive::Uint32T => CrubitAbiType::new("u32", "uint32_t"),
                Primitive::Uint64T => CrubitAbiType::new("u64", "uint64_t"),
                Primitive::StdUint8T => CrubitAbiType::new("u8", "std::uint8_t"),
                Primitive::StdUint16T => CrubitAbiType::new("u16", "std::uint16_t"),
                Primitive::StdUint32T => CrubitAbiType::new("u32", "std::uint32_t"),
                Primitive::StdUint64T => CrubitAbiType::new("u64", "std::uint64_t"),
            };

            Ok(CrubitAbiType::Type {
                rust_abi_path: FullyQualifiedPath::new("::bridge_rust::TransmuteAbi"),
                cpp_abi_path: FullyQualifiedPath::new("::crubit::TransmuteAbi"),
                type_args: Rc::from([inner]),
            })
        }
        RsTypeKind::BridgeType { bridge_type, original_type } => match bridge_type {
            BridgeRsTypeKind::BridgeVoidConverters { .. } => {
                bail!("Void pointer bridge types are not allowed within composable bridging")
            }
            BridgeRsTypeKind::Bridge { abi_rust, abi_cpp, generic_types, .. } => {
                let target =
                    original_type.defining_target.as_ref().unwrap_or(&original_type.owning_target);
                let rust_abi_path = make_rust_abi_path(&abi_rust, db.ir(), target);

                let cpp_abi_path = make_cpp_abi_path(&abi_cpp)?;

                let type_args = generic_types
                    .iter()
                    .map(|t: &RsTypeKind| db.crubit_abi_type(t.clone()))
                    .collect::<Result<Rc<[CrubitAbiType]>>>()?;

                Ok(CrubitAbiType::Type { rust_abi_path, cpp_abi_path, type_args })
            }
            BridgeRsTypeKind::StdOptional(inner) => {
                let inner_abi = db.crubit_abi_type(inner.as_ref().clone())?;
                Ok(CrubitAbiType::Type {
                    rust_abi_path: FullyQualifiedPath::new("::bridge_rust::OptionAbi"),
                    cpp_abi_path: FullyQualifiedPath::new("::crubit::OptionAbi"),
                    type_args: Rc::from([inner_abi]),
                })
            }
            BridgeRsTypeKind::StdPair(first, second) => {
                let first_abi = db.crubit_abi_type(first.as_ref().clone())?;
                let second_abi = db.crubit_abi_type(second.as_ref().clone())?;
                Ok(CrubitAbiType::Pair(Rc::from(first_abi), Rc::from(second_abi)))
            }
            BridgeRsTypeKind::StdString { in_cc_std } => Ok(CrubitAbiType::StdString { in_cc_std }),
        },
        RsTypeKind::Record { record, crate_path, .. } => {
            database::rs_snippet::check_by_value(record.as_ref())?;

            let rust_abi_path = crate_path
                .to_fully_qualified_path(make_rs_ident(record.rs_name.identifier.as_ref()));

            // This inlines the logic of code_gen_utils::expect_format_cc_ident, except it creates
            // an Ident instead of a TokenStream.
            code_gen_utils::check_valid_cc_name(record.cc_name.identifier.as_ref())
                .expect("IR should only contain valid C++ types");
            let ident =
                Ident::new(record.cc_name.identifier.as_ref(), proc_macro2::Span::call_site());
            let cpp_abi_path =
                FullyQualifiedPath { start_with_colon2: true, parts: Rc::from([ident]) };

            Ok(CrubitAbiType::Type {
                rust_abi_path: FullyQualifiedPath::new("::bridge_rust::TransmuteAbi"),
                cpp_abi_path: FullyQualifiedPath::new("::crubit::TransmuteAbi"),
                type_args: Rc::from([CrubitAbiType::Type {
                    rust_abi_path,
                    cpp_abi_path,
                    type_args: Rc::default(),
                }]),
            })
        }
        _ => bail!("Unsupported RsTypeKind: {}", rs_type_kind.display(db)),
    }
}

/// Parses the given Rust path into a [`FullyQualifiedPath`]. If the path doesn't start with "::",
/// it will be prepended with the crate name, or the keyword "crate" if the type is owned by the
/// current target.
fn make_rust_abi_path(mut rust_path: &str, ir: &IR, target: &BazelLabel) -> FullyQualifiedPath {
    let start_with_colon2 = strip_leading_colon2(&mut rust_path);
    let prefix = if start_with_colon2 {
        None
    } else if ir.is_current_target(target) {
        Some(Ident::new("crate", proc_macro2::Span::call_site()))
    } else {
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
fn make_cpp_abi_path(mut cpp_path: &str) -> Result<FullyQualifiedPath> {
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
