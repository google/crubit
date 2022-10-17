// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{anyhow, ensure, Result};
use once_cell::sync::Lazy;
use proc_macro2::TokenStream;
use std::collections::HashSet;

// TODO(lukasza): Consider adding more items into `code_gen_utils` (this crate).
// For example, the following items from `src_code_gen.rs` will be most likely
// reused from `cc_bindings_from_rs`:
// - `make_rs_ident`
// - `NamespaceQualifier`

/// Formats a C++ identifier. Panics when `ident` is a C++ reserved keyword.
pub fn format_cc_ident(ident: &str) -> Result<TokenStream> {
    // C++ doesn't have an equivalent of
    // https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html and therefore
    // an error is returned when `ident` is a C++ reserved keyword.
    ensure!(
        !RESERVED_CC_KEYWORDS.contains(ident),
        "`{}` is a C++ reserved keyword and can't be used as a C++ identifier",
        ident
    );

    ident.parse().map_err(
        // Explicitly mapping the error via `anyhow!`, because `LexError` is not `Sync`
        // (required for `anyhow::Error` to implement `From<LexError>`) and
        // therefore we can't just use `?`.
        |lex_error| anyhow!("Can't format `{ident}` as a C++ identifier: {lex_error}"),
    )
}

static RESERVED_CC_KEYWORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    // `RESERVED_CC_KEYWORDS` are based on https://en.cppreference.com/w/cpp/keyword
    [
        "alignas",
        "alignof",
        "and",
        "and_eq",
        "asm",
        "atomic_cancel",
        "atomic_commit",
        "atomic_noexcept",
        "auto",
        "bitand",
        "bitor",
        "bool",
        "break",
        "case",
        "catch",
        "char",
        "char8_t",
        "char16_t",
        "char32_t",
        "class",
        "compl",
        "concept",
        "const",
        "consteval",
        "constexpr",
        "constinit",
        "const_cast",
        "continue",
        "co_await",
        "co_return",
        "co_yield",
        "decltype",
        "default",
        "delete",
        "do",
        "double",
        "dynamic_cast",
        "else",
        "enum",
        "explicit",
        "export",
        "extern",
        "false",
        "float",
        "for",
        "friend",
        "goto",
        "if",
        "inline",
        "int",
        "long",
        "mutable",
        "namespace",
        "new",
        "noexcept",
        "not",
        "not_eq",
        "nullptr",
        "operator",
        "or",
        "or_eq",
        "private",
        "protected",
        "public",
        "reflexpr",
        "register",
        "reinterpret_cast",
        "requires",
        "return",
        "short",
        "signed",
        "sizeof",
        "static",
        "static_assert",
        "static_cast",
        "struct",
        "switch",
        "synchronized",
        "template",
        "this",
        "thread_local",
        "throw",
        "true",
        "try",
        "typedef",
        "typeid",
        "typename",
        "union",
        "unsigned",
        "using",
        "virtual",
        "void",
        "volatile",
        "wchar_t",
        "while",
        "xor",
        "xor_eq",
    ]
    .into_iter()
    .collect()
});

#[cfg(test)]
pub mod tests {
    use super::*;
    use quote::quote;
    use token_stream_matchers::assert_cc_matches;

    #[test]
    fn test_format_cc_ident_basic() {
        assert_cc_matches!(
            format_cc_ident("foo").expect("No errors expected in this test"),
            quote! { foo }
        );
    }

    #[test]
    fn test_format_cc_ident_reserved_rust_keyword() {
        assert_cc_matches!(
            format_cc_ident("impl").expect("No errors expected in this test"),
            quote! { impl }
        );
    }

    #[test]
    fn test_format_cc_ident_reserved_cc_keyword() {
        let err = format_cc_ident("reinterpret_cast").expect_err("This test expects an error");
        let msg = err.to_string();
        assert!(msg.contains("`reinterpret_cast`"));
        assert!(msg.contains("C++ reserved keyword"));
    }

    #[test]
    fn test_format_cc_ident_unfinished_group() {
        let err = format_cc_ident("(foo") // No closing `)`.
            .expect_err("This test expects an error");
        let msg = err.to_string();
        assert!(msg.contains("Can't format `(foo` as a C++ identifier"));
        assert!(msg.contains("cannot parse"));
    }

    #[test]
    fn test_format_cc_ident_unqualified_identifiers() {
        // https://en.cppreference.com/w/cpp/language/identifiers#Unqualified_identifiers

        // These may appear in `IR::Func::name`.
        assert_cc_matches!(
            format_cc_ident("operator==").expect("No errors expected in this test"),
            quote! { operator== }
        );
        assert_cc_matches!(
            format_cc_ident("operator new").expect("No errors expected in this test"),
            quote! { operator new }
        );

        // This may appear in `IR::Record::cc_name` (although in practice these will
        // be namespace-qualified most of the time).
        assert_cc_matches!(
            format_cc_ident("MyTemplate<int>").expect("No errors expected in this test"),
            quote! { MyTemplate<int> }
        );

        // These forms of unqualified identifiers are not used by Crubit in practice,
        assert_cc_matches!(
            format_cc_ident("~MyClass").expect("No errors expected in this test"),
            quote! { ~MyClass }
        );
        assert_cc_matches!(
            format_cc_ident(r#" operator "" _km "#).expect("No errors expected in this test"),
            quote! { operator "" _km }
        );
    }

    #[test]
    fn test_format_cc_ident_qualified_identifiers() {
        // https://en.cppreference.com/w/cpp/language/identifiers#Qualified_identifiers

        // This may appear in `IR::Record::cc_name`.
        assert_cc_matches!(
            format_cc_ident("std::vector<int>").expect("No errors expected in this test"),
            quote! { std::vector<int> }
        );
    }
}
