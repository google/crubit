// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use once_cell::sync::Lazy;
use proc_macro2::TokenStream;
use std::collections::HashSet;

// TODO(lukasza): Consider adding more items into `code_gen_utils` (this crate).
// For example, the following items from `src_code_gen.rs` will be most likely
// reused from `cc_bindings_from_rs`:
// - `make_rs_ident`
// - `NamespaceQualifier`

/// Formats a C++ identifier. Panics when `ident` is a C++ reserved keyword.
pub fn format_cc_ident(ident: &str) -> TokenStream {
    // C++ doesn't have an equivalent of
    // https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html and therefore
    // we panic when `ident` is a C++ reserved keyword.
    assert!(
        !RESERVED_CC_KEYWORDS.contains(ident),
        "The following reserved keyword can't be used as a C++ identifier: {}",
        ident
    );

    ident.parse().unwrap()
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
        assert_cc_matches!(format_cc_ident("foo"), quote! { foo });
    }

    #[test]
    fn test_format_cc_ident_reserved_rust_keyword() {
        assert_cc_matches!(format_cc_ident("impl"), quote! { impl });
    }

    #[test]
    #[should_panic(expected = "can't be used as a C++ identifier: reinterpret_cast")]
    fn test_format_cc_ident_reserved_cc_keyword() {
        format_cc_ident("reinterpret_cast");
    }
}
