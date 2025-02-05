// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

pub use generate_function;
pub use generate_function_thunk;
pub mod generate_struct_and_union;

use arc_anyhow::{Context, Error, Result};
use code_gen_utils::{format_cc_includes, make_rs_ident, CcInclude};
use database::code_snippet::{
    required_crubit_features, ApiSnippets, BindingsTokens, RequiredCrubitFeature,
};
use database::code_snippet::{Bindings, FfiBindings};
use database::db::FatalErrors;
use database::db::{BindingsGenerator, Database, ReportFatalError};
use database::rs_snippet::{CratePath, Lifetime, Mutability, PrimitiveType, RsTypeKind};
use error_report::ErrorReport;
use error_report::{anyhow, bail, ensure, ErrorReporting};
use ffi_types::*;
use generate_comment::generate_top_level_comment;
use generate_comment::{generate_comment, generate_doc_comment, generate_unsupported};
use generate_struct_and_union::generate_incomplete_record;
use ir::*;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};
use std::panic::catch_unwind;
use std::path::Path;
use std::process;
use std::rc::Rc;
use token_stream_printer::{
    cc_tokens_to_formatted_string, rs_tokens_to_formatted_string, RustfmtConfig,
};

/// Deserializes IR from `json` and generates bindings source code.
///
/// This function panics on error.
///
/// # Safety
///
/// Expectations:
///    * `json` should be a FfiU8Slice for a valid array of bytes with the given
///      size.
///    * `crubit_support_path_format` should be a FfiU8Slice for a valid array
///      of bytes representing an UTF8-encoded string
///    * `rustfmt_exe_path` and `rustfmt_config_path` should both be a
///      FfiU8Slice for a valid array of bytes representing an UTF8-encoded
///      string (without the UTF-8 requirement, it seems that Rust doesn't offer
///      a way to convert to OsString on Windows)
///    * `json`, `crubit_support_path_format`, `rustfmt_exe_path`, and
///      `rustfmt_config_path` shouldn't change during the call.
///
/// Ownership:
///    * function doesn't take ownership of (in other words it borrows) the
///      input params: `json`, `crubit_support_path_format`, `rustfmt_exe_path`,
///      and `rustfmt_config_path`
///    * function passes ownership of the returned value to the caller
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateBindingsImpl(
    json: FfiU8Slice,
    crubit_support_path_format: FfiU8Slice,
    clang_format_exe_path: FfiU8Slice,
    rustfmt_exe_path: FfiU8Slice,
    rustfmt_config_path: FfiU8Slice,
    generate_error_report: bool,
    generate_source_loc_doc_comment: SourceLocationDocComment,
) -> FfiBindings {
    let json: &[u8] = json.as_slice();
    let crubit_support_path_format: &str =
        std::str::from_utf8(crubit_support_path_format.as_slice()).unwrap();
    let clang_format_exe_path: OsString =
        std::str::from_utf8(clang_format_exe_path.as_slice()).unwrap().into();
    let rustfmt_exe_path: OsString =
        std::str::from_utf8(rustfmt_exe_path.as_slice()).unwrap().into();
    let rustfmt_config_path: OsString =
        std::str::from_utf8(rustfmt_config_path.as_slice()).unwrap().into();
    catch_unwind(|| {
        let error_report: Option<ErrorReport>;
        let errors: &dyn ErrorReporting = if generate_error_report {
            error_report = Some(ErrorReport::new());
            error_report.as_ref().unwrap()
        } else {
            error_report = None;
            &error_report::IgnoreErrors
        };
        let fatal_errors = FatalErrors::new();
        let Bindings { rs_api, rs_api_impl } = generate_bindings(
            json,
            crubit_support_path_format,
            &clang_format_exe_path,
            &rustfmt_exe_path,
            &rustfmt_config_path,
            errors,
            &fatal_errors,
            generate_source_loc_doc_comment,
        )
        .unwrap();
        FfiBindings {
            rs_api: FfiU8SliceBox::from_boxed_slice(rs_api.into_bytes().into_boxed_slice()),
            rs_api_impl: FfiU8SliceBox::from_boxed_slice(
                rs_api_impl.into_bytes().into_boxed_slice(),
            ),
            error_report: FfiU8SliceBox::from_boxed_slice(
                error_report
                    .map(|s| s.to_json_string().into_bytes().into_boxed_slice())
                    .unwrap_or_else(|| Box::new([])),
            ),
            fatal_errors: FfiU8SliceBox::from_boxed_slice(
                fatal_errors.take_string().into_bytes().into_boxed_slice(),
            ),
        }
    })
    .unwrap_or_else(|_| process::abort())
}

fn generate_bindings(
    json: &[u8],
    crubit_support_path_format: &str,
    clang_format_exe_path: &OsStr,
    rustfmt_exe_path: &OsStr,
    rustfmt_config_path: &OsStr,
    errors: &dyn ErrorReporting,
    fatal_errors: &dyn ReportFatalError,
    generate_source_loc_doc_comment: SourceLocationDocComment,
) -> Result<Bindings> {
    let ir = deserialize_ir(json)?;

    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(
        &ir,
        crubit_support_path_format,
        errors,
        fatal_errors,
        generate_source_loc_doc_comment,
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

    let top_level_comment = generate_top_level_comment(&ir);
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
    let ident = make_rs_ident(&type_alias.identifier.identifier);
    let doc_comment = generate_doc_comment(
        type_alias.doc_comment.as_deref(),
        Some(&type_alias.source_loc),
        db.generate_source_loc_doc_comment(),
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

fn generate_namespace(db: &dyn BindingsGenerator, namespace: &Namespace) -> Result<ApiSnippets> {
    let ir = db.ir();
    let mut items = vec![];
    let mut thunks = vec![];
    let mut cc_details = vec![];
    let mut assertions = vec![];
    let mut features = BTreeSet::new();

    for item_id in namespace.child_item_ids.iter() {
        let item = ir.find_decl(*item_id).with_context(|| {
            format!("Failed to look up namespace.child_item_ids for {:?}", namespace)
        })?;
        let generated = generate_item(db, item)?;
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
        make_rs_ident(&namespace.name.identifier)
    } else {
        make_rs_ident(&format!("{}_{}", &namespace.name.identifier, reopened_namespace_idx))
    };

    let use_stmt_for_previous_namespace = if reopened_namespace_idx == 0 {
        quote! {}
    } else {
        let previous_namespace_ident = make_rs_ident(&format!(
            "{}_{}",
            &namespace.name.identifier,
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
fn generate_item(db: &dyn BindingsGenerator, item: &Item) -> Result<ApiSnippets> {
    let err = match generate_item_impl(db, item) {
        Ok(generated) => return Ok(generated),
        Err(err) => err,
    };

    // We didn't guarantee that bindings would exist, so it is not invalid to
    // write down the error but continue.
    let unsupported_item = match item {
        Item::Enum(enum_) => {
            // For now, we special case on enums because they previously reported their own errors from generate_enum and it has more information than the general case.
            let unsupported_item_path = UnsupportedItemPath {
                ident: UnqualifiedIdentifier::Identifier(enum_.identifier.clone()),
                enclosing_item_id: enum_.enclosing_item_id,
            };
            UnsupportedItem::new_with_cause(db.ir(), enum_, Some(unsupported_item_path), err)
        }
        _ => {
            if has_bindings(db, item) == HasBindings::Yes {
                return Err(err);
            }
            // FIXME(cramertj): get paths here in more cases. It may be that
            // `generate_item_impl` failed in such a way that the path is still available.
            UnsupportedItem::new_with_cause(db.ir(), item, /* path= */ None, err)
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

#[derive(Clone, PartialEq, Eq)]
enum HasBindings {
    /// This item is guaranteed to have bindings. If the translation unit
    /// defining the item fails to generate bindings for it, it will not
    /// compile.
    Yes,

    /// This item is not guaranteed to have bindings. There is no way to tell if
    /// bindings were generated unless the item is defined in the current
    /// translation unit.
    Maybe,

    /// These bindings are guaranteed not to exist.
    No(NoBindingsReason),
}

#[derive(Clone, PartialEq, Eq)]
enum NoBindingsReason {
    MissingRequiredFeatures {
        context: Rc<str>,
        missing_features: Vec<RequiredCrubitFeature>,
    },
    DependencyFailed {
        context: Rc<str>,
        error: Error,
    },
    /// This is directly unsupported.
    Unsupported {
        context: Rc<str>,
        error: Error,
    },
}

#[must_use]
fn has_bindings(db: &dyn BindingsGenerator, item: &Item) -> HasBindings {
    let ir = db.ir();

    match required_crubit_features(db, item) {
        Ok(missing_features) if missing_features.is_empty() => {}
        Ok(missing_features) => {
            return HasBindings::No(NoBindingsReason::MissingRequiredFeatures {
                context: item.debug_name(&db.ir()),
                missing_features,
            });
        }
        Err(error) => {
            return HasBindings::No(NoBindingsReason::DependencyFailed {
                context: item.debug_name(&db.ir()),
                error,
            });
        }
    }

    if let Some(parent) = item.enclosing_item_id() {
        let parent = ir.find_untyped_decl(parent);

        match has_bindings(db, parent) {
            HasBindings::No(no_parent_bindings) => {
                return HasBindings::No(NoBindingsReason::DependencyFailed {
                    context: item.debug_name(&ir),
                    error: no_parent_bindings.into(),
                });
            }
            HasBindings::Maybe => {
                // This shouldn't happen, Maybe is meant for Func items.
                return HasBindings::No(NoBindingsReason::DependencyFailed {
                    context: item.debug_name(&ir),
                    error: anyhow!("parent item might not be defined"),
                });
            }
            HasBindings::Yes => {}
        }

        // TODO(b/200067824): Allow nested type items inside records.
        if item.is_type_definition() {
            if let ir::Item::Record(_) = parent {
                return HasBindings::No(NoBindingsReason::Unsupported {
                    context: item.debug_name(&ir),
                    error: anyhow!(
                        "b/200067824: type definitions nested inside records are not yet supported"
                    ),
                });
            }
        }
    }

    match item {
        // Function bindings aren't guaranteed, because they don't _need_ to be guaranteed. We
        // choose not to generate code which relies on functions existing in other TUs.
        Item::Func(..) => HasBindings::Maybe,
        Item::TypeAlias(alias) => match db.rs_type_kind(alias.underlying_type.rs_type.clone()) {
            Ok(_) => HasBindings::Yes,
            Err(error) => HasBindings::No(NoBindingsReason::DependencyFailed {
                context: alias.debug_name(&ir),
                error,
            }),
        },
        Item::Enum(enum_) => match db.generate_enum(Rc::clone(enum_)) {
            Ok(_) => HasBindings::Yes,
            Err(error) => HasBindings::No(NoBindingsReason::DependencyFailed {
                context: enum_.debug_name(ir),
                error,
            }),
        },
        // TODO(b/392882224): Records might not generated if an error occurs in generation.
        _ => HasBindings::Yes,
    }
}

impl From<NoBindingsReason> for Error {
    fn from(reason: NoBindingsReason) -> Error {
        match reason {
            NoBindingsReason::MissingRequiredFeatures { context, missing_features } => {
                // This maybe could use .context(), but the ordering is backward.
                let mut all_missing = vec![];
                for missing in missing_features {
                    all_missing.push(missing.to_string());
                }
                anyhow!(
                    "Can't generate bindings for {context}, because of missing required features (<internal link>):\n{}",
                    all_missing.join("\n")
                )
            }
            NoBindingsReason::DependencyFailed { context, error } => error.context(format!(
                "Can't generate bindings for {context} due to missing bindings for its dependency"
            )),
            NoBindingsReason::Unsupported { context, error } => error.context(format!(
                "Can't generate bindings for {context}, because it is unsupported"
            )),
        }
    }
}

/// Creats a new database. Public for testing.
pub fn new_database<'db>(
    ir: &'db IR,
    errors: &'db dyn ErrorReporting,
    fatal_errors: &'db dyn ReportFatalError,
    generate_source_loc_doc_comment: SourceLocationDocComment,
) -> Database<'db> {
    Database::new(
        ir,
        errors,
        fatal_errors,
        generate_source_loc_doc_comment,
        is_rs_type_kind_unsafe,
        generate_enum::generate_enum,
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
    generate_source_loc_doc_comment: SourceLocationDocComment,
) -> Result<BindingsTokens> {
    let db = new_database(ir, errors, fatal_errors, generate_source_loc_doc_comment);
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
        let item =
            ir.find_decl(*top_level_item_id).context("Failed to look up ir.top_level_item_ids")?;
        let generated = generate_item(&db, item)?;
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

    // Allows the use of #[__crubit::foo] attributes to control the behavior of
    // cc_bindings_from_rs on the generated code.
    //
    // Note that we use `__crubit`, not `crubit`. This way, namespaces and types can
    // be named `crubit` without causing obscure internal failures during
    // bindings generation. In particular, well, crubit itself does use
    // `namespace crubit`...
    //
    // Note also that there is only one tool namespace we use, __crubit. So we can
    // use the existence of a register_tool feature requirement to signal
    // whether or not we need to bother registering __crubit, and make the
    // bindings more compact for headers that don't define any types.
    let register_crubit_tool = if features.contains(&make_rs_ident("register_tool")) {
        quote! {#![register_tool(__crubit)] __NEWLINE__}
    } else {
        quote! {}
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
            #register_crubit_tool

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

fn rs_type_kind(db: &dyn BindingsGenerator, ty: ir::RsType) -> Result<RsTypeKind> {
    match &ty {
        ir::RsType::UnknownAttr { unknown_attr } => {
            // In most places, we only bail for unknown attributes in supported. However,
            // it's difficult and expensive to generate an RsTypeKind differently
            // depending on the translation unit for the item that contains it.
            // Rather than trying to keep going in experimental, we bail
            // unconditionally.
            //
            // The correct fix for this error is to add support for the attributes which are
            // not yet understood, but need to be used in practice.
            bail!("unknown attribute(s): {unknown_attr}")
        }
        ir::RsType::ItemIdType { decl_id } => {
            let ir = db.ir();
            let item = ir.find_untyped_decl(*decl_id);
            let fallback_type = match item {
                // Type aliases are unique among items, in that if the item defining the alias fails
                // to receive bindings, we can still use the aliased type.
                ir::Item::TypeAlias(alias) => Some(&alias.underlying_type.rs_type),
                _ => None,
            };
            match (has_bindings(db, item), fallback_type) {
                (HasBindings::Yes, _) => {}
                // Additionally, we should not "see through" type aliases that are specifically not
                // on targets that intend to support Rust users of those type aliases.
                // (If we did, then a C++ library owner could break Rust callers, which is a
                // maintenance responsibility that they did not sign up for!)
                (has_bindings, Some(fallback_type))
                    if !matches!(
                        has_bindings,
                        HasBindings::No(NoBindingsReason::MissingRequiredFeatures { .. })
                    ) =>
                {
                    return db.rs_type_kind(fallback_type.clone());
                }
                (HasBindings::Maybe, _) => {
                    bail!(
                        "Type {} may or may not exist, and cannot be used.",
                        item.debug_name(&ir)
                    );
                }
                (HasBindings::No(reason), _) => {
                    return Err(reason.into());
                }
            }
            match item {
                Item::IncompleteRecord(incomplete_record) => Ok(RsTypeKind::IncompleteRecord {
                    incomplete_record: incomplete_record.clone(),
                    crate_path: Rc::new(CratePath::new(
                        &ir,
                        ir.namespace_qualifier(incomplete_record),
                        rs_imported_crate_name(&incomplete_record.owning_target, &ir),
                    )),
                }),
                Item::Record(record) => RsTypeKind::new_record(db, record.clone(), ir),
                Item::Enum(enum_) => RsTypeKind::new_enum(enum_.clone(), &ir),
                Item::TypeAlias(type_alias) => RsTypeKind::new_type_alias(db, type_alias.clone()),
                Item::TypeMapOverride(type_map_override) => {
                    RsTypeKind::new_type_map_override(db, type_map_override)
                }
                other_item => bail!("Item does not define a type: {other_item:?}"),
            }
        }
        ir::RsType::NamedType { name, lifetime_args, type_args } => {
            let ir = db.ir();
            // The lambdas deduplicate code needed by multiple `match` branches.
            let get_type_args = || -> Result<Vec<RsTypeKind>> {
                type_args.iter().map(|type_arg| db.rs_type_kind(type_arg.clone())).collect()
            };
            let get_pointee = || -> Result<Rc<RsTypeKind>> {
                if type_args.len() != 1 {
                    bail!("Missing pointee/referent type (need exactly 1 type argument): {:?}", ty);
                }
                // TODO(b/351976044): Support bridge types by pointer/reference.
                let pointee = get_type_args()?.pop().unwrap();
                if pointee.is_bridge_type() {
                    bail!("Bridging types are not supported as pointee/referent types.");
                }
                Ok(Rc::new(pointee))
            };
            let get_lifetime = || -> Result<Lifetime> {
                if lifetime_args.len() != 1 {
                    bail!(
                        "Missing reference lifetime (need exactly 1 lifetime argument): {:?}",
                        ty
                    );
                }
                let lifetime_id = lifetime_args[0];
                ir.get_lifetime(lifetime_id)
                    .ok_or_else(|| anyhow!("no known lifetime with id {lifetime_id:?}"))
                    .map(Lifetime::from)
            };

            let result = match name.as_ref() {
                "*mut" => {
                    RsTypeKind::Pointer { pointee: get_pointee()?, mutability: Mutability::Mut }
                }
                "*const" => {
                    RsTypeKind::Pointer { pointee: get_pointee()?, mutability: Mutability::Const }
                }
                "&mut" => RsTypeKind::Reference {
                    referent: get_pointee()?,
                    mutability: Mutability::Mut,
                    lifetime: get_lifetime()?,
                },
                "&" => RsTypeKind::Reference {
                    referent: get_pointee()?,
                    mutability: Mutability::Const,
                    lifetime: get_lifetime()?,
                },
                "#RvalueReference mut" => RsTypeKind::RvalueReference {
                    referent: get_pointee()?,
                    mutability: Mutability::Mut,
                    lifetime: get_lifetime()?,
                },
                "#RvalueReference const" => RsTypeKind::RvalueReference {
                    referent: get_pointee()?,
                    mutability: Mutability::Const,
                    lifetime: get_lifetime()?,
                },
                "Option" => {
                    let mut type_args = get_type_args()?;
                    ensure!(
                        type_args.len() == 1,
                        "Option should have exactly 1 type argument (got {})",
                        type_args.len()
                    );
                    RsTypeKind::Option(Rc::new(type_args.remove(0)))
                }
                name => {
                    let mut type_args = get_type_args()?;

                    if let Some(primitive) = PrimitiveType::from_str(name) {
                        if !type_args.is_empty() {
                            bail!("{name} type must not have type arguments: {:?}", ty);
                        }
                        RsTypeKind::Primitive(primitive)
                    } else if let Some(abi) = name.strip_prefix("#funcPtr ") {
                        // Assert that function pointers in the IR either have static lifetime or
                        // no lifetime.
                        if let Ok(lifetime) = get_lifetime() {
                            assert_eq!(lifetime.0.as_ref(), "static");
                        }

                        assert!(
                            !type_args.is_empty(),
                            "In well-formed IR function pointers include at least the return type",
                        );
                        ensure!(
                            type_args.iter().all(|t| t.is_c_abi_compatible_by_value()),
                            "Either the return type or some of the parameter types require \
                            an FFI thunk (and function pointers don't have a thunk)",
                        );
                        RsTypeKind::FuncPtr {
                            abi: abi.into(),
                            return_type: Rc::new(type_args.remove(type_args.len() - 1)),
                            param_types: Rc::from(type_args),
                        }
                    } else {
                        bail!("Unknown type: {name}")
                    }
                }
            };
            Ok(result)
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
