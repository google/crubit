// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::rc::Rc;

/// Abstract representation of the type selector for bridge operations on a type.
///
/// In Rust, this could look like `ByBridge<(ByTransmute, ByTransmute)>`, equivalently
/// `ByBridge<ByTransmute, ByTransmute>` in C++.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BridgeSchema {
    /// The value should be passed by value via a memcpy/copy_nonoverlapping.
    ByTransmute,

    /// The value should be passed by bridge.
    ByBridge { parameters: Rc<[Self]> },
}

/// A [`ToTokens`] implementation for [`BridgeSchema`] to format the schema as Rust
/// tokens.
pub struct BridgeSchemaToRustTokens<'a>(pub &'a BridgeSchema);

/// A [`ToTokens`] implementation for [`BridgeSchema`] to format the schema as C++
/// tokens.
pub struct BridgeSchemaToCppTokens<'a>(pub &'a BridgeSchema);

impl ToTokens for BridgeSchemaToRustTokens<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            BridgeSchema::ByTransmute => quote! {::bridge_rust::ByTransmute}.to_tokens(tokens),
            BridgeSchema::ByBridge { parameters } => {
                if let [t] = parameters.as_ref() {
                    let param_tokens = BridgeSchemaToRustTokens(t);
                    quote! {::bridge_rust::ByBridge<#param_tokens>}.to_tokens(tokens)
                } else {
                    let params_tokens = parameters.iter().map(BridgeSchemaToRustTokens);
                    quote! {::bridge_rust::ByBridge<(#(#params_tokens),*)>}.to_tokens(tokens)
                }
            }
        }
    }
}

impl ToTokens for BridgeSchemaToCppTokens<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            BridgeSchema::ByTransmute => quote! {::crubit::ByTransmute}.to_tokens(tokens),
            BridgeSchema::ByBridge { parameters } => {
                let param_tokens = parameters.iter().map(BridgeSchemaToCppTokens);
                quote! {::crubit::ByBridge<#(#param_tokens),*>}.to_tokens(tokens)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn zero_params_by_bridge_test() {
        let schema = BridgeSchema::ByBridge { parameters: Rc::from([]) };

        let rust_tokens = BridgeSchemaToRustTokens(&schema).to_token_stream().to_string();
        expect_eq!(rust_tokens, quote! {::bridge_rust::ByBridge<()>}.to_string());

        let cpp_tokens = BridgeSchemaToCppTokens(&schema).to_token_stream().to_string();
        expect_eq!(cpp_tokens, quote! {::crubit::ByBridge<>}.to_string());
    }

    #[gtest]
    fn one_param_by_bridge_test() {
        let schema = BridgeSchema::ByBridge { parameters: Rc::from([BridgeSchema::ByTransmute]) };

        let rust_tokens = BridgeSchemaToRustTokens(&schema).to_token_stream().to_string();
        expect_eq!(
            rust_tokens,
            quote! {::bridge_rust::ByBridge<::bridge_rust::ByTransmute>}.to_string()
        );

        let cpp_tokens = BridgeSchemaToCppTokens(&schema).to_token_stream().to_string();
        expect_eq!(cpp_tokens, quote! {::crubit::ByBridge<::crubit::ByTransmute>}.to_string());
    }

    #[gtest]
    fn many_params_by_bridge_test() {
        let schema = BridgeSchema::ByBridge {
            parameters: Rc::from([BridgeSchema::ByTransmute, BridgeSchema::ByTransmute]),
        };
        let rust_tokens = BridgeSchemaToRustTokens(&schema).to_token_stream().to_string();
        expect_eq!(
            rust_tokens,
            quote! {::bridge_rust::ByBridge<(::bridge_rust::ByTransmute, ::bridge_rust::ByTransmute)>}
                .to_string()
        );

        let cpp_tokens = BridgeSchemaToCppTokens(&schema).to_token_stream().to_string();
        expect_eq!(
            cpp_tokens,
            quote! {::crubit::ByBridge<::crubit::ByTransmute, ::crubit::ByTransmute>}.to_string()
        );
    }
}
