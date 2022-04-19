// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{bail, Result};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::{Command, Stdio};

/// Like `tokens_to_string` but also runs the result through rustfmt.
pub fn rs_tokens_to_formatted_string(tokens: TokenStream) -> Result<String> {
    rustfmt(tokens_to_string(tokens)?)
}

/// Produces source code out of the token stream.
///
/// Notable features:
/// * quote! cannot produce a single `#` token (that is not immediately followed
///   by `(`, `[`, `{`, or variable interpolation). For cases when we need `#`
///   to be produced in the C++ source code use the placeholder
///   `__HASH_TOKEN__`.
/// * The Rust tokenizer ignores newlines as they are not significant for Rust.
///   For C++ they are (for example there needs to be a newline after `#include
///   "foo/bar.h"`). We are also using explict newlines for making the generated
///   Rust/C++ source code more readable. Use the placeholder `__NEWLINE__` to
///   insert a newline character.
/// * `TokenStream` cannot encode formatting whitespace, so we use the
///   placeholder `__SPACE__`.
/// * `TokenStream` cannot encode comments, so we use the placeholder
///   `__COMMENT__`, followed by a string literal.
pub fn tokens_to_string(tokens: TokenStream) -> Result<String> {
    let mut result = String::new();
    tokens_to_string_impl(&mut result, tokens)?;
    Ok(result)
}

fn tokens_to_string_impl(result: &mut String, tokens: TokenStream) -> Result<()> {
    let mut it = tokens.into_iter().peekable();
    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Ident(ref tt) if tt == "__NEWLINE__" => writeln!(result)?,
            TokenTree::Ident(ref tt) if tt == "__SPACE__" => write!(result, " ")?,
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
            TokenTree::Group(ref tt) => {
                let (open_delimiter, closed_delimiter) = match tt.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::Brace => ("{ ", " }"),
                    Delimiter::None => ("", ""),
                };
                write!(result, "{}", open_delimiter)?;
                tokens_to_string_impl(result, tt.stream())?;
                write!(result, "{}", closed_delimiter)?;
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
    Ok(())
}

fn is_ident_or_literal(tt: &TokenTree) -> bool {
    match tt {
        TokenTree::Ident(id) => id != "__NEWLINE__" && id != "__SPACE__",
        TokenTree::Literal(_) => true,
        _ => false,
    }
}

fn rustfmt(input: String) -> Result<String> {
    // TODO(forster): This should use rustfmt as a library as soon as b/200503084 is
    // fixed.

    let rustfmt = "third_party/unsupported_toolchains/rust/toolchains/nightly/bin/rustfmt";

    let mut child = Command::new(rustfmt)
        .args(&[
            // TODO(forster): Add a way to specify this as a command line parameter.
            "--config-path=external/rustfmt/rustfmt.toml",
            // We are representing doc comments as attributes in the token stream and use rustfmt
            // to unpack them again.
            "--config=normalize_doc_attributes=true",
            // We don't want rustfmt to reflow C++ doc comments, so we turn off wrapping globally
            // and reflow generated comments manually.
            "--config=wrap_comments=false",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("Failed to spawn rustfmt at '{}'", rustfmt));

    let mut stdin = child.stdin.take().expect("Failed to open rustfmt stdin");
    std::thread::spawn(move || {
        stdin.write_all(input.as_bytes()).expect("Failed to write to rustfmt stdin");
    });
    let output = child.wait_with_output().expect("Failed to read rustfmt stdout");

    if !output.status.success() {
        bail!("rustfmt reported an error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
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
            tokens_to_string(token_stream)?,
            "struct Foo{  }impl Bar for Foo{ fn bar(&self){  } }"
        );
        Ok(())
    }

    #[test]
    fn test_space_idents_and_literals() -> Result<()> {
        let token_stream = quote! { foo 42 bar 23 };
        assert_eq!(tokens_to_string(token_stream)?, "foo 42 bar 23");
        Ok(())
    }

    #[test]
    fn test_dont_space_punctuation() -> Result<()> {
        let token_stream = quote! { foo+42+bar+23 };
        assert_eq!(tokens_to_string(token_stream)?, "foo+42+bar+23");
        Ok(())
    }

    #[test]
    fn test_newline_token() -> Result<()> {
        let token_stream = quote! { a __NEWLINE__ b };
        assert_eq!(tokens_to_string(token_stream)?, "a\nb");
        Ok(())
    }

    #[test]
    fn test_space_token() -> Result<()> {
        let token_stream = quote! { a __SPACE__ = __SPACE__ b };
        assert_eq!(tokens_to_string(token_stream)?, "a = b");
        Ok(())
    }

    #[test]
    fn test_redundant_space_token() -> Result<()> {
        let token_stream = quote! { a __SPACE__ b };
        assert_eq!(tokens_to_string(token_stream)?, "a b");
        Ok(())
    }

    #[test]
    fn test_hash_token() -> Result<()> {
        let token_stream = quote! { a __HASH_TOKEN__ b };
        assert_eq!(tokens_to_string(token_stream)?, "a #b");
        Ok(())
    }

    #[test]
    fn test_include_standard_header() -> Result<()> {
        let token_stream = quote! { __HASH_TOKEN__ include <cstddef> };
        assert_eq!(tokens_to_string(token_stream)?, "#include<cstddef>");
        Ok(())
    }

    #[test]
    fn test_comments() -> Result<()> {
        let token_stream = quote! { __COMMENT__ "line1\nline2" };
        assert_eq!(tokens_to_string(token_stream)?, "// line1\n// line2\n");
        Ok(())
    }

    #[test]
    fn test_invalid_comment() -> Result<()> {
        assert!(tokens_to_string(quote! { __COMMENT__ }).is_err());
        assert!(tokens_to_string(quote! { __COMMENT__ ident }).is_err());
        Ok(())
    }

    #[test]
    fn test_doc_comment() -> Result<()> {
        // token_stream_printer (and rustfmt) don't put a space between /// and the doc
        // comment, if the space is desired, it has to appear in the annotation.
        assert_eq!(
            rs_tokens_to_formatted_string(quote! { #[doc = "hello"] struct X {} })?,
            "///hello\nstruct X {}\n"
        );
        assert_eq!(
            rs_tokens_to_formatted_string(quote! { #[doc = "hello\nworld"] struct X {} })?,
            "///hello\n///world\nstruct X {}\n"
        );
        Ok(())
    }

    #[test]
    fn test_doc_comment_leading_spaces() -> Result<()> {
        assert_eq!(
            rs_tokens_to_formatted_string(quote! { #[doc = " hello"] struct X {} })?,
            "/// hello\nstruct X {}\n"
        );
        assert_eq!(
            rs_tokens_to_formatted_string(quote! { #[doc = " hello\n world"] struct X {} })?,
            "/// hello\n/// world\nstruct X {}\n"
        );
        Ok(())
    }

    #[test]
    fn test_special_tokens_in_groups() -> Result<()> {
        assert_eq!(tokens_to_string(quote! {{ a __NEWLINE__ b }})?, "{ a\nb }");
        assert_eq!(tokens_to_string(quote! {{ a __SPACE__ b }})?, "{ a b }");
        assert_eq!(tokens_to_string(quote! {(a __COMMENT__ "b")})?, "(a // b\n)");
        assert_eq!(tokens_to_string(quote! {[__HASH_TOKEN__ a]})?, "[#a]");
        Ok(())
    }
}
