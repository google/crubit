// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use clap::Parser;
use ra_ap_rustc_lexer::TokenKind;
use std::fmt::Write;
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

/// Coordinates (line/column) where the inner C++ code content begins.
struct ExtractedBracedBody<'a> {
    line: usize,
    column: usize,
    text: &'a str,
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

    fn token_text_at(&self, idx: usize) -> &'a str {
        if idx >= self.tokens.len() {
            return "";
        }
        let mut offset = self.byte_offset;
        for i in self.token_index..idx {
            offset += self.tokens[i].len as usize;
        }
        let len = self.tokens[idx].len as usize;
        &self.rust_source[offset..offset + len]
    }

    fn is_colon_colon_at(&self, idx: usize) -> bool {
        idx + 1 < self.tokens.len()
            && self.tokens[idx].kind == TokenKind::Colon
            && self.tokens[idx + 1].kind == TokenKind::Colon
    }

    /// Returns the number of tokens in the macro path if the tokens starting at `self.token_index`
    /// form a macro invocation path ending in `macro_name!`, such as `inline_cpp!` or
    /// `::crubit_support::inline_cpp!` or `crubit_support::inline_cpp!`.
    fn match_macro_invocation_path(&self, macro_name: &str) -> Option<usize> {
        let mut idx = self.token_index;
        if idx >= self.tokens.len() {
            return None;
        }

        // Optional leading `::`
        if self.is_colon_colon_at(idx) {
            idx += 2;
        }

        while idx < self.tokens.len() {
            if self.tokens[idx].kind != TokenKind::Ident {
                return None;
            }
            let is_target_macro = self.token_text_at(idx) == macro_name;
            // Check what follows this Ident
            let mut next_idx = idx + 1;
            while next_idx < self.tokens.len()
                && self.tokens[next_idx].kind == TokenKind::Whitespace
            {
                next_idx += 1;
            }
            if is_target_macro
                && next_idx < self.tokens.len()
                && self.tokens[next_idx].kind == TokenKind::Bang
            {
                // Found path ending in `macro_name!`
                return Some(idx + 1 - self.token_index);
            }
            // Otherwise, it must be followed by `::` to continue the path
            if self.is_colon_colon_at(idx + 1) {
                idx += 3;
            } else {
                return None;
            }
        }
        None
    }

    fn eat_braced_body(&mut self, file_name: &str) -> Result<ExtractedBracedBody<'a>, String> {
        let start_line = self.line;
        if let Some(t) = self.peek() {
            if t.kind != TokenKind::OpenBrace {
                return Err(format!("Expected '{{' after '!' at {}:{}", file_name, start_line));
            }
        } else {
            return Err(format!("Expected '{{' after '!' at {}:{}", file_name, start_line));
        }

        self.advance();

        let body_start_line = self.line;
        let body_start_col = self.column;
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
                    return Ok(ExtractedBracedBody {
                        line: body_start_line,
                        column: body_start_col,
                        text: &self.rust_source[body_start_pos..body_end_pos],
                    });
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

/// Coordinates (line/column) where the outer macro token begins (e.g. `inline_cpp`).
struct ExtractedMacro<'a> {
    macro_line: usize,
    macro_col: usize,
    body: ExtractedBracedBody<'a>,
}

fn extract_macro_body<'a>(
    parser: &mut TokenParser<'a>,
    macro_name: &str,
    file_name: &str,
) -> Result<Option<ExtractedMacro<'a>>, String> {
    let Some(path_token_count) = parser.match_macro_invocation_path(macro_name) else {
        parser.advance();
        return Ok(None);
    };

    let macro_line = parser.line;
    let macro_col = parser.column;

    for _ in 0..path_token_count {
        parser.advance();
    }
    parser.eat_whitespace();

    if !parser.eat_bang() {
        return Ok(None);
    }

    parser.eat_whitespace();

    let body = parser.eat_braced_body(file_name)?;
    Ok(Some(ExtractedMacro { macro_line, macro_col, body }))
}

pub fn extract_global_cpp(
    rust_source: &str,
    tokens: &[ra_ap_rustc_lexer::Token],
    file_name: &str,
) -> Result<String, String> {
    let mut extracted = String::new();
    let mut parser = TokenParser::new(rust_source, tokens);

    while parser.peek().is_some() {
        let Some(block) = extract_macro_body(&mut parser, "global_cpp", file_name)? else {
            continue;
        };
        extracted.push_str(block.body.text);
        extracted.push('\n');
    }

    Ok(extracted)
}

struct LexToken<'a> {
    kind: ra_ap_rustc_lexer::TokenKind,
    text: &'a str,
}

/// Parses and validates that the raw string contents of an `inline_cpp!` block
/// has the structurally correct C++ signature format `(args) -> return_type { body }`.
fn validate_inline_cpp_syntax(body_text: &str) -> Result<(), String> {
    let mut tokens = Vec::new();
    let mut offset = 0;
    for t in ra_ap_rustc_lexer::tokenize(body_text, ra_ap_rustc_lexer::FrontmatterAllowed::No) {
        let len = t.len as usize;
        tokens.push(LexToken { kind: t.kind, text: &body_text[offset..offset + len] });
        offset += len;
    }

    let Some(first_idx) =
        tokens.iter().position(|t| t.kind != ra_ap_rustc_lexer::TokenKind::Whitespace)
    else {
        return Err("Empty inline_cpp! block is not allowed".to_string());
    };

    if tokens[first_idx].kind != ra_ap_rustc_lexer::TokenKind::OpenParen {
        return Err("inline_cpp! block must start with a parameter list `(args)`".to_string());
    }

    let mut paren_depth = 0;
    let Some(close_paren_idx) = tokens[first_idx..]
        .iter()
        .enumerate()
        .find(|&(_, t)| {
            match t.kind {
                ra_ap_rustc_lexer::TokenKind::OpenParen => paren_depth += 1,
                ra_ap_rustc_lexer::TokenKind::CloseParen => {
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        return true;
                    }
                }
                _ => {}
            }
            false
        })
        .map(|(pos, _)| first_idx + pos)
    else {
        return Err("Mismatched parameter parentheses inside inline_cpp!".to_string());
    };

    let Some(open_brace_idx) = tokens[(close_paren_idx + 1)..]
        .iter()
        .enumerate()
        .find(|&(_, t)| t.kind == ra_ap_rustc_lexer::TokenKind::OpenBrace)
        .map(|(pos, _)| close_paren_idx + 1 + pos)
    else {
        return Err(
            "Missing body open brace '{' after parameter list inside inline_cpp!".to_string()
        );
    };

    let Some(last_idx) =
        tokens.iter().rposition(|t| t.kind != ra_ap_rustc_lexer::TokenKind::Whitespace)
    else {
        return Err("Missing body contents inside inline_cpp!".to_string());
    };
    if tokens[last_idx].kind != ra_ap_rustc_lexer::TokenKind::CloseBrace {
        return Err("Mismatched body braces inside inline_cpp!".to_string());
    }

    // Extract and validate return type (expected to match `-> ReturnType` between parenthesis and body brace)
    let ret_type_raw: String =
        tokens[(close_paren_idx + 1)..open_brace_idx].iter().map(|t| t.text).collect();
    let return_type = ret_type_raw.trim();
    if return_type.is_empty() || !return_type.starts_with("->") {
        return Err("inline_cpp! block must specify a return type starting with `->`".to_string());
    }

    Ok(())
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
        let Some(block) = extract_macro_body(&mut parser, "inline_cpp", file_name)? else {
            continue;
        };
        let thunk_name = inline_cpp_utils::compute_thunk_name(
            target,
            file_name,
            block.macro_line,
            block.macro_col,
        );
        validate_inline_cpp_syntax(block.body.text)
            .map_err(|e| format!("{} at {}:{}", e, file_name, block.macro_line))?;
        // Prefix thunk signature with spaces to map original column offsets in Clang.
        let padding = " ".repeat(block.body.column - 1);

        let _ = write!(
            extracted,
            "inline auto {}\n#line {} \"{}\"\n{}{}\n\n",
            thunk_name, block.body.line, file_name, padding, block.body.text
        );
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
        let input = "let r = inline_cpp! { () -> int { return 42; } };";
        let file_name = "test_src.rs";
        let target = "//test:target";
        let thunk_name = inline_cpp_utils::compute_thunk_name(target, file_name, 1, 9);
        let expected_thunk = format!(
            "inline auto {}\n#line 1 \"test_src.rs\"\n                      () -> int {{ return 42; }} \n\n",
            thunk_name
        );
        expect_eq!(
            extract_inline_cpp(input, &tokenize(input), file_name, target).unwrap(),
            expected_thunk
        );
    }

    #[gtest]
    fn test_extract_inline_cpp_same_line() {
        let input =
            "\ninline_cpp! { () -> int { return 1; } };\ninline_cpp! { () -> int { return 2; } };";
        let file_name = "test_src.rs";
        let target = "//test:target";
        let thunk_name1 = inline_cpp_utils::compute_thunk_name(target, file_name, 2, 1);
        let thunk_name2 = inline_cpp_utils::compute_thunk_name(target, file_name, 3, 1);

        let expected = format!(
            "inline auto {}\n#line 2 \"test_src.rs\"\n              () -> int {{ return 1; }} \n\n\
             inline auto {}\n#line 3 \"test_src.rs\"\n              () -> int {{ return 2; }} \n\n",
            thunk_name1, thunk_name2
        );
        expect_eq!(
            extract_inline_cpp(input, &tokenize(input), file_name, target).unwrap(),
            expected
        );
    }

    #[gtest]
    fn test_extract_inline_cpp_nested_braces() {
        let input = "inline_cpp! { () -> int { if (true) { return 1; } else { return 2; } } };";
        let file_name = "test_src.rs";
        let target = "//test:target";
        let thunk_name = inline_cpp_utils::compute_thunk_name(target, file_name, 1, 1);
        let expected = format!(
            "inline auto {}\n#line 1 \"test_src.rs\"\n              () -> int {{ if (true) {{ return 1; }} else {{ return 2; }} }} \n\n",
            thunk_name
        );
        expect_eq!(
            extract_inline_cpp(input, &tokenize(input), file_name, target).unwrap(),
            expected
        );
    }

    #[gtest]
    fn test_extract_inline_cpp_newlines() {
        let input = "line 1\r\nline 2\r\ninline_cpp! { () -> void { return; } };";
        let file_name = "test_src.rs";
        let target = "//test:target";
        let thunk_name = inline_cpp_utils::compute_thunk_name(target, file_name, 3, 1);
        let expected = format!(
            "inline auto {}\n#line 3 \"test_src.rs\"\n              () -> void {{ return; }} \n\n",
            thunk_name
        );
        expect_eq!(
            extract_inline_cpp(input, &tokenize(input), file_name, target).unwrap(),
            expected
        );
    }

    #[gtest]
    fn test_extract_inline_cpp_with_signature() {
        let input = "inline_cpp! { (int a, double b) -> int { return a + b; } }";
        let file_name = "test_src.rs";
        let target = "//test:target";
        let thunk_name = inline_cpp_utils::compute_thunk_name(target, file_name, 1, 1);
        let expected = format!(
            "inline auto {}\n#line 1 \"test_src.rs\"\n              (int a, double b) -> int {{ return a + b; }} \n\n",
            thunk_name
        );
        expect_eq!(
            extract_inline_cpp(input, &tokenize(input), file_name, target).unwrap(),
            expected
        );
    }

    #[gtest]
    fn test_extract_inline_cpp_multiline() {
        let input = "inline_cpp! {\n    (int a) -> int {\n        return a;\n    }\n}";
        let file_name = "test_src.rs";
        let target = "//test:target";
        let thunk_name = inline_cpp_utils::compute_thunk_name(target, file_name, 1, 1);
        let expected = format!(
            "inline auto {}\n#line 1 \"test_src.rs\"\n             \n    (int a) -> int {{\n        return a;\n    }}\n\n\n",
            thunk_name
        );
        expect_eq!(
            extract_inline_cpp(input, &tokenize(input), file_name, target).unwrap(),
            expected
        );
    }

    #[gtest]
    fn test_extract_inline_cpp_path_qualified() {
        let input = "        ::crubit_support::inline_cpp! { () -> int { return 42; } };";
        let file_name = "test_src.rs";
        let target = "//test:target";
        // Column 9 is the start of `::crubit_support::inline_cpp!`
        let thunk_name = inline_cpp_utils::compute_thunk_name(target, file_name, 1, 9);
        let expected = format!(
            "inline auto {}\n#line 1 \"test_src.rs\"\n                                        () -> int {{ return 42; }} \n\n",
            thunk_name
        );
        expect_eq!(
            extract_inline_cpp(input, &tokenize(input), file_name, target).unwrap(),
            expected
        );
    }
}
