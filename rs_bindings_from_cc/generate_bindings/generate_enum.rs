// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Generates Rust bindings for C++ enums.

use arc_anyhow::Result;
use code_gen_utils::{expect_format_cc_ident, make_rs_ident};
use database::code_snippet::{ApiSnippets, Feature};
use database::BindingsGenerator;
use ir::Enum;
use proc_macro2::Literal;
use quote::{quote, ToTokens};
use std::rc::Rc;

/// Implementation of `BindingsGenerator::generate_enum`.
pub fn generate_enum(db: &dyn BindingsGenerator, enum_: Rc<Enum>) -> Result<ApiSnippets> {
    let ident = expect_format_cc_ident(&enum_.cc_name.identifier);
    let namespace_qualifier = db.ir().namespace_qualifier(&enum_).format_for_cc()?;
    let fully_qualified_cc_name = quote! { #namespace_qualifier #ident }.to_string();
    let name = make_rs_ident(&enum_.rs_name.identifier);
    let underlying_type = db.rs_type_kind(enum_.underlying_type.clone())?;
    let enumerators = enum_.enumerators.iter().flatten().map(|enumerator| {
        if let Some(unknown_attr) = &enumerator.unknown_attr {
            let comment = format!(
                "Omitting bindings for {ident}\nreason: unknown attribute(s): {unknown_attr}",
                ident = &enumerator.identifier.identifier
            );
            return quote! {
                __COMMENT__ #comment
            };
        }
        let ident = make_rs_ident(&enumerator.identifier.identifier);
        let value = if underlying_type.is_bool() {
            if enumerator.value.wrapped_value == 0 {
                quote! {false}
            } else {
                quote! {true}
            }
        } else {
            if enumerator.value.is_negative {
                Literal::i64_unsuffixed(enumerator.value.wrapped_value as i64).into_token_stream()
            } else {
                Literal::u64_unsuffixed(enumerator.value.wrapped_value).into_token_stream()
            }
        };
        quote! {pub const #ident: #name = #name(#value);}
    });
    let underlying_type_tokens = underlying_type.to_token_stream(db);

    let annotation = format!("CRUBIT_ANNOTATE: cpp_type={fully_qualified_cc_name}");
    let item = quote! {
        #[repr(transparent)]
        #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
        #[doc=#annotation]
        pub struct #name(#underlying_type_tokens);
        impl #name {
            #(#enumerators)*
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
    };
    Ok(ApiSnippets {
        main_api: item,
        features: Feature::register_tool.into(),
        ..Default::default()
    })
}
