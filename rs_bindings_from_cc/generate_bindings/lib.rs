// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

mod code_snippet;
mod db;
mod generate_comment;
mod generate_enum;
mod generate_function;
mod generate_function_thunk;
mod generate_struct_and_union;
mod rs_snippet;

use code_snippet::{
    required_crubit_features, Bindings, BindingsTokens, FfiBindings, GeneratedItem,
    RequiredCrubitFeature,
};
use db::{BindingsGenerator, Database};
use generate_comment::{
    generate_comment, generate_doc_comment, generate_top_level_comment, generate_unsupported,
};
use generate_enum::generate_enum;
use generate_struct_and_union::{generate_incomplete_record, generate_record};

use crate::rs_snippet::{CratePath, Lifetime, Mutability, PrimitiveType, RsTypeKind};
use arc_anyhow::{Context, Error, Result};
use code_gen_utils::{format_cc_includes, make_rs_ident, CcInclude};
use error_report::{anyhow, bail, ensure, ErrorReport, ErrorReporting, IgnoreErrors};
use ffi_types::*;
use ir::*;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
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
        // It is ok to abort here.
        let errors: Rc<dyn ErrorReporting> =
            if generate_error_report { Rc::new(ErrorReport::new()) } else { Rc::new(IgnoreErrors) };
        let Bindings { rs_api, rs_api_impl } = generate_bindings(
            json,
            crubit_support_path_format,
            &clang_format_exe_path,
            &rustfmt_exe_path,
            &rustfmt_config_path,
            errors.clone(),
            generate_source_loc_doc_comment,
        )
        .unwrap();
        FfiBindings {
            rs_api: FfiU8SliceBox::from_boxed_slice(rs_api.into_bytes().into_boxed_slice()),
            rs_api_impl: FfiU8SliceBox::from_boxed_slice(
                rs_api_impl.into_bytes().into_boxed_slice(),
            ),
            error_report: FfiU8SliceBox::from_boxed_slice(
                errors.serialize_to_vec().unwrap().into_boxed_slice(),
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
    errors: Rc<dyn ErrorReporting>,
    generate_source_loc_doc_comment: SourceLocationDocComment,
) -> Result<Bindings> {
    let ir = Rc::new(deserialize_ir(json)?);

    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(
        ir.clone(),
        crubit_support_path_format,
        errors,
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

    let top_level_comment = generate_top_level_comment(ir.clone());
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

fn generate_type_alias(db: &Database, type_alias: &TypeAlias) -> Result<GeneratedItem> {
    let ident = make_rs_ident(&type_alias.identifier.identifier);
    let doc_comment = generate_doc_comment(
        type_alias.doc_comment.as_deref(),
        Some(&type_alias.source_loc),
        db.generate_source_loc_doc_comment(),
    );
    let underlying_type = db
        .rs_type_kind(type_alias.underlying_type.rs_type.clone())
        .with_context(|| format!("Failed to format underlying type for {}", type_alias))?;
    Ok(quote! {
        #doc_comment
        pub type #ident = #underlying_type;
    }
    .into())
}

fn generate_namespace(db: &Database, namespace: &Namespace) -> Result<GeneratedItem> {
    let ir = db.ir();
    let mut items = vec![];
    let mut thunks = vec![];
    let mut thunk_impls = vec![];
    let mut assertions = vec![];
    let mut features = BTreeSet::new();

    for item_id in namespace.child_item_ids.iter() {
        let item = ir.find_decl(*item_id).with_context(|| {
            format!("Failed to look up namespace.child_item_ids for {:?}", namespace)
        })?;
        let generated = generate_item(db, item)?;
        items.push(generated.item);
        if !generated.thunks.is_empty() {
            thunks.push(generated.thunks);
        }
        if !generated.thunk_impls.is_empty() {
            thunk_impls.push(generated.thunk_impls);
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

    Ok(GeneratedItem {
        item: namespace_tokens,
        features,
        thunks: quote! { #( #thunks )* },
        thunk_impls: quote! { #( #thunk_impls )* },
        assertions: quote! { #( #assertions )* },
        ..Default::default()
    })
}

/// Returns generated bindings for an item, or `Err` if bindings generation
/// failed in such a way as to make the generated bindings as a whole invalid.
fn generate_item(db: &Database, item: &Item) -> Result<GeneratedItem> {
    match generate_item_impl(db, item) {
        Ok(generated) => Ok(generated),
        Err(err) => {
            let ir = db.ir();
            if has_bindings(db, item) != HasBindings::Yes {
                // We didn't guarantee that bindings would exist, so it is not invalid to
                // write down the error but continue.
                return generate_unsupported(db, &UnsupportedItem::new_with_cause(&ir, item, err));
            }
            Err(err)
        }
    }
}

/// The implementation of generate_item, without the error recovery logic.
///
/// Returns Err if bindings could not be generated for this item.
fn generate_item_impl(db: &Database, item: &Item) -> Result<GeneratedItem> {
    let ir = db.ir();
    if let Some(owning_target) = item.owning_target() {
        if !ir.is_current_target(owning_target) {
            return Ok(GeneratedItem::default());
        }
    }
    let overloaded_funcs = db.overloaded_funcs();
    let generated_item = match item {
        Item::Func(func) => match db.generate_func(func.clone(), None)? {
            None => GeneratedItem::default(),
            Some((item, function_id)) => {
                if overloaded_funcs.contains(&function_id) {
                    bail!("Cannot generate bindings for overloaded function")
                } else {
                    (*item).clone()
                }
            }
        },
        Item::IncompleteRecord(incomplete_record) => {
            generate_incomplete_record(db, incomplete_record)?
        }
        Item::Record(record) => generate_record(db, record)?,
        Item::Enum(enum_) => generate_enum(db, enum_)?,
        Item::TypeAlias(type_alias) => generate_type_alias(db, type_alias)?,
        Item::UnsupportedItem(unsupported) => generate_unsupported(db, unsupported)?,
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
            );
            let assertions = if let Some(size_align) = &type_override.size_align {
                generate_struct_and_union::rs_size_align_assertions(rs_type, size_align)
            } else {
                quote! {}
            };

            GeneratedItem {
                item: quote! {
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

// Returns the Rust code implementing bindings, plus any auxiliary C++ code
// needed to support it.
fn generate_bindings_tokens(
    ir: Rc<IR>,
    crubit_support_path_format: &str,
    errors: Rc<dyn ErrorReporting>,
    generate_source_loc_doc_comment: SourceLocationDocComment,
) -> Result<BindingsTokens> {
    let db = Database::new(ir.clone(), errors, generate_source_loc_doc_comment);
    let mut items = vec![];
    let mut thunks = vec![];
    let mut thunk_impls = vec![
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

    for top_level_item_id in ir.top_level_item_ids() {
        let item =
            ir.find_decl(*top_level_item_id).context("Failed to look up ir.top_level_item_ids")?;
        let generated = generate_item(&db, item)?;
        items.push(generated.item);
        if !generated.thunks.is_empty() {
            thunks.push(generated.thunks);
        }
        if !generated.assertions.is_empty() {
            assertions.push(generated.assertions);
        }
        if !generated.thunk_impls.is_empty() {
            thunk_impls.push(generated.thunk_impls);
        }
        features.extend(generated.features);
    }

    thunk_impls.push(quote! {
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
        rs_api_impl: quote! {#(#thunk_impls  __NEWLINE__ __NEWLINE__ )*},
    })
}

/// Formats a C++ identifier.  Panics if `ident` is a C++ reserved keyword.
fn format_cc_ident(ident: &str) -> TokenStream {
    code_gen_utils::format_cc_ident(ident).expect("IR should only contain valid C++ identifiers")
}

fn rs_type_kind(db: &dyn BindingsGenerator, ty: ir::RsType) -> Result<RsTypeKind> {
    if let Some(unknown_attr) = &ty.unknown_attr {
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

    let ir = db.ir();
    // The lambdas deduplicate code needed by multiple `match` branches.
    let get_type_args = || -> Result<Vec<RsTypeKind>> {
        ty.type_args.iter().map(|type_arg| db.rs_type_kind(type_arg.clone())).collect()
    };
    let get_pointee = || -> Result<Rc<RsTypeKind>> {
        if ty.type_args.len() != 1 {
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
        if ty.lifetime_args.len() != 1 {
            bail!("Missing reference lifetime (need exactly 1 lifetime argument): {:?}", ty);
        }
        let lifetime_id = ty.lifetime_args[0];
        ir.get_lifetime(lifetime_id)
            .ok_or_else(|| anyhow!("no known lifetime with id {lifetime_id:?}"))
            .map(Lifetime::from)
    };

    let result = match ty.name.as_deref() {
        None => {
            ensure!(
                ty.type_args.is_empty(),
                "Type arguments on records nor type aliases are not yet supported: {:?}",
                ty
            );
            let item = ir.item_for_type(&ty)?;
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
                Item::IncompleteRecord(incomplete_record) => RsTypeKind::IncompleteRecord {
                    incomplete_record: incomplete_record.clone(),
                    crate_path: Rc::new(CratePath::new(
                        &ir,
                        ir.namespace_qualifier(incomplete_record)?,
                        rs_imported_crate_name(&incomplete_record.owning_target, &ir),
                    )),
                },
                Item::Record(record) => {
                    if record.bridge_type_info.is_some() {
                        RsTypeKind::new_bridge_type(record.clone())?
                    } else {
                        RsTypeKind::new_record(db, record.clone(), &ir)?
                    }
                }
                Item::Enum(enum_) => RsTypeKind::new_enum(enum_.clone(), &ir)?,
                Item::TypeAlias(type_alias) => RsTypeKind::new_type_alias(db, type_alias.clone())?,
                Item::TypeMapOverride(type_map_override) => {
                    RsTypeKind::new_type_map_override(db, type_map_override)?
                }
                other_item => bail!("Item does not define a type: {other_item:?}"),
            }
        }
        Some(name) => match name {
            "*mut" => RsTypeKind::Pointer { pointee: get_pointee()?, mutability: Mutability::Mut },
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
        },
    };
    Ok(result)
}

fn cpp_type_name_for_record(record: &Record, ir: &IR) -> Result<TokenStream> {
    let tagless = cc_tagless_type_name_for_record(record, ir)?;
    let tag_kind = cc_tag_kind(record);
    Ok(quote! { #tag_kind #tagless })
}

fn cc_tagless_type_name_for_record(record: &Record, ir: &IR) -> Result<TokenStream> {
    let ident = crate::format_cc_ident(record.cc_name.as_ref());
    let namespace_qualifier = ir.namespace_qualifier(record)?.format_for_cc()?;
    Ok(quote! { #namespace_qualifier #ident })
}

fn cpp_type_name_for_item(item: &ir::Item, ir: &IR) -> Result<TokenStream> {
    match item {
        Item::IncompleteRecord(incomplete_record) => {
            let ident = crate::format_cc_ident(incomplete_record.cc_name.as_ref());
            let namespace_qualifier = ir.namespace_qualifier(incomplete_record)?.format_for_cc()?;
            let tag_kind = incomplete_record.record_type;
            Ok(quote! { #tag_kind #namespace_qualifier #ident })
        }
        Item::Record(record) => cpp_type_name_for_record(record, ir),
        Item::Enum(enum_) => {
            let ident = crate::format_cc_ident(&enum_.identifier.identifier);
            let qualifier = cc_qualified_path_prefix(item, ir)?;
            Ok(quote! { #qualifier #ident })
        }
        Item::TypeAlias(type_alias) => {
            let ident = crate::format_cc_ident(&type_alias.identifier.identifier);
            let qualifier = cc_qualified_path_prefix(item, ir)?;
            Ok(quote! { #qualifier #ident })
        }
        Item::TypeMapOverride(type_map_override) => type_map_override
            .cc_name
            .parse::<TokenStream>()
            .map_err(|_| anyhow!("malformed type name: {:?}", type_map_override.cc_name)),
        _ => bail!("Item does not define a type: {:?}", item),
    }
}

/// Returns the namespace / class qualifiers necessary to access the item.
///
/// For example, for `namespace x { struct Y { using X = int; }; }`, the prefix
/// for `X` is `x::Y::`.
fn cc_qualified_path_prefix(item: &ir::Item, ir: &ir::IR) -> Result<TokenStream> {
    let Some(parent) = item.enclosing_item_id() else {
        return Ok(quote! {});
    };
    let parent: &ir::Item = ir.find_decl(parent)?;
    match parent {
        ir::Item::Namespace(_) => Ok(ir.namespace_qualifier(item)?.format_for_cc()?),
        ir::Item::Record(r) => {
            let name = cc_tagless_type_name_for_record(r, ir)?;
            Ok(quote! {#name ::})
        }
        _ => bail!("Unexpected enclosing item: {item:?}"),
    }
}

fn cc_tag_kind(record: &ir::Record) -> TokenStream {
    if record.is_anon_record_with_typedef {
        quote! {}
    } else {
        record.record_type.into_token_stream()
    }
}

// Maps a Rust ABI [1] into a Clang attribute. See also
// `ConvertCcCallConvIntoRsApi` in importer.cc.
// [1]
// https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier
fn format_cc_call_conv_as_clang_attribute(rs_abi: &str) -> Result<TokenStream> {
    match rs_abi {
        "cdecl" => Ok(quote! {}),
        "fastcall" => Ok(quote! { __attribute__((fastcall)) }),
        "stdcall" => Ok(quote! { __attribute__((stdcall)) }),
        "thiscall" => Ok(quote! { __attribute__((thiscall)) }),
        "vectorcall" => Ok(quote! { __attribute__((vectorcall)) }),
        _ => bail!("Unsupported ABI: {}", rs_abi),
    }
}

pub(crate) fn format_cpp_type(ty: &ir::CcType, ir: &IR) -> Result<TokenStream> {
    // Formatting *both* pointers *and* references as pointers, because:
    // - Pointers and references have the same representation in the ABI.
    // - Clang's `-Wreturn-type-c-linkage` warns when using references in C++
    //   function thunks declared as `extern "C"` (see b/238681766).
    format_cpp_type_inner(ty, ir, /* references_ok= */ false)
}

fn format_cpp_type_inner(ty: &ir::CcType, ir: &IR, references_ok: bool) -> Result<TokenStream> {
    let const_fragment = if ty.is_const {
        quote! {const}
    } else {
        quote! {}
    };
    if let Some(ref name) = ty.name {
        match name.as_ref() {
            mut name @ ("*" | "&" | "&&") => {
                if ty.type_args.len() != 1 {
                    bail!("Invalid pointer type (need exactly 1 type argument): {:?}", ty);
                }
                let nested_type = format_cpp_type_inner(&ty.type_args[0], ir, references_ok)?;
                if !references_ok {
                    name = "*";
                }
                let ptr = match name {
                    "*" => quote! {*},
                    "&" => quote! {&},
                    "&&" => quote! {&&},
                    _ => unreachable!(),
                };
                Ok(quote! {#nested_type #ptr #const_fragment})
            }
            cpp_type_name => match cpp_type_name.strip_prefix("#funcValue ") {
                None => {
                    if !ty.type_args.is_empty() {
                        bail!("Type not yet supported: {:?}", ty);
                    }
                    // Not using `code_gen_utils::format_cc_ident`, because
                    // `cpp_type_name` may be a C++ reserved keyword (e.g.
                    // `int`).
                    let cc_ident: TokenStream = cpp_type_name.parse().unwrap();
                    Ok(quote! { #cc_ident #const_fragment })
                }
                Some(abi) => match ty.type_args.split_last() {
                    None => bail!("funcValue type without a return type: {:?}", ty),
                    Some((ret_type, param_types)) => {
                        // Function pointer types don't ignore references, but luckily,
                        // `-Wreturn-type-c-linkage` does. So we can just re-enable references now
                        // so that the function type is exactly correct.
                        let ret_type =
                            format_cpp_type_inner(ret_type, ir, /* references_ok= */ true)?;
                        let param_types = param_types
                            .iter()
                            .map(|t| format_cpp_type_inner(t, ir, /* references_ok= */ true))
                            .collect::<Result<Vec<_>>>()?;
                        let attr = format_cc_call_conv_as_clang_attribute(abi)?;
                        // `type_identity_t` is used below to avoid having to
                        // emit spiral-like syntax where some syntax elements of
                        // an inner type (e.g. function type as below) can
                        // surround syntax elements of an outer type (e.g. a
                        // pointer type). Compare: `int (*foo)(int, int)` VS
                        // `type_identity_t<int(int, int)>* foo`.
                        Ok(quote! { crubit::type_identity_t<
                            #ret_type ( #( #param_types ),* ) #attr
                        >  })
                    }
                },
            },
        }
    } else {
        let item = ir.item_for_type(ty)?;
        let type_name = cpp_type_name_for_item(item, ir)?;
        Ok(quote! {#const_fragment #type_name})
    }
}

pub(crate) fn crate_root_path_tokens(ir: &IR) -> TokenStream {
    match ir.crate_root_path().as_deref().map(make_rs_ident) {
        None => quote! { crate },
        Some(crate_root_path) => quote! { crate :: #crate_root_path },
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use arc_anyhow::Result;
    use googletest::prelude::*;
    use ir_testing::{make_ir_from_items, retrieve_func, with_lifetime_macros};
    use static_assertions::{assert_impl_all, assert_not_impl_any};
    use token_stream_matchers::{
        assert_cc_matches, assert_cc_not_matches, assert_rs_matches, assert_rs_not_matches,
    };
    use token_stream_printer::rs_tokens_to_formatted_string_for_tests;

    pub fn ir_from_cc(header: &str) -> Result<IR> {
        ir_testing::ir_from_cc(multiplatform_testing::test_platform(), header)
    }
    pub fn ir_from_cc_dependency(header: &str, dep_header: &str) -> Result<IR> {
        ir_testing::ir_from_cc_dependency(
            multiplatform_testing::test_platform(),
            header,
            dep_header,
        )
    }
    pub fn ir_record(name: &str) -> Record {
        ir_testing::ir_record(multiplatform_testing::test_platform(), name)
    }

    pub fn generate_bindings_tokens(ir: IR) -> Result<BindingsTokens> {
        super::generate_bindings_tokens(
            Rc::new(ir),
            "crubit/rs_bindings_support",
            Rc::new(IgnoreErrors),
            SourceLocationDocComment::Enabled,
        )
    }

    pub fn db_from_cc(cc_src: &str) -> Result<Database> {
        Ok(Database::new(
            Rc::new(ir_from_cc(cc_src)?),
            Rc::new(ErrorReport::new()),
            SourceLocationDocComment::Enabled,
        ))
    }

    #[gtest]
    fn test_disable_thread_safety_warnings() -> Result<()> {
        let ir = ir_from_cc("inline void foo() {}")?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                ...
                __HASH_TOKEN__ pragma clang diagnostic push
                __HASH_TOKEN__ pragma clang diagnostic ignored "-Wthread-safety-analysis"
                ...

                __HASH_TOKEN__ pragma clang diagnostic pop
                ...
            }
        );
        Ok(())
    }

    // TODO(b/200067824): These should generate nested types.
    #[gtest]
    fn test_nested_type_definitions() -> Result<()> {
        for nested_type in ["enum NotPresent {};", "struct NotPresent {};", "struct NotPresent;"] {
            let ir = ir_from_cc(&format!(
                r#"
                    struct SomeStruct final {{
                        {nested_type}
                    }};
                    SomeStruct::NotPresent* AlsoNotPresent();
                "#
            ))?;
            let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
            assert_rs_not_matches!(rs_api, quote! { NotPresent });
            assert_rs_not_matches!(rs_api, quote! { AlsoNotPresent });
        }
        Ok(())
    }

    /// Unlike other nested type definitions, typedefs can use the aliased type
    /// instead.
    #[gtest]
    fn test_typedef_member() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            struct SomeStruct final {
              typedef int Type;
            };
            inline SomeStruct::Type Function() {return 0;}
        "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        // TODO(b/200067824): This should use the alias's real name in Rust, as well.
        assert_rs_matches!(rs_api, quote! { pub fn Function() -> ::core::ffi::c_int { ... } },);

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" SomeStruct::Type __rust_thunk___Z8Functionv(){ return Function(); }
            },
        );
        Ok(())
    }

    #[gtest]
    fn test_struct_from_other_target() -> Result<()> {
        let ir = ir_from_cc_dependency("// intentionally empty", "struct SomeStruct {};")?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_not_matches!(rs_api, quote! { SomeStruct });
        assert_cc_not_matches!(rs_api_impl, quote! { SomeStruct });
        Ok(())
    }

    #[gtest]
    fn test_func_ptr_where_params_are_primitive_types() -> Result<()> {
        let ir = ir_from_cc(r#" int (*get_ptr_to_func())(float, double); "#)?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ptr_to_func() -> Option<extern "C" fn (f32, f64) -> ::core::ffi::c_int> {
                    unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    unsafe extern "C" {
                        #[link_name = "_Z15get_ptr_to_funcv"]
                        pub(crate) unsafe fn __rust_thunk___Z15get_ptr_to_funcv()
                        -> Option<extern "C" fn(f32, f64) -> ::core::ffi::c_int>;
                    }
                }
            }
        );
        // Verify that no C++ thunk got generated.
        assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });

        // TODO(b/217419782): Add another test for more exotic calling conventions /
        // abis.

        // TODO(b/276461979): Add another test for pointer to a function that requires
        // thunks - e.g. because it takes/returns structs value. See also
        // b/276461979 and <internal link>

        Ok(())
    }

    #[gtest]
    fn test_func_ref() -> Result<()> {
        let ir = ir_from_cc(r#" int (&get_ref_to_func())(float, double); "#)?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ref_to_func() -> extern "C" fn (f32, f64) -> ::core::ffi::c_int {
                    unsafe { crate::detail::__rust_thunk___Z15get_ref_to_funcv() }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_func_ptr_with_non_static_lifetime() -> Result<()> {
        let ir = ir_from_cc(&with_lifetime_macros(
            r#"
            int (* $a get_ptr_to_func())(float, double); "#,
        ))?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_cc_matches!(rs_api, {
            let txt = "Generated from: google3/ir_from_cc_virtual_header.h;l=33\n\
                           Error while generating bindings for item 'get_ptr_to_func':\n\
                           Unable to get lifetime annotations: Type may not be annotated with lifetimes";
            quote! { __COMMENT__ #txt }
        });
        Ok(())
    }

    #[gtest]
    fn test_func_ptr_where_params_are_raw_ptrs() -> Result<()> {
        let ir = ir_from_cc(r#" const int* (*get_ptr_to_func())(const int*); "#)?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ptr_to_func() -> Option<unsafe extern "C" fn (*const ::core::ffi::c_int) -> *const ::core::ffi::c_int> {
                    unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    unsafe extern "C" {
                        #[link_name = "_Z15get_ptr_to_funcv"]
                        pub(crate) unsafe fn __rust_thunk___Z15get_ptr_to_funcv()
                        -> Option<unsafe extern "C" fn(*const ::core::ffi::c_int) -> *const ::core::ffi::c_int>;
                    }
                }
            }
        );
        // Verify that no C++ thunk got generated.
        assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });

        // TODO(b/217419782): Add another test where params (and the return
        // type) are references with lifetimes.  Something like this:
        //     #pragma clang lifetime_elision
        //     const int& (*get_ptr_to_func())(const int&, const int&); "#)?;
        // 1) Need to investigate why this fails - seeing raw pointers in Rust seems to
        //    indicate that no lifetimes are present at the `importer.cc` level. Maybe
        //    lifetime elision doesn't support this scenario? Unclear how to explicitly
        //    apply [[clang::annotate("lifetimes", "a, b -> a")]] to the _inner_
        //    function.
        // 2) It is important to have 2 reference parameters, so see if the problem of
        //    passing `lifetimes` by value would have been caught - see:
        //    cl/428079010/depot/rs_bindings_from_cc/
        // importer.cc?version=s6#823

        // TODO(b/217419782): Decide what to do if the C++ pointer is *not*
        // annotated with a lifetime - emit `unsafe fn(...) -> ...` in that
        // case?

        Ok(())
    }

    mod custom_abi_tests {
        use super::*;
        use ir_matchers::assert_ir_matches;
        #[gtest]
        fn test_func_ptr_with_custom_abi() -> Result<()> {
            if multiplatform_testing::test_platform() != multiplatform_testing::Platform::X86Linux {
                return Ok(());
            }
            let ir =
                ir_from_cc(r#" int (*get_ptr_to_func())(float, double) [[clang::vectorcall]]; "#)?;

            // Verify that the test input correctly represents what we intend to
            // test - we want [[clang::vectorcall]] to apply to the returned
            // function pointer, but *not* apply to the `get_ptr_to_func` function.
            assert_ir_matches!(
                ir,
                quote! {
                    Func(Func {
                        name: "get_ptr_to_func", ...
                        return_type: MappedType {
                            rs_type: RsType {
                                name: Some("Option"), ...
                                type_args: [RsType { name: Some("#funcPtr vectorcall"), ... }], ...
                            },
                            cpp_type: CcType {
                                name: Some("*"), ...
                                type_args: [CcType { name: Some("#funcValue vectorcall"), ... }], ...
                            },
                        }, ...
                        has_c_calling_convention: true, ...
                    }),
                }
            );

            let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
            // Check that the custom "vectorcall" ABI gets propagated into the
            // return type (i.e. into `extern "vectorcall" fn`).
            assert_rs_matches!(
                rs_api,
                quote! {
                    #[inline(always)]
                    pub fn get_ptr_to_func() -> Option<extern "vectorcall" fn (f32, f64) -> ::core::ffi::c_int> {
                        unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
                    }
                }
            );

            // The usual `extern "C"` ABI should be used for "get_ptr_to_func".
            assert_rs_matches!(
                rs_api,
                quote! {
                    mod detail {
                        #[allow(unused_imports)]
                        use super::*;
                        unsafe extern "C" {
                            #[link_name = "_Z15get_ptr_to_funcv"]
                            pub(crate) unsafe fn __rust_thunk___Z15get_ptr_to_funcv()
                            -> Option<extern "vectorcall" fn(f32, f64) -> ::core::ffi::c_int>;
                        }
                    }
                }
            );

            // Verify that no C++ thunk got generated.
            assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });
            Ok(())
        }

        #[gtest]
        fn test_func_ptr_with_custom_abi_thunk() -> Result<()> {
            if multiplatform_testing::test_platform() != multiplatform_testing::Platform::X86Linux {
                return Ok(());
            }
            // Using an `inline` keyword forces generation of a C++ thunk in
            // `rs_api_impl` (i.e. exercises `format_cpp_type`,
            // `format_cc_call_conv_as_clang_attribute` and similar code).
            let ir = ir_from_cc(
                r#"
                inline int (*inline_get_ptr_to_func())(float, double) [[clang::vectorcall]];
            "#,
            )?;

            // Verify that the test input correctly represents what we intend to
            // test - we want [[clang::vectorcall]] to apply to the returned
            // function pointer, but *not* apply to the `get_ptr_to_func` function.
            assert_ir_matches!(
                ir,
                quote! {
                    Func(Func {
                        name: "inline_get_ptr_to_func", ...
                        return_type: MappedType {
                            rs_type: RsType {
                                name: Some("Option"), ...
                                type_args: [RsType { name: Some("#funcPtr vectorcall"), ... }], ...
                            },
                            cpp_type: CcType {
                                name: Some("*"), ...
                                type_args: [CcType { name: Some("#funcValue vectorcall"), ... }], ...
                            },
                        }, ...
                        has_c_calling_convention: true, ...
                    }),
                }
            );

            // This test is quite similar to `test_func_ptr_thunk` - the main
            // difference is verification of the `__attribute__((vectorcall))` in
            // the expected signature of the generated thunk below.
            let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
            assert_cc_matches!(
                rs_api_impl,
                quote! {
                    extern "C" crubit::type_identity_t<
                            int(float , double) __attribute__((vectorcall))
                        >* __rust_thunk___Z22inline_get_ptr_to_funcv() {
                        return inline_get_ptr_to_func();
                    }
                }
            );
            Ok(())
        }

        #[gtest]
        fn test_custom_abi_thunk() -> Result<()> {
            if multiplatform_testing::test_platform() != multiplatform_testing::Platform::X86Linux {
                return Ok(());
            }
            let ir = ir_from_cc(
                r#"
                float f_vectorcall_calling_convention(float p1, float p2) [[clang::vectorcall]];
                double f_c_calling_convention(double p1, double p2);
            "#,
            )?;
            let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
            assert_rs_matches!(
                rs_api,
                quote! {
                    #[inline(always)]
                    pub fn f_vectorcall_calling_convention(p1: f32, p2: f32) -> f32 {
                        unsafe {
                            crate::detail::__rust_thunk___Z31f_vectorcall_calling_conventionff(p1, p2)
                        }
                    }
                }
            );
            assert_rs_matches!(
                rs_api,
                quote! {
                    #[inline(always)]
                    pub fn f_c_calling_convention(p1: f64, p2: f64) -> f64 {
                        unsafe { crate::detail::__rust_thunk___Z22f_c_calling_conventiondd(p1, p2) }
                    }
                }
            );
            // `link_name` (i.e. no thunk) for `f_c_calling_convention`. No
            // `link_name` (i.e. indicates presence of a thunk) for
            // `f_vectorcall_calling_convention`.
            assert_rs_matches!(
                rs_api,
                quote! {
                    mod detail {
                        #[allow(unused_imports)]
                        use super::*;
                        unsafe extern "C" {
                            pub(crate) unsafe fn __rust_thunk___Z31f_vectorcall_calling_conventionff(
                                p1: f32, p2: f32) -> f32;
                            #[link_name = "_Z22f_c_calling_conventiondd"]
                            pub(crate) unsafe fn __rust_thunk___Z22f_c_calling_conventiondd(
                                p1: f64, p2: f64) -> f64;
                        }
                    }
                }
            );
            // C++ thunk needed for `f_vectorcall_calling_convention`.
            assert_cc_matches!(
                rs_api_impl,
                quote! {
                    extern "C" float __rust_thunk___Z31f_vectorcall_calling_conventionff(
                        float p1, float p2) {
                            return f_vectorcall_calling_convention(p1, p2);
                    }
                }
            );
            // No C++ thunk expected for `f_c_calling_convention`.
            assert_cc_not_matches!(rs_api_impl, quote! { f_c_calling_convention });
            Ok(())
        }
    }

    #[gtest]
    fn test_item_order() -> Result<()> {
        let ir = ir_from_cc(
            "int first_func();
             struct FirstStruct {};
             int second_func();
             struct SecondStruct {};",
        )?;

        let rs_api = rs_tokens_to_formatted_string_for_tests(generate_bindings_tokens(ir)?.rs_api)?;

        let idx = |s: &str| rs_api.find(s).ok_or_else(|| anyhow!("'{}' missing", s));

        let f1 = idx("fn first_func")?;
        let f2 = idx("fn second_func")?;
        let s1 = idx("struct FirstStruct")?;
        let s2 = idx("struct SecondStruct")?;
        let t1 = idx("fn __rust_thunk___Z10first_funcv")?;
        let t2 = idx("fn __rust_thunk___Z11second_funcv")?;

        assert!(f1 < s1);
        assert!(s1 < f2);
        assert!(f2 < s2);
        assert!(s2 < t1);
        assert!(t1 < t2);

        Ok(())
    }

    #[gtest]
    fn test_generate_enum_basic() -> Result<()> {
        let ir = ir_from_cc("enum Color { kRed = 5, kBlue };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_uint);
                impl Color {
                    pub const kRed: Color = Color(5);
                    pub const kBlue: Color = Color(6);
                }
                impl From<::core::ffi::c_uint> for Color {
                    fn from(value: ::core::ffi::c_uint) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_uint {
                    fn from(value: Color) -> ::core::ffi::c_uint {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_generate_opaque_enum() -> Result<()> {
        let ir = ir_from_cc("enum Color : int;")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {Color});
        Ok(())
    }

    #[gtest]
    fn test_generate_scoped_enum_basic() -> Result<()> {
        let ir = ir_from_cc("enum class Color { kRed = -5, kBlue };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_int);
                impl Color {
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                }
                impl From<::core::ffi::c_int> for Color {
                    fn from(value: ::core::ffi::c_int) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_int {
                    fn from(value: Color) -> ::core::ffi::c_int {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_generate_enum_with_64_bit_signed_vals() -> Result<()> {
        let ir = ir_from_cc(
            r#"enum Color : long {
                    kViolet = -9223372036854775807 - 1LL,
                    kRed = -5,
                    kBlue,
                    kGreen = 3,
                    kMagenta = 9223372036854775807
                };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_long);
                impl Color {
                    pub const kViolet: Color = Color(-9223372036854775808);
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                    pub const kGreen: Color = Color(3);
                    pub const kMagenta: Color = Color(9223372036854775807);
                }
                impl From<::core::ffi::c_long> for Color {
                    fn from(value: ::core::ffi::c_long) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_long {
                    fn from(value: Color) -> ::core::ffi::c_long {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_generate_enum_with_64_bit_unsigned_vals() -> Result<()> {
        let ir = ir_from_cc(
            r#" enum Color: unsigned long {
                    kRed,
                    kBlue,
                    kLimeGreen = 18446744073709551615
                }; "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_ulong);
                impl Color {
                    pub const kRed: Color = Color(0);
                    pub const kBlue: Color = Color(1);
                    pub const kLimeGreen: Color = Color(18446744073709551615);
                }
                impl From<::core::ffi::c_ulong> for Color {
                    fn from(value: ::core::ffi::c_ulong) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_ulong {
                    fn from(value: Color) -> ::core::ffi::c_ulong {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_generate_enum_with_32_bit_signed_vals() -> Result<()> {
        let ir = ir_from_cc(
            "enum Color { kViolet = -2147483647 - 1, kRed = -5, kBlue, kGreen = 3, kMagenta = 2147483647 };",
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_int);
                impl Color {
                    pub const kViolet: Color = Color(-2147483648);
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                    pub const kGreen: Color = Color(3);
                    pub const kMagenta: Color = Color(2147483647);
                }
                impl From<::core::ffi::c_int> for Color {
                    fn from(value: ::core::ffi::c_int) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_int {
                    fn from(value: Color) -> ::core::ffi::c_int {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_generate_enum_with_32_bit_unsigned_vals() -> Result<()> {
        let ir = ir_from_cc("enum Color: unsigned int { kRed, kBlue, kLimeGreen = 4294967295 };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_uint);
                impl Color {
                    pub const kRed: Color = Color(0);
                    pub const kBlue: Color = Color(1);
                    pub const kLimeGreen: Color = Color(4294967295);
                }
                impl From<::core::ffi::c_uint> for Color {
                    fn from(value: ::core::ffi::c_uint) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_uint {
                    fn from(value: Color) -> ::core::ffi::c_uint {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_generate_enum_bool() -> Result<()> {
        let ir = ir_from_cc("enum Bool : bool { kFalse, kTrue };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Bool")]
                pub struct Bool(bool);
                impl Bool {
                    pub const kFalse: Bool = Bool(false);
                    pub const kTrue: Bool = Bool(true);
                }
                impl From<bool> for Bool {
                    fn from(value: bool) -> Bool {
                        Bool(value)
                    }
                }
                impl From<Bool> for bool {
                    fn from(value: Bool) -> bool {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_generate_enum_bool_alias() -> Result<()> {
        let ir = ir_from_cc("using MyBool = bool; enum Bool : MyBool { kFalse, kTrue };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Bool")]
                pub struct Bool(crate::MyBool);
                impl Bool {
                    pub const kFalse: Bool = Bool(false);
                    pub const kTrue: Bool = Bool(true);
                }
                impl From<crate::MyBool> for Bool {
                    fn from(value: crate::MyBool) -> Bool {
                        Bool(value)
                    }
                }
                impl From<Bool> for crate::MyBool {
                    fn from(value: Bool) -> crate::MyBool {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    /// At the least, a trivial type should have no drop impl if or until we add
    /// empty drop impls.
    #[gtest]
    fn test_no_impl_drop() -> Result<()> {
        let ir = ir_from_cc("struct Trivial {};")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl Drop});
        assert_rs_not_matches!(rs_api, quote! {impl ::ctor::PinnedDrop});
        Ok(())
    }

    /// User-defined destructors *must* become Drop impls with ManuallyDrop
    /// fields
    #[gtest]
    fn test_impl_drop_user_defined_destructor() -> Result<()> {
        let ir = ir_from_cc(
            r#" struct NontrivialStruct { ~NontrivialStruct(); };
            struct UserDefinedDestructor {
                ~UserDefinedDestructor();
                int x;
                NontrivialStruct nts;
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl ::ctor::PinnedDrop for UserDefinedDestructor {
                    #[inline(always)]
                    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
                        crate::detail::__rust_thunk___ZN21UserDefinedDestructorD1Ev(self)
                    }
                }
            }
        );
        assert_rs_matches!(rs_api, quote! {pub x: ::core::ffi::c_int,});
        assert_rs_matches!(
            rs_api,
            quote! {pub nts: ::core::mem::ManuallyDrop<crate::NontrivialStruct>,}
        );
        Ok(())
    }

    /// nontrivial types without user-defined destructors should invoke
    /// the C++ destructor to preserve the order of field destructions.
    #[gtest]
    fn test_impl_drop_nontrivial_member_destructor() -> Result<()> {
        // TODO(jeanpierreda): This would be cleaner if the UserDefinedDestructor code were
        // omitted. For example, we simulate it so that UserDefinedDestructor
        // comes from another library.
        let ir = ir_from_cc(
            r#"struct UserDefinedDestructor final {
                ~UserDefinedDestructor();
            };
            struct TrivialStruct final { int i; };
            struct NontrivialMembers final {
                UserDefinedDestructor udd;
                TrivialStruct ts;
                int x;
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl ::ctor::PinnedDrop for NontrivialMembers {
                    #[inline(always)]
                    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
                        crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self)
                    }
                }
            }
        );
        assert_rs_matches!(rs_api, quote! {pub x: ::core::ffi::c_int,});
        assert_rs_matches!(rs_api, quote! {pub ts: crate::TrivialStruct,});
        assert_rs_matches!(
            rs_api,
            quote! {pub udd: ::core::mem::ManuallyDrop<crate::UserDefinedDestructor>,}
        );
        Ok(())
    }

    #[gtest]
    fn test_c_abi_compatible_type_by_value_with_move() -> Result<()> {
        let ir = ir_from_cc(
            r#"
                typedef int MyTypedefDecl;

                inline void f(MyTypedefDecl a, void* b, int c) {}
            "#,
        )?;
        let BindingsTokens { rs_api_impl, .. } = generate_bindings_tokens(ir)?;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z1fiPvi(MyTypedefDecl a, void* b, int c) {
                    f(std::move(a), b, c);
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_type_alias() -> Result<()> {
        let ir = ir_from_cc(
            r#"
                // MyTypedefDecl doc comment
                typedef int MyTypedefDecl;

                using MyTypeAliasDecl = int;
                using MyTypeAliasDecl_Alias = MyTypeAliasDecl;

                struct S final {};
                using S_Alias = S;
                using S_Alias_Alias = S_Alias;

                inline void f(MyTypedefDecl t) {}
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[doc = " MyTypedefDecl doc comment\n \n Generated from: google3/ir_from_cc_virtual_header.h;l=5"]
                pub type MyTypedefDecl = ::core::ffi::c_int;
            }
        );
        assert_rs_matches!(rs_api, quote! { pub type MyTypeAliasDecl = ::core::ffi::c_int; });
        assert_rs_matches!(
            rs_api,
            quote! { pub type MyTypeAliasDecl_Alias = crate::MyTypeAliasDecl; }
        );
        assert_rs_matches!(rs_api, quote! { pub type S_Alias = crate::S; });
        assert_rs_matches!(rs_api, quote! { pub type S_Alias_Alias = crate::S_Alias; });
        assert_rs_matches!(rs_api, quote! { pub fn f(t: crate::MyTypedefDecl) });
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z1fi(MyTypedefDecl t) { f(std::move(t)); }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_rs_type_kind_implements_copy() -> Result<()> {
        let template = r#" LIFETIMES
            struct [[clang::trivial_abi]] TrivialStruct final { int i; };
            struct [[clang::trivial_abi]] UserDefinedCopyConstructor final {
                UserDefinedCopyConstructor(const UserDefinedCopyConstructor&);
            };
            using IntAlias = int;
            using TrivialAlias = TrivialStruct;
            using NonTrivialAlias = UserDefinedCopyConstructor;
            void func(PARAM_TYPE some_param);
        "#;
        assert_impl_all!(i32: Copy);
        assert_impl_all!(&i32: Copy);
        assert_not_impl_any!(&mut i32: Copy);
        assert_impl_all!(Option<&i32>: Copy);
        assert_not_impl_any!(Option<&mut i32>: Copy);
        assert_impl_all!(*const i32: Copy);
        assert_impl_all!(*mut i32: Copy);
        struct Test {
            // Test inputs:
            cc: &'static str,
            lifetimes: bool,
            // Expected test outputs:
            rs: &'static str,
            is_copy: bool,
        }
        let tests = vec![
            // Validity of the next few tests is verified via
            // `assert_[not_]impl_all!` static assertions above.
            Test { cc: "int", lifetimes: true, rs: ":: core :: ffi :: c_int", is_copy: true },
            Test {
                cc: "const int&",
                lifetimes: true,
                rs: "& 'a :: core :: ffi :: c_int",
                is_copy: true,
            },
            Test {
                cc: "int&",
                lifetimes: true,
                rs: "& 'a mut :: core :: ffi :: c_int",
                is_copy: false,
            },
            Test {
                cc: "const int*",
                lifetimes: true,
                rs: "Option < & 'a :: core :: ffi :: c_int >",
                is_copy: true,
            },
            Test {
                cc: "int*",
                lifetimes: true,
                rs: "Option < & 'a mut :: core :: ffi :: c_int >",
                is_copy: false,
            },
            Test {
                cc: "const int*",
                lifetimes: false,
                rs: "* const :: core :: ffi :: c_int",
                is_copy: true,
            },
            Test {
                cc: "int*",
                lifetimes: false,
                rs: "* mut :: core :: ffi :: c_int",
                is_copy: true,
            },
            Test {
                cc: "void*",
                lifetimes: false,
                rs: "* mut :: core :: ffi :: c_void",
                is_copy: true,
            },
            Test {
                cc: "const void*",
                lifetimes: false,
                rs: "* const :: core :: ffi :: c_void",
                is_copy: true,
            },
            Test {
                cc: "void* const*",
                lifetimes: false,
                rs: "* const * mut :: core :: ffi :: c_void",
                is_copy: true,
            },
            // Tests below have been thought-through and verified "manually".
            // TrivialStruct is expected to derive Copy.
            Test {
                cc: "TrivialStruct",
                lifetimes: true,
                rs: "crate :: TrivialStruct",
                is_copy: true,
            },
            Test {
                cc: "UserDefinedCopyConstructor",
                lifetimes: true,
                rs: "crate :: UserDefinedCopyConstructor",
                is_copy: false,
            },
            Test { cc: "IntAlias", lifetimes: true, rs: "crate :: IntAlias", is_copy: true },
            Test {
                cc: "TrivialAlias",
                lifetimes: true,
                rs: "crate :: TrivialAlias",
                is_copy: true,
            },
            Test {
                cc: "NonTrivialAlias",
                lifetimes: true,
                rs: "crate :: NonTrivialAlias",
                is_copy: false,
            },
        ];
        for test in tests.iter() {
            let test_name = format!("cc='{}', lifetimes={}", test.cc, test.lifetimes);
            let cc_input = template.replace("PARAM_TYPE", test.cc).replace(
                "LIFETIMES",
                if test.lifetimes { "#pragma clang lifetime_elision" } else { "" },
            );
            let db = db_from_cc(&cc_input)?;
            let ir = db.ir();

            let f = retrieve_func(&ir, "func");
            let t = db.rs_type_kind(f.params[0].type_.rs_type.clone())?;

            let fmt = t.to_token_stream().to_string();
            assert_eq!(test.rs, fmt, "Testing: {}", test_name);

            assert_eq!(test.is_copy, t.implements_copy(), "Testing: {}", test_name);
        }
        Ok(())
    }

    #[gtest]
    fn test_rs_type_kind_is_shared_ref_to_with_lifetimes() -> Result<()> {
        let db = db_from_cc(
            "#pragma clang lifetime_elision
            struct SomeStruct {};
            void foo(const SomeStruct& foo_param);
            void bar(SomeStruct& bar_param);",
        )?;
        let ir = db.ir();
        let record = ir.records().next().unwrap();
        let foo_func = retrieve_func(&ir, "foo");
        let bar_func = retrieve_func(&ir, "bar");

        // const-ref + lifetimes in C++  ===>  shared-ref in Rust
        assert_eq!(foo_func.params.len(), 1);
        let foo_param = &foo_func.params[0];
        assert_eq!(foo_param.identifier.identifier.as_ref(), "foo_param");
        let foo_type = db.rs_type_kind(foo_param.type_.rs_type.clone())?;
        assert!(foo_type.is_shared_ref_to(record));
        assert!(matches!(foo_type, RsTypeKind::Reference { mutability: Mutability::Const, .. }));

        // non-const-ref + lifetimes in C++  ===>  mutable-ref in Rust
        assert_eq!(bar_func.params.len(), 1);
        let bar_param = &bar_func.params[0];
        assert_eq!(bar_param.identifier.identifier.as_ref(), "bar_param");
        let bar_type = db.rs_type_kind(bar_param.type_.rs_type.clone())?;
        assert!(!bar_type.is_shared_ref_to(record));
        assert!(matches!(bar_type, RsTypeKind::Reference { mutability: Mutability::Mut, .. }));

        Ok(())
    }

    #[gtest]
    fn test_rs_type_kind_is_shared_ref_to_without_lifetimes() -> Result<()> {
        let db = db_from_cc(
            "struct SomeStruct {};
             void foo(const SomeStruct& foo_param);",
        )?;
        let ir = db.ir();
        let record = ir.records().next().unwrap();
        let foo_func = retrieve_func(&ir, "foo");

        // const-ref + *no* lifetimes in C++  ===>  const-pointer in Rust
        assert_eq!(foo_func.params.len(), 1);
        let foo_param = &foo_func.params[0];
        assert_eq!(foo_param.identifier.identifier.as_ref(), "foo_param");
        let foo_type = db.rs_type_kind(foo_param.type_.rs_type.clone())?;
        assert!(!foo_type.is_shared_ref_to(record));
        assert!(matches!(foo_type, RsTypeKind::Pointer { mutability: Mutability::Const, .. }));

        Ok(())
    }

    #[gtest]
    fn test_rs_type_kind_lifetimes() -> Result<()> {
        let db = db_from_cc(
            r#"
            #pragma clang lifetime_elision
            using TypeAlias = int&;
            struct SomeStruct {};
            void foo(int a, int& b, int&& c, int* d, int** e, TypeAlias f, SomeStruct g); "#,
        )?;
        let ir = db.ir();
        let func = retrieve_func(&ir, "foo");
        let ret = db.rs_type_kind(func.return_type.rs_type.clone())?;
        let a = db.rs_type_kind(func.params[0].type_.rs_type.clone())?;
        let b = db.rs_type_kind(func.params[1].type_.rs_type.clone())?;
        let c = db.rs_type_kind(func.params[2].type_.rs_type.clone())?;
        let d = db.rs_type_kind(func.params[3].type_.rs_type.clone())?;
        let e = db.rs_type_kind(func.params[4].type_.rs_type.clone())?;
        let f = db.rs_type_kind(func.params[5].type_.rs_type.clone())?;
        let g = db.rs_type_kind(func.params[6].type_.rs_type.clone())?;

        assert_eq!(0, ret.lifetimes().count()); // No lifetimes on `void`.
        assert_eq!(0, a.lifetimes().count()); // No lifetimes on `int`.
        assert_eq!(1, b.lifetimes().count()); // `&'a i32` has a single lifetime.
        assert_eq!(1, c.lifetimes().count()); // `RvalueReference<'a, i32>` has a single lifetime.
        assert_eq!(1, d.lifetimes().count()); // `Option<&'b i32>` has a single lifetime.
        assert_eq!(2, e.lifetimes().count()); // `&'c Option<&'d i32>` has two lifetimes.
        assert_eq!(1, f.lifetimes().count()); // Lifetime of underlying type should show through.
        assert_eq!(0, g.lifetimes().count()); // No lifetimes on structs (yet).
        Ok(())
    }

    #[gtest]
    fn test_rs_type_kind_lifetimes_raw_ptr() -> Result<()> {
        let db = db_from_cc("void foo(int* a);")?;
        let ir = db.ir();
        let f = retrieve_func(&ir, "foo");
        let a = db.rs_type_kind(f.params[0].type_.rs_type.clone())?;
        assert_eq!(0, a.lifetimes().count()); // No lifetimes on `int*`.
        Ok(())
    }

    #[gtest]
    fn test_rs_type_kind_rejects_func_ptr_that_returns_struct_by_value() -> Result<()> {
        let db = db_from_cc(
            r#"
            struct SomeStruct {
              int field;
            };
            SomeStruct (*get_ptr_to_func())();
        "#,
        )?;
        let ir = db.ir();
        let f = retrieve_func(&ir, "get_ptr_to_func");

        // Expecting an error, because passing a struct by value requires a thunk and
        // function pointers don't have a thunk.
        let err = db.rs_type_kind(f.return_type.rs_type.clone()).unwrap_err();
        let msg = err.to_string();
        assert_eq!(
            msg,
            "Either the return type or some of the parameter types require \
                    an FFI thunk (and function pointers don't have a thunk)",
        );
        Ok(())
    }

    #[gtest]
    fn test_rs_type_kind_rejects_func_ptr_that_takes_struct_by_value() -> Result<()> {
        let db = db_from_cc(
            r#"
            struct SomeStruct {
              int field;
            };
            void (*get_ptr_to_func())(SomeStruct);
        "#,
        )?;
        let ir = db.ir();
        let f = retrieve_func(&ir, "get_ptr_to_func");

        // Expecting an error, because passing a struct by value requires a thunk and
        // function pointers don't have a thunk.
        let err = db.rs_type_kind(f.return_type.rs_type.clone()).unwrap_err();
        let msg = err.to_string();
        assert_eq!(
            msg,
            "Either the return type or some of the parameter types require \
                    an FFI thunk (and function pointers don't have a thunk)",
        );
        Ok(())
    }

    #[gtest]
    fn test_rust_keywords_are_escaped_in_rs_api_file() -> Result<()> {
        let ir = ir_from_cc("struct type { int dyn; };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(rs_api, quote! { struct r#type { ... r#dyn: ::core::ffi::c_int ... } });
        Ok(())
    }

    #[gtest]
    fn test_rust_keywords_are_not_escaped_in_rs_api_impl_file() -> Result<()> {
        let ir = ir_from_cc("struct type { int dyn; };")?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(CRUBIT_OFFSET_OF(dyn, struct type) ... ) }
        );
        Ok(())
    }

    #[gtest]
    fn test_namespace_module_items() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test_namespace_bindings {
                int func();
                struct S {};
                namespace inner {
                    int inner_func();
                    struct InnerS {};
                }
            }
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub mod test_namespace_bindings {
                    ...
                    pub fn func() -> ::core::ffi::c_int { ... }
                    ...
                    pub struct S { ... }
                    ...
                    pub mod inner {
                        ...
                        pub fn inner_func() -> ::core::ffi::c_int { ... }
                        ...
                        pub struct InnerS { ... }
                        ...
                    }
                    ...
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_detail_outside_of_namespace_module() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test_namespace_bindings {
                int f();
            }
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub mod test_namespace_bindings {
                    ...
                }
                ...
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    unsafe extern "C" {
                        #[link_name = "_ZN23test_namespace_bindings1fEv"]
                        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings1fEv() -> ::core::ffi::c_int;
                    }
                }
                ...
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_assertions_outside_of_namespace_module() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test_namespace_bindings {
                struct S {
                    int i;
                };
            }
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub mod test_namespace_bindings {
                    ...
                }
                ...
                const _: () = {
                    ...
                    assert!(::core::mem::size_of::<crate::test_namespace_bindings::S>() == 4);
                    assert!(::core::mem::align_of::<crate::test_namespace_bindings::S>() == 4);
                    ...
                    assert!(::core::mem::offset_of!(crate::test_namespace_bindings::S, i) == 0);
                    ...
                };
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_reopened_namespaces() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
        namespace test_namespace_bindings {
        namespace inner {}
        }  // namespace test_namespace_bindings

        namespace test_namespace_bindings {
        namespace inner {}
        }  // namespace test_namespace_bindings"#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                ...
                pub mod test_namespace_bindings_0 {
                    pub mod inner_0 {} ...
                }
                ...
                pub mod test_namespace_bindings {
                    __HASH_TOKEN__[allow(unused_imports)]
                    pub use super::test_namespace_bindings_0::*;
                    ...
                    pub mod inner {
                        __HASH_TOKEN__[allow(unused_imports)]
                        pub use super::inner_0::*;
                        ...
                    }
                }
                ...
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_qualified_identifiers_in_impl_file() -> Result<()> {
        let rs_api_impl = generate_bindings_tokens(ir_from_cc(
            r#"
        namespace test_namespace_bindings {
            inline void f() {};
            struct S final {};
        }
        inline void useS(test_namespace_bindings::S s) {};"#,
        )?)?
        .rs_api_impl;

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN23test_namespace_bindings1fEv() {
                    test_namespace_bindings::f();
                }
                ...
                extern "C" void __rust_thunk___Z4useSN23test_namespace_bindings1SE(
                        struct test_namespace_bindings::S* s) {
                    useS(std::move(*s));
                }
                ...
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_inline_namespace() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test_namespace_bindings {
                inline namespace inner {
                    struct MyStruct final {};
                }
                void processMyStruct(MyStruct s);
            }
            void processMyStructOutsideNamespace(test_namespace_bindings::inner::MyStruct s);
            void processMyStructSkipInlineNamespaceQualifier(test_namespace_bindings::MyStruct s);
            "#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                ...
                pub mod test_namespace_bindings {
                    ...
                    pub mod inner {
                        ...
                        pub struct MyStruct {...} ...
                    }
                    __HASH_TOKEN__[allow(unused_imports)]
                    pub use inner::*;
                    ...
                    pub fn processMyStruct(
                        mut s: crate::test_namespace_bindings::inner::MyStruct)
                    ...
                }
                ...
                pub fn processMyStructOutsideNamespace(
                    mut s: crate::test_namespace_bindings::inner::MyStruct)
                ...
                pub fn processMyStructSkipInlineNamespaceQualifier(
                    mut s: crate::test_namespace_bindings::inner::MyStruct)
                ...
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_inline_namespace_not_marked_inline() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            inline namespace my_inline {}
            namespace foo {}
            namespace my_inline {  // still an inline namespace!
                struct MyStruct final {};
            }
            "#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
               ...
               pub mod my_inline_0 {}
               pub mod foo {}
               pub mod my_inline {
                   __HASH_TOKEN__[allow(unused_imports)]
                   pub use super::my_inline_0::*;
                   ...
                   pub struct MyStruct {...}
                   ...
               }
               __HASH_TOKEN__[allow(unused_imports)]
               pub use my_inline::*;
               ...
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_generate_doc_comment_with_no_comment_with_no_source_loc_with_source_loc_enabled() {
        let actual = generate_doc_comment(None, None, SourceLocationDocComment::Enabled);
        assert!(actual.is_empty());
    }

    #[gtest]
    fn test_generate_doc_comment_with_no_comment_with_source_loc_with_source_loc_enabled() {
        let actual = generate_doc_comment(
            None,
            Some("google3/some/header;l=11"),
            SourceLocationDocComment::Enabled,
        );
        assert_rs_matches!(actual, quote! {#[doc = " google3/some/header;l=11"]});
    }

    #[gtest]
    fn test_generate_doc_comment_with_comment_with_source_loc_with_source_loc_enabled() {
        let actual = generate_doc_comment(
            Some("Some doc comment"),
            Some("google3/some/header;l=12"),
            SourceLocationDocComment::Enabled,
        );
        assert_rs_matches!(
            actual,
            quote! {#[doc = " Some doc comment\n \n google3/some/header;l=12"]}
        );
    }

    #[gtest]
    fn test_generate_doc_comment_with_comment_with_no_source_loc_with_source_loc_enabled() {
        let actual =
            generate_doc_comment(Some("Some doc comment"), None, SourceLocationDocComment::Enabled);
        assert_rs_matches!(actual, quote! {#[doc = " Some doc comment"]});
    }

    #[gtest]
    fn test_no_generate_doc_comment_with_no_comment_with_no_source_loc_with_source_loc_disabled() {
        let actual = generate_doc_comment(None, None, SourceLocationDocComment::Disabled);
        assert!(actual.is_empty());
    }

    #[gtest]
    fn test_no_generate_doc_comment_with_no_comment_with_source_loc_with_source_loc_disabled() {
        let actual = generate_doc_comment(
            None,
            Some("google3/some/header;l=13"),
            SourceLocationDocComment::Disabled,
        );
        assert!(actual.is_empty());
    }

    #[gtest]
    fn test_no_generate_doc_comment_with_comment_with_source_loc_with_source_loc_disabled() {
        let actual = generate_doc_comment(
            Some("Some doc comment"),
            Some("google3/some/header;l=14"),
            SourceLocationDocComment::Disabled,
        );
        assert_rs_matches!(actual, quote! {#[doc = " Some doc comment"]});
    }

    #[gtest]
    fn test_no_generate_doc_comment_with_comment_with_no_source_loc_with_source_loc_disabled() {
        let actual = generate_doc_comment(
            Some("Some doc comment"),
            None,
            SourceLocationDocComment::Disabled,
        );
        assert_rs_matches!(actual, quote! {#[doc = " Some doc comment"]});
    }

    struct TestItem {
        source_loc: Option<Rc<str>>,
    }
    impl ir::GenericItem for TestItem {
        fn id(&self) -> ItemId {
            ItemId::new_for_testing(123)
        }
        fn debug_name(&self, _: &IR) -> Rc<str> {
            "test_item".into()
        }
        fn source_loc(&self) -> Option<Rc<str>> {
            self.source_loc.clone()
        }
        fn unknown_attr(&self) -> Option<Rc<str>> {
            None
        }
    }

    #[gtest]
    fn test_generate_unsupported_item_with_source_loc_enabled() -> Result<()> {
        let db = Database::new(
            Rc::new(make_ir_from_items([])),
            Rc::new(ErrorReport::new()),
            SourceLocationDocComment::Enabled,
        );
        let actual = generate_unsupported(
            &db,
            &UnsupportedItem::new_with_static_message(
                &db.ir(),
                &TestItem { source_loc: Some("Generated from: google3/some/header;l=1".into()) },
                "unsupported_message",
            ),
        )?;
        let expected = "Generated from: google3/some/header;l=1\nError while generating bindings for item 'test_item':\nunsupported_message";
        assert_rs_matches!(actual.item, quote! { __COMMENT__ #expected});
        Ok(())
    }

    /// Not all items currently have source_loc(), e.g. comments.
    ///
    /// For these, we omit the mention of the location.
    #[gtest]
    fn test_generate_unsupported_item_with_missing_source_loc() -> Result<()> {
        let db = Database::new(
            Rc::new(make_ir_from_items([])),
            Rc::new(ErrorReport::new()),
            SourceLocationDocComment::Enabled,
        );
        let actual = generate_unsupported(
            &db,
            &UnsupportedItem::new_with_static_message(
                &db.ir(),
                &TestItem { source_loc: None },
                "unsupported_message",
            ),
        )?;
        let expected = "Error while generating bindings for item 'test_item':\nunsupported_message";
        assert_rs_matches!(actual.item, quote! { __COMMENT__ #expected});
        Ok(())
    }

    #[gtest]
    fn test_generate_unsupported_item_with_source_loc_disabled() -> Result<()> {
        let db = Database::new(
            Rc::new(make_ir_from_items([])),
            Rc::new(ErrorReport::new()),
            SourceLocationDocComment::Disabled,
        );
        let actual = generate_unsupported(
            &db,
            &UnsupportedItem::new_with_static_message(
                &db.ir(),
                &TestItem { source_loc: Some("Generated from: google3/some/header;l=1".into()) },
                "unsupported_message",
            ),
        )?;
        let expected = "Error while generating bindings for item 'test_item':\nunsupported_message";
        assert_rs_matches!(actual.item, quote! { __COMMENT__ #expected});
        Ok(())
    }

    /// Enumerators with unknown attributes on otherwise-ok enums are omitted.
    ///
    /// This is hard to test any other way than token comparison!
    #[gtest]
    fn test_supported_unknown_attr_enumerator() -> Result<()> {
        let mut ir = ir_from_cc(
            r#"
            enum Enum {
                kHidden [[deprecated]],
            };
            "#,
        )?;
        *ir.target_crubit_features_mut(&ir.current_target().clone()) =
            crubit_feature::CrubitFeature::Supported.into();
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(rs_api, quote! {pub struct Enum});
        assert_rs_not_matches!(rs_api, quote! {kHidden});
        Ok(())
    }

    /// Namespaces with an unknown attribute are not present in supported.
    ///
    /// This is hard to test any other way than token comparison, because it's
    /// hard to test for the nonexistence of a module.
    #[gtest]
    fn test_supported_unknown_attr_namespace() -> Result<()> {
        for nested_notpresent in
            ["struct NotPresent {};", "struct NotPresent;", "enum NotPresent {};"]
        {
            let mut ir = ir_from_cc(&format!(
                r#"
                namespace [[deprecated]] unknown_attr_namespace {{
                    {nested_notpresent}
                }}
                extern "C" {{
                    void NotPresent(unknown_attr_namespace::NotPresent);
                    unknown_attr_namespace::NotPresent AlsoNotPresent();
                }}
                "#
            ))?;
            *ir.target_crubit_features_mut(&ir.current_target().clone()) =
                crubit_feature::CrubitFeature::Supported.into();
            let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
            // The namespace, and everything in it or using it, will be missing from the
            // output.
            assert_rs_not_matches!(rs_api, quote! {NotPresent});
            assert_rs_not_matches!(rs_api, quote! {AlsoNotPresent});
            assert_rs_not_matches!(rs_api, quote! {unknown_attr_namespace});
        }
        Ok(())
    }

    /// Namespaces with an unknown attribute are still merged with the same
    /// namespace with no unknown attribute.
    #[gtest]
    fn test_supported_unknown_attr_namespace_merge() -> Result<()> {
        let mut ir = ir_from_cc(
            r#"
            namespace unknown_attr_namespace {
                enum Present {};
            }
            namespace [[deprecated]] unknown_attr_namespace {
                enum NotPresent {};
            }
            namespace unknown_attr_namespace {
                enum AlsoPresent {};
            }
            "#,
        )?;
        *ir.target_crubit_features_mut(&ir.current_target().clone()) =
            crubit_feature::CrubitFeature::Supported.into();
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        // The namespace, and everything in it or using it, will be missing from the
        // output.
        assert_rs_not_matches!(rs_api, quote! {NotPresent});
        assert_rs_matches!(rs_api, quote! {Present});
        assert_rs_matches!(rs_api, quote! {AlsoPresent});
        assert_rs_matches!(rs_api, quote! {unknown_attr_namespace});
        Ok(())
    }

    /// Namespaces with an unknown attribute are not present in supported, but
    /// their typedefs are.
    #[gtest]
    fn test_supported_unknown_attr_namespace_typedef() -> Result<()> {
        let mut ir = ir_from_cc(
            r#"
            namespace [[deprecated]] unknown_attr_namespace {
                using NotPresent = int;
            }
            extern "C" {
                void Func(unknown_attr_namespace::NotPresent x);
                unknown_attr_namespace::NotPresent Func2();
            }
            "#,
        )?;
        *ir.target_crubit_features_mut(&ir.current_target().clone()) =
            crubit_feature::CrubitFeature::Supported.into();
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        // The namespace, and everything in it or using it, will be missing from the
        // output.
        assert_rs_not_matches!(rs_api, quote! {NotPresent});
        assert_rs_matches!(rs_api, quote! {pub fn Func(x: ::core::ffi::c_int)});
        assert_rs_matches!(rs_api, quote! {pub fn Func2() -> ::core::ffi::c_int});
        Ok(())
    }

    /// The default crubit feature set currently doesn't include supported.
    #[gtest]
    fn test_default_crubit_features_disabled_supported() -> Result<()> {
        for item in [
            "extern \"C\" void NotPresent() {}",
            "struct NotPresent {};",
            "extern \"C\" int NotPresent() {}",
        ] {
            let mut ir = ir_from_cc(item)?;
            ir.target_crubit_features_mut(&ir.current_target().clone()).clear();
            let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
            assert_rs_not_matches!(rs_api, quote! {NotPresent});
            assert_cc_not_matches!(rs_api_impl, quote! {NotPresent});
            let contents = rs_tokens_to_formatted_string_for_tests(rs_api)?;
            // using a string comparison and leaving off the end, because the exact reason
            // why differs per item.
            let expected = "\
                // Generated from: google3/ir_from_cc_virtual_header.h;l=3\n\
                // Error while generating bindings for item 'NotPresent':\n\
                // Can't generate bindings for NotPresent, because of missing required features (<internal link>):\n\
                // //test:testing_target needs [//features:supported] for NotPresent";
            assert!(contents.contains(expected), "Missing expected string: {contents}\n")
        }
        Ok(())
    }

    /// The default crubit feature set currently doesn't include experimetnal.
    #[gtest]
    fn test_default_crubit_features_disabled_experimental() -> Result<()> {
        let mut ir = ir_from_cc("struct NotPresent;")?;
        ir.target_crubit_features_mut(&ir.current_target().clone()).clear();
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_not_matches!(rs_api, quote! {NotPresent});
        assert_cc_not_matches!(rs_api_impl, quote! {NotPresent});
        let expected = "\
            Error while generating bindings for item 'NotPresent':\n\
            Can't generate bindings for NotPresent, because of missing required features (<internal link>):\n\
            //test:testing_target needs [//features:experimental] for NotPresent (incomplete type)";
        assert_rs_matches!(rs_api, quote! { __COMMENT__ #expected});
        Ok(())
    }

    #[gtest]
    fn test_default_crubit_features_disabled_dependency_supported_function_parameter() -> Result<()>
    {
        for dependency in ["struct NotPresent {};"] {
            let mut ir = ir_from_cc_dependency("void Func(NotPresent);", dependency)?;
            ir.target_crubit_features_mut(&ir::BazelLabel("//test:dependency".into())).clear();
            let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
            assert_rs_not_matches!(rs_api, quote! {Func});
            assert_cc_not_matches!(rs_api_impl, quote! {Func});
            let expected = "\
                Generated from: google3/ir_from_cc_virtual_header.h;l=3\n\
                Error while generating bindings for item 'Func':\n\
                Failed to format type of parameter 0: Can't generate bindings for NotPresent, because of missing required features (<internal link>):\n\
                //test:dependency needs [//features:supported] for NotPresent";
            assert_rs_matches!(rs_api, quote! { __COMMENT__ #expected});
        }
        Ok(())
    }

    #[gtest]
    fn test_default_crubit_features_disabled_dependency_experimental_function_parameter(
    ) -> Result<()> {
        let mut ir = ir_from_cc_dependency(
            "void Func(NotPresent);",
            "template <typename T> struct NotPresentTemplate {T x;}; using NotPresent = NotPresentTemplate<int>;",
        )?;
        ir.target_crubit_features_mut(&ir::BazelLabel("//test:dependency".into())).clear();
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_not_matches!(rs_api, quote! {Func});
        assert_cc_not_matches!(rs_api_impl, quote! {Func});
        let expected = "\
            Generated from: google3/ir_from_cc_virtual_header.h;l=3\n\
            Error while generating bindings for item 'Func':\n\
            Failed to format type of parameter 0: Can't generate bindings for NotPresentTemplate<int>, because of missing required features (<internal link>):\n\
            //test:dependency needs [//features:experimental] for NotPresentTemplate<int> (crate::__CcTemplateInst18NotPresentTemplateIiE is a template instantiation)";
        assert_rs_matches!(rs_api, quote! { __COMMENT__ #expected});
        Ok(())
    }

    #[gtest]
    fn test_default_crubit_features_disabled_dependency_supported_function_return_type(
    ) -> Result<()> {
        let mut ir = ir_from_cc_dependency("NotPresent Func();", "struct NotPresent {};")?;
        ir.target_crubit_features_mut(&ir::BazelLabel("//test:dependency".into())).clear();
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_not_matches!(rs_api, quote! {Func});
        assert_cc_not_matches!(rs_api_impl, quote! {Func});
        let expected = "\
            Generated from: google3/ir_from_cc_virtual_header.h;l=3\n\
            Error while generating bindings for item 'Func':\n\
            Failed to format return type: Can't generate bindings for NotPresent, because of missing required features (<internal link>):\n\
            //test:dependency needs [//features:supported] for NotPresent";
        assert_rs_matches!(rs_api, quote! { __COMMENT__ #expected});
        Ok(())
    }

    #[gtest]
    fn test_default_crubit_features_disabled_dependency_experimental_function_return_type(
    ) -> Result<()> {
        let mut ir = ir_from_cc_dependency(
            "NotPresent Func();",
            "template <typename T> struct NotPresentTemplate {T x;}; using NotPresent = NotPresentTemplate<int>;")?;
        ir.target_crubit_features_mut(&ir::BazelLabel("//test:dependency".into())).clear();
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_not_matches!(rs_api, quote! {Func});
        assert_cc_not_matches!(rs_api_impl, quote! {Func});
        let expected = "\
            Generated from: google3/ir_from_cc_virtual_header.h;l=3\n\
            Error while generating bindings for item 'Func':\n\
            Failed to format return type: Can't generate bindings for NotPresentTemplate<int>, because of missing required features (<internal link>):\n\
            //test:dependency needs [//features:experimental] for NotPresentTemplate<int> (crate::__CcTemplateInst18NotPresentTemplateIiE is a template instantiation)";
        assert_rs_matches!(rs_api, quote! { __COMMENT__ #expected});
        Ok(())
    }

    #[gtest]
    fn test_default_crubit_features_disabled_dependency_struct() -> Result<()> {
        for dependency in ["struct NotPresent {signed char x;};", "using NotPresent = signed char;"]
        {
            let mut ir = ir_from_cc_dependency("struct Present {NotPresent field;};", dependency)?;
            ir.target_crubit_features_mut(&ir::BazelLabel("//test:dependency".into())).clear();
            let BindingsTokens { rs_api, rs_api_impl: _ } = generate_bindings_tokens(ir)?;
            assert_rs_matches!(
                rs_api,
                quote! {
                    pub struct Present {
                        ...
                        pub(crate) field: [::core::mem::MaybeUninit<u8>; 1],
                    }
                }
            );
        }
        Ok(())
    }

    #[gtest]
    fn test_default_crubit_features_disabled_template_explicit_specialization() -> Result<()> {
        let mut ir = ir_from_cc(
            r#"
            template <typename T>
            struct X {
                T t;
            };

            template <>
            struct X<int> {
                int val;
                X<int>() : val(42) {}
            };

            inline X<int> NotPresent() { return X<int>(); }"#,
        )?;
        *ir.target_crubit_features_mut(&ir.current_target().clone()) =
            crubit_feature::CrubitFeature::Supported.into();
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_not_matches!(rs_api, quote! {NotPresent});
        assert_cc_not_matches!(rs_api_impl, quote! {NotPresent});
        Ok(())
    }

    #[gtest]
    fn test_type_map_override_assert() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#" #pragma clang lifetime_elision
                // Broken class: uses i32 but has size 1.
                // (These asserts would fail if this were compiled.)
                class [[clang::annotate("crubit_internal_rust_type", "i32")]] Class final {};"#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                assert!(::core::mem::size_of::<i32>() == 1);
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                assert!(::core::mem::align_of::<i32>() == 1);
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_type_map_override_c_abi_incompatible() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#" #pragma clang lifetime_elision
                // Broken class: uses i32 but has size 1.
                // (These asserts would fail if this were compiled.)
                class [[clang::annotate("crubit_internal_rust_type", "i8")]] MyI8 {unsigned char field;};
                MyI8 Make();"#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn Make() -> i8 {...}
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                pub(crate) unsafe fn __rust_thunk___Z4Makev(__return: &mut ::core::mem::MaybeUninit<i8>);
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_type_map_override_c_abi_compatible() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#" #pragma clang lifetime_elision
                class
                    [[clang::annotate("crubit_internal_rust_type", "i8")]]
                    [[clang::annotate("crubit_internal_same_abi")]]
                    MyI8 {unsigned char field;};
                MyI8 Make();"#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn Make() -> i8 {...}
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                pub(crate) unsafe fn __rust_thunk___Z4Makev() -> i8;
            }
        );
        Ok(())
    }

    /// We cannot generate size/align assertions for incomplete types.
    #[gtest]
    fn test_type_map_override_assert_incomplete() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#" #pragma clang lifetime_elision
                // Broken class: uses i32 but has size 1.
                // (These asserts would fail if this were compiled.)
                class [[clang::annotate("crubit_internal_rust_type", "i32")]] Incomplete;
            "#,
        )?)?
        .rs_api;

        assert_rs_not_matches!(
            rs_api,
            quote! {
            const _: () = { ... ::core::mem::size_of::<i32>() ... } }
        );

        assert_rs_not_matches!(
            rs_api,
            quote! {
            const _: () = { ... ::core::mem::align_of::<i32>() ... }}
        );
        Ok(())
    }
}
