// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{bail, Context, Result};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::ffi::{OsStr, OsString};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// TODO(b/231320237): The `RustfmtConfig` struct should be replaced with
// `rustfmt_nightly::Config` once we switch to using rustfmt as a library
// (instead of invoking the `rustfmt` executable).
pub struct RustfmtConfig {
    /// Path to the `rustfmt` executable.
    exe_path: PathBuf,

    /// Cmdline arguments to be passed to the `rustfmt` executable.
    cmdline_args: Vec<OsString>,
}

pub const RUSTFMT_EXE_PATH_FOR_TESTING: &str =
    "rustfmt";

pub const CLANG_FORMAT_EXE_PATH_FOR_TESTING: &str =
    "clang-format";

impl RustfmtConfig {
    /// Creates a config that will invoke `rustfmt` at the given
    /// `rustfmt_exe_path`.  If `rustfmt_config_path` is specified, then a
    /// `rustfmt.toml` file at that path will be used to configure the
    /// formatting details;  otherwise a default formatting will be used.
    pub fn new(rustfmt_exe_path: &Path, rustfmt_config_path: Option<&Path>) -> Self {
        Self {
            exe_path: rustfmt_exe_path.to_path_buf(),
            cmdline_args: match rustfmt_config_path {
                None => Self::default_cmdline_args(),
                Some(path) => Self::cmdline_args_with_custom_config_path(path),
            },
        }
    }

    fn for_testing() -> Self {
        Self {
            exe_path: PathBuf::from(RUSTFMT_EXE_PATH_FOR_TESTING),
            cmdline_args: Self::default_cmdline_args(),
        }
    }

    fn cmdline_args_with_custom_config_path(rustfmt_config_path: &Path) -> Vec<OsString> {
        let mut config_path_arg: OsString = "--config-path=".into();
        config_path_arg.push(rustfmt_config_path);
        Self::append_config_overrides(vec![config_path_arg])
    }

    fn default_cmdline_args() -> Vec<OsString> {
        Self::append_config_overrides(vec!["--edition=2021".into(), "--config=version=Two".into()])
    }

    fn append_config_overrides(mut cmdline_args: Vec<OsString>) -> Vec<OsString> {
        cmdline_args.extend(vec![
            // We are representing doc comments as attributes in the token stream and use rustfmt
            // to unpack them again.
            "--config=normalize_doc_attributes=true".into(),
            // We don't want rustfmt to reflow C++ doc comments, so we turn off wrapping globally
            // and reflow generated comments manually.
            "--config=wrap_comments=false".into(),
        ]);
        cmdline_args
    }
}

/// Like `tokens_to_string` but also runs the result through `rustfmt`.
pub fn rs_tokens_to_formatted_string(
    tokens: TokenStream,
    config: &RustfmtConfig,
) -> Result<String> {
    let tokens_string = tokens_to_string(tokens)?
        // NOTE: This is a terrible hack. `rustfmt` became more strict about appearances of `...`
        // (the `DotDotDot` token) at some point in the past. This is not a precise or general
        // solution, but rewriting this token to a comment produces formattable code in some cases,
        // making test failure messages better.
        .replace("...", "/*...*/");
    let err = format!("Failed to rustfmt the following Rust tokens:\n\n{tokens_string}");
    rustfmt(tokens_string, config).context(err)
}

/// Like `rs_tokens_to_formatted_string`, but always using a Crubit-internal,
/// default rustfmt config.  This should only be called by tests - product code
/// should support custom `rustfmt.toml` and take the path to `rustfmt` binary
/// as a cmdline argument.
pub fn rs_tokens_to_formatted_string_for_tests(input: TokenStream) -> Result<String> {
    rs_tokens_to_formatted_string(input, &RustfmtConfig::for_testing())
}

/// Like `tokens_to_string` but also runs the result through `clang-format`.
pub fn cc_tokens_to_formatted_string(
    tokens: TokenStream,
    clang_format_exe_path: &Path,
) -> Result<String> {
    clang_format(tokens_to_string(tokens)?, clang_format_exe_path)
}

/// Like `cc_tokens_to_formatted_string`, but always using a hardcoded path to
/// where the `clang-format` binary is in Crubit's test environment.  This
/// should only be called by tests - product code should take the path to the
/// `clang-format` binary as a cmdline argument.
pub fn cc_tokens_to_formatted_string_for_tests(tokens: TokenStream) -> Result<String> {
    clang_format(tokens_to_string(tokens)?, Path::new(CLANG_FORMAT_EXE_PATH_FOR_TESTING))
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
pub fn write_unformatted_tokens(
    result: &mut impl std::fmt::Write,
    tokens: TokenStream,
) -> Result<()> {
    let mut it = tokens.into_iter().peekable();
    let mut tt_prev = None;
    while let Some(tt) = it.next() {
        match &tt {
            TokenTree::Ident(tt) if tt == "__NEWLINE__" => writeln!(result)?,
            TokenTree::Ident(tt) if tt == "__SPACE__" => write!(result, " ")?,
            TokenTree::Ident(tt) if tt == "__HASH_TOKEN__" => write!(result, "#")?,

            TokenTree::Ident(tt) if tt == "__COMMENT__" => {
                if let Some(TokenTree::Literal(lit)) = it.next() {
                    writeln!(
                        result,
                        "// {}",
                        lit.to_string()
                            .trim_matches('"')
                            .replace("\\\"", "\"")
                            .replace("\\n", "\n// ")
                    )?;
                } else {
                    bail!("__COMMENT__ must be followed by a literal")
                }
            }
            TokenTree::Ident(tt) if tt == "__LITERALLY__" => {
                if let Some(TokenTree::Literal(lit)) = it.next() {
                    // TokenTree::Literal does not provide a structured way to
                    // get the value out, so we have to use the Display impl to
                    // print the string literal as Rust syntax and unescape it.
                    writeln!(
                        result,
                        "{}",
                        lit.to_string()
                            .trim_matches('"')
                            .replace("\\\"", "\"")
                            .replace("\\\\", "\\")
                    )?;
                } else {
                    bail!("__LITERALLY__ must be followed by a literal")
                }
            }
            TokenTree::Group(tt) => {
                let (open_delimiter, closed_delimiter) = match tt.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::Brace => ("{ ", " }"),
                    Delimiter::None => ("", ""),
                };
                write!(result, "{}", open_delimiter)?;
                write_unformatted_tokens(result, tt.stream())?;
                write!(result, "{}", closed_delimiter)?;
            }
            _ => {
                write!(result, "{}", tt)?;

                // Insert spaces between tokens when they are needed to separate tokens.
                // In particular, `a b` is different than `ab`, and `: ::` is different from
                // `:::`.
                if let Some(tt_next) = it.peek() {
                    if tokens_require_whitespace(tt_prev.as_ref(), &tt, tt_next) {
                        write!(result, " ")?;
                    }
                }
            }
        }
        tt_prev = Some(tt);
    }
    Ok(())
}

/// Writes unformatted tokens into a string.
pub fn tokens_to_string(tokens: TokenStream) -> Result<String> {
    let mut result = String::new();
    write_unformatted_tokens(&mut result, tokens)?;
    Ok(result)
}

/// Returns true if `current` and `next` should have whitespace between them,
/// and false if they should not.
///
/// For example, `a b` is different than `ab`, and `: ::` is different from
/// `:::`.
fn tokens_require_whitespace(
    prev: Option<&TokenTree>,
    current: &TokenTree,
    next: &TokenTree,
) -> bool {
    if is_ident_or_literal(current) && is_ident_or_literal(next) {
        return true;
    }

    // We (currently) only add a space for `:`.
    // A lone `:` always gets a space after it. A lone `::` doesn't.
    // So if the current character is a colon, it gets a space after it if it is not
    // the start or end of a `::`.
    fn get_colon(tt: &TokenTree) -> Option<&proc_macro2::Punct> {
        let TokenTree::Punct(p) = tt else {
            return None;
        };
        if p.as_char() != ':' {
            return None;
        }
        Some(p)
    }

    let Some(current_colon) = get_colon(current) else {
        return false;
    };
    if current_colon.spacing() == proc_macro2::Spacing::Joint {
        // This is the first `:` in `::`
        return false;
    }
    match prev.and_then(get_colon) {
        // this is the second `:` in `::`
        Some(prev_colon) if prev_colon.spacing() == proc_macro2::Spacing::Joint => {
            // a `::` shouldn't have a space after it, generally, but we add one if the next
            // token is a `:` because otherwise it looks awful.
            get_colon(next).is_some()
        }
        // this is a standalone `:`.
        _ => true,
    }
}

fn is_ident_or_literal(tt: &TokenTree) -> bool {
    match tt {
        TokenTree::Ident(id) => id != "__NEWLINE__" && id != "__SPACE__",
        TokenTree::Literal(_) => true,
        _ => false,
    }
}

fn pipe_string_through_process<'a>(
    input: String,
    exe_name: &str,
    exe_path: &Path,
    args: impl IntoIterator<Item = &'a OsStr>,
) -> Result<String> {
    let mut child = Command::new(exe_path)
        .args(args.into_iter())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to spawn {exe_name} at {exe_path:?}: {e}"));

    let mut stdin = child.stdin.take().expect("Failed to open {exe_name} stdin");
    let handle = std::thread::spawn(move || {
        stdin.write_all(input.as_bytes()).expect("Failed to write to {exe_name} stdin");
    });
    let output = child.wait_with_output().expect("Failed to read {exe_name} stdout");

    handle.join().unwrap();

    if !output.status.success() {
        bail!("{exe_name} reported an error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn rustfmt(input: String, config: &RustfmtConfig) -> Result<String> {
    pipe_string_through_process(
        input,
        "rustfmt",
        &config.exe_path,
        config.cmdline_args.iter().map(OsString::as_os_str),
    )
}

fn clang_format(input: String, clang_format_exe_path: &Path) -> Result<String> {
    pipe_string_through_process(
        input,
        "clang-format",
        clang_format_exe_path,
        [OsStr::new("--style=google")],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    use super::Result;
    use quote::quote;
    use tempfile::tempdir;

    #[gtest]
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

    #[gtest]
    fn test_space_idents_and_literals() -> Result<()> {
        let token_stream = quote! { foo 42 bar 23 };
        assert_eq!(tokens_to_string(token_stream)?, "foo 42 bar 23");
        Ok(())
    }

    #[gtest]
    fn test_dont_space_punctuation() -> Result<()> {
        let token_stream = quote! { foo+42+bar+23 };
        assert_eq!(tokens_to_string(token_stream)?, "foo+42+bar+23");
        Ok(())
    }

    /// `foo : ::bar` is valid syntax, but `foo:::bar` is not.
    #[gtest]
    fn test_paamayim_nekudotayim() -> Result<()> {
        assert_eq!(tokens_to_string(quote! { x : :: y })?, "x: ::y");
        // The following variants are not syntactically valid, but good to have working
        // anyway.
        assert_eq!(tokens_to_string(quote! { x : : y })?, "x: : y");
        assert_eq!(tokens_to_string(quote! { x :: :: y })?, "x:: ::y");
        assert_eq!(tokens_to_string(quote! { x :: : y })?, "x:: : y");
        Ok(())
    }

    #[gtest]
    fn test_newline_token() -> Result<()> {
        let token_stream = quote! { a __NEWLINE__ b };
        assert_eq!(tokens_to_string(token_stream)?, "a\nb");
        Ok(())
    }

    #[gtest]
    fn test_space_token() -> Result<()> {
        let token_stream = quote! { a __SPACE__ = __SPACE__ b };
        assert_eq!(tokens_to_string(token_stream)?, "a = b");
        Ok(())
    }

    #[gtest]
    fn test_redundant_space_token() -> Result<()> {
        let token_stream = quote! { a __SPACE__ b };
        assert_eq!(tokens_to_string(token_stream)?, "a b");
        Ok(())
    }

    #[gtest]
    fn test_hash_token() -> Result<()> {
        let token_stream = quote! { a __HASH_TOKEN__ b };
        assert_eq!(tokens_to_string(token_stream)?, "a #b");
        Ok(())
    }

    #[gtest]
    fn test_include_standard_header() -> Result<()> {
        let token_stream = quote! { __HASH_TOKEN__ include <cstddef> };
        assert_eq!(tokens_to_string(token_stream)?, "#include<cstddef>");
        Ok(())
    }

    #[gtest]
    fn test_comments() -> Result<()> {
        let token_stream = quote! { __COMMENT__ "line1\nline2" };
        assert_eq!(tokens_to_string(token_stream)?, "// line1\n// line2\n");
        Ok(())
    }

    #[gtest]
    fn test_invalid_comment() -> Result<()> {
        assert!(tokens_to_string(quote! { __COMMENT__ }).is_err());
        assert!(tokens_to_string(quote! { __COMMENT__ ident }).is_err());
        Ok(())
    }

    #[gtest]
    fn test_doc_comment() -> Result<()> {
        // token_stream_printer (and rustfmt) don't put a space between /// and the doc
        // comment, if the space is desired, it has to appear in the annotation.
        assert_eq!(
            rs_tokens_to_formatted_string_for_tests(quote! { #[doc = "hello"] struct X {} })?,
            "///hello\nstruct X {}\n"
        );
        assert_eq!(
            rs_tokens_to_formatted_string_for_tests(
                quote! { #[doc = "hello\nworld"] struct X {} }
            )?,
            "///hello\n///world\nstruct X {}\n"
        );
        Ok(())
    }

    #[gtest]
    fn test_doc_comment_leading_spaces() -> Result<()> {
        assert_eq!(
            rs_tokens_to_formatted_string_for_tests(quote! { #[doc = " hello"] struct X {} })?,
            "/// hello\nstruct X {}\n"
        );
        assert_eq!(
            rs_tokens_to_formatted_string_for_tests(
                quote! { #[doc = " hello\n world"] struct X {} }
            )?,
            "/// hello\n/// world\nstruct X {}\n"
        );
        Ok(())
    }

    #[gtest]
    fn test_special_tokens_in_groups() -> Result<()> {
        assert_eq!(tokens_to_string(quote! {{ a __NEWLINE__ b }})?, "{ a\nb }");
        assert_eq!(tokens_to_string(quote! {{ a __SPACE__ b }})?, "{ a b }");
        assert_eq!(tokens_to_string(quote! {(a __COMMENT__ "b")})?, "(a // b\n)");
        assert_eq!(tokens_to_string(quote! {[__HASH_TOKEN__ a]})?, "[#a]");
        Ok(())
    }

    #[gtest]
    fn test_rs_tokens_to_formatted_string_for_tests() {
        let input = quote! {
            fn foo() {}
            fn bar() {}
        };
        let output = rs_tokens_to_formatted_string_for_tests(input).unwrap();
        assert_eq!(
            output,
            r#"fn foo() {}
fn bar() {}
"#
        );
    }

    #[gtest]
    fn test_rs_tokens_to_formatted_string() {
        let cfg = RustfmtConfig::new(Path::new(RUSTFMT_EXE_PATH_FOR_TESTING), None);
        let input = quote! {
            fn bar() {}
            fn foo(x: i32, y: i32) -> i32 { x + y }
        };
        let output = rs_tokens_to_formatted_string(input, &cfg).unwrap();
        assert_eq!(
            output,
            r#"fn bar() {}
fn foo(x: i32, y: i32) -> i32 {
    x + y
}
"#
        );
    }

    #[gtest]
    fn test_rs_tokens_to_formatted_string_with_custom_rustfmt_toml() -> Result<()> {
        let tmpdir = tempdir()?;
        let rustfmt_toml_path = tmpdir.path().join("rustfmt-for-tests.toml");
        std::fs::write(
            &rustfmt_toml_path,
            r#" edition = "2021"
                version = "Two"
                fn_args_layout="Vertical" "#,
        )?;
        let cfg =
            RustfmtConfig::new(Path::new(RUSTFMT_EXE_PATH_FOR_TESTING), Some(&rustfmt_toml_path));
        let input = quote! {
            fn bar() {}
            fn foo(x: i32, y: i32) -> i32 { x + y }
        };

        let output = rs_tokens_to_formatted_string(input, &cfg).unwrap();
        assert_eq!(
            output,
            r#"fn bar() {}
fn foo(
    x: i32,
    y: i32,
) -> i32 {
    x + y
}
"#
        );
        Ok(())
    }

    #[gtest]
    fn test_cc_tokens_to_formatted_string_for_tests() {
        let input = quote! {
            namespace ns {
            void foo() {}
            void bar() {}
            }
        };
        let output = cc_tokens_to_formatted_string_for_tests(input).unwrap();
        assert_eq!(
            output,
            r#"namespace ns {
void foo() {}
void bar() {}
}  // namespace ns"#
        );
    }

    #[gtest]
    fn test_quote_in_comment() -> Result<()> {
        assert_eq!(
            tokens_to_string(quote! { a __COMMENT__ "test \" - quote" })?,
            r#"a // test " - quote
"#
        );
        Ok(())
    }
}
