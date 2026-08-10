// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use clap::Parser;
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

    /// Optional custom macro name to extract (defaults to global_cpp, DO_NOT_SUBMIT_CPP_DECL, cpp_decl) // NOLINT
    #[arg(long, default_value = "")]
    macro_name: String,

    /// Fail if bindable C++ declarations are found inside fallback blocks
    #[arg(long, default_value_t = false)]
    fail_on_bindable: bool,
}

pub fn lint_bindable_cpp(body_text: &str, file_name: &str, line: usize) -> Vec<String> {
    let mut warnings = Vec::new();
    let trimmed = body_text.trim();
    if !trimmed.contains("template")
        && !trimmed.contains("#include")
        && (trimmed.contains("struct ")
            || trimmed.contains("class ")
            || (trimmed.contains('(') && trimmed.contains(')') && trimmed.contains(';')))
    {
        warnings.push(format!(
            "Lint warning at {}:{}: Item inside fallback C++ block is natively bindable by Crubit. Consider upgrading it to a standard Rust binding.",
            file_name, line
        ));
    }
    warnings
}

pub fn extract_global_cpp(
    rust_source: &str,
    _tokens: &[ra_ap_rustc_lexer::Token],
    file_name: &str,
    _custom_macro_name: Option<&str>,
) -> Result<(String, Vec<String>), String> {
    let mut extracted = String::new();
    let mut lint_warnings = Vec::new();
    let macros = parse_inline_cpp_macros::parse_inline_cpp_macros(rust_source, file_name)?;

    for m in macros {
        if m.kind != parse_inline_cpp_macros::MacroKind::GlobalCpp
            && m.kind != parse_inline_cpp_macros::MacroKind::DoNotSubmitCppDecl
        {
            continue;
        }
        let padding = " ".repeat(m.body_col.saturating_sub(1));
        let _ = write!(
            extracted,
            "#line {} \"{}\"\n{}{}\n",
            m.body_line, file_name, padding, m.body_text
        );
        let warnings = lint_bindable_cpp(m.body_text, file_name, m.body_line);
        lint_warnings.extend(warnings);
    }

    Ok((extracted, lint_warnings))
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
    _tokens: &[ra_ap_rustc_lexer::Token],
    file_name: &str,
    target: &str,
) -> Result<String, String> {
    let mut extracted = String::new();
    let macros = parse_inline_cpp_macros::parse_inline_cpp_macros(rust_source, file_name)?;

    for m in macros {
        if m.kind != parse_inline_cpp_macros::MacroKind::InlineCpp {
            continue;
        }
        let thunk_name =
            inline_cpp_utils::compute_thunk_name(target, file_name, m.macro_line, m.macro_col);
        validate_inline_cpp_syntax(m.body_text)
            .map_err(|e| format!("{} at {}:{}", e, file_name, m.macro_line))?;
        let padding = " ".repeat(m.body_col.saturating_sub(1));

        let _ = write!(
            extracted,
            "inline auto {}\n#line {} \"{}\"\n{}{}\n\n",
            thunk_name, m.body_line, file_name, padding, m.body_text
        );
    }

    Ok(extracted)
}

fn main() {
    let args = Args::parse();

    let mut all_cpp_snippets = String::new();
    let custom_macro =
        if args.macro_name.is_empty() { None } else { Some(args.macro_name.as_str()) };
    let mut total_warnings = Vec::new();

    for src in &args.srcs {
        let content = fs::read_to_string(src).unwrap_or_else(|e| {
            eprintln!("Failed to read file {}: {}", src.display(), e);
            process::exit(1);
        });
        let file_name = src.display().to_string();
        let tokens =
            ra_ap_rustc_lexer::tokenize(&content, ra_ap_rustc_lexer::FrontmatterAllowed::No);
        let token_list = tokens.collect::<Vec<_>>();

        let (scraped_global, warnings) =
            extract_global_cpp(&content, &token_list, &file_name, custom_macro).unwrap_or_else(
                |e| {
                    eprintln!("Extraction error: {}", e);
                    process::exit(1);
                },
            );
        total_warnings.extend(warnings);
        let scraped_inline = extract_inline_cpp(&content, &token_list, &file_name, &args.target)
            .unwrap_or_else(|e| {
                eprintln!("Extraction error: {}", e);
                process::exit(1);
            });
        all_cpp_snippets.push_str(&scraped_global);
        all_cpp_snippets.push_str(&scraped_inline);
    }

    if !total_warnings.is_empty() {
        for w in &total_warnings {
            eprintln!("{}", w);
        }
        if args.fail_on_bindable {
            eprintln!("Error: Bindable C++ declarations found in fallback blocks with --fail_on_bindable enabled.");
            process::exit(1);
        }
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
        let expected = "#line 1 \"test.rs\"\n              int x; \n";
        let (extracted, _) = extract_global_cpp(input, &tokenize(input), "test.rs", None).unwrap();
        expect_eq!(extracted, expected);
    }

    #[gtest]
    fn test_do_not_submit_cpp_decl_extract() {
        let input = "DO_NOT_SUBMIT_CPP_DECL! { template <typename T> void Foo(T t); }"; // NOLINT
        let expected = "#line 1 \"test.rs\"\n                          template <typename T> void Foo(T t); \n";
        let (extracted, warnings) =
            extract_global_cpp(input, &tokenize(input), "test.rs", None).unwrap();
        expect_eq!(extracted, expected);
        expect_that!(warnings, is_empty());
    }

    #[gtest]
    fn test_lint_bindable_cpp_warning() {
        let input = "DO_NOT_SUBMIT_CPP_DECL! { int Add(int a, int b); }"; // NOLINT
        let (_, warnings) = extract_global_cpp(input, &tokenize(input), "test.rs", None).unwrap();
        expect_that!(warnings.len(), eq(1));
        expect_true!(warnings[0].contains("natively bindable"));
    }

    #[gtest]
    fn test_nested_braces() {
        let input = "global_cpp! { namespace foo { int x; } }";
        let expected = "#line 1 \"test.rs\"\n              namespace foo { int x; } \n";
        let (extracted, _) = extract_global_cpp(input, &tokenize(input), "test.rs", None).unwrap();
        expect_eq!(extracted, expected);
    }

    #[gtest]
    fn test_multiple_blocks() {
        let input = "global_cpp! { int x; } some rust code global_cpp! { int y; }";
        let expected = "#line 1 \"test.rs\"\n              int x; \n#line 1 \"test.rs\"\n                                                    int y; \n";
        let (extracted, _) = extract_global_cpp(input, &tokenize(input), "test.rs", None).unwrap();
        expect_eq!(extracted, expected);
    }

    #[gtest]
    fn test_extract_global_cpp_templates_and_explicit_instantiations() {
        let input = r#"global_cpp! {
    namespace foo::bar {
        template <typename T>
        class MyTemplate {
         public:
          T GetValue(T val) { return val; }
        };

        template class MyTemplate<int>;
    }
}"#;
        let expected = concat!(
            "#line 1 \"test.rs\"\n             \n",
            "    namespace foo::bar {\n",
            "        template <typename T>\n",
            "        class MyTemplate {\n",
            "         public:\n",
            "          T GetValue(T val) { return val; }\n",
            "        };\n\n",
            "        template class MyTemplate<int>;\n",
            "    }\n\n"
        );
        expect_eq!(
            extract_global_cpp(input, &tokenize(input), "test.rs", None).unwrap().0,
            expected
        );
    }

    #[gtest]
    fn test_extract_global_cpp_nested_namespaces() {
        let input = "global_cpp! { namespace outer::inner { int x = 10; } }";
        let expected =
            "#line 1 \"test.rs\"\n              namespace outer::inner { int x = 10; } \n";
        expect_eq!(
            extract_global_cpp(input, &tokenize(input), "test.rs", None).unwrap().0,
            expected
        );
    }

    #[gtest]
    fn test_extract_global_cpp_multiline_line_number_mapping() {
        let input = "line 1\nline 2\nglobal_cpp! {\n    int x = 42;\n}\n";
        let expected = "#line 3 \"test.rs\"\n             \n    int x = 42;\n\n";
        expect_eq!(
            extract_global_cpp(input, &tokenize(input), "test.rs", None).unwrap().0,
            expected
        );
    }

    #[gtest]
    fn test_unclosed_block() {
        let input = "global_cpp! { int x;";
        let err = extract_global_cpp(input, &tokenize(input), "test.rs", None).unwrap_err();
        let expected_err = "Unmatched delimiter starting at test.rs:1: Context around open brace:\nglobal_cpp! { int x;";
        expect_eq!(err, expected_err);
    }

    #[gtest]
    fn test_unclosed_block_with_context() {
        let input = "line 1\nline 2\nline 3\nglobal_cpp! { int x;\nline 5\nline 6\nline 7";
        let err = extract_global_cpp(input, &tokenize(input), "test.rs", None).unwrap_err();
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
