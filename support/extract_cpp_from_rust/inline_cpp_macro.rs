// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
/// The `inline_cpp!` macro embeds an inline C++ function definition directly in
/// Rust source code.
///
/// Crubit extracts the embedded C++ code, compiles it into a C++ helper library,
/// and expands the macro in Rust to a call to the generated bindings thunk.
///
/// # Syntax
/// Accepts a C++ parameter list, a trailing return type, and a compound function body:
///
/// ```ignore
/// inline_cpp! {
///     (const carcinize_test::Point& p) -> int {
///         return p.x;
///     }
/// }
/// ```
///
/// # Example
///
/// ```ignore
/// let add = inline_cpp! {
///     (int a, int b) -> int {
///         return a + b;
///     }
/// };
/// assert_eq!(add(2, 3), 5);
/// ```
#[proc_macro]
pub fn inline_cpp(_input: TokenStream) -> TokenStream {
    let span = proc_macro::Span::call_site();
    let file = span.file();
    let line = span.line();
    let col = span.column();

    let Ok(target) = std::env::var("CRUBIT_TARGET") else {
        // Fallback placeholder for IDE and tooling environments (e.g. rust-analyzer)
        // where CRUBIT_TARGET is unset. Expands to a closure matching the parameter
        // arity of the inline C++ signature so invocations parse as valid callable expressions.
        let param_count = match _input.into_iter().next() {
            Some(proc_macro::TokenTree::Group(group))
                if group.delimiter() == proc_macro::Delimiter::Parenthesis =>
            {
                let stream = group.stream();
                if stream.is_empty() {
                    0
                } else {
                    stream
                        .into_iter()
                        .filter(|tt| match tt {
                            proc_macro::TokenTree::Punct(p) => p.as_char() == ',',
                            _ => false,
                        })
                        .count()
                        + 1
                }
            }
            // Fallback to arity 1 if the input does not begin with a parenthesized
            // parameter list (e.g. empty, malformed, or incomplete editor input).
            _ => 1,
        };
        let wildcard_params = (0..param_count).map(|_| quote! { _ });
        return TokenStream::from(quote! {
            (|#( #wildcard_params ),*| unreachable!())
        });
    };
    let name_str = inline_cpp_utils::compute_thunk_name(&target, &file, line, col);
    let thunk_name = quote::format_ident!("{}", name_str);

    let expanded = quote! {
        inline_cpp_generated_bindings::#thunk_name
    };

    TokenStream::from(expanded)
}

/// The `global_cpp!` macro embeds top-level C++ declarations into the extracted
/// C++ header.
///
/// The macro expands to an empty token stream in Rust, but Crubit extracts its
/// contents at build time and includes them at file scope in the generated C++ header.
///
/// # Syntax
/// Accepts arbitrary top-level C++ declarations, including `#include` directives,
/// struct/class definitions, type aliases, and helper functions:
///
/// ```ignore
/// global_cpp! {
///     #include "third_party/absl/strings/string_view.h"
///
///     struct Point {
///         int x;
///         int y;
///     };
/// }
/// ```
#[proc_macro]
pub fn global_cpp(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
