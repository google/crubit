// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{bail, Context, Result};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io::Write as _;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;

// TODO(b/231320237): The `RustfmtConfig` struct should be replaced with
// `rustfmt_nightly::Config` once we switch to using rustfmt as a library
// (instead of invoking the `rustfmt` executable).
pub struct RustfmtConfig {
    /// Path to the `rustfmt` executable.
    exe_path: PathBuf,

    /// Cmdline arguments to be passed to the `rustfmt` executable.
    cmdline_args: Vec<OsString>,
}

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
            exe_path: PathBuf::from(external_binaries::RUSTFMT_EXE_PATH),
            cmdline_args: Self::default_cmdline_args(),
        }
    }

    fn cmdline_args_with_custom_config_path(rustfmt_config_path: &Path) -> Vec<OsString> {
        let mut config_path_arg: OsString = "--config-path=".into();
        config_path_arg.push(rustfmt_config_path);
        Self::append_config_overrides(vec![config_path_arg])
    }

    fn default_cmdline_args() -> Vec<OsString> {
        Self::append_config_overrides(vec![
            "--edition=2021".into(),
            "--config=style_edition=2024".into(),
        ])
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
    config: Option<&RustfmtConfig>,
) -> Result<String> {
    let mut tokens_string = tokens_to_string(tokens)?;
    // NOTE: This is a terrible hack. `rustfmt` became more strict about appearances of `...`
    // (the `DotDotDot` token) at some point in the past. This is not a precise or general
    // solution, but rewriting this token to a comment produces formattable code in some cases,
    // making test failure messages better.
    if tokens_string.contains("...") {
        tokens_string = tokens_string.replace("...", "/*...*/");
    }
    if let Some(config) = config {
        tokens_string = rustfmt(&tokens_string, config).with_context(|| {
            format!("Failed to rustfmt the following Rust tokens:\n\n{tokens_string}")
        })?;
    }
    Ok(tokens_string)
}

/// Like `tokens_to_string` but also runs the result through `rustfmt` and returns provenance.
pub fn rs_tokens_to_formatted_string_with_provenance(
    tokens: TokenStream,
    config: Option<&RustfmtConfig>,
) -> Result<(String, SubstringProvenanceMap)> {
    let (mut tokens_string, provenance_map) = tokens_to_string_with_provenance(tokens)?;
    // NOTE: This is a terrible hack. `rustfmt` became more strict about appearances of `...`
    // (the `DotDotDot` token) at some point in the past. This is not a precise or general
    // solution, but rewriting this token to a comment produces formattable code in some cases,
    // making test failure messages better.
    if tokens_string.contains("...") {
        tokens_string = tokens_string.replace("...", "/*...*/");
    }
    if let Some(config) = config {
        tokens_string = rustfmt(&tokens_string, config).with_context(|| {
            format!("Failed to rustfmt the following Rust tokens:\n\n{tokens_string}")
        })?;
    }
    Ok((tokens_string, provenance_map))
}

/// Like `rs_tokens_to_formatted_string`, but always using a Crubit-internal,
/// default rustfmt config.  This should only be called by tests - product code
/// should support custom `rustfmt.toml` and take the path to `rustfmt` binary
/// as a cmdline argument.
pub fn rs_tokens_to_formatted_string_for_tests(input: TokenStream) -> Result<String> {
    rs_tokens_to_formatted_string(input, Some(&RustfmtConfig::for_testing()))
}

/// Like `tokens_to_string` but also runs the result through `clang-format`.
pub fn cc_tokens_to_formatted_string(
    tokens: TokenStream,
    clang_format_exe_path: Option<&Path>,
) -> Result<String> {
    let mut result = tokens_to_string(tokens)?;
    if let Some(clang_format_exe_path) = clang_format_exe_path {
        result = clang_format(result, clang_format_exe_path)?;
    }
    Ok(result)
}

/// Like `tokens_to_string` but also runs the result through `clang-format` and returns provenance.
pub fn cc_tokens_to_formatted_string_with_provenance(
    tokens: TokenStream,
    clang_format_exe_path: Option<&Path>,
) -> Result<(String, SubstringProvenanceMap)> {
    let (mut result, provenance_map) = tokens_to_string_with_provenance(tokens)?;
    if let Some(clang_format_exe_path) = clang_format_exe_path {
        result = clang_format(result, clang_format_exe_path)?;
    }
    Ok((result, provenance_map))
}

/// Like `cc_tokens_to_formatted_string`, but always using a hardcoded path to
/// where the `clang-format` binary is in Crubit's test environment.  This
/// should only be called by tests - product code should take the path to the
/// `clang-format` binary as a cmdline argument.
pub fn cc_tokens_to_formatted_string_for_tests(tokens: TokenStream) -> Result<String> {
    clang_format(tokens_to_string(tokens)?, Path::new(external_binaries::CLANG_FORMAT_EXE_PATH))
}

/// Tracks the provenance of a substring in source code. Our goal is to attach provenance data
/// (here, `original_path`, `original_start` and `original_end`; these are provided by the
/// special `__CAPTURE_TAG__` token) to (single) tokens bracketed by `__CAPTURE_BEGIN__` and
/// `__CAPTURE_END__`. This is complicated because the tokens may be moved around by an external
/// formatter and because the textual content of the tokens may appear multiple times (we're
/// interested in a *particular* span of text after a `__CAPTURE_TAG__`, not any span of matching
/// text).
#[derive(Default, Clone)]
pub struct GeneratedOffsetsWithProvenance {
    /// The offset for the search start location.
    destination_search_start: usize,
    /// The destination offset start for this substring, before formatting.
    destination_start: usize,
    /// The destination offset end for this substring, before formatting.
    destination_end: usize,
    /// The provenance path for this substring.
    original_path: String,
    /// The uninterpreted starting offset for this substring.
    original_start: String,
    /// The uninterpreted ending offset for this substring.
    original_end: String,
}

type GeneratedOffsetsProvenanceMap = HashMap<usize, GeneratedOffsetsWithProvenance>;

#[derive(Debug, Default, Clone)]
pub struct SubstringWithProvenance {
    /// The substring to match. Empty `substring` values indicate a useless record.
    pub substring: String,
    /// The instance of `substring` to match, starting from `destination_search_start`.
    pub index: usize,
    /// The provenance path for this substring.
    pub original_path: String,
    /// The uninterpreted starting offset for this substring.
    pub original_start: String,
    /// The uninterpreted ending offset for this substring.
    pub original_end: String,
}

pub type SubstringProvenanceMap = HashMap<usize, SubstringWithProvenance>;

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct FormattedOffsetsWithProvenance {
    /// The formatted offset start for this substring.
    pub formatted_start: usize,
    /// The formatted offset end for this substring.
    pub formatted_end: usize,
    /// The provenance path for this substring.
    pub original_path: String,
    /// The uninterpreted starting offset for this substring.
    pub original_start: String,
    /// The uninterpreted ending offset for this substring.
    pub original_end: String,
}

/// Maps provenance-generating strings (i.e., "/// Generated by:") to their associated substring
/// records.
pub type FormattedProvenanceMap = HashMap<usize, FormattedOffsetsWithProvenance>;

/// Unescapes a string literal token (for __CAPTURE_TAG__).
fn clean_literal(lit: proc_macro2::Literal) -> String {
    // We'd like to use str_value, but that's only on procmacro2_semver_exempt.
    lit.to_string().trim_matches('"').replace("\\\"", "\"")
}

pub fn write_unformatted_tokens(
    result: &mut impl std::fmt::Write,
    tokens: TokenStream,
) -> Result<()> {
    write_unformatted_tokens_with_provenance(result, |_| None, tokens)?;
    Ok(())
}

struct TokenOutputState<'a, T: std::fmt::Write> {
    /// The result stream.
    result: &'a mut T,
    /// Function that when passed `result` returns its length (or None).
    cur_len: fn(&T) -> Option<usize>,
    /// The location of the last __CAPTURE_BEGIN__
    last_capture_begin: Option<usize>,
    /// The index of the current __CAPTURE_TAG__ (1-based).
    current_capture_index: usize,
    /// The current provenance entry being built.
    provenance_entry: GeneratedOffsetsWithProvenance,
    /// The provenance map to return.
    provenance_map: HashMap<usize, GeneratedOffsetsWithProvenance>,
}

/// State for `write_unformatted_tokens_with_provenance`.
impl<'a, T: std::fmt::Write> TokenOutputState<'a, T> {
    fn new(result: &'a mut T, cur_len: fn(&T) -> Option<usize>) -> Self {
        Self {
            result,
            cur_len,
            last_capture_begin: None,
            current_capture_index: 0,
            provenance_entry: GeneratedOffsetsWithProvenance::default(),
            provenance_map: HashMap::new(),
        }
    }

    /// Possibly recursive function to append `tokens` to this `TokenOutputState`.
    fn write_stream(&mut self, tokens: TokenStream) -> Result<()> {
        let mut it = tokens.into_iter().peekable();
        let mut tt_prev = None;
        while let Some(tt) = it.next() {
            match &tt {
                TokenTree::Ident(tt) if tt == "__NEWLINE__" => writeln!(self.result)?,
                TokenTree::Ident(tt) if tt == "__SPACE__" => write!(self.result, " ")?,
                TokenTree::Ident(tt) if tt == "__HASH_TOKEN__" => write!(self.result, "#")?,
                TokenTree::Ident(tt) if tt == "__CAPTURE_TAG__" => {
                    if let (
                        Some(TokenTree::Literal(original_path)),
                        Some(TokenTree::Literal(original_start)),
                        Some(TokenTree::Literal(original_end)),
                    ) = (it.next(), it.next(), it.next())
                    {
                        if let Some(search_start) = (self.cur_len)(self.result) {
                            // __CAPTURE_TAG__ srcpath srcofsstart srcofsend
                            self.current_capture_index += 1;
                            self.provenance_entry.original_path = clean_literal(original_path);
                            self.provenance_entry.original_start = clean_literal(original_start);
                            self.provenance_entry.original_end = clean_literal(original_end);
                            self.provenance_entry.destination_search_start = search_start;
                        }
                    } else {
                        bail!("__CAPTURE_TAG__ must be followed by three literals")
                    }
                }
                TokenTree::Ident(tt) if tt == "__CAPTURE_BEGIN__" => {
                    self.last_capture_begin = (self.cur_len)(self.result)
                }
                TokenTree::Ident(tt) if tt == "__CAPTURE_END__" => {
                    if let (Some(last_capture_begin), Some(last_capture_end)) =
                        (self.last_capture_begin, (self.cur_len)(self.result))
                    {
                        self.provenance_map.insert(
                            self.current_capture_index,
                            GeneratedOffsetsWithProvenance {
                                destination_start: last_capture_begin,
                                destination_end: last_capture_end,
                                ..self.provenance_entry.clone()
                            },
                        );
                    }
                }
                TokenTree::Ident(tt) if tt == "__COMMENT__" => {
                    if let Some(TokenTree::Literal(lit)) = it.next() {
                        writeln!(
                            self.result,
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
                            self.result,
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
                    write!(self.result, "{}", open_delimiter)?;
                    self.write_stream(tt.stream())?;
                    write!(self.result, "{}", closed_delimiter)?;
                }
                _ => {
                    write!(self.result, "{}", tt)?;

                    // Insert spaces between tokens when they are needed to separate tokens.
                    // In particular, `a b` is different than `ab`, and `: ::` is different from
                    // `:::`.
                    if let Some(tt_next) = it.peek() {
                        if tokens_require_whitespace(tt_prev.as_ref(), &tt, tt_next) {
                            write!(self.result, " ")?;
                        }
                    }
                }
            }
            tt_prev = Some(tt);
        }
        Ok(())
    }
}

/// Produces source code out of the token stream.
///
/// Notable features:
/// * quote! cannot produce a single `#` token (that is not immediately followed by `(`, `[`, `{`,
///   or variable interpolation). For cases when we need `#` to be produced in the C++ source code
///   use the placeholder `__HASH_TOKEN__`.
/// * The Rust tokenizer ignores newlines as they are not significant for Rust. For C++ they are
///   (for example there needs to be a newline after `#include "foo/bar.h"`). We are also using
///   explict newlines for making the generated Rust/C++ source code more readable. Use the
///   placeholder `__NEWLINE__` to insert a newline character.
/// * `TokenStream` cannot encode formatting whitespace, so we use the placeholder `__SPACE__`.
/// * `TokenStream` cannot encode comments, so we use the placeholder `__COMMENT__`, followed by a
///   string literal.
/// * Generates a `ProvenanceMap`: for each `__CAPTURE_TAG__ "path" "start" "end"`, counting from 1,
///   stores the offsets of the last (single) token seen bracketed by __CAPTURE_BEGIN__ and
///   __CAPTURE_END__ before the next __CAPTURE_TAG__. This requires a `cur_len` that returns the
///   current length of `result`.
fn write_unformatted_tokens_with_provenance<T: std::fmt::Write>(
    result: &mut T,
    cur_len: fn(&T) -> Option<usize>,
    tokens: TokenStream,
) -> Result<GeneratedOffsetsProvenanceMap> {
    let mut state = TokenOutputState::new(result, cur_len);
    state.write_stream(tokens)?;
    Ok(state.provenance_map)
}

/// Writes unformatted tokens into a string.
pub fn tokens_to_string(tokens: TokenStream) -> Result<String> {
    let mut result = String::new();
    write_unformatted_tokens_with_provenance(&mut result, |s| Some(s.len()), tokens)?;
    Ok(result)
}

/// Writes unformatted tokens into a string, also returning the provenance map.
pub fn tokens_to_string_with_provenance(
    tokens: TokenStream,
) -> Result<(String, SubstringProvenanceMap)> {
    let mut result = String::new();
    let provenance_map =
        write_unformatted_tokens_with_provenance(&mut result, |s| Some(s.len()), tokens)?;
    let mut substring_provenance_map = SubstringProvenanceMap::new();
    // Fix up the provenance map by adding substrings.
    for (tag_index, v) in &provenance_map {
        if v.destination_start >= v.destination_end
            || v.destination_search_start > v.destination_start
        {
            // Skip empty or invalid records.
            continue;
        }
        let tok = &result[v.destination_start..v.destination_end].trim();
        let (adj_start, adj_end) = (
            v.destination_start - v.destination_search_start,
            v.destination_end - v.destination_search_start,
        );
        // There must be at least one instance of the substring; otherwise, we would never have
        // put an entry in the provenance map, nor would we have been able to construct a substring
        // in the first place.
        let index = result[v.destination_search_start..]
            .match_indices(tok)
            .position(|(ix, _)| ix >= adj_start && ix < adj_end)
            .unwrap();
        substring_provenance_map.insert(
            *tag_index,
            SubstringWithProvenance {
                substring: tok.to_string(),
                index,
                original_path: v.original_path.clone(),
                original_start: v.original_start.clone(),
                original_end: v.original_end.clone(),
            },
        );
    }
    Ok((result, substring_provenance_map))
}

/// Fills in `formatted_start` and `formatted_end` where possible in `provenance_map`,
/// assuming `pattern` identifies the place where `destination_search_start` should map
/// (modulo whitespace). We assume that the formatter does not introduce any new instances of
/// the substrings in the provenance map or of `pattern`, and that the ith occurrence of `pattern`
/// corresponds with the i(+1)th entry in the provenance map.
pub fn fix_provenance_map_postformatting(
    content: &str,
    pattern: &str,
    substring_provenance_map: &SubstringProvenanceMap,
) -> FormattedProvenanceMap {
    let mut formatted_provenance_map = FormattedProvenanceMap::new();
    for (tag_index, (offset, _)) in content.match_indices(pattern).enumerate() {
        let v = match substring_provenance_map.get(&(tag_index + 1)) {
            Some(v) => v,
            None => {
                // No record for this match.
                continue;
            }
        };
        if v.substring.is_empty() {
            // Skip empty records.
            continue;
        }
        if let Some((formatted_offset, _)) =
            content[offset..].match_indices(&v.substring).nth(v.index)
        {
            formatted_provenance_map.insert(
                tag_index + 1,
                FormattedOffsetsWithProvenance {
                    formatted_start: formatted_offset + offset,
                    formatted_end: formatted_offset + offset + v.substring.len(),
                    original_path: v.original_path.clone(),
                    original_start: v.original_start.clone(),
                    original_end: v.original_end.clone(),
                },
            );
        }
    }
    formatted_provenance_map
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
    input: &str,
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

    let mut stdin = child.stdin.take().unwrap_or_else(|| panic!("Failed to open {exe_name} stdin"));
    let (output, write_res) = std::thread::scope(|s| {
        let handle = s.spawn(move || stdin.write_all(input.as_bytes()));
        let output =
            child.wait_with_output().unwrap_or_else(|_| panic!("Failed to read {exe_name} stdout"));
        let write_res = handle.join().unwrap();
        (output, write_res)
    });

    if !output.status.success() {
        bail!("{exe_name} reported an error: {}", String::from_utf8_lossy(&output.stderr));
    }
    write_res.unwrap_or_else(|e| panic!("Failed to write to {exe_name} stdin: {e}"));

    Ok(String::from_utf8(output.stdout)
        .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned()))
}

fn rustfmt(input: &str, config: &RustfmtConfig) -> Result<String> {
    pipe_string_through_process(
        input,
        "rustfmt",
        &config.exe_path,
        config.cmdline_args.iter().map(OsString::as_os_str),
    )
}

/// Target size in bytes for splitting large C++ inputs before invoking `clang-format`.
///
/// `clang-format` (`--style=google`) exhibits superlinear `O(N_namespaces * FileSize)` scaling in
/// `calculateRangesAfterReplacements` when inserting `// namespace ...` comments across multi-MB
/// generated headers. Splitting into ~128 KiB chunks at safe top-level declaration boundaries keeps
/// each invocation's working set small and allows independent chunks to be formatted in parallel
/// while producing byte-for-byte identical output.
const CLANG_FORMAT_CHUNK_TARGET_BYTES: usize = 128 * 1024;
const CLANG_FORMAT_MAX_WORKERS: usize = 8;

fn clang_format_single(input: &str, clang_format_exe_path: &Path) -> Result<String> {
    pipe_string_through_process(
        input,
        "clang-format",
        clang_format_exe_path,
        [OsStr::new("--style=google"), OsStr::new("--sort-includes=false")],
    )
}

fn clang_format(input: String, clang_format_exe_path: &Path) -> Result<String> {
    clang_format_with_chunk_size(&input, clang_format_exe_path, CLANG_FORMAT_CHUNK_TARGET_BYTES)
}

fn clang_format_with_chunk_size(
    input: &str,
    clang_format_exe_path: &Path,
    target_chunk_bytes: usize,
) -> Result<String> {
    let chunks = split_cc_for_clang_format(input, target_chunk_bytes);
    if chunks.len() <= 1 {
        return clang_format_single(input, clang_format_exe_path);
    }

    let available = std::thread::available_parallelism().map(NonZeroUsize::get).unwrap_or(4);
    let num_workers = chunks.len().min(available).min(CLANG_FORMAT_MAX_WORKERS).max(1);
    if num_workers == 1 {
        let mut formatted_chunks = Vec::with_capacity(chunks.len());
        for chunk in &chunks {
            formatted_chunks.push(clang_format_single(chunk, clang_format_exe_path)?);
        }
        return Ok(join_formatted_chunks(&formatted_chunks));
    }

    let next_idx = AtomicUsize::new(0);
    let failed = AtomicBool::new(false);
    let results: Mutex<Vec<(usize, Result<String>)>> = Mutex::new(Vec::with_capacity(chunks.len()));

    std::thread::scope(|s| {
        for _ in 0..num_workers {
            s.spawn(|| loop {
                if failed.load(Ordering::Relaxed) {
                    break;
                }
                let idx = next_idx.fetch_add(1, Ordering::Relaxed);
                let Some(chunk) = chunks.get(idx) else {
                    break;
                };
                let res = clang_format_single(chunk, clang_format_exe_path);
                if res.is_err() {
                    failed.store(true, Ordering::Relaxed);
                }
                results.lock().unwrap().push((idx, res));
            });
        }
    });

    let mut indexed_results = results.into_inner().unwrap();
    indexed_results.sort_unstable_by_key(|(idx, _)| *idx);
    let mut formatted_chunks = Vec::with_capacity(chunks.len());
    for (_, res) in indexed_results {
        formatted_chunks.push(res?);
    }
    Ok(join_formatted_chunks(&formatted_chunks))
}

fn join_formatted_chunks(chunks: &[String]) -> String {
    let sep_bytes = chunks.len().saturating_sub(1) * 2;
    let total_len: usize = chunks.iter().map(String::len).sum::<usize>() + sep_bytes;
    let mut out = String::with_capacity(total_len);
    for (i, chunk) in chunks.iter().enumerate() {
        if i > 0 {
            out.push_str("\n\n");
        }
        out.push_str(chunk);
    }
    out
}

fn escaped_newline_len(bytes: &[u8], i: usize) -> Option<usize> {
    if bytes.get(i) != Some(&b'\\') {
        return None;
    }
    match bytes.get(i + 1) {
        Some(&b'\n') => Some(2),
        Some(&b'\r') if bytes.get(i + 2) == Some(&b'\n') => Some(3),
        _ => None,
    }
}

fn ident_prefix_before(bytes: &[u8], i: usize) -> &[u8] {
    let mut start = i;
    while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
        start -= 1;
    }
    &bytes[start..i]
}

fn raw_string_delimiter(bytes: &[u8], quote_idx: usize) -> Option<&[u8]> {
    let prefix = ident_prefix_before(bytes, quote_idx);
    if !matches!(prefix, b"R" | b"LR" | b"uR" | b"UR" | b"u8R") {
        return None;
    }
    let delim_start = quote_idx + 1;
    let max_end = bytes.len().min(delim_start + 17);
    for idx in delim_start..max_end {
        let b = bytes[idx];
        if b == b'(' {
            return Some(&bytes[delim_start..idx]);
        }
        if b.is_ascii_whitespace() || b == b')' || b == b'\\' {
            return None;
        }
    }
    None
}

fn is_cpp_digit_separator(bytes: &[u8], quote_idx: usize) -> bool {
    let prefix = ident_prefix_before(bytes, quote_idx);
    !prefix.is_empty() && !matches!(prefix, b"L" | b"u" | b"U" | b"u8")
}

fn update_clang_format_toggle(comment_body: &[u8], clang_format_off: &mut bool) {
    let trimmed = comment_body.trim_ascii();
    if trimmed.starts_with(b"clang-format off") {
        *clang_format_off = true;
    } else if trimmed.starts_with(b"clang-format on") {
        *clang_format_off = false;
    }
}

/// Splits `input` into chunks of at least `target_chunk_bytes` at top-level blank lines (`\n\n`)
/// following complete top-level declarations or namespace blocks (`;` or `}`), preserving leading
/// and trailing whitespace on the first and last chunks so joining formatted chunks with `"\n\n"`
/// produces byte-for-byte identical output to formatting `input` in a single `clang-format` pass.
fn split_cc_for_clang_format(input: &str, target_chunk_bytes: usize) -> Vec<&str> {
    enum LexState<'a> {
        Normal,
        LineComment { start: usize },
        BlockComment { start: usize },
        StringLiteral,
        CharLiteral,
        RawString(&'a [u8]),
    }

    if input.len() <= target_chunk_bytes {
        return vec![input];
    }

    let bytes = input.as_bytes();
    let mut chunks = Vec::new();
    let mut chunk_start = 0usize;
    let mut state = LexState::Normal;
    let mut brace_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut in_pp_directive = false;
    let mut at_line_start = true;
    let mut seen_top_level_code = false;
    let mut pp_depth = 0usize;
    let mut outer_guard_depth: Option<usize> = None;
    let mut clang_format_off = false;
    let mut last_top_level_code_byte = 0u8;

    let mut i = 0usize;
    while i < bytes.len() {
        match state {
            LexState::Normal => {
                if let Some(skip) = escaped_newline_len(bytes, i) {
                    i += skip;
                    continue;
                }
                let b = bytes[i];
                if b == b'\n' {
                    let was_pp_directive = in_pp_directive;
                    in_pp_directive = false;
                    at_line_start = true;

                    let mut newline_count = 1usize;
                    let mut last_nl = i;
                    let mut j = i + 1;
                    while j < bytes.len() && matches!(bytes[j], b' ' | b'\t' | b'\r' | b'\n') {
                        if bytes[j] == b'\n' {
                            newline_count += 1;
                            last_nl = j;
                        }
                        j += 1;
                    }

                    let sep_start = i;
                    let sep_end = last_nl + 1;
                    if newline_count >= 2
                        && sep_start - chunk_start >= target_chunk_bytes
                        && !was_pp_directive
                        && !clang_format_off
                        && brace_depth == 0
                        && paren_depth == 0
                        && bracket_depth == 0
                        && pp_depth == outer_guard_depth.unwrap_or(0)
                        && matches!(last_top_level_code_byte, b'}' | b';')
                        && j < bytes.len()
                        && !matches!(bytes[j], b';' | b',')
                        && bytes[chunk_start..sep_start].iter().any(|c| !c.is_ascii_whitespace())
                    {
                        chunks.push(&input[chunk_start..sep_start]);
                        chunk_start = sep_end;
                        last_top_level_code_byte = 0;
                    }
                    i = sep_end;
                    continue;
                }
                if matches!(b, b' ' | b'\t' | b'\r') {
                    i += 1;
                    continue;
                }
                if b == b'/' && bytes.get(i + 1) == Some(&b'/') {
                    state = LexState::LineComment { start: i + 2 };
                    at_line_start = false;
                    i += 2;
                    continue;
                }
                if b == b'/' && bytes.get(i + 1) == Some(&b'*') {
                    state = LexState::BlockComment { start: i + 2 };
                    i += 2;
                    continue;
                }
                if b == b'#' && at_line_start {
                    in_pp_directive = true;
                    at_line_start = false;
                    last_top_level_code_byte = 0;
                    let mut k = i + 1;
                    while k < bytes.len() && matches!(bytes[k], b' ' | b'\t') {
                        k += 1;
                    }
                    let kw_start = k;
                    while k < bytes.len() && bytes[k].is_ascii_alphabetic() {
                        k += 1;
                    }
                    match &bytes[kw_start..k] {
                        b"ifndef" => {
                            if !seen_top_level_code && outer_guard_depth.is_none() && pp_depth == 0
                            {
                                outer_guard_depth = Some(1);
                            }
                            pp_depth += 1;
                        }
                        b"if" | b"ifdef" => {
                            pp_depth += 1;
                        }
                        b"else" | b"elif" | b"elifdef" | b"elifndef" => {
                            if pp_depth == 1 && outer_guard_depth == Some(1) {
                                outer_guard_depth = Some(0);
                            }
                        }
                        b"endif" => {
                            pp_depth = pp_depth.saturating_sub(1);
                            if pp_depth == 0 && outer_guard_depth == Some(1) {
                                outer_guard_depth = Some(0);
                            }
                        }
                        _ => {}
                    }
                    i = k;
                    continue;
                }

                at_line_start = false;
                if !in_pp_directive {
                    seen_top_level_code = true;
                }

                match b {
                    b'"' => {
                        if !in_pp_directive
                            && brace_depth == 0
                            && paren_depth == 0
                            && bracket_depth == 0
                        {
                            last_top_level_code_byte = b'"';
                        }
                        if let Some(delim) = raw_string_delimiter(bytes, i) {
                            let after_open_paren = i + 2 + delim.len();
                            state = LexState::RawString(delim);
                            i = after_open_paren;
                            continue;
                        }
                        state = LexState::StringLiteral;
                    }
                    b'\'' => {
                        if !is_cpp_digit_separator(bytes, i) {
                            if !in_pp_directive
                                && brace_depth == 0
                                && paren_depth == 0
                                && bracket_depth == 0
                            {
                                last_top_level_code_byte = b'\'';
                            }
                            state = LexState::CharLiteral;
                        }
                    }
                    b'{' if !in_pp_directive => {
                        brace_depth += 1;
                        last_top_level_code_byte = b'{';
                    }
                    b'}' if !in_pp_directive => {
                        brace_depth = brace_depth.saturating_sub(1);
                        if brace_depth == 0 && paren_depth == 0 && bracket_depth == 0 {
                            last_top_level_code_byte = b'}';
                        }
                    }
                    b'(' if !in_pp_directive => {
                        paren_depth += 1;
                    }
                    b')' if !in_pp_directive => {
                        paren_depth = paren_depth.saturating_sub(1);
                        if brace_depth == 0 && paren_depth == 0 && bracket_depth == 0 {
                            last_top_level_code_byte = b')';
                        }
                    }
                    b'[' if !in_pp_directive => {
                        bracket_depth += 1;
                    }
                    b']' if !in_pp_directive => {
                        bracket_depth = bracket_depth.saturating_sub(1);
                        if brace_depth == 0 && paren_depth == 0 && bracket_depth == 0 {
                            last_top_level_code_byte = b']';
                        }
                    }
                    _ => {
                        if !in_pp_directive
                            && brace_depth == 0
                            && paren_depth == 0
                            && bracket_depth == 0
                        {
                            last_top_level_code_byte = b;
                        }
                    }
                }
                i += 1;
            }
            LexState::LineComment { start } => {
                if let Some(skip) = escaped_newline_len(bytes, i) {
                    i += skip;
                    continue;
                }
                if bytes[i] == b'\n' {
                    update_clang_format_toggle(&bytes[start..i], &mut clang_format_off);
                    state = LexState::Normal;
                    continue;
                }
                i += 1;
            }
            LexState::BlockComment { start } => {
                if bytes[i] == b'*' && bytes.get(i + 1) == Some(&b'/') {
                    update_clang_format_toggle(&bytes[start..i], &mut clang_format_off);
                    state = LexState::Normal;
                    i += 2;
                    continue;
                }
                if bytes[i] == b'\n' && !in_pp_directive {
                    at_line_start = true;
                }
                i += 1;
            }
            LexState::StringLiteral => {
                if bytes[i] == b'\\' {
                    i = (i + 2).min(bytes.len());
                    continue;
                }
                if bytes[i] == b'"' {
                    state = LexState::Normal;
                    i += 1;
                    continue;
                }
                if bytes[i] == b'\n' {
                    in_pp_directive = false;
                    at_line_start = true;
                    state = LexState::Normal;
                    i += 1;
                    continue;
                }
                i += 1;
            }
            LexState::CharLiteral => {
                if bytes[i] == b'\\' {
                    i = (i + 2).min(bytes.len());
                    continue;
                }
                if bytes[i] == b'\'' {
                    state = LexState::Normal;
                    i += 1;
                    continue;
                }
                if bytes[i] == b'\n' {
                    in_pp_directive = false;
                    at_line_start = true;
                    state = LexState::Normal;
                    i += 1;
                    continue;
                }
                i += 1;
            }
            LexState::RawString(delim) => {
                if bytes[i] == b')'
                    && bytes.get(i + 1..i + 1 + delim.len()) == Some(delim)
                    && bytes.get(i + 1 + delim.len()) == Some(&b'"')
                {
                    state = LexState::Normal;
                    at_line_start = false;
                    i += 2 + delim.len();
                    continue;
                }
                i += 1;
            }
        }
    }

    if chunk_start < input.len() {
        chunks.push(&input[chunk_start..]);
    }
    if chunks.is_empty() {
        chunks.push(input);
    }
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::gtest;

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
    fn test_capture_tokens_dont_change_output() -> Result<()> {
        let token_stream = quote! { a __CAPTURE_BEGIN__ b __CAPTURE_END__ c };
        assert_eq!(tokens_to_string(token_stream)?, "a b c");
        Ok(())
    }

    #[gtest]
    fn test_capture_tokens_produce_provenance_map() -> Result<()> {
        let token_stream =
            quote! { a __CAPTURE_TAG__ "foo" "44" "45" b __CAPTURE_BEGIN__ b __CAPTURE_END__ c };
        let (result, provenance_map) = tokens_to_string_with_provenance(token_stream).unwrap();
        assert_eq!(result, "a b b c");
        let b_data = provenance_map.get(&1).unwrap();
        assert_eq!(b_data.substring, "b");
        assert_eq!(b_data.original_path, "foo");
        assert_eq!(b_data.original_start, "44");
        assert_eq!(b_data.original_end, "45");
        assert_eq!(b_data.index, 1);
        Ok(())
    }

    #[gtest]
    fn test_capture_tokens_produce_provenance_map_with_brackets() -> Result<()> {
        let token_stream = quote! { a __CAPTURE_TAG__ "foo" "44" "45" b [ __CAPTURE_BEGIN__ b __CAPTURE_END__ ] c };
        let (result, provenance_map) = tokens_to_string_with_provenance(token_stream).unwrap();
        assert_eq!(result, "a b[b ]c");
        let b_data = provenance_map.get(&1).unwrap();
        assert_eq!(b_data.substring, "b");
        assert_eq!(b_data.original_path, "foo");
        assert_eq!(b_data.original_start, "44");
        assert_eq!(b_data.original_end, "45");
        assert_eq!(b_data.index, 1);
        Ok(())
    }

    #[gtest]
    fn test_fixup_provenance_map_works_on_multiple_instances_of_same_token() -> Result<()> {
        let token_stream =
            quote! { a __CAPTURE_TAG__ "foo" "44" "45" b __CAPTURE_BEGIN__ b __CAPTURE_END__ c };
        let (result, mut provenance_map) = tokens_to_string_with_provenance(token_stream).unwrap();
        assert_eq!(result, "a b b c");
        let formatted_result = "a /// G: b  \n  b  c";
        let second_b = formatted_result.rmatch_indices("b").nth(0).unwrap().0;
        let fixed_provenance_map =
            fix_provenance_map_postformatting(formatted_result, "/// G:", &mut provenance_map);
        let b_data = provenance_map.get(&1).unwrap();
        let fixed_b_data = fixed_provenance_map.get(&1).unwrap();
        assert_eq!(b_data.substring, "b");
        assert_eq!(fixed_b_data.original_path, "foo");
        assert_eq!(fixed_b_data.original_start, "44");
        assert_eq!(fixed_b_data.original_end, "45");
        assert_eq!(fixed_b_data.formatted_start, second_b);
        assert_eq!(fixed_b_data.formatted_end, second_b + 1);
        Ok(())
    }

    #[gtest]
    fn test_produces_provenance_map_end_to_end_on_single_captured_token() -> Result<()> {
        let token_stream =
            quote! { a __CAPTURE_TAG__ "foo" "44" "45" __CAPTURE_BEGIN__ b __CAPTURE_END__ c };
        let (result, mut provenance_map) = tokens_to_string_with_provenance(token_stream).unwrap();
        assert_eq!(result, "a b c");
        let formatted_result = "a b c";
        let b = formatted_result.rmatch_indices("b").nth(0).unwrap().0;
        let fixed_provenance_map =
            fix_provenance_map_postformatting(formatted_result, "a", &mut provenance_map);
        let b_data = provenance_map.get(&1).unwrap();
        let fixed_b_data = fixed_provenance_map.get(&1).unwrap();
        assert_eq!(b_data.substring, "b");
        assert_eq!(fixed_b_data.original_path, "foo");
        assert_eq!(fixed_b_data.original_start, "44");
        assert_eq!(fixed_b_data.original_end, "45");
        assert_eq!(fixed_b_data.formatted_start, b);
        assert_eq!(fixed_b_data.formatted_end, b + 1);
        Ok(())
    }

    #[gtest]
    fn test_produces_provenance_map_with_missing_captures() -> Result<()> {
        let token_stream = quote! { a __CAPTURE_TAG__ "foo" "44" "45" };
        let (result, mut provenance_map) = tokens_to_string_with_provenance(token_stream).unwrap();
        assert_eq!(result, "a ");
        let formatted_result = "a ";
        let fixed_provenance_map =
            fix_provenance_map_postformatting(formatted_result, "a", &mut provenance_map);
        assert!(fixed_provenance_map.is_empty());
        Ok(())
    }

    #[gtest]
    fn test_produces_provenance_map_end_to_end_on_multiple_captured_tokens() -> Result<()> {
        let token_stream = quote! { a __CAPTURE_TAG__ "foo" "44" "45" __CAPTURE_BEGIN__ b __CAPTURE_END__ a __CAPTURE_TAG__ "x" "1" "2" a __CAPTURE_TAG__ "bar" "46" "47" __CAPTURE_BEGIN__ b __CAPTURE_END__ c };
        let (result, mut provenance_map) = tokens_to_string_with_provenance(token_stream).unwrap();
        assert_eq!(result, "a b a a b c");
        let formatted_result = "   a   b    a a   b    c ";
        let first_b = formatted_result.match_indices("b").nth(0).unwrap().0;
        let second_b = formatted_result.match_indices("b").nth(1).unwrap().0;
        let fixed_provenance_map =
            fix_provenance_map_postformatting(formatted_result, "a", &mut provenance_map);
        let first_b_data = provenance_map.get(&1).unwrap();
        let fixed_first_b_data = fixed_provenance_map.get(&1).unwrap();
        assert_eq!(first_b_data.substring, "b");
        assert_eq!(fixed_first_b_data.original_path, "foo");
        assert_eq!(fixed_first_b_data.original_start, "44");
        assert_eq!(fixed_first_b_data.original_end, "45");
        assert_eq!(fixed_first_b_data.formatted_start, first_b);
        assert_eq!(fixed_first_b_data.formatted_end, first_b + 1);
        let second_b_data = provenance_map.get(&3).unwrap();
        let fixed_second_b_data = fixed_provenance_map.get(&3).unwrap();
        assert_eq!(second_b_data.substring, "b");
        assert_eq!(fixed_second_b_data.original_path, "bar");
        assert_eq!(fixed_second_b_data.original_start, "46");
        assert_eq!(fixed_second_b_data.original_end, "47");
        assert_eq!(fixed_second_b_data.formatted_start, second_b);
        assert_eq!(fixed_second_b_data.formatted_end, second_b + 1);
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
        let cfg = RustfmtConfig::new(Path::new(external_binaries::RUSTFMT_EXE_PATH), None);
        let input = quote! {
            fn bar() {}
            fn foo(x: i32, y: i32) -> i32 { x + y }
        };
        let output = rs_tokens_to_formatted_string(input, Some(&cfg)).unwrap();
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
        let cfg = RustfmtConfig::new(
            Path::new(external_binaries::RUSTFMT_EXE_PATH),
            Some(&rustfmt_toml_path),
        );
        let input = quote! {
            fn bar() {}
            fn foo(x: i32, y: i32) -> i32 { x + y }
        };

        let output = rs_tokens_to_formatted_string(input, Some(&cfg)).unwrap();
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

    #[gtest]
    fn test_chunked_clang_format_matches_single_pass_with_include_guard() -> Result<()> {
        let exe = Path::new(external_binaries::CLANG_FORMAT_EXE_PATH);
        let input = r#"// Automatically @generated C++ bindings.
#ifndef THIRD_PARTY_CRUBIT_TEST_HEADER_H_
#define THIRD_PARTY_CRUBIT_TEST_HEADER_H_

#ifdef KYTHE_IS_RUNNING
#pragma kythe_inline_metadata "This file contains Kythe metadata."
#endif

#include <cstddef>
#include <cstdint>

namespace crubit_test::alpha {
struct Alpha final {
  std::int32_t field_one;
  std::int64_t field_two;
};
}

namespace crubit_test::beta {
extern "C" void __crubit_thunk_beta(std::int32_t x, std::int32_t* out);

inline std::int32_t call_beta(std::int32_t x) {
  std::int32_t out = 0;
  __crubit_thunk_beta(x, &out);
  return out;
}
}

namespace crubit_test::gamma {
static_assert(sizeof(crubit_test::alpha::Alpha) == 16);
using BetaAlias = crubit_test::alpha::Alpha;
}

#endif  // THIRD_PARTY_CRUBIT_TEST_HEADER_H_
"#;
        let chunks = split_cc_for_clang_format(input, 64);
        assert!(chunks.len() >= 4, "expected multiple chunks, got {}", chunks.len());
        let single = clang_format_single(input, exe)?;
        let chunked = clang_format_with_chunk_size(input, exe, 64)?;
        assert_eq!(chunked, single);
        Ok(())
    }

    #[gtest]
    fn test_chunked_clang_format_respects_lexical_and_preprocessor_scopes() -> Result<()> {
        let exe = Path::new(external_binaries::CLANG_FORMAT_EXE_PATH);
        let input = r#"#pragma once

namespace lex_scope {
constexpr int kLarge = 1'000'000 + 0xdead'beef;
constexpr char kQuote = '\'';
constexpr char8_t kUtf8 = u8'a';
constexpr const char* kRaw = R"delim(
struct FakeInsideRawString { int x; };

namespace not_a_real_split {
}
)delim";
}

/* Block comment with fake boundary:
};

namespace not_a_real_comment_split {}
*/
namespace after_comment {
void f();
}

#ifdef OPTIONAL_FEATURE
namespace inside_ifdef_one {
void a();
}

namespace inside_ifdef_two {
void b();
}
#else
namespace inside_else {
void c();
}
#endif

#define MULTILINE_MACRO(x) \
  do {                     \
    (void)(x);             \
  } while (0)

template <typename T>

struct TemplateAcrossBlankLine {
  T value;
};

// clang-format off
struct   UnformattedStruct {   int   a ;

int   b ; };
// clang-format on

namespace final_ns {
void done();
}
"#;
        let chunks = split_cc_for_clang_format(input, 48);
        assert!(chunks.len() >= 4, "expected multiple chunks, got {}", chunks.len());
        for chunk in &chunks {
            assert!(
                !(chunk.contains("inside_ifdef_one") ^ chunk.contains("inside_ifdef_two")),
                "must not split inside #ifdef block: {chunk:?}"
            );
        }
        let single = clang_format_single(input, exe)?;
        let chunked = clang_format_with_chunk_size(input, exe, 48)?;
        assert_eq!(chunked, single);
        Ok(())
    }

    #[gtest]
    fn test_chunked_clang_format_preserves_edge_newlines_and_provenance() -> Result<()> {
        let exe = Path::new(external_binaries::CLANG_FORMAT_EXE_PATH);
        let tokens = quote! {
            __NEWLINE__
            __NEWLINE__
            namespace first_ns {
            __COMMENT__ "Generated from: first.rs;l=1"
            __CAPTURE_TAG__ "first.rs" "10" "18"
            struct __CAPTURE_BEGIN__ FirstType __CAPTURE_END__ { int x; };
            }
            __NEWLINE__
            __NEWLINE__
            __NEWLINE__
            namespace second_ns {
            __COMMENT__ "Generated from: second.rs;l=2"
            __CAPTURE_TAG__ "second.rs" "20" "30"
            struct __CAPTURE_BEGIN__ SecondType __CAPTURE_END__ { int y; };
            }
            __NEWLINE__
            __NEWLINE__
            namespace third_ns {
            __COMMENT__ "Generated from: third.rs;l=3"
            __CAPTURE_TAG__ "third.rs" "40" "49"
            void __CAPTURE_BEGIN__ third_fn __CAPTURE_END__ ();
            }
        };
        let (unformatted, provenance_map) = tokens_to_string_with_provenance(tokens)?;
        let chunks = split_cc_for_clang_format(&unformatted, 48);
        assert!(chunks.len() >= 3, "expected >= 3 chunks, got {}", chunks.len());

        let single = clang_format_single(&unformatted, exe)?;
        let chunked = clang_format_with_chunk_size(&unformatted, exe, 48)?;
        assert_eq!(chunked, single);

        let single_prov =
            fix_provenance_map_postformatting(&single, "// Generated from:", &provenance_map);
        let chunked_prov =
            fix_provenance_map_postformatting(&chunked, "// Generated from:", &provenance_map);
        assert_eq!(chunked_prov, single_prov);
        assert_eq!(chunked_prov.len(), 3);
        Ok(())
    }
}
