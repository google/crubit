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
#[derive(Clone, Debug)]
pub enum CrubitAbiType {
    SignedChar,
    UnsignedChar,
    UnsignedShort,
    UnsignedInt,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    /// The Crubit ABI of a pointer is just transmuting the pointer
    Ptr {
        is_const: bool,
        /// Is this a Rust slice (*mut [T] / rs_std::SliceRef<T>),
        /// or just a pointer (*mut T / T*).
        is_rust_slice: bool,
        rust_type: TokenStream,
        cpp_type: TokenStream,
    },
    Pair(Rc<CrubitAbiType>, Rc<CrubitAbiType>),
    StdString {
        /// If true, use `crate::...` instead of `::cc_std::...` in Rust tokens.
        in_cc_std: bool,
    },
    Transmute {
        rust_type: FullyQualifiedPath,
        cpp_type: TokenStream,
    },
    /// A proto message type. This is a special case of CrubitAbiType::Type, where the Rust type is
    /// a ::foo_proto::ProtoMessageRustBridge<M>, and the C++ type is a
    /// ::crubit::BoxedAbi<::foo_proto::Message>.
    ///
    /// Importantly, constructing instances of these types is notably different from other
    /// CrubitAbiType::Types, since ::crubit::BoxedAbi<T> doesn't take a T argument, it's just an
    /// empty record. On the other hand, CrubitAbiType::Type will always take an Abi object for each
    /// type argument.
    ProtoMessage {
        /// `ProtoMessageRustBridge`, with the correct module path.
        /// rust ::foo_proto::ProtoMessageRustBridge
        proto_message_rust_bridge: FullyQualifiedPath,
        /// rust ::foo_proto::Message
        rust_proto_path: FullyQualifiedPath,
        /// cpp foo::Message
        cpp_proto_path: FullyQualifiedPath,
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

    pub fn option(inner: Self) -> Self {
        CrubitAbiType::Type {
            rust_abi_path: FullyQualifiedPath::new("::bridge_rust::OptionAbi"),
            cpp_abi_path: FullyQualifiedPath::new("::crubit::OptionAbi"),
            type_args: Rc::from([inner]),
        }
    }

    pub fn transmute(rust_type: &str, cpp_type: &str) -> Self {
        CrubitAbiType::Transmute {
            rust_type: FullyQualifiedPath::new(rust_type),
            cpp_type: cpp_type.parse().unwrap_or_else(|e| {
                panic!("Failed to parse C++ type `{cpp_type}` as a TokenStream: {e}")
            }),
        }
    }
}

/// A [`ToTokens`] implementation for [`CrubitAbiType`] to format the schema as Rust
/// tokens.
pub struct CrubitAbiTypeToRustTokens<'a>(pub &'a CrubitAbiType);

/// A [`ToTokens`] implementation for [`CrubitAbiType`] to construct an instance of the type.
pub struct CrubitAbiTypeToRustExprTokens<'a>(pub &'a CrubitAbiType);

/// A [`ToTokens`] implementation for [`CrubitAbiType`] to format the schema as C++
/// tokens.
pub struct CrubitAbiTypeToCppTokens<'a>(pub &'a CrubitAbiType);

impl ToTokens for CrubitAbiTypeToRustTokens<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            CrubitAbiType::SignedChar => {
                quote! { ::bridge_rust::TransmuteAbi<::core::ffi::c_schar> }.to_tokens(tokens)
            }
            CrubitAbiType::UnsignedChar => {
                quote! { ::bridge_rust::TransmuteAbi<::core::ffi::c_uchar> }.to_tokens(tokens)
            }
            CrubitAbiType::UnsignedShort => {
                quote! { ::bridge_rust::TransmuteAbi<::core::ffi::c_ushort> }.to_tokens(tokens)
            }
            CrubitAbiType::UnsignedInt => {
                quote! { ::bridge_rust::TransmuteAbi<::core::ffi::c_uint> }.to_tokens(tokens)
            }
            CrubitAbiType::UnsignedLong => {
                quote! { ::bridge_rust::TransmuteAbi<::core::ffi::c_ulong> }.to_tokens(tokens)
            }
            CrubitAbiType::LongLong => {
                quote! { ::bridge_rust::TransmuteAbi<::core::ffi::c_longlong> }.to_tokens(tokens)
            }
            CrubitAbiType::UnsignedLongLong => {
                quote! { ::bridge_rust::TransmuteAbi<::core::ffi::c_ulonglong> }.to_tokens(tokens)
            }
            CrubitAbiType::Ptr { is_const, is_rust_slice, rust_type, .. } => {
                let mut ty = rust_type.clone();
                if *is_rust_slice {
                    ty = quote! { [#ty] };
                }
                if *is_const {
                    ty = quote! { *const #ty };
                } else {
                    ty = quote! { *mut #ty };
                }
                quote! { ::bridge_rust::TransmuteAbi<#ty> }.to_tokens(tokens);
            }
            CrubitAbiType::Pair(first, second) => {
                let first_tokens = Self(first);
                let second_tokens = Self(second);
                quote! { (#first_tokens, #second_tokens) }.to_tokens(tokens);
            }
            CrubitAbiType::StdString { in_cc_std } => {
                let root = if *in_cc_std {
                    quote! { crate }
                } else {
                    quote! { ::cc_std }
                };
                quote! { #root::std::BoxedCppStringAbi }.to_tokens(tokens)
            }
            CrubitAbiType::Transmute { rust_type, .. } => {
                quote! { ::bridge_rust::TransmuteAbi<#rust_type> }.to_tokens(tokens);
            }
            CrubitAbiType::ProtoMessage { proto_message_rust_bridge, rust_proto_path, .. } => {
                quote! { #proto_message_rust_bridge<#rust_proto_path> }.to_tokens(tokens);
            }
            CrubitAbiType::Type { rust_abi_path, type_args, .. } => {
                rust_abi_path.to_tokens(tokens);
                if !type_args.is_empty() {
                    let type_args_tokens = type_args.iter().map(Self);
                    quote! { < #(#type_args_tokens),* > }.to_tokens(tokens);
                }
            }
        }
    }
}

impl ToTokens for CrubitAbiTypeToRustExprTokens<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            CrubitAbiType::SignedChar
            | CrubitAbiType::UnsignedChar
            | CrubitAbiType::UnsignedShort
            | CrubitAbiType::UnsignedInt
            | CrubitAbiType::UnsignedLong
            | CrubitAbiType::LongLong
            | CrubitAbiType::UnsignedLongLong
            | CrubitAbiType::Ptr { .. } => {
                quote! { ::bridge_rust::transmute_abi() }.to_tokens(tokens);
            }
            CrubitAbiType::Pair(first, second) => {
                let first_tokens = Self(first);
                let second_tokens = Self(second);
                quote! { (#first_tokens, #second_tokens) }.to_tokens(tokens);
            }
            CrubitAbiType::StdString { in_cc_std } => {
                let root = if *in_cc_std {
                    quote! { crate }
                } else {
                    quote! { ::cc_std }
                };
                quote! { #root::std::BoxedCppStringAbi }.to_tokens(tokens)
            }
            CrubitAbiType::Transmute { .. } => {
                quote! { ::bridge_rust::transmute_abi() }.to_tokens(tokens);
            }
            CrubitAbiType::ProtoMessage { proto_message_rust_bridge, .. } => {
                quote! { #proto_message_rust_bridge(::core::marker::PhantomData) }
                    .to_tokens(tokens);
            }
            CrubitAbiType::Type { rust_abi_path, type_args, .. } => {
                rust_abi_path.to_tokens(tokens);
                if !type_args.is_empty() {
                    let type_args_tokens = type_args.iter().map(Self);
                    // We expect that the user's type is a tuple struct with public fields.
                    quote! { ( #(#type_args_tokens),* ) }.to_tokens(tokens);
                }
            }
        }
    }
}

impl ToTokens for CrubitAbiTypeToCppTokens<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            CrubitAbiType::SignedChar => {
                quote! { ::crubit::TransmuteAbi<signed char> }.to_tokens(tokens)
            }
            CrubitAbiType::UnsignedChar => {
                quote! { ::crubit::TransmuteAbi<unsigned char> }.to_tokens(tokens)
            }
            CrubitAbiType::UnsignedShort => {
                quote! { ::crubit::TransmuteAbi<unsigned short> }.to_tokens(tokens)
            }
            CrubitAbiType::UnsignedInt => {
                quote! { ::crubit::TransmuteAbi<unsigned int> }.to_tokens(tokens)
            }
            CrubitAbiType::UnsignedLong => {
                quote! { ::crubit::TransmuteAbi<unsigned long> }.to_tokens(tokens)
            }
            CrubitAbiType::LongLong => {
                quote! { ::crubit::TransmuteAbi<long long> }.to_tokens(tokens)
            }
            CrubitAbiType::UnsignedLongLong => {
                quote! { ::crubit::TransmuteAbi<unsigned long long> }.to_tokens(tokens)
            }
            CrubitAbiType::Ptr { is_const, is_rust_slice, cpp_type, .. } => {
                let mut ty = cpp_type.clone();
                if *is_const {
                    ty = quote! { const #ty };
                }
                if *is_rust_slice {
                    ty = quote! { ::rs_std::SliceRef<#ty> };
                } else {
                    ty = quote! { #ty * };
                }
                quote! { ::crubit::TransmuteAbi<#ty> }.to_tokens(tokens);
            }
            CrubitAbiType::Pair(first, second) => {
                let first_tokens = Self(first);
                let second_tokens = Self(second);
                quote! { ::crubit::PairAbi<#first_tokens, #second_tokens> }.to_tokens(tokens);
            }
            CrubitAbiType::StdString { .. } => {
                quote! { ::crubit::BoxedAbi<std::string> }.to_tokens(tokens)
            }
            CrubitAbiType::Transmute { cpp_type, .. } => {
                quote! { ::crubit::TransmuteAbi<#cpp_type> }.to_tokens(tokens);
            }
            CrubitAbiType::ProtoMessage { cpp_proto_path, .. } => {
                quote! { ::crubit::BoxedAbi<#cpp_proto_path> }.to_tokens(tokens);
            }
            CrubitAbiType::Type { cpp_abi_path, type_args, .. } => {
                cpp_abi_path.to_tokens(tokens);
                if !type_args.is_empty() {
                    let type_args_tokens = type_args.iter().map(Self);
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
        let abi = CrubitAbiType::transmute("i32", "int32_t");

        let rust_tokens = CrubitAbiTypeToRustTokens(&abi).to_token_stream().to_string();
        expect_eq!(rust_tokens, quote! { ::bridge_rust::TransmuteAbi<i32> }.to_string());

        let cpp_tokens = CrubitAbiTypeToCppTokens(&abi).to_token_stream().to_string();
        expect_eq!(cpp_tokens, quote! { ::crubit::TransmuteAbi<int32_t> }.to_string());
    }

    #[gtest]
    fn many_params_by_bridge_test() {
        let abi = CrubitAbiType::Pair(
            Rc::new(CrubitAbiType::transmute("i32", "int32_t")),
            Rc::new(CrubitAbiType::Type {
                rust_abi_path: FullyQualifiedPath::new("crate::StatusAbi"),
                cpp_abi_path: FullyQualifiedPath::new("::crubit::absl::StatusAbi"),
                type_args: Rc::from([]),
            }),
        );

        let rust_tokens = CrubitAbiTypeToRustTokens(&abi).to_token_stream().to_string();
        expect_eq!(
            rust_tokens,
            quote! { (::bridge_rust::TransmuteAbi<i32>, crate::StatusAbi) }.to_string()
        );

        let cpp_tokens = CrubitAbiTypeToCppTokens(&abi).to_token_stream().to_string();
        expect_eq!(
            cpp_tokens,
            quote! {
                ::crubit::PairAbi<
                    ::crubit::TransmuteAbi<int32_t>,
                    ::crubit::absl::StatusAbi
                >
            }
            .to_string()
        );
    }
}
