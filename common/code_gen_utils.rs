// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use proc_macro2::TokenStream;

// TODO(lukasza): Consider adding more items into `code_gen_utils` (this crate).
// For example, the following items from `src_code_gen.rs` will be most likely
// reused from `cc_bindings_from_rs`:
// - `make_rs_ident`
// - `NamespaceQualifier`

/// Formats a C++ identifier. Does not escape C++ keywords.
pub fn format_cc_ident(ident: &str) -> TokenStream {
    ident.parse().unwrap()
}

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
    fn test_format_cc_ident_reserved_cc_keyword() {
        assert_cc_matches!(format_cc_ident("int"), quote! { int });
    }
}
