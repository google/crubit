// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Helper crate for spelling out Crubit ABI types in Rust and C++ code.

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use std::rc::Rc;

/// A type path without type arguments in Rust or C++, e.g. `crate::Struct`, `::crubit::StructAbi`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FullyQualifiedPath {
    pub start_with_colon2: bool,
    pub parts: Rc<[Ident]>,
}

impl FullyQualifiedPath {
    /// Parses a fully qualified path from a string, e.g. `::crubit::StructAbi`.
    /// Note that type arguments are not supported here, and are instead stored in the
    /// `CrubitAbiType::Type` enum variant.
    pub fn new(p: &str) -> FullyQualifiedPath {
        let start_with_colon2 = p.starts_with("::");
        let parts = p
            .split("::")
            .skip(start_with_colon2 as usize)
            .map(|part| Ident::new(part, Span::call_site()))
            .collect();
        FullyQualifiedPath { start_with_colon2, parts }
    }
}

impl ToTokens for FullyQualifiedPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.start_with_colon2 {
            quote! { :: }.to_tokens(tokens);
        }
        let parts = self.parts.as_ref();
        quote! { #(#parts)::* }.to_tokens(tokens);
    }
}

/// Abstract representation of the type selector for bridge operations on a type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CrubitAbiType {
    SignedChar,
    UnsignedChar,
    UnsignedShort,
    UnsignedInt,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Pair(Rc<CrubitAbiType>, Rc<CrubitAbiType>),
    StdString {
        /// If true, use `crate::std::BoxedCppStringAbi` instead of
        /// `::cc_std::std::BoxedCppStringAbi` in Rust tokens.
        in_cc_std: bool,
    },
    Type {
        rust_abi_path: FullyQualifiedPath,
        cpp_abi_path: FullyQualifiedPath,
        type_args: Rc<[CrubitAbiType]>,
    },
}

impl CrubitAbiType {
    pub fn new(rust_abi_path: &str, cpp_abi_path: &str) -> Self {
        Self::Type {
            rust_abi_path: FullyQualifiedPath::new(rust_abi_path),
            cpp_abi_path: FullyQualifiedPath::new(cpp_abi_path),
            type_args: Rc::from([]),
        }
    }
}

/// A [`ToTokens`] implementation for [`CrubitAbiType`] to format the schema as Rust
/// tokens.
pub struct CrubitAbiTypeToRustTokens<'a>(pub &'a CrubitAbiType);

/// A [`ToTokens`] implementation for [`CrubitAbiType`] to format the schema as C++
/// tokens.
pub struct CrubitAbiTypeToCppTokens<'a>(pub &'a CrubitAbiType);

impl ToTokens for CrubitAbiTypeToRustTokens<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            CrubitAbiType::SignedChar => quote! { ::core::ffi::c_schar }.to_tokens(tokens),
            CrubitAbiType::UnsignedChar => quote! { ::core::ffi::c_uchar }.to_tokens(tokens),
            CrubitAbiType::UnsignedShort => quote! { ::core::ffi::c_ushort }.to_tokens(tokens),
            CrubitAbiType::UnsignedInt => quote! { ::core::ffi::c_uint }.to_tokens(tokens),
            CrubitAbiType::UnsignedLong => quote! { ::core::ffi::c_ulong }.to_tokens(tokens),
            CrubitAbiType::LongLong => quote! { ::core::ffi::c_longlong }.to_tokens(tokens),
            CrubitAbiType::UnsignedLongLong => {
                quote! { ::core::ffi::c_ulonglong }.to_tokens(tokens)
            }
            CrubitAbiType::Pair(first, second) => {
                let first_tokens = CrubitAbiTypeToRustTokens(first);
                let second_tokens = CrubitAbiTypeToRustTokens(second);
                // std::pair maps to the Rust's TupleAbi
                quote! { ::bridge_rust::TupleAbi<(#first_tokens, #second_tokens)> }
                    .to_tokens(tokens);
            }
            CrubitAbiType::StdString { in_cc_std } => {
                let root = if *in_cc_std {
                    quote! { crate }
                } else {
                    quote! { ::cc_std }
                };
                quote! { #root::std::BoxedCppStringAbi }.to_tokens(tokens)
            }
            CrubitAbiType::Type { rust_abi_path, type_args, .. } => {
                rust_abi_path.to_tokens(tokens);
                if !type_args.is_empty() {
                    let type_args_tokens = type_args.iter().map(CrubitAbiTypeToRustTokens);
                    quote! { < #(#type_args_tokens),* > }.to_tokens(tokens);
                }
            }
        }
    }
}

impl ToTokens for CrubitAbiTypeToCppTokens<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            CrubitAbiType::SignedChar => quote! { signed char }.to_tokens(tokens),
            CrubitAbiType::UnsignedChar => quote! { unsigned char }.to_tokens(tokens),
            CrubitAbiType::UnsignedShort => quote! { unsigned short }.to_tokens(tokens),
            CrubitAbiType::UnsignedInt => quote! { unsigned int }.to_tokens(tokens),
            CrubitAbiType::UnsignedLong => quote! { unsigned long }.to_tokens(tokens),
            CrubitAbiType::LongLong => quote! { long long }.to_tokens(tokens),
            CrubitAbiType::UnsignedLongLong => quote! { unsigned long long }.to_tokens(tokens),
            CrubitAbiType::Pair(first, second) => {
                let first_tokens = CrubitAbiTypeToCppTokens(first);
                let second_tokens = CrubitAbiTypeToCppTokens(second);
                quote! { ::crubit::PairAbi<#first_tokens, #second_tokens> }.to_tokens(tokens);
            }
            CrubitAbiType::StdString { .. } => {
                quote! { ::crubit::BoxedAbi<std::string> }.to_tokens(tokens)
            }
            CrubitAbiType::Type { cpp_abi_path, type_args, .. } => {
                cpp_abi_path.to_tokens(tokens);
                if !type_args.is_empty() {
                    let type_args_tokens = type_args.iter().map(CrubitAbiTypeToCppTokens);
                    quote! { < #(#type_args_tokens),* > }.to_tokens(tokens);
                }
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
        let abi = CrubitAbiType::new("i32", "int32_t");

        let rust_tokens = CrubitAbiTypeToRustTokens(&abi).to_token_stream().to_string();
        expect_eq!(rust_tokens, quote! { i32 }.to_string());

        let cpp_tokens = CrubitAbiTypeToCppTokens(&abi).to_token_stream().to_string();
        expect_eq!(cpp_tokens, quote! { int32_t }.to_string());
    }

    #[gtest]
    fn one_param_by_bridge_test() {
        let abi = CrubitAbiType::Type {
            rust_abi_path: FullyQualifiedPath::new("::bridge_rust::TransmuteAbi"),
            cpp_abi_path: FullyQualifiedPath::new("::crubit::TransmuteAbi"),
            type_args: Rc::from([CrubitAbiType::Type {
                rust_abi_path: FullyQualifiedPath::new("i32"),
                cpp_abi_path: FullyQualifiedPath::new("int32_t"),
                type_args: Rc::from([]),
            }]),
        };

        let rust_tokens = CrubitAbiTypeToRustTokens(&abi).to_token_stream().to_string();
        expect_eq!(rust_tokens, quote! { ::bridge_rust::TransmuteAbi<i32> }.to_string());

        let cpp_tokens = CrubitAbiTypeToCppTokens(&abi).to_token_stream().to_string();
        expect_eq!(cpp_tokens, quote! { ::crubit::TransmuteAbi<int32_t> }.to_string());
    }

    #[gtest]
    fn many_params_by_bridge_test() {
        let abi = CrubitAbiType::Type {
            rust_abi_path: FullyQualifiedPath::new("::bridge_rust::TupleAbi"),
            cpp_abi_path: FullyQualifiedPath::new("::crubit::TupleAbi"),
            type_args: Rc::from([
                CrubitAbiType::Type {
                    rust_abi_path: FullyQualifiedPath::new("::bridge_rust::TransmuteAbi"),
                    cpp_abi_path: FullyQualifiedPath::new("::crubit::TransmuteAbi"),
                    type_args: Rc::from([CrubitAbiType::Type {
                        rust_abi_path: FullyQualifiedPath::new("i32"),
                        cpp_abi_path: FullyQualifiedPath::new("int32_t"),
                        type_args: Rc::from([]),
                    }]),
                },
                CrubitAbiType::Type {
                    rust_abi_path: FullyQualifiedPath::new("crate::StatusAbi"),
                    cpp_abi_path: FullyQualifiedPath::new("::crubit::absl::StatusAbi"),
                    type_args: Rc::from([]),
                },
            ]),
        };

        let rust_tokens = CrubitAbiTypeToRustTokens(&abi).to_token_stream().to_string();
        expect_eq!(
            rust_tokens,
            quote! {
                ::bridge_rust::TupleAbi<
                    ::bridge_rust::TransmuteAbi<i32>,
                    crate::StatusAbi
                >
            }
            .to_string()
        );

        let cpp_tokens = CrubitAbiTypeToCppTokens(&abi).to_token_stream().to_string();
        expect_eq!(
            cpp_tokens,
            quote! {
                ::crubit::TupleAbi<
                    ::crubit::TransmuteAbi<int32_t>,
                    ::crubit::absl::StatusAbi
                >
            }
            .to_string()
        );
    }
}
