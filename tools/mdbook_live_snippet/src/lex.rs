// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! A lightweight lexer and definition finder for the example files used in
//! Crubit's docs.
//!
//! This is not a parser: it only needs to find top-level-ish definitions in
//! small, well-formatted example files, approximating Code Search's
//! `function:`, `class:`, and `symbol:` queries.

use crate::{Def, DefKind};
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TokenKind {
    Ident,
    Punct,
    Literal,
    /// A C++ preprocessor directive, including its continuation lines.
    Directive,
}

#[derive(Clone, Copy, Debug)]
struct Token<'a> {
    kind: TokenKind,
    text: &'a str,
    start: usize,
    end: usize,
}

impl Token<'_> {
    fn is(&self, text: &str) -> bool {
        self.kind != TokenKind::Literal && self.text == text
    }
}

fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn is_ident_continue(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

/// Splits C++ or Rust source into tokens, dropping whitespace and comments.
///
/// Punctuation is split into single characters, except for `::` and `->`.
fn lex_c_family(source: &str, is_rust: bool) -> Vec<Token<'_>> {
    let mut tokens = vec![];
    let mut i = 0;
    let mut at_line_start = true;
    while i < source.len() {
        let c = source[i..].chars().next().unwrap();
        let rest = &source[i..];
        let start = i;
        if c == '\n' {
            at_line_start = true;
            i += 1;
            continue;
        }
        if c.is_whitespace() {
            i += c.len_utf8();
            continue;
        }
        let was_line_start = std::mem::replace(&mut at_line_start, false);
        let kind = if rest.starts_with("//") {
            i += rest.find('\n').unwrap_or(rest.len());
            continue;
        } else if let Some(after) = rest.strip_prefix("/*") {
            i += after.find("*/").map_or(rest.len(), |end| end + 4);
            continue;
        } else if c == '#' && !is_rust && was_line_start {
            let mut end = i;
            loop {
                let line_end = source[end..].find('\n').map_or(source.len(), |e| end + e);
                if line_end < source.len() && source[..line_end].ends_with('\\') {
                    end = line_end + 1;
                } else {
                    i = line_end;
                    break;
                }
            }
            TokenKind::Directive
        } else if c == '"' {
            i = skip_quoted(source, i, b'"');
            TokenKind::Literal
        } else if c == '\'' {
            let after = &rest[1..];
            let ident_len = after.find(|c| !is_ident_continue(c)).unwrap_or(after.len());
            let is_lifetime = is_rust
                && after.starts_with(is_ident_start)
                && !after[ident_len..].starts_with('\'');
            if is_lifetime {
                i += 1 + ident_len;
                TokenKind::Ident
            } else {
                i = skip_quoted(source, i, b'\'');
                TokenKind::Literal
            }
        } else if c.is_ascii_digit() {
            i += rest.find(|c: char| !(is_ident_continue(c) || c == '.')).unwrap_or(rest.len());
            TokenKind::Literal
        } else if is_ident_start(c) {
            i += rest.find(|c| !is_ident_continue(c)).unwrap_or(rest.len());
            let ident = &source[start..i];
            match raw_string_end(source, i, ident, is_rust) {
                Some(end) => {
                    i = end;
                    TokenKind::Literal
                }
                None => TokenKind::Ident,
            }
        } else if rest.starts_with("::") || rest.starts_with("->") {
            i += 2;
            TokenKind::Punct
        } else {
            i += c.len_utf8();
            TokenKind::Punct
        };
        tokens.push(Token { kind, text: &source[start..i], start, end: i });
    }
    tokens
}

/// Returns the end of the quoted literal starting at `start`.
fn skip_quoted(source: &str, start: usize, quote: u8) -> usize {
    let bytes = source.as_bytes();
    let mut i = start + 1;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 2,
            b if b == quote => return i + 1,
            b'\n' if quote == b'\'' => return i,
            _ => i += 1,
        }
    }
    bytes.len()
}

/// If the identifier `prefix` ending at `i` starts a raw string literal,
/// returns the end of the literal.
fn raw_string_end(source: &str, i: usize, prefix: &str, is_rust: bool) -> Option<usize> {
    let rest = &source[i..];
    if is_rust && matches!(prefix, "r" | "br" | "cr") {
        let hashes = rest.chars().take_while(|&c| c == '#').count();
        if !rest[hashes..].starts_with('"') {
            return None;
        }
        let terminator = format!("\"{}", "#".repeat(hashes));
        let body = i + hashes + 1;
        return Some(
            source[body..].find(&terminator).map_or(source.len(), |e| body + e + terminator.len()),
        );
    }
    if !is_rust && prefix.ends_with('R') && rest.starts_with('"') {
        let delimiter = &rest[1..rest.find('(')?];
        let terminator = format!("){delimiter}\"");
        let body = i + 1 + delimiter.len() + 1;
        return Some(
            source[body..].find(&terminator).map_or(source.len(), |e| body + e + terminator.len()),
        );
    }
    None
}

/// Returns the index of the token matching the open bracket at `open`.
fn matching_close(tokens: &[Token], open: usize) -> Option<usize> {
    let (open_text, close_text) = match tokens[open].text {
        "(" => ("(", ")"),
        "[" => ("[", "]"),
        "{" => ("{", "}"),
        "<" => ("<", ">"),
        _ => return None,
    };
    let mut depth = 0;
    for (i, token) in tokens.iter().enumerate().skip(open) {
        if token.is(open_text) {
            depth += 1;
        } else if token.is(close_text) {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        } else if open_text == "<" && (token.is(";") || token.is("{") || token.is("}")) {
            return None;
        }
    }
    None
}

/// Returns the index of the token matching the close bracket at `close`.
fn matching_open(tokens: &[Token], close: usize) -> Option<usize> {
    let (open_text, close_text) = match tokens[close].text {
        ")" => ("(", ")"),
        "]" => ("[", "]"),
        "}" => ("{", "}"),
        ">" => ("<", ">"),
        _ => return None,
    };
    let mut depth = 0;
    for i in (0..=close).rev() {
        let token = &tokens[i];
        if token.is(close_text) {
            depth += 1;
        } else if token.is(open_text) {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        } else if open_text == "<" && (token.is(";") || token.is("{") || token.is("}")) {
            return None;
        }
    }
    None
}

/// Keywords that can't precede the name of a function being declared.
const NON_DECLARATION_KEYWORDS: &[&str] = &[
    "return",
    "co_return",
    "co_await",
    "co_yield",
    "throw",
    "new",
    "delete",
    "else",
    "case",
    "goto",
    "sizeof",
    "if",
    "while",
    "for",
    "switch",
    "do",
    "let",
    "match",
    "await",
    "in",
];

const CPP_CLASS_KEYWORDS: &[&str] = &["class", "struct", "union", "enum"];

/// Walks backwards from the token before `i` over a declaration prefix (types,
/// qualifiers, attributes, templates, and macro invocations), returning the
/// index of the first token of the declaration, or `None` if the tokens
/// before `i` don't look like a declaration prefix.
fn declaration_start(tokens: &[Token], i: usize) -> Option<usize> {
    let mut start = i;
    while start > 0 {
        let prev = &tokens[start - 1];
        match prev.kind {
            TokenKind::Directive => break,
            TokenKind::Literal => {
                // `extern "C"`.
                if start >= 2 && tokens[start - 2].is("extern") {
                    start -= 1;
                    continue;
                }
                return None;
            }
            TokenKind::Ident => {
                if NON_DECLARATION_KEYWORDS.contains(&prev.text) {
                    return None;
                }
                start -= 1;
            }
            TokenKind::Punct => match prev.text {
                ";" | "{" | "}" => break,
                ":" => {
                    // An access specifier like `public:` ends the previous
                    // declaration; anything else is part of an expression.
                    if start >= 2
                        && ["public", "private", "protected"].contains(&tokens[start - 2].text)
                    {
                        break;
                    }
                    return None;
                }
                "::" | "*" | "&" | "~" | "!" | "#" => start -= 1,
                "," if inside_template_args(tokens, start - 1) => start -= 1,
                ")" | "]" | ">" => {
                    let open = matching_open(tokens, start - 1)?;
                    // A Rust inner attribute (`#![...]`) applies to the
                    // enclosing module, not to the following item.
                    if open >= 2 && tokens[open - 1].is("!") && tokens[open - 2].is("#") {
                        break;
                    }
                    start = open;
                }
                _ => return None,
            },
        }
    }
    Some(start)
}

/// Whether the token at `i` is inside a `<...>` group within the current
/// declaration (e.g. the comma in `template <typename T, typename U>`).
fn inside_template_args(tokens: &[Token], i: usize) -> bool {
    let mut depth = 0;
    for token in tokens[..i].iter().rev() {
        if token.is(">") {
            depth += 1;
        } else if token.is("<") {
            if depth == 0 {
                return true;
            }
            depth -= 1;
        } else if token.is(";") || token.is("{") || token.is("}") || token.is("(") || token.is(")")
        {
            return false;
        }
    }
    false
}

/// Returns the index of the token ending the item that contains the token at
/// `i`: the first `;` at bracket depth 0, or the `}` matching the first `{`
/// (plus a following `;`, for C++ classes).
fn item_end(tokens: &[Token], i: usize) -> Option<usize> {
    let mut j = i;
    while j < tokens.len() {
        let token = &tokens[j];
        if token.is(";") {
            return Some(j);
        } else if token.is("{") {
            let close = matching_close(tokens, j)?;
            return Some(if tokens.get(close + 1).is_some_and(|t| t.is(";")) {
                close + 1
            } else {
                close
            });
        } else if token.is("(") || token.is("[") {
            j = matching_close(tokens, j)?;
        } else if token.is("}") {
            return None;
        }
        j += 1;
    }
    None
}

/// Finds the definitions and declarations of functions, classes, and other
/// symbols in C++ or Rust source.
pub(crate) fn c_family_definitions(source: &str, is_rust: bool) -> Vec<Def> {
    let tokens = lex_c_family(source, is_rust);
    let mut defs = vec![];
    for i in 0..tokens.len() {
        if let Some(name) = defined_macro(&tokens[i]) {
            defs.push(Def {
                name: name.to_owned(),
                kind: DefKind::Other,
                span: tokens[i].start..tokens[i].end,
            });
            continue;
        }
        if tokens[i].kind != TokenKind::Ident {
            continue;
        }
        let found = if is_rust { rust_definition(&tokens, i) } else { cpp_definition(&tokens, i) };
        let Some((kind, start)) = found else { continue };
        let Some(end) = item_end(&tokens, i) else { continue };
        defs.push(Def {
            name: tokens[i].text.to_owned(),
            kind,
            span: tokens[start].start..tokens[end].end,
        });
    }
    defs
}

/// If `token` is a `#define` directive, returns the name of the macro.
fn defined_macro<'a>(token: &Token<'a>) -> Option<&'a str> {
    if token.kind != TokenKind::Directive {
        return None;
    }
    let rest = token.text.strip_prefix('#')?.trim_start().strip_prefix("define")?;
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }
    let rest = rest.trim_start();
    let len = rest.find(|c| !is_ident_continue(c)).unwrap_or(rest.len());
    (len > 0).then(|| &rest[..len])
}

/// If the identifier at `i` names a Rust item, returns its kind and the index
/// of its first token.
fn rust_definition(tokens: &[Token], i: usize) -> Option<(DefKind, usize)> {
    let keyword = tokens.get(i.checked_sub(1)?)?;
    let kind = match keyword.text {
        "fn" => DefKind::Function,
        "struct" | "enum" | "union" | "trait" => DefKind::Class,
        "type" | "const" | "static" | "mod" => DefKind::Other,
        _ => return None,
    };
    if keyword.kind != TokenKind::Ident {
        return None;
    }
    Some((kind, declaration_start(tokens, i - 1)?))
}

/// If the identifier at `i` names a C++ declaration, returns its kind and the
/// index of its first token.
fn cpp_definition(tokens: &[Token], i: usize) -> Option<(DefKind, usize)> {
    let name = &tokens[i];
    if NON_DECLARATION_KEYWORDS.contains(&name.text) || CPP_CLASS_KEYWORDS.contains(&name.text) {
        return None;
    }
    let next = tokens.get(i + 1);
    if next.is_some_and(|t| t.is("(")) {
        return cpp_function(tokens, i);
    }
    // `using Name = ...;` and `namespace Name {`.
    if i > 0
        && (tokens[i - 1].is("namespace")
            || (tokens[i - 1].is("using") && next.is_some_and(|t| t.is("="))))
    {
        return Some((DefKind::Other, declaration_start(tokens, i - 1)?));
    }
    cpp_class(tokens, i)
}

fn cpp_function(tokens: &[Token], i: usize) -> Option<(DefKind, usize)> {
    let close = matching_close(tokens, i + 1)?;
    // After the parameter list, a declaration may only have qualifiers, a
    // trailing return type, a constructor initializer list, or `= default`
    // etc. before its body or `;`.
    let mut j = close + 1;
    let has_body = loop {
        let token = tokens.get(j)?;
        match token.text {
            "{" if token.kind == TokenKind::Punct => break true,
            ";" | "=" | ":" if token.kind == TokenKind::Punct => break false,
            "(" | "[" if token.kind == TokenKind::Punct => j = matching_close(tokens, j)? + 1,
            "->" | "&" | "*" | "::" | "<" | ">" => j += 1,
            _ if token.kind == TokenKind::Ident => j += 1,
            _ => return None,
        }
    };
    let start = declaration_start(tokens, i)?;
    // Without a return type, only accept things with a body, like
    // `TEST(Foo, Bar) { ... }`, to avoid matching calls.
    if start == i && !has_body {
        return None;
    }
    Some((DefKind::Function, start))
}

fn cpp_class(tokens: &[Token], i: usize) -> Option<(DefKind, usize)> {
    // Walk back over the class head to the class keyword, e.g.
    // `struct CRUBIT_INTERNAL_RUST_TYPE("...") alignas(4) [[...]] Name`.
    let mut j = i;
    loop {
        let prev = tokens.get(j.checked_sub(1)?)?;
        j -= 1;
        match prev.kind {
            TokenKind::Ident if CPP_CLASS_KEYWORDS.contains(&prev.text) => break,
            TokenKind::Ident => {}
            TokenKind::Punct if prev.is("::") => {}
            TokenKind::Punct if prev.is(")") || prev.is("]") => j = matching_open(tokens, j)?,
            _ => return None,
        }
    }
    // The name must be followed by the class body, possibly after template
    // arguments, `final`, or base classes.
    let mut k = i + 1;
    loop {
        let token = tokens.get(k)?;
        match token.kind {
            TokenKind::Punct if token.is("{") => break,
            TokenKind::Punct if token.is("<") => k = matching_close(tokens, k)? + 1,
            TokenKind::Punct if token.is(":") || token.is("::") || token.is(",") => k += 1,
            TokenKind::Ident => k += 1,
            _ => return None,
        }
    }
    Some((DefKind::Class, declaration_start(tokens, j)?))
}

/// Splits Starlark source into tokens, dropping whitespace and comments.
fn lex_starlark(source: &str) -> Vec<Token<'_>> {
    let mut tokens = vec![];
    let mut i = 0;
    while i < source.len() {
        let c = source[i..].chars().next().unwrap();
        let rest = &source[i..];
        let start = i;
        let kind = if c.is_whitespace() {
            i += c.len_utf8();
            continue;
        } else if c == '#' {
            i += rest.find('\n').unwrap_or(rest.len());
            continue;
        } else if rest.starts_with("\"\"\"") || rest.starts_with("'''") {
            let quote = &rest[..3];
            i += 3 + rest[3..].find(quote).map_or(rest.len() - 3, |e| e + 3);
            TokenKind::Literal
        } else if c == '"' || c == '\'' {
            i = skip_quoted(source, i, c as u8);
            TokenKind::Literal
        } else if is_ident_continue(c) {
            i += rest.find(|c| !is_ident_continue(c)).unwrap_or(rest.len());
            TokenKind::Ident
        } else {
            i += c.len_utf8();
            TokenKind::Punct
        };
        tokens.push(Token { kind, text: &source[start..i], start, end: i });
    }
    tokens
}

/// Finds the targets (calls with a `name = "..."` argument) in Starlark
/// source. Each target spans from the rule name to the closing parenthesis.
pub(crate) fn starlark_targets(source: &str) -> Vec<Def> {
    let tokens = lex_starlark(source);
    let mut defs = vec![];
    let mut open_parens: Vec<usize> = vec![];
    for (i, token) in tokens.iter().enumerate() {
        if token.is("(") {
            open_parens.push(i);
        } else if token.is(")") {
            open_parens.pop();
        }
        let is_name_arg = token.is("name")
            && open_parens.len() == 1
            && tokens.get(i + 1).is_some_and(|t| t.is("="))
            && tokens.get(i + 2).is_some_and(|t| t.kind == TokenKind::Literal);
        if !is_name_arg {
            continue;
        }
        let open = open_parens[0];
        let Some(close) = matching_close(&tokens, open) else { continue };
        // Include dotted rule names, like `native.cc_library`.
        let mut start = open;
        while start > 0 && (tokens[start - 1].kind == TokenKind::Ident || tokens[start - 1].is("."))
        {
            start -= 1;
        }
        let value = tokens[i + 2].text;
        defs.push(Def {
            name: value.trim_matches(|c| c == '"' || c == '\'').to_owned(),
            kind: DefKind::Other,
            span: span(&tokens, start..close),
        });
    }
    defs
}

fn span(tokens: &[Token], range: Range<usize>) -> Range<usize> {
    tokens[range.start].start..tokens[range.end].end
}
