// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use clap::Parser;
use ra_ap_rustc_lexer::TokenKind;
use std::fs;
use std::path::PathBuf;
use std::process;

/// Scrapes global_cpp! blocks from Rust source files and generates a C++ header.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input Rust source files (.rs) to scan
    #[arg(long, value_name = "FILE", required = true, num_args = 1..)]
    srcs: Vec<PathBuf>,

    /// The target label of the parent library target
    #[arg(long, required = true)]
    target: String,

    /// The output C++ header file (.h)
    #[arg(long, value_name = "FILE", required = true)]
    out: PathBuf,
}

struct TokenParser<'a> {
    tokens: &'a [ra_ap_rustc_lexer::Token],
    rust_source: &'a str,
    token_index: usize,
    byte_offset: usize,
    line: usize,
    column: usize,
}

impl<'a> TokenParser<'a> {
    fn new(rust_source: &'a str, tokens: &'a [ra_ap_rustc_lexer::Token]) -> Self {
        TokenParser { tokens, rust_source, token_index: 0, byte_offset: 0, line: 1, column: 1 }
    }

    fn is_eof(&self) -> bool {
        self.token_index >= self.tokens.len()
    }

    fn peek(&self) -> Option<&'a ra_ap_rustc_lexer::Token> {
        self.tokens.get(self.token_index)
    }

    fn peek_text(&self) -> &'a str {
        if let Some(token) = self.peek() {
            &self.rust_source[self.byte_offset..self.byte_offset + token.len as usize]
        } else {
            ""
        }
    }

    fn advance(&mut self) {
        let Some(token) = self.peek() else {
            return;
        };
        let text = self.peek_text();
        for c in text.chars() {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.byte_offset += token.len as usize;
        self.token_index += 1;
    }

    fn eat_whitespace(&mut self) {
        while let Some(t) = self.peek() {
            if t.kind == TokenKind::Whitespace {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn eat_bang(&mut self) -> bool {
        if matches!(self.peek(), Some(t) if t.kind == TokenKind::Bang) {
            self.advance();
            return true;
        }
        false
    }

    fn eat_braced_body(&mut self, file_name: &str) -> Result<&'a str, String> {
        let start_line = self.line;
        if let Some(t) = self.peek() {
            if t.kind != TokenKind::OpenBrace {
                return Err(format!("Expected '{{' after '!' at {}:{}", file_name, start_line));
            }
        } else {
            return Err(format!("Expected '{{' after '!' at {}:{}", file_name, start_line));
        }

        self.advance();

        let body_start_pos = self.byte_offset;
        let mut depth = 1;

        while let Some(t) = self.peek() {
            let body_end_pos = self.byte_offset;
            self.advance();

            if t.kind == TokenKind::OpenBrace {
                depth += 1;
            } else if t.kind == TokenKind::CloseBrace {
                depth -= 1;
                if depth == 0 {
                    return Ok(&self.rust_source[body_start_pos..body_end_pos]);
                }
            }
        }

        Err(format!(
            "Unmatched delimiter starting at {}:{}: Context around open brace:\n{}",
            file_name,
            start_line,
            self.rust_source
                .lines()
                .skip(start_line.saturating_sub(3))
                .take(5)
                .collect::<Vec<_>>()
                .join("\n")
        ))
    }
}

fn extract_macro_body<'a>(
    parser: &mut TokenParser<'a>,
    macro_name: &str,
    file_name: &str,
) -> Result<Option<(usize, usize, &'a str)>, String> {
    let Some(token) = parser.peek() else {
        return Ok(None);
    };
    let token_text = parser.peek_text();

    if token.kind != TokenKind::Ident || token_text != macro_name {
        parser.advance();
        return Ok(None);
    }

    let macro_line = parser.line;
    let macro_col = parser.column;

    parser.advance();
    parser.eat_whitespace();

    if !parser.eat_bang() {
        return Ok(None);
    }

    parser.eat_whitespace();

    let braced_body_text = parser.eat_braced_body(file_name)?;
    Ok(Some((macro_line, macro_col, braced_body_text)))
}

pub fn extract_global_cpp(
    rust_source: &str,
    tokens: &[ra_ap_rustc_lexer::Token],
    file_name: &str,
) -> Result<String, String> {
    let mut extracted = String::new();
    let mut parser = TokenParser::new(rust_source, tokens);

    while parser.peek().is_some() {
        if let Some((_, _, braced_body_text)) =
            extract_macro_body(&mut parser, "global_cpp", file_name)?
        {
            extracted.push_str(braced_body_text);
            extracted.push('\n');
        }
    }

    Ok(extracted)
}

pub fn extract_inline_cpp(
    rust_source: &str,
    tokens: &[ra_ap_rustc_lexer::Token],
    file_name: &str,
    target: &str,
) -> Result<String, String> {
    let mut extracted = String::new();
    let mut parser = TokenParser::new(rust_source, tokens);

    while parser.peek().is_some() {
        if let Some((macro_line, macro_col, braced_body_text)) =
            extract_macro_body(&mut parser, "inline_cpp", file_name)?
        {
            let thunk_name =
                inline_cpp_utils::compute_thunk_name(target, file_name, macro_line, macro_col);
            extracted.push_str(&format!(
                "extern \"C\" void {}() {{\n{}\n}}\n\n",
                thunk_name, braced_body_text
            ));
        }
    }

    Ok(extracted)
}

fn main() {
    let args = Args::parse();

    let mut all_cpp_snippets = String::new();

    for src in &args.srcs {
        let content = fs::read_to_string(src).unwrap_or_else(|e| {
            eprintln!("Failed to read file {}: {}", src.display(), e);
            process::exit(1);
        });
        let file_name = src.display().to_string();
        let tokens =
            ra_ap_rustc_lexer::tokenize(&content, ra_ap_rustc_lexer::FrontmatterAllowed::No);
        let token_list = tokens.collect::<Vec<_>>();

        let scraped_global =
            extract_global_cpp(&content, &token_list, &file_name).unwrap_or_else(|e| {
                eprintln!("Extraction error: {}", e);
                process::exit(1);
            });
        let scraped_inline = extract_inline_cpp(&content, &token_list, &file_name, &args.target)
            .unwrap_or_else(|e| {
                eprintln!("Extraction error: {}", e);
                process::exit(1);
            });
        all_cpp_snippets.push_str(&scraped_global);
        all_cpp_snippets.push_str(&scraped_inline);
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

    fn tokenize(rust_source: &str) -> Vec<ra_ap_rustc_lexer::Token> {
        ra_ap_rustc_lexer::tokenize(rust_source, ra_ap_rustc_lexer::FrontmatterAllowed::No)
            .collect()
    }

    #[gtest]
    fn test_basic_extract() {
        let input = "global_cpp! { int x; }";
        let expected = " int x; \n";
        expect_eq!(extract_global_cpp(input, &tokenize(input), "test.rs").unwrap(), expected);
    }

    #[gtest]
    fn test_nested_braces() {
        let input = "global_cpp! { namespace foo { int x; } }";
        let expected = " namespace foo { int x; } \n";
        expect_eq!(extract_global_cpp(input, &tokenize(input), "test.rs").unwrap(), expected);
    }

    #[gtest]
    fn test_multiple_blocks() {
        let input = "global_cpp! { int x; } some rust code global_cpp! { int y; }";
        let expected = " int x; \n int y; \n";
        expect_eq!(extract_global_cpp(input, &tokenize(input), "test.rs").unwrap(), expected);
    }

    #[gtest]
    fn test_unclosed_block() {
        let input = "global_cpp! { int x;";
        let err = extract_global_cpp(input, &tokenize(input), "test.rs").unwrap_err();
        let expected_err = "Unmatched delimiter starting at test.rs:1: Context around open brace:\nglobal_cpp! { int x;";
        expect_eq!(err, expected_err);
    }

    #[gtest]
    fn test_unclosed_block_with_context() {
        let input = "line 1\nline 2\nline 3\nglobal_cpp! { int x;\nline 5\nline 6\nline 7";
        let err = extract_global_cpp(input, &tokenize(input), "test.rs").unwrap_err();
        let expected_err = "Unmatched delimiter starting at test.rs:4: Context around open brace:\nline 2\nline 3\nglobal_cpp! { int x;\nline 5\nline 6";
        expect_eq!(err, expected_err);
    }

    #[gtest]
    fn test_extract_inline_cpp() {
        let input = "let r = inline_cpp! { return 42; };";
        let file_name = "test_src.rs";
        let target = "//test:target";
        let thunk_name = inline_cpp_utils::compute_thunk_name(target, file_name, 1, 9);
        let expected_thunk = format!("extern \"C\" void {}() {{\n return 42; \n}}\n\n", thunk_name);
        expect_eq!(
            extract_inline_cpp(input, &tokenize(input), file_name, target).unwrap(),
            expected_thunk
        );
    }

    #[gtest]
    fn test_extract_inline_cpp_same_line() {
        let input = "inline_cpp! { return 1; }; inline_cpp! { return 2; };";
        let file_name = "test_src.rs";
        let target = "//test:target";
        let thunk_name1 = inline_cpp_utils::compute_thunk_name(target, file_name, 1, 1);
        let thunk_name2 = inline_cpp_utils::compute_thunk_name(target, file_name, 1, 28);

        let expected = format!(
            "extern \"C\" void {}() {{\n return 1; \n}}\n\n\
             extern \"C\" void {}() {{\n return 2; \n}}\n\n",
            thunk_name1, thunk_name2
        );
        expect_eq!(
            extract_inline_cpp(input, &tokenize(input), file_name, target).unwrap(),
            expected
        );
    }

    #[gtest]
    fn test_extract_inline_cpp_nested_braces() {
        let input = "inline_cpp! { if (true) { return 1; } else { return 2; } };";
        let file_name = "test_src.rs";
        let target = "//test:target";
        let thunk_name = inline_cpp_utils::compute_thunk_name(target, file_name, 1, 1);
        let expected = format!(
            "extern \"C\" void {}() {{\n if (true) {{ return 1; }} else {{ return 2; }} \n}}\n\n",
            thunk_name
        );
        expect_eq!(
            extract_inline_cpp(input, &tokenize(input), file_name, target).unwrap(),
            expected
        );
    }

    #[gtest]
    fn test_extract_inline_cpp_newlines() {
        let input = "line 1\r\nline 2\r\ninline_cpp! { return 4; };";
        let file_name = "test_src.rs";
        let target = "//test:target";
        let thunk_name = inline_cpp_utils::compute_thunk_name(target, file_name, 3, 1);
        let expected = format!("extern \"C\" void {}() {{\n return 4; \n}}\n\n", thunk_name);
        expect_eq!(
            extract_inline_cpp(input, &tokenize(input), file_name, target).unwrap(),
            expected
        );
    }
}
