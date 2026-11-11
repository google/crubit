// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use external_binaries::{CLANG_FORMAT_EXE_PATH, RUSTFMT_EXE_PATH};
use googletest::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use tempfile::{tempdir, TempDir};

fn rs_bindings_from_cc_cmd() -> Command {
    Command::new(env!("RS_BINDINGS_FROM_CC"))
}

fn target_args_json(headers: &[PathBuf]) -> String {
    let quoted_headers: Vec<String> =
        headers.iter().map(|h| format!("\"{}\"", h.display())).collect();
    format!(
        r#"[{{"t": "//foo/bar:baz", "h": [{}], "f": ["supported"]}}]"#,
        quoted_headers.join(", ")
    )
}

struct TestInvocation {
    tempdir: TempDir,
    rs_out: PathBuf,
    cc_out: PathBuf,
    support_path_format: &'static str,
    clang_format_exe_path: Option<&'static str>,
    rustfmt_exe_path: Option<&'static str>,
    rustfmt_config_path: Option<PathBuf>,
    public_headers: Vec<PathBuf>,
    extra_crubit_args: Vec<String>,
    clang_args: Vec<String>,
}

impl TestInvocation {
    fn new() -> Self {
        let tempdir = tempdir().expect("failed to create tempdir");
        let rs_out = tempdir.path().join("rs_api.rs");
        let cc_out = tempdir.path().join("rs_api_impl.cc");
        Self {
            tempdir,
            rs_out,
            cc_out,
            support_path_format: "<test/crubit/support/path/{header}>",
            clang_format_exe_path: Some(CLANG_FORMAT_EXE_PATH),
            rustfmt_exe_path: Some(RUSTFMT_EXE_PATH),
            rustfmt_config_path: None,
            public_headers: Vec::new(),
            extra_crubit_args: Vec::new(),
            clang_args: Vec::new(),
        }
    }

    fn write_header(&mut self, name: &str, contents: &str) -> PathBuf {
        let path = self.tempdir.path().join(name);
        fs::write(&path, contents).expect("failed to write test header");
        self.public_headers.push(path.clone());
        path
    }

    fn run(&self) -> Output {
        let mut cmd = rs_bindings_from_cc_cmd();
        cmd.arg("--target=//foo/bar:baz")
            .arg(format!("--rs_out={}", self.rs_out.display()))
            .arg(format!("--cc_out={}", self.cc_out.display()))
            .arg(format!("--crubit_support_path_format={}", self.support_path_format));
        if let Some(clang_format) = self.clang_format_exe_path {
            cmd.arg(format!("--clang_format_exe_path={clang_format}"));
        }
        if let Some(rustfmt) = self.rustfmt_exe_path {
            cmd.arg(format!("--rustfmt_exe_path={rustfmt}"));
        }
        if let Some(rustfmt_config) = &self.rustfmt_config_path {
            cmd.arg(format!("--rustfmt_config_path={}", rustfmt_config.display()));
        }
        let public_headers_arg: Vec<String> =
            self.public_headers.iter().map(|h| h.display().to_string()).collect();
        cmd.arg(format!("--public_headers={}", public_headers_arg.join(",")))
            .arg(format!("--target_args={}", target_args_json(&self.public_headers)));
        for arg in &self.extra_crubit_args {
            cmd.arg(arg);
        }
        if !self.clang_args.is_empty() {
            cmd.arg("--");
            for arg in &self.clang_args {
                cmd.arg(arg);
            }
        }
        cmd.output().expect("failed to execute rs_bindings_from_cc")
    }

    fn run_and_expect_success(&self) -> Output {
        let out = self.run();
        assert_true!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
        out
    }
}

fn expect_cmd_fails_with(cmd: &mut Command, expected_stderr: &str) {
    let out = cmd.output().unwrap();
    expect_false!(out.status.success());
    expect_that!(String::from_utf8_lossy(&out.stderr), contains_substring(expected_stderr));
}

#[gtest]
fn test_cmd_line_api() {
    let mut inv = TestInvocation::new();
    let hdr = inv.write_header("hello_world.h", "int foo();\n");

    let mut cmd = rs_bindings_from_cc_cmd();
    expect_cmd_fails_with(&mut cmd, "please specify --target");

    cmd.arg("--target=//:target");
    expect_cmd_fails_with(&mut cmd, "please specify --rs_out");

    cmd.arg(format!("--rs_out={}", inv.rs_out.display()));
    expect_cmd_fails_with(&mut cmd, "please specify --cc_out");

    cmd.arg(format!("--cc_out={}", inv.cc_out.display()));
    expect_cmd_fails_with(&mut cmd, "please specify --crubit_support_path_format");

    cmd.arg("--crubit_support_path_format=<test/crubit/support/path/{header}>")
        .arg(format!("--clang_format_exe_path={CLANG_FORMAT_EXE_PATH}"))
        .arg(format!("--rustfmt_exe_path={RUSTFMT_EXE_PATH}"));
    expect_cmd_fails_with(&mut cmd, "please specify --public_headers");

    cmd.arg(format!("--public_headers={}", hdr.display()));
    expect_cmd_fails_with(&mut cmd, "please specify --target_args");

    inv.run_and_expect_success();
    expect_ne!(fs::read_to_string(&inv.rs_out).unwrap(), "");
    expect_ne!(fs::read_to_string(&inv.cc_out).unwrap(), "");
}

/// Successful runs on ordinary headers should not print anything to stderr
/// (except for the known lines allowlisted below).
/// `--stderrthreshold=1` makes warnings visible (as they are in builds that do
/// not set the threshold, e.g. the OSS build).
#[gtest]
fn test_no_log_spam_on_success() {
    let mut inv = TestInvocation::new();
    inv.write_header("hello_world.h", "int foo();\n");
    inv.extra_crubit_args.push("--stderrthreshold=1".to_string());

    let out = inv.run_and_expect_success();

    // Known, benign stderr lines:
    // * `DEBUG BINARY` is logged by `InitGoogle` in non-optimized builds.
    // * The `'linker' input unused` clang warning (and the `1 warning generated.` summary that
    //   follows it) is caused by this test's command line.
    let stderr = String::from_utf8_lossy(&out.stderr);
    let unexpected_stderr: Vec<&str> = stderr
        .lines()
        .filter(|line| {
            !line.ends_with("] DEBUG BINARY -- Performance may suffer")
                && !line.contains("'linker' input unused when '-fsyntax-only' is present")
                && *line != "1 warning generated."
        })
        .collect();
    expect_that!(
        unexpected_stderr,
        is_empty(),
        "Unexpected stderr output:\n{}",
        unexpected_stderr.join("\n")
    );
}

#[gtest]
fn test_optional_formatting_paths() {
    let mut inv = TestInvocation::new();
    inv.clang_format_exe_path = None;
    inv.rustfmt_exe_path = None;
    inv.public_headers.push(PathBuf::from("no_such_file.h"));
    inv.extra_crubit_args.push("--do_nothing".to_string());

    inv.run_and_expect_success();
    expect_ne!(fs::read_to_string(&inv.rs_out).unwrap(), "");
    expect_ne!(fs::read_to_string(&inv.cc_out).unwrap(), "");
}

#[gtest]
fn test_do_nothing() {
    let mut inv = TestInvocation::new();
    inv.public_headers.push(PathBuf::from("no_such_file.h"));
    inv.extra_crubit_args.push("--do_nothing".to_string());

    inv.run_and_expect_success();
    expect_that!(
        fs::read_to_string(&inv.rs_out).unwrap(),
        contains_substring("// intentionally left empty because --do_nothing was passed.")
    );
    expect_that!(
        fs::read_to_string(&inv.cc_out).unwrap(),
        contains_substring("// intentionally left empty because --do_nothing was passed.")
    );
}

/// This test is a bit redundant with
/// `GenerateBindingsAndMetadataTest.FailsOnUnknownClangDriverArgument`,
/// but we need this test to also verify the non-zero exit code.
#[gtest]
fn test_tool_returns_nonzero_on_invalid_input() {
    let mut inv = TestInvocation::new();
    inv.write_header("hello_world.h", "int foo(); But this is not C++;\n");

    let out = inv.run();
    expect_false!(out.status.success());
    // No output files should be created if the C++ input was invalid.
    expect_false!(inv.rs_out.exists());
    expect_false!(inv.cc_out.exists());
}

#[gtest]
fn test_tool_returns_nonzero_on_clang_driver_error() {
    let mut inv = TestInvocation::new();
    inv.write_header("hello_world.h", "int foo();\n");
    inv.clang_args.push("-fthis-flag-does-not-exist".to_string());

    let out = inv.run();
    expect_false!(out.status.success());
    // No output files should be created if the clang driver reported errors.
    expect_false!(inv.rs_out.exists());
    expect_false!(inv.cc_out.exists());
}

#[gtest]
fn test_public_headers() {
    let mut inv = TestInvocation::new();
    inv.write_header("header_1.h", "int function_1();\n");
    inv.write_header("header_2.h", "int function_2();\n");

    inv.run_and_expect_success();
    let rs_api = fs::read_to_string(&inv.rs_out).unwrap();
    expect_that!(rs_api, contains_substring("function_1"), "function_1 was not imported");
    expect_that!(rs_api, contains_substring("function_2"), "function_2 was not imported");
}

#[gtest]
fn test_rustfmt_config_path() {
    let mut inv = TestInvocation::new();
    inv.write_header("hello_world.h", "float MyFunction(float arg1, float arg2);\n");

    // Testing the default `rustfmt` config:
    //     pub fn MyFunction(arg1: f32, arg2: f32) -> f32
    inv.run_and_expect_success();
    let default_rs_api = fs::read_to_string(&inv.rs_out).unwrap();
    expect_that!(
        default_rs_api,
        contains_regex(r"MyFunction.*arg1:.*f32,.*arg2:.*f32.*\)"),
        "Verify function args are on single line when using default rustfmt config (1)"
    );
    expect_that!(
        default_rs_api,
        not(contains_regex(r"(?m)^[^a-z]*arg1:[^a-z]*f32,[^a-z]*$")),
        "Verify function args are on single line when using default rustfmt config (2)"
    );

    // Testing a custom `rustfmt` config:
    //     pub fn MyFunction(
    //         arg1: f32,
    //         arg2: f32,
    //     ) -> f32
    let rustfmt_config_path = inv.tempdir.path().join("rustfmt.toml");
    fs::write(
        &rustfmt_config_path,
        "edition = \"2021\"\nversion = \"Two\"\nfn_args_layout=\"Vertical\"\n",
    )
    .unwrap();
    inv.rustfmt_config_path = Some(rustfmt_config_path);

    inv.run_and_expect_success();
    let custom_rs_api = fs::read_to_string(&inv.rs_out).unwrap();
    expect_that!(
        custom_rs_api,
        not(contains_regex(r"MyFunction.*arg1:.*f32,.*arg2:.*f32.*\)")),
        "Verify function args are *not* on single line when using custom rustfmt config (1)"
    );
    expect_that!(
        custom_rs_api,
        contains_regex(r"(?m)^[^a-z]*arg1:[^a-z]*f32,[^a-z]*$"),
        "Verify function args are *not* on single line when using custom rustfmt config (2)"
    );
}

#[gtest]
fn test_crubit_support_path_format() {
    let mut inv = TestInvocation::new();
    inv.write_header("hello_world.h", "int MyFunction();\n");
    inv.support_path_format = "<test/specific/crubit/support/path/{header}>";

    inv.run_and_expect_success();
    let cc_api_impl = fs::read_to_string(&inv.cc_out).unwrap();
    expect_that!(
        cc_api_impl,
        contains_regex(r"#include <test/specific/crubit/support/path/.*>"),
        "Verify #include paths are based on the argument of --crubit_support_path_format"
    );
}

#[gtest]
fn test_clang_depfile_args() {
    let mut inv = TestInvocation::new();
    let included_hdr = inv.tempdir.path().join("included.h");
    fs::write(&included_hdr, "int MyFunction();\n").unwrap();
    inv.write_header("hello_world.h", "#include \"included.h\"\n");

    let depfile = inv.tempdir.path().join("rs_api.d");
    inv.clang_args = vec![
        "-MD".to_string(),
        "-MF".to_string(),
        depfile.display().to_string(),
        "-MT".to_string(),
        inv.rs_out.display().to_string(),
    ];

    inv.run_and_expect_success();
    let depfile_contents = fs::read_to_string(&depfile).unwrap();
    expect_that!(
        depfile_contents,
        starts_with(format!("{}: ", inv.rs_out.display())),
        "Verify that the depfile target comes from the -MT argument"
    );
    expect_that!(
        depfile_contents,
        contains_substring(included_hdr.display().to_string()),
        "Verify that the depfile includes transitive dependencies"
    );
}
