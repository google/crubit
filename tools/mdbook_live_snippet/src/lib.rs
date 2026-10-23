// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Expands docs-style `live-snippet` blocks for mdBook.
//!
//! Crubit's docs embed example code with blocks like:
//!
//! ````markdown
//! ```live-snippet
//! cs/file:examples/cpp/function/example.h function:add_two_integers
//! ```
//! ````
//!
//! The path is relative to the root of the Crubit repository (the mdBook
//! root). The optional query selects part of the file, approximating the
//! Code Search semantics that docs uses:
//!
//! * (none): the whole file, minus the license header.
//! * `content:<regex>`: every line touched by a match of the regex. As in docs, the regex is
//!   matched against the whole file, `^` and `$` match at line boundaries, and `.` matches newlines
//!   (use `[^\n]` to stay on one line).
//! * `function:<regex>`, `class:<regex>`, `symbol:<regex>`: the full definitions (or declarations)
//!   of the functions, classes, or any symbols whose name fully matches the regex, including
//!   directly preceding comments and attributes. For `BUILD` and `.bzl` files, all queries select
//!   the targets whose `name` matches.
//!
//! Every snippet must resolve to a non-empty result; otherwise the
//! preprocessor fails, so that broken snippets break the docs build instead of
//! silently rendering the wrong code.

mod lex;

use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use serde_json::{Map, Value};
use std::ops::Range;
use std::path::{Component, Path};

/// Runs the preprocessor on mdBook's `[context, book]` JSON input, returning
/// the processed book as JSON.
pub fn run(input: &str) -> Result<String> {
    let (context, mut book): (Value, Value) =
        serde_json::from_str(input).context("failed to parse mdBook input")?;
    let root = context
        .get("root")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("mdBook context has no `root`"))?;
    let root = Path::new(root);
    let mut errors = vec![];
    for_each_chapter(&mut book, &mut |chapter| {
        let name = ["source_path", "path", "name"]
            .iter()
            .find_map(|key| chapter.get(*key).and_then(Value::as_str))
            .unwrap_or("<unknown chapter>")
            .to_owned();
        if let Some(Value::String(content)) = chapter.get_mut("content") {
            match expand_live_snippets(content, root) {
                Ok(expanded) => *content = expanded,
                Err(e) => errors.push(format!("in {name}: {e:#}")),
            }
        }
    });
    if !errors.is_empty() {
        bail!("{}", errors.join("\n"));
    }
    Ok(serde_json::to_string(&book)?)
}

/// Calls `f` on every chapter in the book.
///
/// This walks the JSON generically rather than depending on the exact shape
/// of mdBook's `Book` type, which differs between mdBook versions.
fn for_each_chapter(value: &mut Value, f: &mut dyn FnMut(&mut Map<String, Value>)) {
    match value {
        Value::Object(map) => {
            if let Some(Value::Object(chapter)) = map.get_mut("Chapter") {
                f(chapter);
            }
            for v in map.values_mut() {
                for_each_chapter(v, f);
            }
        }
        Value::Array(items) => {
            for v in items {
                for_each_chapter(v, f);
            }
        }
        _ => {}
    }
}

/// Replaces every `live-snippet` block in `markdown` with a code block
/// containing the selected code. Paths are resolved relative to `root`.
pub fn expand_live_snippets(markdown: &str, root: &Path) -> Result<String> {
    let lines: Vec<&str> = markdown.split_inclusive('\n').collect();
    let mut out = String::with_capacity(markdown.len());
    let mut errors = vec![];
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_end();
        let indent = &trimmed[..trimmed.len() - trimmed.trim_start().len()];
        if trimmed.trim_start() != "```live-snippet" {
            out.push_str(line);
            i += 1;
            continue;
        }
        let query = lines.get(i + 1).map(|l| l.trim());
        let close = lines.get(i + 2).map(|l| l.trim());
        let (Some(query), Some("```")) = (query, close) else {
            errors.push(format!(
                "line {}: a `live-snippet` block must contain exactly one line",
                i + 1
            ));
            out.push_str(line);
            i += 1;
            continue;
        };
        match render_snippet(query, root) {
            Ok((lang, code)) => out.push_str(&code_block(indent, lang, &code)),
            Err(e) => errors.push(format!("line {}: `{query}`: {e:#}", i + 2)),
        }
        i += 3;
    }
    if !errors.is_empty() {
        bail!("{}", errors.join("\n"));
    }
    Ok(out)
}

/// Formats `code` as a fenced code block indented by `indent`.
fn code_block(indent: &str, lang: &str, code: &str) -> String {
    // Use a fence longer than any backtick fence inside the code.
    let longest_backtick_run = code
        .lines()
        .map(|l| l.trim_start().chars().take_while(|&c| c == '`').count())
        .max()
        .unwrap_or(0);
    let fence = "`".repeat(std::cmp::max(3, longest_backtick_run + 1));
    let mut out = format!("{indent}{fence}{lang}\n");
    for line in code.lines() {
        if !line.trim().is_empty() {
            out.push_str(indent);
            out.push_str(line);
        }
        out.push('\n');
    }
    out.push_str(&format!("{indent}{fence}\n"));
    out
}

/// Resolves a single snippet line like `cs/file:<path> <query>`, returning
/// the code block language and the selected code.
fn render_snippet(snippet: &str, root: &Path) -> Result<(&'static str, String)> {
    let Some(rest) = snippet.strip_prefix("cs/file:") else {
        bail!("expected `cs/file:<path> [query]`");
    };
    let (path, query) = match rest.split_once(char::is_whitespace) {
        Some((path, query)) => (path, query.trim()),
        None => (rest, ""),
    };
    let rel_path = Path::new(path);
    if !rel_path.components().all(|c| matches!(c, Component::Normal(_))) {
        bail!("path must be relative to the repository root");
    }
    let file = root.join(rel_path);
    let source = std::fs::read_to_string(&file)
        .with_context(|| format!("failed to read {}", file.display()))?;
    let lang = Lang::from_path(path);
    let code = select(&source, lang, query)?;
    Ok((lang.code_block_name(), code))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lang {
    Rust,
    Cpp,
    Starlark,
    Other,
}

impl Lang {
    fn from_path(path: &str) -> Lang {
        let file_name = path.rsplit('/').next().unwrap_or(path);
        let extension = file_name.rsplit_once('.').map(|(_, ext)| ext);
        match (file_name, extension) {
            (_, Some("rs")) => Lang::Rust,
            (_, Some("h" | "hh" | "hpp" | "cc" | "cpp" | "cxx")) => Lang::Cpp,
            ("BUILD" | "WORKSPACE", _) | (_, Some("bzl" | "bazel")) => Lang::Starlark,
            _ => Lang::Other,
        }
    }

    fn code_block_name(self) -> &'static str {
        match self {
            // Snippets depend on Crubit, so they can't run on the Rust playground.
            Lang::Rust => "rust,noplayground",
            Lang::Cpp => "cpp",
            // highlight.js has no Starlark mode; Python is close enough.
            Lang::Starlark => "python",
            Lang::Other => "",
        }
    }
}

/// Returns the part of `source` selected by `query`.
fn select(source: &str, lang: Lang, query: &str) -> Result<String> {
    let lines: Vec<&str> = source.lines().collect();
    let header_lines = license_header_lines(&lines);
    if query.is_empty() {
        return Ok(render(&lines, std::slice::from_ref(&(header_lines..lines.len()))));
    }
    let Some((kind, pattern)) = query.split_once(':') else {
        bail!("unsupported query `{query}`");
    };
    let pattern = normalize_regex(pattern);
    let regions = match kind {
        "content" => content_regions(source, &pattern)?,
        "function" | "class" | "symbol" => {
            let kind = match kind {
                "function" => Some(DefKind::Function),
                "class" => Some(DefKind::Class),
                _ => None,
            };
            let name_re = Regex::new(&format!("^(?:{pattern})$"))
                .with_context(|| format!("invalid regex `{pattern}`"))?;
            let defs = match lang {
                Lang::Rust | Lang::Cpp => lex::c_family_definitions(source, lang == Lang::Rust),
                Lang::Starlark => lex::starlark_targets(source),
                Lang::Other => bail!("`{query}` is not supported for this file type"),
            };
            defs.into_iter()
                .filter(|def| lang == Lang::Starlark || kind.is_none() || kind == Some(def.kind))
                .filter(|def| name_re.is_match(&def.name))
                .map(|def| {
                    let range = line_range(source, def.span);
                    let start = extend_over_leading_comments(&lines, range.start, lang);
                    std::cmp::max(start, header_lines)..range.end
                })
                .collect()
        }
        _ => bail!("unsupported query `{query}`"),
    };
    if regions.is_empty() {
        bail!("query matched nothing");
    }
    Ok(render(&lines, &regions))
}

/// Converts a Code Search regex to `regex` crate syntax.
fn normalize_regex(pattern: &str) -> String {
    // Code Search allows quoting the whole regex.
    let pattern = pattern.strip_prefix('"').and_then(|p| p.strip_suffix('"')).unwrap_or(pattern);
    // Code Search queries escape spaces, which the `regex` crate rejects.
    let mut out = String::with_capacity(pattern.len());
    let mut chars = pattern.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some(' ') => out.push(' '),
            Some(next) => {
                out.push('\\');
                out.push(next);
            }
            None => out.push('\\'),
        }
    }
    out
}

fn content_regions(source: &str, pattern: &str) -> Result<Vec<Range<usize>>> {
    // docs matches live-snippet regexes against the whole file, with `.`
    // matching newlines: Crubit's docs rely on this, e.g. with
    // `content:^inline.*?^\}`.
    let re = Regex::new(&format!("(?ms){pattern}"))
        .with_context(|| format!("invalid regex `{pattern}`"))?;
    Ok(re
        .find_iter(source)
        .filter(|m| !m.is_empty())
        .map(|m| line_range(source, m.start()..m.end()))
        .collect())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DefKind {
    Function,
    Class,
    Other,
}

/// A definition or declaration found in a source file.
#[derive(Debug)]
pub(crate) struct Def {
    pub(crate) name: String,
    pub(crate) kind: DefKind,
    /// The byte range of the definition, excluding leading comments.
    pub(crate) span: Range<usize>,
}

/// Returns the range of (0-based) line numbers touched by `span`.
fn line_range(source: &str, span: Range<usize>) -> Range<usize> {
    let line_of = |offset: usize| source[..offset].matches('\n').count();
    let last = std::cmp::max(span.start, span.end.saturating_sub(1));
    line_of(span.start)..line_of(last) + 1
}

/// Moves `start` up over directly preceding comment and attribute lines.
fn extend_over_leading_comments(lines: &[&str], mut start: usize, lang: Lang) -> usize {
    let prefixes: &[&str] = match lang {
        Lang::Rust => &["//", "#[", "/*", "*"],
        Lang::Cpp => &["//", "/*", "*"],
        Lang::Starlark => &["#"],
        Lang::Other => &[],
    };
    while start > 0 {
        let prev = lines[start - 1].trim_start();
        if prev.is_empty() || !prefixes.iter().any(|p| prev.starts_with(p)) {
            break;
        }
        start -= 1;
    }
    start
}

/// Returns the number of lines taken up by the license header (and the blank
/// lines after it) at the top of the file.
fn license_header_lines(lines: &[&str]) -> usize {
    const HEADER: [&str; 3] = [
        "Part of the Crubit project, under the Apache License v2.0 with LLVM",
        "Exceptions. See /LICENSE for license information.",
        "SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception",
    ];
    let is_header = lines.len() >= HEADER.len()
        && lines.iter().zip(HEADER).all(|(line, text)| {
            let line = line.trim();
            ["//", "#"].iter().any(|c| line.strip_prefix(c).map(str::trim) == Some(text))
        });
    if !is_header {
        return 0;
    }
    HEADER.len() + lines[HEADER.len()..].iter().take_while(|l| l.trim().is_empty()).count()
}

/// Renders the given line ranges. Ranges separated only by blank lines are
/// rendered as in the source; other gaps are rendered as a single blank line.
fn render(lines: &[&str], regions: &[Range<usize>]) -> String {
    let is_blank = |i: usize| lines[i].trim().is_empty();
    let mut regions: Vec<Range<usize>> = regions
        .iter()
        .map(|r| {
            // Drop blank lines at the edges, e.g. from a regex starting with `^\s*`.
            let mut r = r.clone();
            while r.start < r.end && is_blank(r.start) {
                r.start += 1;
            }
            while r.start < r.end && is_blank(r.end - 1) {
                r.end -= 1;
            }
            r
        })
        .filter(|r| !r.is_empty())
        .collect();
    regions.sort_by_key(|r| (r.start, r.end));
    let mut merged: Vec<Range<usize>> = vec![];
    for region in regions {
        if let Some(last) = merged.last_mut()
            && (region.start <= last.end || (last.end..region.start).all(is_blank))
        {
            last.end = std::cmp::max(last.end, region.end);
            continue;
        }
        merged.push(region);
    }
    let mut out = String::new();
    for (i, region) in merged.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        for line in &lines[region.clone()] {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests;
