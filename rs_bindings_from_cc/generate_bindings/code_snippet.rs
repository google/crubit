// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Generate the final bindings, including structures for code snippet, feature
/// gating, etc.
use proc_macro2::{Ident, TokenStream};

use ffi_types::FfiU8SliceBox;
use std::collections::BTreeSet;

/// FFI equivalent of `Bindings`.
#[repr(C)]
pub struct FfiBindings {
    pub rs_api: FfiU8SliceBox,
    pub rs_api_impl: FfiU8SliceBox,
    pub error_report: FfiU8SliceBox,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct GeneratedItem {
    pub item: TokenStream,
    pub thunks: TokenStream,
    // C++ source code for helper functions.
    pub thunk_impls: TokenStream,
    pub assertions: TokenStream,
    pub features: BTreeSet<Ident>,
}

impl From<TokenStream> for GeneratedItem {
    fn from(item: TokenStream) -> Self {
        GeneratedItem { item, ..Default::default() }
    }
}

/// Source code for generated bindings.
pub(crate) struct Bindings {
    // Rust source code.
    pub rs_api: String,
    // C++ source code.
    pub rs_api_impl: String,
}

/// Source code for generated bindings, as tokens.
///
/// This is public within the crate for testing purposes.
pub(crate) struct BindingsTokens {
    // Rust source code.
    pub rs_api: TokenStream,
    // C++ source code.
    pub rs_api_impl: TokenStream,
}
