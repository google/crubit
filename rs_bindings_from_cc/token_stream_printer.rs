// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::bail;
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
///   or variable interpolation). For cases when we need `#` to be produced in the C++ source code
///   use the placeholder `__HASH_TOKEN__`.
/// * The Rust tokenizer ignores newlines as they are not significant for Rust. For C++ they are
///   (for example there needs to be a newline after `#include "foo/bar.h"`). We are also using
///   explict newlines for making the generated Rust/C++ source code more readable. Use the
///   placeholder `__NEWLINE__` to insert a newline character.
/// * `TokenStream` cannot encode comments, so we use the placeholder `__COMMENT__`, followed by a
///   string literal.
pub fn tokens_to_string(tokens: TokenStream) -> Result<String> {
    let mut result = String::new();
    let mut it = tokens.into_iter().peekable();
    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Ident(ref tt) if tt == "__NEWLINE__" => writeln!(result)?,
            TokenTree::Ident(ref tt) if tt == "__HASH_TOKEN__" => write!(result, "#")?,

            TokenTree::Ident(ref tt) if tt == "__COMMENT__" => {
                if let Some(TokenTree::Literal(lit)) = it.next() {
                    writeln!(
                        result,
                        "// {}",
                        lit.to_string().trim_matches('"').replace("\\n", "\n// ")
                    )?;
                } else {
                    bail!("__COMMENT__ must be followed by a literal")
                }
            }

            _ => {
                write!(result, "{}", tt)?;

                // Insert spaces between tokens only when they are needed to separate
                // identifiers or literals from each other.
                if is_ident_or_literal(&tt)
                    && matches!(it.peek(), Some(tt_next) if is_ident_or_literal(tt_next))
                {
                    write!(result, " ")?;
                }
            }
        }
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
        assert_eq!(tokens_to_string(token_stream.clone())?, "a \nb");
        Ok(())
    }

    #[test]
    fn test_hash_token() -> Result<()> {
        let token_stream = quote! { a __HASH_TOKEN__ b };
        assert_eq!(tokens_to_string(token_stream.clone())?, "a #b");
        Ok(())
    }

    #[test]
    fn test_include_standard_header() -> Result<()> {
        let token_stream = quote! { __HASH_TOKEN__ include <cstddef> };
        assert_eq!(tokens_to_string(token_stream.clone())?, "#include<cstddef>");
        Ok(())
    }

    #[test]
    fn test_comments() -> Result<()> {
        let token_stream = quote! { __COMMENT__ "line1\nline2" };
        assert_eq!(tokens_to_string(token_stream.clone())?, "// line1\n// line2\n");
        Ok(())
    }

    #[test]
    fn test_invalid_comment() -> Result<()> {
        assert!(tokens_to_string(quote! { __COMMENT__ }).is_err());
        assert!(tokens_to_string(quote! { __COMMENT__ ident }).is_err());
        Ok(())
    }
}
