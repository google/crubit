// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Shared parsing library for extracting embedded C++ macro invocation blocks
//! (`inline_cpp!`, `global_cpp!`, `DO_NOT_SUBMIT_CPP_DECL!`) from Rust source text.

use ra_ap_rustc_lexer::{FrontmatterAllowed, TokenKind};

/// Identifies which embedded C++ macro variant was parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroKind {
    InlineCpp,
    GlobalCpp,
    DoNotSubmitCppDecl,
}

impl MacroKind {
    pub fn name(&self) -> &'static str {
        match self {
            MacroKind::InlineCpp => "inline_cpp",
            MacroKind::GlobalCpp => "global_cpp",
            MacroKind::DoNotSubmitCppDecl => "DO_NOT_SUBMIT_CPP_DECL",
        }
    }
}

/// A parsed embedded C++ macro block with exact source coordinates and byte offsets.
#[derive(Debug, PartialEq, Eq)]
pub struct ParsedMacro<'a> {
    pub kind: MacroKind,
    /// Byte offset where the macro identifier or path starts (e.g.
    /// `::inline_cpp_macro::inline_cpp!`).
    pub macro_start_offset: usize,
    /// Line (1-indexed) where the outer macro token starts.
    pub macro_line: usize,
    /// Column (1-indexed) where the outer macro token starts.
    pub macro_col: usize,
    /// Byte offset where the inner C++ body begins (immediately after `{`).
    pub body_start_offset: usize,
    /// Byte offset where the inner C++ body ends (immediately before `}`).
    pub body_end_offset: usize,
    /// Line (1-indexed) where the inner C++ body content begins.
    pub body_line: usize,
    /// Column (1-indexed) where the inner C++ body content begins.
    pub body_col: usize,
    /// The unformatted inner C++ code string slice.
    pub body_text: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatchedMacro {
    Cpp(MacroKind),
    Quote,
}

#[derive(Debug, Clone, Copy)]
struct SpannedToken {
    kind: TokenKind,
    start: usize,
    len: usize,
    line: usize,
    col: usize,
}

impl SpannedToken {
    fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.start + self.len]
    }
}

/// Token-level parser that traverses Rust source text to identify and extract macro bodies.
pub struct MacroParser<'s, 'f> {
    source: &'s str,
    file_name: &'f str,
    tokens: Vec<SpannedToken>,
    cursor: usize,
}

impl<'s, 'f> MacroParser<'s, 'f> {
    pub fn new(source: &'s str, file_name: &'f str) -> Self {
        let mut tokens = Vec::new();
        let mut offset = 0;
        let mut line = 1;
        let mut col = 1;

        for token in ra_ap_rustc_lexer::tokenize(source, FrontmatterAllowed::No) {
            let len = token.len as usize;
            let tok_start = offset;
            let tok_line = line;
            let tok_col = col;

            let text = &source[tok_start..tok_start + len];
            for c in text.chars() {
                if c == '\n' {
                    line += 1;
                    col = 1;
                } else {
                    col += 1;
                }
            }
            offset += len;

            tokens.push(SpannedToken {
                kind: token.kind,
                start: tok_start,
                len,
                line: tok_line,
                col: tok_col,
            });
        }

        Self { source, file_name, tokens, cursor: 0 }
    }

    fn peek(&self) -> Option<&SpannedToken> {
        self.tokens.get(self.cursor)
    }

    fn advance(&mut self) {
        if self.cursor < self.tokens.len() {
            self.cursor += 1;
        }
    }

    fn eat_whitespace(&mut self) {
        while let Some(t) = self.peek()
            && t.kind == TokenKind::Whitespace
        {
            self.advance();
        }
    }

    fn try_eat_path_separator_at(&self, idx: &mut usize) -> bool {
        if *idx + 1 < self.tokens.len()
            && self.tokens[*idx].kind == TokenKind::Colon
            && self.tokens[*idx + 1].kind == TokenKind::Colon
        {
            *idx += 2;
            true
        } else {
            false
        }
    }

    fn try_eat(&mut self, expected: TokenKind) -> bool {
        if let Some(t) = self.peek()
            && t.kind == expected
        {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Checks if the tokens starting at `cursor` form an embedded C++ macro invocation path
    /// or a `quote!` / `quote_spanned!` macro invocation path ending in `!`.
    /// Returns `(MatchedMacro, token_count)`.
    fn check_macro_path(&self) -> Option<(MatchedMacro, usize)> {
        let mut idx = self.cursor;
        let mut last_ident = None;

        // Optional leading `::`
        self.try_eat_path_separator_at(&mut idx);

        while idx < self.tokens.len() {
            let token = &self.tokens[idx];
            if token.kind != TokenKind::Ident {
                return None;
            }

            last_ident = Some(token.text(self.source));
            idx += 1;

            // Path separator `::`
            if !self.try_eat_path_separator_at(&mut idx) {
                break;
            }
        }

        let matched = match last_ident? {
            "inline_cpp" => MatchedMacro::Cpp(MacroKind::InlineCpp),
            "global_cpp" => MatchedMacro::Cpp(MacroKind::GlobalCpp),
            "quote" | "quote_spanned" => MatchedMacro::Quote,
            ident if ident == MacroKind::DoNotSubmitCppDecl.name() => {
                MatchedMacro::Cpp(MacroKind::DoNotSubmitCppDecl)
            }
            _ => return None,
        };

        // Skip optional whitespace before `!`
        let mut bang_idx = idx;
        while bang_idx < self.tokens.len() && self.tokens[bang_idx].kind == TokenKind::Whitespace {
            bang_idx += 1;
        }

        // Ensure this is a macro invocation ending in `!`
        if bang_idx < self.tokens.len() && self.tokens[bang_idx].kind == TokenKind::Bang {
            Some((matched, bang_idx + 1 - self.cursor))
        } else {
            None
        }
    }

    /// Skips a balanced `{ ... }`, `( ... )`, or `[ ... ]` delimiter group if present at `cursor`.
    fn skip_delimited_group(&mut self) {
        let (open_kind, close_kind) = match self.peek().map(|t| t.kind) {
            Some(TokenKind::OpenBrace) => (TokenKind::OpenBrace, TokenKind::CloseBrace),
            Some(TokenKind::OpenParen) => (TokenKind::OpenParen, TokenKind::CloseParen),
            Some(TokenKind::OpenBracket) => (TokenKind::OpenBracket, TokenKind::CloseBracket),
            _ => return,
        };
        self.advance();
        let mut depth = 1usize;
        while let Some(t) = self.peek() {
            let kind = t.kind;
            self.advance();
            if kind == open_kind {
                depth += 1;
            } else if kind == close_kind {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
        }
    }

    /// Advances the parser and extracts the next macro block if one exists.
    pub fn next_macro(&mut self) -> Result<Option<ParsedMacro<'s>>, String> {
        while self.peek().is_some() {
            let (kind, macro_tokens_count) = match self.check_macro_path() {
                Some((MatchedMacro::Cpp(kind), count)) => (kind, count),
                Some((MatchedMacro::Quote, count)) => {
                    for _ in 0..count {
                        self.advance();
                    }
                    self.eat_whitespace();
                    self.skip_delimited_group();
                    continue;
                }
                None => {
                    self.advance();
                    continue;
                }
            };

            let macro_tok = self.tokens[self.cursor];
            let macro_start_offset = macro_tok.start;
            let macro_line = macro_tok.line;
            let macro_col = macro_tok.col;

            for _ in 0..macro_tokens_count {
                self.advance();
            }
            self.eat_whitespace();

            // Expect `{`
            if !self.try_eat(TokenKind::OpenBrace) {
                continue;
            }

            let open_brace = self.tokens[self.cursor - 1];
            let body_start_offset = open_brace.start + open_brace.len;

            // Determine line/column of the first body character
            let (body_line, body_col) = match self.peek() {
                Some(t) => (t.line, t.col),
                None => (open_brace.line, open_brace.col + open_brace.len),
            };

            let mut depth = 1;
            while let Some(t) = self.peek() {
                let token_kind = t.kind;
                let token_start = t.start;
                self.advance();

                if token_kind == TokenKind::OpenBrace {
                    depth += 1;
                } else if token_kind == TokenKind::CloseBrace {
                    depth -= 1;
                    if depth == 0 {
                        let body_end_offset = token_start;
                        let body_text = &self.source[body_start_offset..body_end_offset];
                        return Ok(Some(ParsedMacro {
                            kind,
                            macro_start_offset,
                            macro_line,
                            macro_col,
                            body_start_offset,
                            body_end_offset,
                            body_line,
                            body_col,
                            body_text,
                        }));
                    }
                }
            }

            let context = self
                .source
                .lines()
                .skip(macro_line.saturating_sub(3))
                .take(5)
                .collect::<Vec<_>>()
                .join("\n");
            let loc = if self.file_name.is_empty() {
                format!("line {macro_line}")
            } else {
                format!("{}:{macro_line}", self.file_name)
            };
            return Err(format!(
                "Unmatched delimiter starting at {loc}: Context around open brace:\n{context}"
            ));
        }

        Ok(None)
    }
}

/// Parses all embedded C++ macro invocation blocks in `source`.
pub fn parse_inline_cpp_macros<'a>(
    source: &'a str,
    file_name: &str,
) -> Result<Vec<ParsedMacro<'a>>, String> {
    let mut parser = MacroParser::new(source, file_name);
    let mut macros = Vec::new();
    while let Some(m) = parser.next_macro()? {
        macros.push(m);
    }
    Ok(macros)
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_parse_all_macro_kinds() {
        let source = r#"
            ::inline_cpp_macro::inline_cpp! { (int a, int b) -> int { return a + b; } }
            global_cpp! { int g = 1; }
            DO_NOT_SUBMIT_CPP_DECL! { void foo(); }
        "#;
        let macros = parse_inline_cpp_macros(source, "test.rs").unwrap();
        expect_eq!(macros.len(), 3);
        expect_eq!(macros[0].kind, MacroKind::InlineCpp);
        expect_eq!(macros[1].kind, MacroKind::GlobalCpp);
        expect_eq!(macros[2].kind, MacroKind::DoNotSubmitCppDecl);
    }

    #[gtest]
    fn test_ignores_non_macro_identifiers_and_comments() {
        let source = r#"
            // inline_cpp! { not a macro }
            let s = "global_cpp! { in string }";
            let inline_cpp = true;
            let global_cpp = 42;
            let DO_NOT_SUBMIT_CPP_DECL = "decl";
            inline_cpp! { (int x) -> int { return x * 2; } }
        "#;
        let macros = parse_inline_cpp_macros(source, "test.rs").unwrap();
        expect_eq!(macros.len(), 1);
        expect_eq!(macros[0].kind, MacroKind::InlineCpp);
    }

    #[gtest]
    fn test_captures_correct_line_and_column() {
        let source = r#"global_cpp! { int x = 1; }"#;
        let macros = parse_inline_cpp_macros(source, "test.rs").unwrap();
        expect_eq!(macros.len(), 1);
        expect_eq!(macros[0].macro_line, 1);
        expect_eq!(macros[0].macro_col, 1);
        expect_eq!(macros[0].body_line, 1);
        expect_eq!(macros[0].body_col, 14);
        expect_eq!(macros[0].body_text, r#" int x = 1; "#);
    }

    #[gtest]
    fn test_nested_braces_in_multiline_block() {
        let source = r#"
            global_cpp! {
                struct S {
                    int a;
                    int b;
                };
            }
        "#;
        let macros = parse_inline_cpp_macros(source, "test.rs").unwrap();
        expect_eq!(macros.len(), 1);
        expect_eq!(macros[0].kind, MacroKind::GlobalCpp);
    }

    #[gtest]
    fn test_unclosed_block_error_with_context() {
        let source = r#"
            line 1
            line 2
            global_cpp! { int x;
            line 4
            line 5
        "#;
        let err = parse_inline_cpp_macros(source, "test.rs").unwrap_err();
        expect_that!(
            err.as_str(),
            contains_substring(
                r#"Unmatched delimiter starting at test.rs:4: Context around open brace:"#
            )
        );
    }

    #[gtest]
    fn test_ignores_macros_inside_quote_blocks() {
        let source = r#"
            let q1 = quote! {
                unsafe {
                    (::inline_cpp_macro::inline_cpp! {
                        ( #( #param_types #param_idents ),* ) -> #return_type_name #body_block
                    })( #( #adjusted_thunk_args ),* )
                }
            };
            let q2 = ::quote::quote! {
                ::inline_cpp_macro::global_cpp! {
                    #parsed_tokens
                }
            };
            let q3 = quote::quote_spanned! (span=> inline_cpp! { () -> void {} });
            inline_cpp! { (int x) -> int { return x + 1; } }
        "#;
        let macros = parse_inline_cpp_macros(source, "test.rs").unwrap();
        expect_eq!(macros.len(), 1);
        expect_eq!(macros[0].kind, MacroKind::InlineCpp);
        expect_eq!(macros[0].body_text, " (int x) -> int { return x + 1; } ");
    }
}
