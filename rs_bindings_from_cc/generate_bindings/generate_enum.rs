// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Generates Rust bindings for C++ enums.

use arc_anyhow::Result;
use code_gen_utils::{format_cc_ident, make_rs_ident};
use database::code_snippet::{
    integer_constant_to_token_stream, ApiSnippets, DeprecatedAttr, DisplayImpl, GeneratedItem,
    MustUseAttr, Thunk, ThunkImpl,
};
use database::BindingsGenerator;
use generate_comment::{generate_doc_comment, parse_extended_source_loc};
use ir::Enum;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::rc::Rc;

/// Implementation of `BindingsGenerator::generate_enum`.
pub fn generate_enum(db: &BindingsGenerator, enum_: Rc<Enum>) -> Result<ApiSnippets> {
    db.errors().add_category(error_report::Category::Type);
    let ident = format_cc_ident(&enum_.cc_name.identifier)?;
    let namespace_qualifier = db.namespace_qualifier(&enum_).format_for_cc()?;
    let fully_qualified_cc_name = quote! { #namespace_qualifier #ident };
    let name = make_rs_ident(&enum_.rs_name.identifier);
    let underlying_type = db.rs_type_kind(enum_.underlying_type.clone())?;

    let enumerators: TokenStream = enum_
        .enumerators
        .iter()
        .flatten()
        .map(|enumerator| {
            let omitting_bindings_comment = |reason: String| {
                let comment = format!(
                    "Omitting bindings for {ident}\nreason: {reason}",
                    ident = &enumerator.identifier.identifier
                );
                quote! {
                    __COMMENT__ #comment
                }
            };
            if let Some(unknown_attr) = &enumerator.unknown_attr {
                return omitting_bindings_comment(format!("unknown attribute(s): {unknown_attr}"));
            }
            let ident = make_rs_ident(&enumerator.identifier.identifier);
            let value =
                match integer_constant_to_token_stream(db, enumerator.value, &underlying_type) {
                    Ok(value) => value,
                    Err(err) => return omitting_bindings_comment(err.to_string()),
                };
            let deprecated_attr = enumerator.deprecated.clone().map(DeprecatedAttr);
            quote! { #deprecated_attr pub const #ident: #name = #name(#value); }
        })
        .collect();
    let underlying_type_tokens = underlying_type.to_token_stream(db);
    let mut thunks: Vec<Thunk> = vec![];
    let mut cc_details: Vec<ThunkImpl> = vec![];
    let display_impl: TokenStream = if enum_.detected_formatter {
        let fmt_fn_name = make_rs_ident(&format!(
            "__crubit_fmt__{type_name}_{odr_suffix}",
            type_name = enum_.cc_name,
            odr_suffix = enum_.owning_target.convert_to_cc_identifier(),
        ));
        let crate_root_path = db.ir().crate_root_path_tokens();
        let namespace_qualifier = db.namespace_qualifier(&enum_).format_for_rs();
        let qualified_name = {
            quote! { #crate_root_path:: #namespace_qualifier #name }
        };
        thunks.push(Thunk::Fmt { fmt_fn_name: fmt_fn_name.clone(), param_type: qualified_name });
        cc_details.push(ThunkImpl::Fmt {
            fmt_fn_name: fmt_fn_name.clone(),
            param_type: fully_qualified_cc_name.clone(),
        });
        let display_impl = DisplayImpl { type_name: quote! { #name }, fmt_fn_name };
        quote! {
            #display_impl
        }
    } else {
        quote! {}
    };

    let annotation = format!("CRUBIT_ANNOTATE: cpp_type={fully_qualified_cc_name}");
    // TODO(b/494281055): enums don't have doc_comments.
    let doc_comment = generate_doc_comment(
        None,
        None,
        Some(&enum_.source_loc),
        db.is_golden_test(),
        db.kythe_annotations(),
    );
    let capture_tags = if db.kythe_annotations()
        && let Some((file_name, start, end)) = parse_extended_source_loc(&enum_.source_loc)
    {
        quote! { __CAPTURE_TAG__ #file_name #start #end }
    } else if db.kythe_annotations() {
        quote! { __CAPTURE_TAG__ "" "0" "0" }
    } else {
        quote! {}
    };
    let bracketed_enum_name = if db.kythe_annotations() {
        quote! { __CAPTURE_BEGIN__ #name __CAPTURE_END__ }
    } else {
        quote! { #name }
    };
    let deprecated_attr = enum_.deprecated.clone().map(DeprecatedAttr);
    let must_use_attr = enum_.nodiscard.clone().map(MustUseAttr);
    let item = quote! {
        #capture_tags #doc_comment
        #[repr(transparent)]
        #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
        #deprecated_attr
        #must_use_attr
        #[doc=#annotation]
        pub struct #bracketed_enum_name(#underlying_type_tokens);
        impl #name {
            #enumerators
        }
        impl From<#underlying_type_tokens> for #name {
            fn from(value: #underlying_type_tokens) -> #name {
                #name(value)
            }
        }
        impl From<#name> for #underlying_type_tokens {
            fn from(value: #name) -> #underlying_type_tokens {
                value.0
            }
        }
        #display_impl
    };
    Ok(ApiSnippets {
        generated_items: HashMap::from([(enum_.id, GeneratedItem::Enum(item))]),
        thunks,
        cc_details,
        ..Default::default()
    })
}
