// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn cc_import(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as cc_import_internal::CcImportMacroInput);
    input
        .expand_imports()
        .unwrap_or_else(|errors| errors.into_iter().map(|e| e.into_compile_error()).collect())
        .into()
}
