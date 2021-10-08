// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use std::fmt::Write;

fn is_ident_or_literal(tt: &TokenTree) -> bool {
    matches!(tt, TokenTree::Ident(_) | TokenTree::Literal(_))
}

/// Produces C++ source code out of the token stream.
///
/// Notable features:
/// * quote! cannot produce a single `#` token (that is not immediately followed by `(`, `[`, `{`,
///  or variable interpolation). For cases when we need `#` to be produced in the C++ source code
///  use the placeholder `__HASH_TOKEN__`.
/// * Rust tokenizer ignores newlines as they are not significant for Rust. For C++ they are (for
///  example there needs to be a newline after `#include "foo/bar.h"`). Use the placeholder
///  `__NEWLINE__` to insert a newline character.
pub fn tokens_to_string(tokens: TokenStream) -> Result<String> {
    let mut result = "".to_string();
    let mut last = None;
    for tt in tokens.into_iter() {
        // Insert spaces between tokens only when they are needed to separate
        // identifiers or literals from each other. We don't currently check for
        // the "special" identifiers __NEWLINE__ or __HASH_TOKEN__, so these are
        // also separated by spaces, but this is harmless.
        if let Some(last_tt) = last {
            if is_ident_or_literal(&last_tt) && is_ident_or_literal(&tt) {
                write!(result, " ")?;
            }
        }

        match tt {
            TokenTree::Ident(ref tt) if tt == "__NEWLINE__" => writeln!(result)?,
            TokenTree::Ident(ref tt) if tt == "__HASH_TOKEN__" => write!(result, "#")?,

            _ => write!(result, "{}", tt)?,
        }

        last = Some(tt);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::Result;
    use quote::quote;

    #[test]
    fn test_simple_token_stream() -> Result<()> {
        let token_stream = quote! {
          struct Foo {}

          impl Bar for Foo {
            fn bar(&self) {}
          }
        };
        assert_eq!(
            tokens_to_string(token_stream.clone())?,
            "struct Foo{ }impl Bar for Foo{ fn bar (& self) { } }"
        );
        Ok(())
    }

    #[test]
    fn test_space_idents_and_literals() -> Result<()> {
        let token_stream = quote! { foo 42 bar 23 };
        assert_eq!(tokens_to_string(token_stream.clone())?, "foo 42 bar 23");
        Ok(())
    }

    #[test]
    fn test_dont_space_punctuation() -> Result<()> {
        let token_stream = quote! { foo+42+bar+23 };
        assert_eq!(tokens_to_string(token_stream.clone())?, "foo+42+bar+23");
        Ok(())
    }

    #[test]
    fn test_newline_token() -> Result<()> {
        let token_stream = quote! { a __NEWLINE__ b };
        assert_eq!(tokens_to_string(token_stream.clone())?, "a \n b");
        Ok(())
    }

    #[test]
    fn test_hash_token() -> Result<()> {
        let token_stream = quote! { a __HASH_TOKEN__ b };
        assert_eq!(tokens_to_string(token_stream.clone())?, "a # b");
        Ok(())
    }

    #[test]
    fn test_include_standard_header() -> Result<()> {
        let token_stream = quote! { __HASH_TOKEN__ include <cstddef> };
        assert_eq!(tokens_to_string(token_stream.clone())?, "# include<cstddef>");
        Ok(())
    }
}
