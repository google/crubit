// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
#[proc_macro]
pub fn inline_cpp(_input: TokenStream) -> TokenStream {
    let span = proc_macro::Span::call_site();
    let file = span.file();
    let line = span.line();
    let col = span.column();

    let target = std::env::var("CRUBIT_TARGET").unwrap_or_default();
    let name_str = inline_cpp_utils::compute_thunk_name(&target, &file, line, col);
    let thunk_name = quote::format_ident!("{}", name_str);

    let expanded = quote! {
        {
            unsafe extern "C" {
                fn #thunk_name();
            }
            unsafe { #thunk_name() }
        }
    };

    TokenStream::from(expanded)
}
