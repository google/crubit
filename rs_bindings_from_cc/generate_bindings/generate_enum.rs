// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Generate bindings for C++ enum.
use crate::code_snippet::GeneratedItem;
use crate::db::{BindingsGenerator, Database};
use crate::generate_comment::generate_unsupported;
use arc_anyhow::Result;
use code_gen_utils::make_rs_ident;
use ir::{Enum, UnsupportedItem};
use proc_macro2::Literal;
use quote::{quote, ToTokens};
use std::collections::BTreeSet;

pub fn generate_enum(db: &Database, enum_: &Enum) -> Result<GeneratedItem> {
    let ident = crate::format_cc_ident(&enum_.identifier.identifier);
    let namespace_qualifier = db.ir().namespace_qualifier(enum_)?.format_for_cc()?;
    let fully_qualified_cc_name = quote! { #namespace_qualifier #ident }.to_string();
    let name = make_rs_ident(&enum_.identifier.identifier);
    let underlying_type = db.rs_type_kind(enum_.underlying_type.rs_type.clone())?;
    let Some(enumerators) = &enum_.enumerators else {
        return generate_unsupported(
            db,
            &UnsupportedItem::new_with_static_message(
                &db.ir(),
                enum_,
                "b/322391132: Forward-declared (opaque) enums are not supported yet",
            ),
        );
    };
    let enumerators = enumerators.iter().map(|enumerator| {
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

    let item = quote! {
        #[repr(transparent)]
        #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
        #[__crubit::annotate(cpp_type=#fully_qualified_cc_name)]
        pub struct #name(#underlying_type);
        impl #name {
            #(#enumerators)*
        }
        impl From<#underlying_type> for #name {
            fn from(value: #underlying_type) -> #name {
                #name(value)
            }
        }
        impl From<#name> for #underlying_type {
            fn from(value: #name) -> #underlying_type {
                value.0
            }
        }
    };
    Ok(GeneratedItem {
        item,
        features: BTreeSet::from([make_rs_ident("register_tool")]),
        ..Default::default()
    })
}
