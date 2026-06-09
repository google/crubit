// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process;

/// Scrapes global_cpp! blocks from Rust source files and generates a C++ header.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input Rust source files (.rs) to scan
    #[arg(long, value_name = "FILE", required = true)]
    srcs: Vec<PathBuf>,

    /// The output C++ header file (.h)
    #[arg(long, value_name = "FILE", required = true)]
    out: PathBuf,
}

#[derive(PartialEq, Eq)]
enum State {
    Searching,
    FoundIdent,
    FoundBang,
    Extracting,
}

pub fn extract_global_cpp(rust_source: &str, file_name: &str) -> Result<String, String> {
    let mut extracted = String::new();

    let tokens =
        ra_ap_rustc_lexer::tokenize(rust_source, ra_ap_rustc_lexer::FrontmatterAllowed::No);
    let mut current_pos = 0;

    let mut state = State::Searching;
    let mut brace_counter = 0;
    let mut block_start = 0;

    for token in tokens {
        let token_text = &rust_source[current_pos..current_pos + token.len as usize];

        match state {
            State::Searching => {
                if token.kind == ra_ap_rustc_lexer::TokenKind::Ident && token_text == "global_cpp" {
                    state = State::FoundIdent;
                }
            }
            State::FoundIdent => {
                if token.kind == ra_ap_rustc_lexer::TokenKind::Bang {
                    state = State::FoundBang;
                } else if token.kind != ra_ap_rustc_lexer::TokenKind::Whitespace {
                    state = State::Searching;
                }
            }
            State::FoundBang => {
                if token.kind == ra_ap_rustc_lexer::TokenKind::OpenBrace {
                    state = State::Extracting;
                    brace_counter = 1;
                    block_start = current_pos + token.len as usize;
                } else if token.kind != ra_ap_rustc_lexer::TokenKind::Whitespace {
                    state = State::Searching;
                }
            }
            State::Extracting => {
                if token.kind == ra_ap_rustc_lexer::TokenKind::OpenBrace {
                    brace_counter += 1;
                } else if token.kind == ra_ap_rustc_lexer::TokenKind::CloseBrace {
                    brace_counter -= 1;
                    if brace_counter == 0 {
                        extracted.push_str(&rust_source[block_start..current_pos]);
                        extracted.push('\n');
                        state = State::Searching;
                    }
                }
            }
        }

        current_pos += token.len as usize;
    }

    if state == State::Extracting {
        let line_number = rust_source[..block_start].chars().filter(|&c| c == '\n').count() + 1;

        // Target 2 lines before the brace, the line with the brace, and 2 lines after
        let start_line_idx = line_number.saturating_sub(3);
        let context =
            rust_source.lines().skip(start_line_idx).take(5).collect::<Vec<_>>().join("\n");

        return Err(format!(
            "Unclosed global_cpp! block in {}:{}. Context around open brace:\n{}",
            file_name, line_number, context
        ));
    }

    Ok(extracted)
}

fn main() {
    let args = Args::parse();

    let mut all_cpp_snippets = String::new();

    for src in args.srcs {
        let content = fs::read_to_string(&src).unwrap_or_else(|e| {
            eprintln!("Failed to read file {}: {}", src.display(), e);
            process::exit(1);
        });
        let scraped =
            extract_global_cpp(&content, &src.display().to_string()).unwrap_or_else(|e| {
                eprintln!("Extraction error: {}", e);
                process::exit(1);
            });
        all_cpp_snippets.push_str(&scraped);
    }

    let guard_name = "CRUBIT_EXTRACTED_GLOBAL_CPP_H_";
    let final_header = format!(
        "#ifndef {name}\n#define {name}\n\n{content}\n#endif  // {name}\n",
        name = guard_name,
        content = all_cpp_snippets
    );

    fs::write(&args.out, final_header).unwrap_or_else(|e| {
        eprintln!("Failed to write output to {}: {}", args.out.display(), e);
        process::exit(1);
    });
}

#[cfg(test)]
mod tests {
        use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_basic_extract() {
        let input = "global_cpp! { int x; }";
        let expected = " int x; \n";
        expect_eq!(extract_global_cpp(input, "test.rs").unwrap(), expected);
    }

    #[gtest]
    fn test_nested_braces() {
        let input = "global_cpp! { namespace foo { int x; } }";
        let expected = " namespace foo { int x; } \n";
        expect_eq!(extract_global_cpp(input, "test.rs").unwrap(), expected);
    }

    #[gtest]
    fn test_multiple_blocks() {
        let input = "global_cpp! { int x; } some rust code global_cpp! { int y; }";
        let expected = " int x; \n int y; \n";
        expect_eq!(extract_global_cpp(input, "test.rs").unwrap(), expected);
    }

    #[gtest]
    fn test_unclosed_block() {
        let input = "global_cpp! { int x;";
        let err = extract_global_cpp(input, "test.rs").unwrap_err();
        let expected_err = "Unclosed global_cpp! block in test.rs:1. Context around open brace:\nglobal_cpp! { int x;";
        expect_eq!(err, expected_err);
    }

    #[gtest]
    fn test_unclosed_block_with_context() {
        let input = "line 1\nline 2\nline 3\nglobal_cpp! { int x;\nline 5\nline 6\nline 7";
        let err = extract_global_cpp(input, "test.rs").unwrap_err();
        let expected_err = "Unclosed global_cpp! block in test.rs:4. Context around open brace:\nline 2\nline 3\nglobal_cpp! { int x;\nline 5\nline 6";
        expect_eq!(err, expected_err);
    }
}
