// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use super::{expand_live_snippets, run, select, Lang};
use googletest::matchers::{contains_substring, displays_as, eq, err};
use googletest::{expect_that, gtest};
use std::path::PathBuf;

const LICENSE: &str = "\
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

";

/// Creates a fresh directory containing the given files.
fn make_root(test_name: &str, files: &[(&str, &str)]) -> PathBuf {
    let base = std::env::var_os("TEST_TMPDIR").map_or_else(std::env::temp_dir, PathBuf::from);
    let root = base.join(format!("mdbook_live_snippet_{test_name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    for (path, contents) in files {
        let path = root.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, contents).unwrap();
    }
    root
}

#[gtest]
fn whole_file_strips_license_header() {
    let source = format!("{LICENSE}pub fn f() {{}}\n");
    expect_that!(select(&source, Lang::Rust, "").unwrap(), eq("pub fn f() {}\n"));
}

#[gtest]
fn content_selects_matching_lines() {
    let source = "a one\nb\nc one\nd one\n";
    expect_that!(
        select(source, Lang::Other, r"content:\ one$").unwrap(),
        eq("a one\n\nc one\nd one\n")
    );
}

#[gtest]
fn content_dot_matches_newline() {
    let source = "start\nmiddle\nend\nafter\n";
    expect_that!(select(source, Lang::Other, r"content:^start[^\n]*").unwrap(), eq("start\n"));
    expect_that!(
        select(source, Lang::Other, r"content:^start.*?^end").unwrap(),
        eq("start\nmiddle\nend\n")
    );
}

#[gtest]
fn cpp_macro_definition() {
    let source = "#include <x>\n\n// Docs.\n#define FOO(x) \\\n  x\n\nint y;\n";
    expect_that!(
        select(source, Lang::Cpp, "symbol:FOO").unwrap(),
        eq("// Docs.\n#define FOO(x) \\\n  x\n")
    );
}

#[gtest]
fn content_accepts_quoted_regex() {
    let source = "impl Bar {\n    fn f() {}\n}\n";
    expect_that!(
        select(source, Lang::Rust, r#"content:"(?m)impl Bar[\s\S]*?^\}""#).unwrap(),
        eq(source)
    );
}

#[gtest]
fn rust_function_includes_doc_comments_and_attributes() {
    let source = format!(
        "{LICENSE}use foo;\n\n/// Docs.\n#[inline]\npub fn add(x: i32) -> i32 {{\n    x + 1\n}}\n\nfn other() {{}}\n"
    );
    expect_that!(
        select(&source, Lang::Rust, "function:add").unwrap(),
        eq("/// Docs.\n#[inline]\npub fn add(x: i32) -> i32 {\n    x + 1\n}\n")
    );
}

#[gtest]
fn rust_class_and_symbol() {
    let source =
        "#[derive(Default)]\npub struct Point {\n    x: i32,\n}\n\npub type Alias = Point;\n";
    expect_that!(
        select(source, Lang::Rust, "class:Point").unwrap(),
        eq("#[derive(Default)]\npub struct Point {\n    x: i32,\n}\n")
    );
    expect_that!(
        select(source, Lang::Rust, "symbol:Alias").unwrap(),
        eq("pub type Alias = Point;\n")
    );
    expect_that!(
        select(source, Lang::Rust, "class:Alias"),
        err(displays_as(contains_substring("matched nothing")))
    );
}

#[gtest]
fn rust_item_excludes_inner_attributes() {
    let source = "#![allow(unused)]\n#![deny(warnings)]\n#[repr(C)]\npub struct S {}\n";
    expect_that!(
        select(source, Lang::Rust, "class:S").unwrap(),
        eq("#[repr(C)]\npub struct S {}\n")
    );
}

#[gtest]
fn content_trims_blank_lines_at_edges() {
    let source = "a\n\nb {\n}\n";
    expect_that!(select(source, Lang::Other, r"content:^\s*b \{.*?^\}").unwrap(), eq("b {\n}\n"));
}

#[gtest]
fn cpp_function_declaration_but_not_call() {
    let source = "#include <x>\n\n// Adds.\nint add(int x, int y);\n\nint main() {\n  int z = add(1, 2);\n  add(3, 4);\n  return add(5, 6);\n}\n";
    expect_that!(
        select(source, Lang::Cpp, "function:add").unwrap(),
        eq("// Adds.\nint add(int x, int y);\n")
    );
}

#[gtest]
fn cpp_test_macro_with_body() {
    let source = "CO_TEST(Foo, Bar) {\n  co_return;\n}\n";
    expect_that!(select(source, Lang::Cpp, "function:CO_TEST").unwrap(), eq(source));
}

#[gtest]
fn cpp_class_with_macro_head_and_alternation() {
    let source = "namespace ns {\n\nstruct CRUBIT_INTERNAL_RUST_TYPE(\":: c :: Color\") alignas(4)\n    [[clang::trivial_abi]] Color final {\n public:\n  int x;\n};\n\nstruct Color;\n}  // namespace ns\n";
    expect_that!(
        select(source, Lang::Cpp, "class:CRUBIT_INTERNAL_RUST_TYPE|Color").unwrap(),
        eq("struct CRUBIT_INTERNAL_RUST_TYPE(\":: c :: Color\") alignas(4)\n    [[clang::trivial_abi]] Color final {\n public:\n  int x;\n};\n")
    );
}

#[gtest]
fn cpp_template_specialization() {
    let source = "template <>\nstruct rs_std::impl<A, B> {\n  static int f();\n};\n";
    expect_that!(select(source, Lang::Cpp, "class:impl").unwrap(), eq(source));
}

#[gtest]
fn starlark_targets_by_name() {
    let source = format!(
        "{LICENSE}load(\"//foo:defs.bzl\", \"rule\")\n\n# The library.\ncc_library(\n    name = \"lib\",\n    srcs = glob([\"*.cc\"]),\n)\n\nrule(\n    name = \"lib_rust\",\n    deps = [\":lib\"],\n)\n\nrule(\n    name = \"other\",\n)\n"
    );
    expect_that!(
        select(&source, Lang::Starlark, r"symbol:\blib\b|lib_rust").unwrap(),
        eq("# The library.\ncc_library(\n    name = \"lib\",\n    srcs = glob([\"*.cc\"]),\n)\n\nrule(\n    name = \"lib_rust\",\n    deps = [\":lib\"],\n)\n")
    );
}

#[gtest]
fn unsupported_query_is_an_error() {
    expect_that!(
        select("x\n", Lang::Rust, "file:x"),
        err(displays_as(contains_substring("unsupported query")))
    );
    expect_that!(
        select("x\n", Lang::Other, "function:x"),
        err(displays_as(contains_substring("not supported")))
    );
}

#[gtest]
fn expands_indented_snippets() {
    let root = make_root("indented", &[("ex/a.rs", &format!("{LICENSE}fn f() {{\n\n    1\n}}\n"))]);
    let markdown =
        "1.  Step:\n\n    ```live-snippet\n    cs/file:ex/a.rs function:f\n    ```\n\nDone.\n";
    expect_that!(
        expand_live_snippets(markdown, &root).unwrap(),
        eq("1.  Step:\n\n    ```rust,noplayground\n    fn f() {\n\n        1\n    }\n    ```\n\nDone.\n")
    );
}

#[gtest]
fn uses_longer_fence_than_contents() {
    let root = make_root("fence", &[("doc.md", "```\ncode\n```\n")]);
    expect_that!(
        expand_live_snippets("```live-snippet\ncs/file:doc.md\n```\n", &root).unwrap(),
        eq("````\n```\ncode\n```\n````\n")
    );
}

#[gtest]
fn reports_all_broken_snippets() {
    let root = make_root("broken", &[("a.rs", "fn f() {}\n")]);
    let markdown = "```live-snippet\ncs/file:missing.rs\n```\n\n```live-snippet\ncs/file:a.rs function:g\n```\n\n```live-snippet\ncs/file:/etc/passwd\n```\n";
    let result = expand_live_snippets(markdown, &root);
    expect_that!(
        result,
        err(displays_as(contains_substring("line 2: `cs/file:missing.rs`: failed to read")))
    );
    expect_that!(
        result,
        err(displays_as(contains_substring(
            "line 6: `cs/file:a.rs function:g`: query matched nothing"
        )))
    );
    expect_that!(result, err(displays_as(contains_substring("must be relative"))));
}

#[gtest]
fn malformed_block_is_an_error() {
    let root = make_root("malformed", &[]);
    expect_that!(
        expand_live_snippets("```live-snippet\ncs/file:a.rs\nextra\n```\n", &root),
        err(displays_as(contains_substring("exactly one line")))
    );
}

#[gtest]
fn run_processes_nested_chapters() {
    let root = make_root("run", &[("a.rs", "fn f() {}\n")]);
    let snippet = "```live-snippet\\ncs/file:a.rs\\n```\\n";
    let input = format!(
        r#"[{{"root": {root:?}}}, {{"items": [{{"Chapter": {{"name": "A", "content": "{snippet}", "path": "a.md", "sub_items": [{{"Chapter": {{"name": "B", "content": "{snippet}", "path": "b.md", "sub_items": []}}}}]}}}}, "Separator"]}}]"#,
        root = root.to_str().unwrap()
    );
    let output: serde_json::Value = serde_json::from_str(&run(&input).unwrap()).unwrap();
    let expected = "```rust,noplayground\nfn f() {}\n```\n";
    expect_that!(output["items"][0]["Chapter"]["content"].as_str(), eq(Some(expected)));
    expect_that!(
        output["items"][0]["Chapter"]["sub_items"][0]["Chapter"]["content"].as_str(),
        eq(Some(expected))
    );
}

#[gtest]
fn run_names_the_chapter_on_error() {
    let root = make_root("run_error", &[]);
    let input = format!(
        r#"[{{"root": {root:?}}}, {{"items": [{{"Chapter": {{"name": "A", "content": "```live-snippet\ncs/file:nope.rs\n```\n", "path": "a.md", "sub_items": []}}}}]}}]"#,
        root = root.to_str().unwrap()
    );
    expect_that!(run(&input), err(displays_as(contains_substring("in a.md: line 2"))));
}
