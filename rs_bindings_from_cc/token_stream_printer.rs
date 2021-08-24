// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use std::fmt::Write;

/// Produces C++ source code out of the token stream.
///
/// Notable features:
/// * quote! cannot produce a single `#` token (that is not immediately followed by `(`, `[`, `{`,
///  or variable interpolation). For cases when we need `#` to be produced in the C++ source code
///  use the placeholder `__HASH_TOKEN__`.
/// * Rust tokenizer ignores newlines as they are not significant for Rust. For C++ they are (for
///  example there needs to be a newline after `#include "foo/bar.h"`). Use the placeholder
///  `__NEWLINE__` to insert a newline character.
pub fn cc_tokens_to_string(tokens: TokenStream) -> Result<String> {
    let mut result = "".to_string();
    let mut first = true;
    for tt in tokens.into_iter() {
        if !first {
            write!(result, " ")?;
        }
        first = false;
        match tt {
            TokenTree::Ident(ref tt) if tt == "__NEWLINE__" => writeln!(result)?,
            TokenTree::Ident(ref tt) if tt == "__HASH_TOKEN__" => write!(result, "#")?,

            _ => write!(result, "{}", tt)?,
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
        assert_eq!(cc_tokens_to_string(token_stream.clone())?, token_stream.to_string());
        Ok(())
    }

    #[test]
    fn test_newline_token() -> Result<()> {
        let token_stream = quote! { a __NEWLINE__ b };
        assert_eq!(cc_tokens_to_string(token_stream.clone())?, "a \n b");
        Ok(())
    }

    #[test]
    fn test_hash_token() -> Result<()> {
        let token_stream = quote! { a __HASH_TOKEN__ b };
        assert_eq!(cc_tokens_to_string(token_stream.clone())?, "a # b");
        Ok(())
    }
}
