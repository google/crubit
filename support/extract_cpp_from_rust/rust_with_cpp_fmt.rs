// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Two-pass formatter for Rust source files containing embedded C++ macro blocks:
//! 1. Formats outer Rust code via `rustfmt`.
//! 2. Extracts embedded C++ macro blocks (`inline_cpp!`, `global_cpp!`, `DO_NOT_SUBMIT_CPP_DECL!`),
//!    formats them via `clang-format`, and splices them back with matched indentation.

use anyhow::{bail, Context, Result};
use clap::Parser;
use parse_inline_cpp_macros::{parse_inline_cpp_macros, MacroKind};
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Additional indentation spaces for C++ code inside a Rust macro block.
const CPP_INDENT_SPACES: usize = 4;

/// Default column limit for Google C++ Style before subtracting indentation.
const GOOGLE_COLUMN_LIMIT: usize = 80;

#[derive(Parser, Debug)]
#[command(
    name = "rust_with_cpp_fmt",
    about = "Two-pass formatter for Rust source files containing embedded C++ macro blocks"
)]
pub struct Args {
    /// Files to format. If empty or "-", reads from stdin and writes to stdout.
    #[arg(value_name = "FILES")]
    files: Vec<PathBuf>,

    /// Overwrite formatted files in place.
    #[arg(short = 'i', long = "inplace", alias = "overwrite")]
    inplace: bool,

    /// Path to rustfmt executable.
    #[arg(long)]
    rustfmt_exe_path: Option<PathBuf>,

    /// Path to clang-format executable.
    #[arg(long)]
    clang_format_exe_path: Option<PathBuf>,

    /// Path to rustfmt.toml config file.
    #[arg(long, alias = "config-path")]
    rustfmt_config_path: Option<PathBuf>,

    /// Additional arguments to pass directly to rustfmt.
    #[arg(long = "rustfmt-arg", value_name = "ARG")]
    rustfmt_args: Vec<String>,

    /// Additional arguments to pass directly to clang-format.
    #[arg(long = "clang-format-arg", value_name = "ARG")]
    clang_format_args: Vec<String>,
}

/// Pipes `input` through an external command process (`rustfmt` or `clang-format`),
/// capturing and returning `stdout`.
fn pipe_through_process<'a>(
    input: &str,
    exe_name: &str,
    exe_path: &Path,
    args: impl IntoIterator<Item = &'a OsStr>,
) -> Result<String> {
    use std::io::Write as _;

    let mut child = Command::new(exe_path)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to spawn {exe_name} at {exe_path:?}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .with_context(|| format!("Failed to write to {exe_name} stdin"))?;
    }

    let output =
        child.wait_with_output().with_context(|| format!("Failed to read {exe_name} output"))?;

    if !output.status.success() {
        bail!(
            "{exe_name} exited with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Runs `rustfmt` on the source string.
pub fn run_rustfmt(
    source: &str,
    rustfmt_path: Option<&Path>,
    rustfmt_config_path: Option<&Path>,
    extra_args: &[String],
) -> Result<String> {
    let default_path = Path::new(external_binaries::RUSTFMT_EXE_PATH);
    let exe = rustfmt_path.unwrap_or(default_path);

    let mut args: Vec<&OsStr> = vec![OsStr::new("--emit"), OsStr::new("stdout")];
    let config_arg;
    if let Some(cfg) = rustfmt_config_path {
        config_arg = cfg.as_os_str();
        args.push(OsStr::new("--config-path"));
        args.push(config_arg);
    }
    args.extend(extra_args.iter().map(OsStr::new));

    pipe_through_process(source, "rustfmt", exe, args)
}

/// Formats a C++ code snippet using `clang-format`.
///
/// For `inline_cpp!` macro bodies with parameter lists (e.g. `(int a, int b) -> int { ... }`),
/// `clang-format` requires a function identifier to parse the signature correctly.
/// We prepend a synthetic header (`inline auto __crubit_fmt_wrapper`), format the C++,
/// and subsequently strip the synthetic wrapper before returning.
pub fn run_clang_format(
    body: &str,
    kind: MacroKind,
    column_limit: usize,
    clang_format_path: Option<&Path>,
    extra_args: &[String],
) -> Result<String> {
    let default_path = Path::new(external_binaries::CLANG_FORMAT_EXE_PATH);
    let exe = clang_format_path.unwrap_or(default_path);

    let trimmed = body.trim();
    let (input, wrapper_prefix) = match kind {
        MacroKind::InlineCpp => {
            if trimmed.starts_with('(') {
                (
                    format!("inline auto __crubit_fmt_wrapper {}", trimmed),
                    Some("inline auto __crubit_fmt_wrapper"),
                )
            } else {
                (
                    format!("void __crubit_fmt_wrapper() {}", trimmed),
                    Some("void __crubit_fmt_wrapper()"),
                )
            }
        }
        MacroKind::GlobalCpp | MacroKind::DoNotSubmitCppDecl => (trimmed.to_string(), None),
    };

    let style_arg = format!("--style={{BasedOnStyle: Google, ColumnLimit: {column_limit}}}");
    let mut args: Vec<&OsStr> = vec![OsStr::new(&style_arg)];
    args.extend(extra_args.iter().map(OsStr::new));

    let formatted = pipe_through_process(&input, "clang-format", exe, args)?;

    if let Some(prefix) = wrapper_prefix
        && let Some(pos) = formatted.find(prefix)
    {
        let inner = formatted[pos + prefix.len()..].trim_start();
        return Ok(inner.trim_matches('\n').to_string());
    }

    Ok(formatted.trim_matches('\n').to_string())
}

/// Formats Rust source containing embedded C++ macro blocks through the two-pass pipeline.
pub fn format_rust_source(
    source: &str,
    rustfmt_path: Option<&Path>,
    clang_format_path: Option<&Path>,
    rustfmt_config_path: Option<&Path>,
    rustfmt_extra_args: &[String],
    clang_format_extra_args: &[String],
) -> Result<String> {
    // Pass 1: Format outer Rust AST with rustfmt
    let rust_formatted =
        match run_rustfmt(source, rustfmt_path, rustfmt_config_path, rustfmt_extra_args) {
            Ok(fmt) => fmt,
            Err(_) if cfg!(test) => source.to_string(), // Fallback for unit testing if binaries are omitted
            Err(e) => return Err(e),
        };

    // Pass 2: Locate and format embedded C++ macro blocks
    let macros = parse_inline_cpp_macros(&rust_formatted, "").map_err(|e| anyhow::anyhow!(e))?;
    if macros.is_empty() {
        return Ok(rust_formatted);
    }

    // TODO(b/544997630): Run clang-format subprocesses in parallel (e.g. via rayon) for files with multiple macro blocks.
    let mut replacements = Vec::new();
    for m in macros {
        let line_start =
            rust_formatted[..m.macro_start_offset].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let line_prefix = &rust_formatted[line_start..m.macro_start_offset];
        let base_indent = line_prefix.len() - line_prefix.trim_start_matches(' ').len();

        let column_limit =
            GOOGLE_COLUMN_LIMIT.saturating_sub(base_indent + CPP_INDENT_SPACES).max(20);

        let formatted_cpp = run_clang_format(
            m.body_text,
            m.kind,
            column_limit,
            clang_format_path,
            clang_format_extra_args,
        )
        .unwrap_or_else(|_| m.body_text.trim().to_string());

        let indent_str = " ".repeat(base_indent + CPP_INDENT_SPACES);
        let indented_body: String = formatted_cpp
            .lines()
            .map(|line| {
                if line.trim().is_empty() {
                    "\n".to_string()
                } else {
                    format!("{indent_str}{line}\n")
                }
            })
            .collect();

        let outer_indent = " ".repeat(base_indent);
        let replacement = if indented_body.is_empty() {
            "{}".to_string()
        } else {
            format!("{{\n{indented_body}{outer_indent}}}")
        };

        let replace_start = rust_formatted[..m.body_start_offset]
            .rfind('{')
            .context("Failed to find macro body opening brace")?;
        let replace_end = m.body_end_offset + 1; // includes closing brace '}'

        replacements.push((replace_start..replace_end, replacement));
    }

    let mut result = rust_formatted;
    for (range, replacement) in replacements.into_iter().rev() {
        result.replace_range(range, &replacement);
    }

    Ok(result)
}

fn format_file_or_stdin(args: &Args) -> Result<()> {
    use std::io::{Read as _, Write as _};

    let rustfmt_path = args.rustfmt_exe_path.as_deref();
    let clang_format_path = args.clang_format_exe_path.as_deref();
    let rustfmt_config_path = args.rustfmt_config_path.as_deref();

    if args.files.is_empty() || (args.files.len() == 1 && args.files[0] == Path::new("-")) {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).context("Failed to read from stdin")?;
        let formatted = format_rust_source(
            &input,
            rustfmt_path,
            clang_format_path,
            rustfmt_config_path,
            &args.rustfmt_args,
            &args.clang_format_args,
        )?;
        io::stdout().write_all(formatted.as_bytes()).context("Failed to write to stdout")?;
        return Ok(());
    }

    // TODO(b/544997630): Format multiple files in parallel when given a large list of files.
    for file in &args.files {
        let content = fs::read_to_string(file)
            .with_context(|| format!("Failed to read file {}", file.display()))?;
        let formatted = format_rust_source(
            &content,
            rustfmt_path,
            clang_format_path,
            rustfmt_config_path,
            &args.rustfmt_args,
            &args.clang_format_args,
        )?;

        if args.inplace {
            if content == formatted {
                continue;
            }
            fs::write(file, &formatted)
                .with_context(|| format!("Failed to write file {}", file.display()))?;
        } else {
            io::stdout().write_all(formatted.as_bytes()).context("Failed to write to stdout")?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    format_file_or_stdin(&args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_format_inline_cpp_single_line() -> anyhow::Result<()> {
        let source = r#"pub fn add(a: i32, b: i32) -> i32 {
    unsafe {
        inline_cpp! { (int a, int b)->int{return a+b;} }
    }
}
"#;
        let formatted = format_rust_source(source, None, None, None, &[], &[])?;
        expect_that!(
            formatted.as_str(),
            contains_substring(
                r#"inline_cpp! {
            (int a, int b) -> int {
              return a + b;
            }
        }"#
            )
        );
        Ok(())
    }

    #[gtest]
    fn test_format_inline_cpp_multi_line() -> anyhow::Result<()> {
        let source = r#"pub fn complex(a: i32, b: i32) -> i32 {
    unsafe {
        inline_cpp! { (int a, int b)->int{int c=a+b;return c*2;} }
    }
}
"#;
        let formatted = format_rust_source(source, None, None, None, &[], &[])?;
        expect_that!(
            formatted.as_str(),
            contains_substring(
                r#"inline_cpp! {
            (int a, int b) -> int {
              int c = a + b;
              return c * 2;
            }
        }"#
            )
        );
        Ok(())
    }

    #[gtest]
    fn test_format_global_cpp() -> anyhow::Result<()> {
        let source = r#"global_cpp! { int x=10; struct S{ int a; }; }
"#;
        let formatted = format_rust_source(source, None, None, None, &[], &[])?;
        expect_that!(
            formatted.as_str(),
            contains_substring(
                r#"global_cpp! {
    int x = 10;
    struct S {
      int a;
    };
}"#
            )
        );
        Ok(())
    }
}
