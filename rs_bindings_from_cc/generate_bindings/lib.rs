// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

use arc_anyhow::{anyhow, ensure, Context, Result};
use code_gen_utils::{format_cc_includes, is_cpp_reserved_keyword, make_rs_ident, CcInclude};
use cpp_type_name::format_cpp_type_with_references;
use crubit_abi_type::{
    CrubitAbiType, CrubitAbiTypeToCppTokens, CrubitAbiTypeToRustExprTokens,
    CrubitAbiTypeToRustTokens, FullyQualifiedPath,
};
use crubit_feature::CrubitFeature;
use database::code_snippet::{
    self, ApiSnippets, Bindings, BindingsTokens, CppDetails, CppIncludes, Feature, GeneratedItem,
};
use database::db::{BindingsGenerator, CodegenFunctions};
use database::rs_snippet::{
    BridgeRsTypeKind, Callable, FnTrait, Mutability, PassingConvention, RsTypeKind, RustPtrKind,
    Safety,
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
use std::fmt::Write;
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

fn generate_type_alias(db: &BindingsGenerator, type_alias: Rc<TypeAlias>) -> Result<ApiSnippets> {
    db.errors().add_category(error_report::Category::Alias);
    // Skip the type alias if it maps to a bridge type.
    // NOTE: rs_type_kind() gives a poor error message ("no bindings for <Alias>") if the underlying
    // type is unsupported, so that most users of rs_type_kind (e.g. function definitions, structs)
    // will fail with an error message about the _alias_ being unsupported, not the alias-ee.
    //
    // We, however, want to be more specific. To get the better error message, we call directly
    // into has_bindings().
    //
    // Since rs_type_kind() can succeed even if this alias is unsupported (it is "seen through"),
    // we only do so after rs_type_kind() fails.
    let Ok(rs_type_kind) = db.rs_type_kind((&*type_alias).into()) else {
        // Return the un-hidden raw error from has_bindings().
        let Err(e) = db.has_bindings(ir::Item::TypeAlias(type_alias)) else {
            unreachable!(
                "Crubit promised to have bindings for a type alias, but didn't. This is a bug."
            )
        };
        return Err(e.into());
    };

    let underlying_type = db
        .rs_type_kind(type_alias.underlying_type.clone())
        .with_context(|| format!("Failed to format underlying type for {type_alias}"))?;

    // If this type alias refers to a record with nested types,
    // we need to also re-export the generated module.
    let mut underlying_nested_module_path = None;
    if let RsTypeKind::Record { record, crate_path, .. } = &underlying_type
        && generate_struct_and_union::child_items(record, db).any(|child_item| child_item.is_nested)
    {
        let underlying_nested_module_name =
            make_rs_ident(&record.rs_name.identifier.to_snake_case());
        underlying_nested_module_path = Some(quote! { #crate_path #underlying_nested_module_name });
    }

    let generated_item = GeneratedItem::TypeAlias {
        doc_comment: generate_doc_comment(
            type_alias.doc_comment.as_deref(),
            None,
            Some(&type_alias.source_loc),
            db.environment(),
        ),
        visibility: db.type_visibility(&type_alias.owning_target, rs_type_kind).unwrap_or_default(),
        ident: make_rs_ident(&type_alias.rs_name.identifier),
        underlying_type: underlying_type.to_token_stream(db),
        underlying_nested_module_path,
    };
    Ok(ApiSnippets {
        generated_items: HashMap::from([(type_alias.id, generated_item)]),
        ..Default::default()
    })
}

fn generate_global_var(db: &BindingsGenerator, var: Rc<GlobalVar>) -> Result<ApiSnippets> {
    db.errors().add_category(error_report::Category::Variable);
    let type_ = db.rs_type_kind(var.type_.clone())?;

    Ok(ApiSnippets {
        generated_items: HashMap::from([(
            var.id,
            GeneratedItem::GlobalVar {
                link_name: var.mangled_name.clone(),
                is_mut: !var.type_.is_const,
                ident: make_rs_ident(&var.rs_name.identifier),
                type_tokens: type_.to_token_stream(db),
                visibility: db.type_visibility(&var.owning_target, type_).unwrap_or_default(),
            },
        )]),
        ..Default::default()
    })
}

fn generate_namespace(db: &BindingsGenerator, namespace: Rc<Namespace>) -> Result<ApiSnippets> {
    db.errors().add_category(error_report::Category::Namespace);
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
fn generate_item(db: &BindingsGenerator, item: Item) -> Result<ApiSnippets> {
    if let Some(owning_target) = item.owning_target() {
        if !db.ir().is_current_target(&owning_target) {
            return Ok(ApiSnippets::default());
        }
    }
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
fn generate_item_impl(db: &BindingsGenerator, item: &Item) -> Result<ApiSnippets> {
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
        Item::UnsupportedItem(unsupported) => {
            // Categorize unsupported items directly produced from the C++ importer.
            // We let generate_record, generate_enum, etc. handle categorization when the item
            // had a more specific type, which is why this categorization goes here, and not
            // in generate_unsupported.
            use UnsupportedItemKind::*;
            match unsupported.kind {
                Func | Constructor => {
                    db.errors().add_category(error_report::Category::Function);
                }
                GlobalVar => {
                    db.errors().add_category(error_report::Category::Variable);
                }
                Class | Struct | Union | Enum => {
                    db.errors().add_category(error_report::Category::Type);
                }
                TypeAlias => {
                    db.errors().add_category(error_report::Category::Alias);
                }
                Namespace | Other => {}
            }
            generate_unsupported(db, unsupported.clone())
        }
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
                cpp_type = existing_rust_type.debug_name(db.ir()),
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
) -> BindingsGenerator<'db> {
    BindingsGenerator::new(
        ir,
        errors,
        fatal_errors,
        environment,
        CodegenFunctions {
            generate_enum: generate_enum::generate_enum,
            generate_item,
            generate_record: generate_struct_and_union::generate_record,
        },
        rs_type_kind_safety,
        record_field_safety,
        record_safety,
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
                // If the record doesn't belong to a target with the Callables feature, skip.
                if !ir
                    .target_crubit_features(&record.owning_target)
                    .contains(CrubitFeature::Callables)
                {
                    return None;
                }

                // It doesn't matter since we're just checking for presence of DynCallable.
                let has_reference_param = false;

                // Find records that are template instantiations of `rs_std::DynCallable`.
                let Ok(Some(BridgeRsTypeKind::DynCallable(dyn_callable))) =
                    BridgeRsTypeKind::new(record, has_reference_param, &db)
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

/// Implementation of `BindingsGenerator::rs_type_kind_safety`.
fn rs_type_kind_safety(db: &BindingsGenerator, rs_type_kind: RsTypeKind) -> Safety {
    match rs_type_kind {
        RsTypeKind::Error { error, .. } => {
            Safety::unsafe_because(format!("Crubit cannot assume unknown types are safe: {error}"))
        }
        RsTypeKind::Pointer { .. } => Safety::unsafe_because("raw pointer"),
        RsTypeKind::Reference { referent: t, .. }
        | RsTypeKind::RvalueReference { referent: t, .. }
        | RsTypeKind::TypeAlias { underlying_type: t, .. } => {
            db.rs_type_kind_safety(t.as_ref().clone())
        }
        RsTypeKind::FuncPtr { return_type, param_types, .. } => {
            callable_safety(db, &param_types, &return_type)
        }
        RsTypeKind::IncompleteRecord { .. } => {
            // TODO(b/390474240): Add a way to mark a forward declaration as being an unsafe
            // type.
            Safety::Safe
        }
        RsTypeKind::Enum { .. }
        | RsTypeKind::Primitive(..)
        | RsTypeKind::ExistingRustType { .. } => Safety::Safe,
        RsTypeKind::BridgeType { bridge_type, original_type } => match bridge_type {
            // TODO(b/390621592): Should bridge types just delegate to the underlying type?
            BridgeRsTypeKind::BridgeVoidConverters { .. } | BridgeRsTypeKind::Bridge { .. } => {
                if record_safety(db, original_type.clone()).is_safe() {
                    Safety::Safe
                } else {
                    // Full unsafe reason is not shown here, it's documented on the type instead.
                    Safety::unsafe_because("unsafe bridge type")
                }
            }
            BridgeRsTypeKind::ProtoMessageBridge { .. } => {
                if record_safety(db, original_type.clone()).is_safe() {
                    Safety::Safe
                } else {
                    // Full unsafe reason is not shown here, it's documented on the type instead.
                    Safety::unsafe_because("unsafe proto message type")
                }
            }
            BridgeRsTypeKind::StdOptional(t) => db.rs_type_kind_safety(t.as_ref().clone()),
            BridgeRsTypeKind::StdPair(t1, t2) => {
                let s1 = db.rs_type_kind_safety(t1.as_ref().clone());
                let s2 = db.rs_type_kind_safety(t2.as_ref().clone());

                let r1 = s1
                    .unsafe_reason()
                    .map(|reason| format!("pair's first element is unsafe: {reason}"));
                let r2 = s2
                    .unsafe_reason()
                    .map(|reason| format!("pair's second element is unsafe: {reason}"));
                if let (Some(r1), Some(r2)) = (&r1, &r2) {
                    Safety::unsafe_because(format!("{r1}; {r2}"))
                } else if let Some(r1) = r1 {
                    Safety::unsafe_because(r1)
                } else if let Some(r2) = r2 {
                    Safety::unsafe_because(r2)
                } else {
                    Safety::Safe
                }
            }
            BridgeRsTypeKind::StdString { .. } => Safety::Safe,
            BridgeRsTypeKind::DynCallable(dyn_callable) => {
                callable_safety(db, &dyn_callable.param_types, &dyn_callable.return_type)
            }
            BridgeRsTypeKind::C9Co { result_type, .. } => {
                // A Co<T> logically produces a T, so it is unsafe iff T is unsafe.
                db.rs_type_kind_safety(result_type.as_ref().clone())
            }
        },
        RsTypeKind::Record { record, .. } => {
            if record_safety(db, record.clone()).is_safe() {
                Safety::Safe
            } else {
                // Full unsafe reason is not shown here, it's documented on the type instead.
                Safety::unsafe_because("unsafe struct or union")
            }
        }
    }
}

/// Helper function for `rs_type_kind_safety`.
/// Returns whether a callable is unsafe due to its parameters or return type.
fn callable_safety(
    db: &BindingsGenerator,
    param_types: &[RsTypeKind],
    return_type: &RsTypeKind,
) -> Safety {
    let param_reasons = param_types
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(i, param_type)| {
            let safety = db.rs_type_kind_safety(param_type);
            let reason = safety.unsafe_reason()?;
            let i = i + 1;
            Some(format!("param {i} is of unsafe type {reason}"))
        })
        .collect_vec();
    let return_safety = db.rs_type_kind_safety(return_type.clone());

    if param_reasons.is_empty() && return_safety.is_safe() {
        return Safety::Safe;
    }

    let mut reasons = if param_reasons.is_empty() {
        String::new()
    } else {
        format!("Callable takes unsafe parameters: {}", param_reasons.join(", "))
    };

    if let Some(return_reason) = return_safety.unsafe_reason() {
        if reasons.is_empty() {
            reasons = format!("Callable return type is unsafe: {return_reason}");
        } else {
            reasons = format!("{reasons}; and return type is unsafe: {return_reason}")
        }
    }

    Safety::unsafe_because(reasons)
}

/// Implementation of `BindingsGenerator::record_field_safety`.
fn record_field_safety(db: &BindingsGenerator, field: Field) -> Safety {
    if field.access != AccessSpecifier::Public {
        return Safety::Safe;
    }
    let field_rs_type_kind = match db.rs_type_kind(field.type_.clone()) {
        Ok(field_rs_type_kind) => field_rs_type_kind,
        Err(err) => {
            // If we can't get the RsTypeKind for a public field, we assume it's unsafe.
            return Safety::unsafe_because(
                format!("Rust type is unknown; safety requirements cannot be automatically generated: {err}"),
            );
        }
    };
    db.rs_type_kind_safety(field_rs_type_kind)
}

/// Implementation of `BindingsGenerator::record_safety`.
fn record_safety(db: &BindingsGenerator, record: Rc<Record>) -> Safety {
    let mut doc = String::new();

    match record.safety_annotation {
        SafetyAnnotation::DisableUnsafe => {
            return Safety::Safe;
        }
        SafetyAnnotation::Unsafe => {
            // TODO(b/480191443): allow C++ annotations to provide a specific reason.
            doc += "* The C++ type is explicitly annotated as unsafe. Ensure that its safety requirements are upheld.";
        }
        SafetyAnnotation::Unannotated => {}
    }

    if record.is_union() {
        doc += "* The callee does not read an incorrect field out of the union.\n";
    }

    let reasons: Vec<_> = record
        .fields
        .iter()
        .filter_map(|field| {
            let reason = db.record_field_safety(field.clone()).unsafe_reason()?;

            // TODO(nicholasbishop): handle unnamed better.
            let mut name = field
                .rust_identifier
                .as_ref()
                .map(|i| format!("`{}`", i.as_str()))
                .unwrap_or("unnamed field".to_owned());
            write!(name, ": {reason}").unwrap();
            Some(name)
        })
        .collect();

    if matches!(record.safety_annotation, SafetyAnnotation::Unannotated)
        && !record.is_union()
        && reasons.is_empty()
    {
        return Safety::Safe;
    }

    if !reasons.is_empty() {
        doc += "* Document why the following public unsafe fields of this type cannot be misused by callee:\n";

        for reason in reasons {
            writeln!(doc, "  * {reason}").unwrap();
        }
    }

    // Verify that we didn't generate an empty safety doc.
    assert!(!doc.is_empty());

    Safety::unsafe_because(format!(
        "To call a function that accepts this type, you must uphold these requirements:\n{doc}"
    ))
}

fn generate_rs_api_impl_includes(
    db: &BindingsGenerator,
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
        // We don't actually use the possible c9::Co, but need to pass in something to `new`.
        let has_reference_param = false;

        // Err means that this bridge type has some issues. For the purpose of generating includes,
        // we can ignore it.
        if let Ok(Some(bridge_type)) = BridgeRsTypeKind::new(record, has_reference_param, db) {
            match bridge_type {
                BridgeRsTypeKind::BridgeVoidConverters { .. } => {
                    internal_includes.insert(CcInclude::SupportLibHeader(
                        crubit_support_path_format.clone(),
                        "internal/lazy_init.h".into(),
                    ));
                }
                BridgeRsTypeKind::C9Co { .. } => {
                    internal_includes.insert(CcInclude::SupportLibHeader(
                        crubit_support_path_format.clone(),
                        "bridge.h".into(),
                    ));
                    internal_includes.insert(CcInclude::user_header(
                        "util/c9/internal/rust/co_crubit_abi.h".into(),
                    ));
                }
                _ => {
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

        if ir
            .target_crubit_features(&record.owning_target)
            .contains(crubit_feature::CrubitFeature::Fmt)
            && record.detected_formatter
        {
            internal_includes.insert(CcInclude::SupportLibHeader(
                crubit_support_path_format.clone(),
                "rs_std/lossy_formatter_for_bindings.h".into(),
            ));
            internal_includes.insert(CcInclude::SupportLibHeader(
                crubit_support_path_format.clone(),
                "internal/fmt.h".into(),
            ));
        }
    }

    for e in ir.enums() {
        if ir.target_crubit_features(&e.owning_target).contains(crubit_feature::CrubitFeature::Fmt)
            && e.detected_formatter
        {
            internal_includes.insert(CcInclude::SupportLibHeader(
                crubit_support_path_format.clone(),
                "rs_std/lossy_formatter_for_bindings.h".into(),
            ));
            internal_includes.insert(CcInclude::SupportLibHeader(
                crubit_support_path_format.clone(),
                "internal/fmt.h".into(),
            ));
        }
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
    db: &BindingsGenerator,
) -> Result<CrubitAbiType> {
    let rust_type = rs_name
        .parse()
        .map_err(|e| anyhow!("Failed to parse Rust type `{rs_name}` as a TokenStream: {e}"))?;

    let cpp_type = make_cpp_type_from_item(item, cc_name, db)?;

    Ok(CrubitAbiType::Transmute { rust_type, cpp_type })
}

/// Implementation of `BindingsGenerator::crubit_abi_type`.
fn crubit_abi_type(db: &BindingsGenerator, rs_type_kind: RsTypeKind) -> Result<CrubitAbiType> {
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
                ensure!(
                    db.ir().target_crubit_features(&original_type.owning_target).contains(CrubitFeature::Callables),
                    "Callables require the `callables` feature, but target `{:?}` does not have it enabled.", original_type.owning_target,
                );
                generate_dyn_callable::dyn_callable_crubit_abi_type(db, &dyn_callable)
            }
            BridgeRsTypeKind::C9Co { result_type, .. } => {
                let result_type_tokens = if result_type.is_void() {
                    quote! { () }
                } else {
                    result_type.to_token_stream(db)
                };
                let rust_type_tokens = quote! {
                    ::co::internal_crubit::CoCrubitAbi<#result_type_tokens>
                };

                let result_type_crubit_abi_type = if result_type.is_void() {
                    None
                } else {
                    Some(db.crubit_abi_type(result_type.as_ref().clone())?)
                };

                let rust_expr_tokens = {
                    let consume_result_fn = match &result_type_crubit_abi_type {
                        None => quote! { ::co::internal_crubit::consume_void_result },
                        Some(result_type_crubit_abi_type) => {
                            let result_type_crubit_abi_type_tokens =
                                CrubitAbiTypeToRustTokens(result_type_crubit_abi_type);
                            let result_type_crubit_abi_expr_tokens =
                                CrubitAbiTypeToRustExprTokens(result_type_crubit_abi_type);

                            // This closure takes a function pointer and a context pointer, where
                            // function pointer takes the context and a Crubit ABI bridge buffer
                            // out pointer. This closure uses unstable_return!() to allocate a stack
                            // buffer for the encoded result, call the function pointer with context
                            // and a pointer to the buffer, and then decode the stack buffer into
                            // the native Rust value.
                            quote! {
                                |consume_result_into_buffer: ::co::internal_crubit::ConsumeResultIntoBufferFn,
                                 context: *mut ::core::ffi::c_void| -> #result_type_tokens {
                                    ::bridge_rust::unstable_return!(@
                                        // Crubit ABI details
                                        #result_type_crubit_abi_expr_tokens,
                                        #result_type_crubit_abi_type_tokens,

                                        // Make C++ encode the C++ result into the buffer.
                                        |buffer: *mut u8| {
                                            (consume_result_into_buffer.unwrap())(
                                                context,
                                                buffer,
                                                <#result_type_crubit_abi_type_tokens as ::bridge_rust::CrubitAbi>::SIZE,
                                            );
                                        }
                                        // unstable_return! handles decoding the result into Rust
                                    )
                                 }
                            }
                        }
                    };
                    quote! {
                        ::co::internal_crubit::CoCrubitAbi::new(#consume_result_fn)
                    }
                };

                let cpp_type_tokens = {
                    let result_type_crubit_abi_type_tokens = match &result_type_crubit_abi_type {
                        None => {
                            // For coroutines that return void, there is no CrubitAbi type. To avoid
                            // needing many C++ types, we'll use `void` as the placeholder since
                            // it's never used on as the CrubitAbi type in this context anyways.
                            quote! { void }
                        }
                        Some(result_type_crubit_abi_type) => {
                            let cpp_tokens = CrubitAbiTypeToCppTokens(result_type_crubit_abi_type);
                            quote! { #cpp_tokens }
                        }
                    };
                    quote! {
                        ::c9::internal::rust::CoCrubitAbi<#result_type_crubit_abi_type_tokens>
                    }
                };

                // The default constructor knows what `start_coroutine` and `destroy_at_initial_suspend`
                // functions to use.
                let cpp_expr_tokens = quote! { #cpp_type_tokens() };

                Ok(CrubitAbiType::C9Co {
                    rust_type_tokens,
                    rust_expr_tokens,
                    cpp_type_tokens,
                    cpp_expr_tokens,
                })
            }
        },
        RsTypeKind::Record { record, crate_path, uniform_repr_template_type, .. } => {
            ensure!(
                record.is_unpin(),
                "Type `{}` must be Rust-movable in order to memcpy through a bridge buffer. See crubit.rs/cpp/classes_and_structs#rust_movable",
                record.cc_name
            );

            let rust_type = if let Some(generic_monomorphization) = uniform_repr_template_type {
                generic_monomorphization.to_token_stream(db)
            } else {
                let rs_name = make_rs_ident(record.rs_name.identifier.as_ref());
                quote! { #crate_path #rs_name }
            };

            // This inlines the logic of code_gen_utils::format_cc_ident and joins the namespace parts,
            // except that it creates an Ident instead of a TokenStream.
            code_gen_utils::check_valid_cc_name(&record.cc_name.identifier)
                .expect("IR should only contain valid C++ types");

            let cpp_type =
                make_cpp_type_from_item(record.as_ref(), record.cc_name.identifier.as_ref(), db)?;

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
    db: &BindingsGenerator,
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
            match param_type.passing_convention() {
                PassingConvention::AbiCompatible => {
                    cpp_type_name::format_cpp_type(param_type, db.ir()).ok()
                }
                PassingConvention::LayoutCompatible => {
                    let param_type_tokens =
                        cpp_type_name::format_cpp_type(param_type, db.ir()).ok()?;
                    Some(quote! { #param_type_tokens* })
                }
                PassingConvention::ComposablyBridged => Some(quote! { unsigned char* }),
                PassingConvention::Ctor => None,
                PassingConvention::OwnedPtr => None,
                PassingConvention::Void => unreachable!("parameter types cannot be void"),
            }
        })
        .collect::<Option<Vec<TokenStream>>>()?;

    let out_param_ident;
    let out_param_type;
    let decl_return_type_tokens;
    match dyn_callable.return_type.passing_convention() {
        PassingConvention::AbiCompatible => {
            out_param_ident = None;
            out_param_type = None;
            decl_return_type_tokens =
                cpp_type_name::format_cpp_type(&dyn_callable.return_type, db.ir()).ok()?;
        }
        PassingConvention::LayoutCompatible => {
            let return_type_tokens =
                cpp_type_name::format_cpp_type(&dyn_callable.return_type, db.ir()).ok()?;
            out_param_ident = Some(format_ident!("out"));
            out_param_type = Some(quote! { #return_type_tokens* });
            decl_return_type_tokens = quote! { void };
        }
        PassingConvention::ComposablyBridged => {
            out_param_ident = Some(format_ident!("out"));
            out_param_type = Some(quote! { unsigned char* });
            decl_return_type_tokens = quote! { void };
        }
        PassingConvention::Ctor => {
            return None;
        }
        PassingConvention::OwnedPtr => {
            return None;
        }
        PassingConvention::Void => {
            out_param_ident = None;
            out_param_type = None;
            decl_return_type_tokens = quote! { void };
        }
    }

    let param_idents = param_idents.iter().chain(out_param_ident.as_ref());
    let param_types = param_types.iter().chain(out_param_type.as_ref());
    let invoker_ident = &dyn_callable.invoker_ident;
    let manager_ident = &dyn_callable.manager_ident;

    Some(quote! {
        extern "C" #decl_return_type_tokens #invoker_ident(
            ::rs_std::internal_dyn_callable::TypeErasedState* state
            #(
                , #param_types #param_idents
            )*
        );
        extern "C" void #manager_ident(
            ::absl::internal_any_invocable::FunctionToCall operation,
            ::absl::internal_any_invocable::TypeErasedState* from,
            ::absl::internal_any_invocable::TypeErasedState* to
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
    db: &BindingsGenerator,
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
            match ty.passing_convention() {
                PassingConvention::AbiCompatible => {
                    Some(ty.to_token_stream(db))
                }
                PassingConvention::LayoutCompatible => {
                    ffi_to_rust_transforms.extend(quote! {
                        let #ident = ::core::ptr::read(#ident);
                    });
                    let ty_tokens = ty.to_token_stream(db);
                    Some(quote! { *mut #ty_tokens })
                }
                PassingConvention::ComposablyBridged => {
                    let crubit_abi_type = db.crubit_abi_type(ty.clone()).ok()?;
                    let crubit_abi_type_expr_tokens = CrubitAbiTypeToRustExprTokens(&crubit_abi_type);
                    ffi_to_rust_transforms.extend(quote! {
                        let #ident = ::bridge_rust::internal::decode(#crubit_abi_type_expr_tokens, #ident);
                    });
                    Some(quote! { *mut ::core::ffi::c_uchar })
                }
                PassingConvention::Ctor => None,
                PassingConvention::OwnedPtr => None,
                PassingConvention::Void => unreachable!("parameter types cannot be void"),
            }
        })
        .collect::<Option<Vec<TokenStream>>>()?;

    let unwrapper = match dyn_callable.fn_trait {
        FnTrait::Fn => quote! { &*f },
        FnTrait::FnMut => quote! { &mut *f },
        FnTrait::FnOnce => {
            // Replace the FnOnce with an empty instance, so it can still be dropped.
            // Since it's a ZST, no allocation will be performed, and it can even be forgotten
            // without worrying about leaks.
            let rust_return_type_fragment = dyn_callable.rust_return_type_fragment(db);
            let param_type_tokens =
                dyn_callable.param_types.iter().map(|param_ty| param_ty.to_token_stream(db));
            quote! {
                // SAFETY: f is guaranteed to be valid for reads and writes, is properly aligned,
                // and points to a properly initialized value of the correct type.
                ::core::ptr::replace(f, ::alloc::boxed::Box::new(
                    |#(_: #param_type_tokens),*| #rust_return_type_fragment {
                        ::core::unreachable!("Called FnOnce after it was moved");
                    }
                ))
            }
        }
    };
    let mut invoke_rust_and_return_to_ffi = quote! {
        (unsafe { #unwrapper })(#(#param_idents),*)
    };

    let return_type_fragment;
    let out_param_ident;
    let out_param_type;
    match dyn_callable.return_type.passing_convention() {
        PassingConvention::AbiCompatible => {
            let ffi_return_type = dyn_callable.return_type.to_token_stream(db);
            return_type_fragment = Some(quote! { -> #ffi_return_type });
            out_param_ident = None;
            out_param_type = None;
        }
        PassingConvention::LayoutCompatible => {
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
        PassingConvention::ComposablyBridged => {
            let crubit_abi_type =
                db.crubit_abi_type(dyn_callable.return_type.as_ref().clone()).ok()?;
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
        }
        PassingConvention::Ctor => {
            return None;
        }
        PassingConvention::OwnedPtr => {
            return None;
        }
        PassingConvention::Void => {
            // Put a semicolon at the end to clarify that we do not return anything.
            invoke_rust_and_return_to_ffi = quote! {
                #invoke_rust_and_return_to_ffi;
            };

            return_type_fragment = None;
            out_param_ident = None;
            out_param_type = None;
        }
    }

    let param_idents = param_idents.iter().chain(out_param_ident.as_ref());
    let param_types_tokens = param_types_tokens.iter().chain(out_param_type.as_ref());
    let dyn_fn_spelling = dyn_callable.dyn_fn_spelling(db);
    let invoker_ident = &dyn_callable.invoker_ident;
    let manager_ident = &dyn_callable.manager_ident;

    Some(quote! {
        #[unsafe(no_mangle)]
        unsafe extern "C" fn #invoker_ident(
            f: *mut ::alloc::boxed::Box<#dyn_fn_spelling>,
            #(
                #param_idents: #param_types_tokens,
            )*
        ) #return_type_fragment {
            #ffi_to_rust_transforms

            #invoke_rust_and_return_to_ffi
        }
        #[unsafe(no_mangle)]
        unsafe extern "C" fn #manager_ident(
            operation: ::dyn_callable_rs::FunctionToCall,
            from: *mut ::alloc::boxed::Box<#dyn_fn_spelling>,
            to: *mut ::alloc::boxed::Box<#dyn_fn_spelling>
        ) {
            ::dyn_callable_rs::manager(operation, from, to);
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
    cc_name: &str,
    db: &BindingsGenerator,
) -> Result<TokenStream> {
    let namespace_qualifier = db.ir().namespace_qualifier(item);
    let namespace_parts = namespace_qualifier.parts().map(|part| make_rs_ident(part));
    let cpp_type = cc_name
        .parse::<TokenStream>()
        .map_err(|e| anyhow!("Failed to parse C++ name `{cc_name}`: {e}"))?;
    Ok(quote! { :: #(#namespace_parts::)* #cpp_type })
}
