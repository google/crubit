// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Generates Rust bindings for C++ enums.

use arc_anyhow::Result;
use code_gen_utils::{format_cc_ident, make_rs_ident};
use database::code_snippet::{
    integer_constant_to_token_stream, ApiSnippets, DisplayImpl, Feature, GeneratedItem, Thunk,
    ThunkImpl,
};
use database::BindingsGenerator;
use ir::Enum;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::rc::Rc;

/// Implementation of `BindingsGenerator::generate_enum`.
pub fn generate_enum(db: &BindingsGenerator, enum_: Rc<Enum>) -> Result<ApiSnippets> {
    db.errors().add_category(error_report::Category::Type);
    let ident = format_cc_ident(&enum_.cc_name.identifier)?;
    let namespace_qualifier = db.ir().namespace_qualifier(&enum_).format_for_cc()?;
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
            let value = match integer_constant_to_token_stream(enumerator.value, &underlying_type) {
                Ok(value) => value,
                Err(err) => return omitting_bindings_comment(err.to_string()),
            };
            quote! {pub const #ident: #name = #name(#value);}
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
        let namespace_qualifier = db.ir().namespace_qualifier(&enum_).format_for_rs();
        let qualified_name = {
            quote! { #crate_root_path:: #namespace_qualifier #name }
        };
        thunks.push(Thunk::Fmt { fmt_fn_name: fmt_fn_name.clone(), param_type: qualified_name });
        cc_details.push(ThunkImpl::Fmt {
            fmt_fn_name: fmt_fn_name.clone(),
            param_type: fully_qualified_cc_name.clone(),
        });
        let display_impl = DisplayImpl { type_name: name.clone(), fmt_fn_name };
        quote! {
            #display_impl
        }
    } else {
        quote! {}
    };

    let annotation = format!("CRUBIT_ANNOTATE: cpp_type={fully_qualified_cc_name}");
    let item = quote! {
        #[repr(transparent)]
        #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
        #[doc=#annotation]
        pub struct #name(#underlying_type_tokens);
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
        features: Feature::register_tool.into(),
        thunks,
        cc_details,
        ..Default::default()
    })
}
