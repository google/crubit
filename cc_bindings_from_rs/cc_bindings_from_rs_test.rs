// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]
#![feature(proc_macro_hygiene)]
#![feature(cfg_accessible)]

extern crate rustc_driver;

use arc_anyhow::{bail, Result};
use cmdline::Cmdline;
use cpp_api_from_rust_lib::run_with_cmdline_args;
use external_binaries::{CLANG_FORMAT_EXE_PATH, RUSTFMT_EXE_PATH};
use googletest::matchers::{contains_substring, eq, starts_with};
use googletest::{assert_that, gtest};
use regex::{Regex, RegexBuilder};
use run_compiler_test_support::{setup_rustc_target_for_testing, sysroot_path};
use std::path::PathBuf;
use tempfile::{tempdir, TempDir};

fn run_rustc(args: &[String]) -> Result<()> {
    struct Callbacks;
    impl rustc_driver::Callbacks for Callbacks {}
    match rustc_driver::catch_fatal_errors(|| {
        #[cfg_accessible(rustc_driver::run_compiler)]
        rustc_driver::run_compiler(args, &mut Callbacks);
        #[cfg_accessible(rustc_driver::compiler_entrypoint)]
        rustc_driver::compiler_entrypoint(args, &mut Callbacks);
    }) {
        Ok(()) => Ok(()),
        Err(_) => bail!("Errors reported by Rust compiler."),
    }
}

/// Test data builder (see also
/// https://testing.googleblog.com/2018/02/testing-on-toilet-cleanly-create-test.html).
struct TestArgs {
    rs_input: Option<String>,
    h_path: Option<String>,
    error_report_out: Option<String>,
    extra_crubit_args: Vec<String>,

    /// Arg for the following `rustc` flag: `--codegen=panic=<arg>`.
    panic_mechanism: String,

    /// Other `rustc` flags.
    extra_rustc_args: Vec<String>,

    tempdir: TempDir,
    include_guard: Option<String>,
    target: Option<String>,
}

/// Result of `TestArgs::run` that helps tests access test outputs (e.g. the
/// internally generated `h_path` and/or `rs_input_path`).
#[derive(Debug)]
struct TestResult {
    h_path: PathBuf,
    rs_path: PathBuf,
    error_report_out_path: Option<PathBuf>,
}

impl TestArgs {
    fn default_args() -> Result<Self> {
        Ok(Self {
            rs_input: None,
            h_path: None,
            error_report_out: None,
            extra_crubit_args: vec![],
            panic_mechanism: "abort".to_string(),
            extra_rustc_args: vec![],
            tempdir: tempdir()?,
            include_guard: None,
            target: None,
        })
    }

    /// Use the specified `h_path` rather than auto-generating one in
    /// `self`-managed temporary directory.
    fn with_h_path(mut self, h_path: &str) -> Self {
        self.h_path = Some(h_path.to_string());
        self
    }

    /// Specify the path to the error report output file.
    fn with_error_report_out(mut self, error_report_out: &str) -> Self {
        self.error_report_out = Some(error_report_out.to_string());
        self
    }

    /// Specify the test Rust input.
    fn with_rs_input(mut self, rs_input: &str) -> Self {
        self.rs_input = Some(rs_input.to_string());
        self
    }

    /// Overrides `--codegen=panic=abort` with the specified
    /// `panic_mechanism`.
    fn with_panic_mechanism(mut self, panic_mechanism: &str) -> Self {
        self.panic_mechanism = panic_mechanism.to_string();
        self
    }

    /// Appends `extra_rustc_args` at the end of the cmdline (i.e. as
    /// additional rustc args, in addition to `--sysroot`,
    /// `--crate-type=...`, etc.).
    fn with_extra_rustc_args(mut self, extra_rustc_args: &[&str]) -> Self {
        self.extra_rustc_args = extra_rustc_args.iter().map(|t| t.to_string()).collect();
        self
    }

    /// Appends `extra_crubit_args` before the first `--`.
    fn with_extra_crubit_args(mut self, extra_crubit_args: &[&str]) -> Self {
        self.extra_crubit_args = extra_crubit_args.iter().map(|t| t.to_string()).collect();
        self
    }

    fn with_include_guard(mut self, include_guard: &str) -> Self {
        self.include_guard = Some(include_guard.to_string());
        self
    }

    fn with_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }

    /// Invokes `cpp_api_from_rust_lib::run_with_cmdline_args` with default
    /// `test_crate.rs` input (and with other default args + args gathered by
    /// `self`).
    ///
    /// Returns the path to the `h_out` file.  The file's lifetime is the
    /// same as `&self`.
    fn run(&self) -> Result<TestResult> {
        let h_path = match self.h_path.as_ref() {
            None => self.tempdir.path().join("test_crate_cc_api.h"),
            Some(s) => PathBuf::from(s),
        };

        let rs_path = self.tempdir.path().join("test_crate_cc_api_impl.rs");

        let rs_input_path = self.tempdir.path().join("test_crate.rs");
        let rs_input = if let Some(rs_input) = &self.rs_input {
            rs_input
        } else {
            r#" pub mod public_module {
                    pub fn public_function() {
                        private_function()
                    }

                    fn private_function() {}
                }
            "#
        };
        std::fs::write(&rs_input_path, rs_input)?;

        let mut args = vec![
            "cc_bindings_from_rs_unittest_executable".to_string(),
            format!("--h-out={}", h_path.display()),
            format!("--rs-out={}", rs_path.display()),
            "--crubit-support-path-format=<crubit/support/{header}>".to_string(),
            format!("--clang-format-exe-path={CLANG_FORMAT_EXE_PATH}"),
            format!("--rustfmt-exe-path={RUSTFMT_EXE_PATH}"),
        ];

        let mut error_report_out_path = None;
        if let Some(error_report_out) = self.error_report_out.as_ref() {
            error_report_out_path = Some(self.tempdir.path().join(error_report_out));
            args.push(format!(
                "--error-report-out={}",
                error_report_out_path.as_ref().unwrap().display()
            ));
        }

        if let Some(include_guard) = &self.include_guard {
            args.push(format!("--h-out-include-guard={include_guard}"));
        }

        args.extend(self.extra_crubit_args.iter().cloned());

        let target =
            self.target.clone().or_else(|| setup_rustc_target_for_testing(self.tempdir.path()));

        let mut extern_args = vec![];
        if self.target.is_some() {
            let std_rs_path = self.tempdir.path().join("std.rs");
            std::fs::write(
                &std_rs_path,
                r#"
                #![allow(internal_features)]
                #![feature(no_core, lang_items, compiler_builtins)]
                #![no_core]
                #![compiler_builtins]

                #[lang = "pointee_sized"]
                pub trait PointeeSized {}

                #[lang = "meta_sized"]
                pub trait MetaSized: PointeeSized {}

                #[lang = "sized"]
                pub trait Sized: MetaSized {}

                #[lang = "copy"]
                pub trait Copy {}

                pub mod prelude {
                    pub mod rust_2015 {}
                    pub mod rust_2018 {}
                    pub mod rust_2021 {}
                    pub mod rust_2024 {}
                    pub mod v1 {}
                }
                "#,
            )?;
            let std_rmeta_path = self.tempdir.path().join("libstd.rmeta");
            let mut std_rustc_args = vec![
                "rustc".to_string(),
                format!("--codegen=panic={}", &self.panic_mechanism),
                "--crate-name=std".to_string(),
                "--crate-type=lib".to_string(),
                "--emit=metadata".to_string(),
                "-o".to_string(),
                std_rmeta_path.display().to_string(),
                "-Zunstable-options".to_string(),
                std_rs_path.display().to_string(),
            ];
            if let Some(target) = &target {
                std_rustc_args.push(format!("--target={}", target));
            }
            std_rustc_args.extend(self.extra_rustc_args.iter().cloned());
            run_rustc(&std_rustc_args)?;
            extern_args.push(format!("--extern=std={}", std_rmeta_path.display()));
            extern_args.push(format!("-L{}", self.tempdir.path().display()));
        }

        let rmeta_path = self.tempdir.path().join("libtest_crate.rmeta");
        let mut rustc_args = vec![
            "rustc".to_string(),
            format!("--codegen=panic={}", &self.panic_mechanism),
            "--crate-type=lib".to_string(),
            "--emit=metadata".to_string(),
            "-o".to_string(),
            rmeta_path.display().to_string(),
            "-Zunstable-options".to_string(),
            format!(
                "-Cunsafe-allow-abi-mismatch={}",
                run_compiler::target_modifier_flag_names().join(",")
            ),
            rs_input_path.display().to_string(),
        ];
        if let Some(sysroot) = sysroot_path() {
            rustc_args.push(format!("--sysroot={}", sysroot.display()));
        }
        if let Some(target) = &target {
            rustc_args.push(format!("--target={}", target));
        }
        rustc_args.extend(extern_args.iter().cloned());
        rustc_args.extend(self.extra_rustc_args.iter().cloned());
        run_rustc(&rustc_args)?;

        args.push("--source-crate-name=test_crate".to_string());
        args.extend(extern_args);
        args.push(format!("--extern=test_crate={}", rmeta_path.display()));
        if let Some(sysroot) = sysroot_path() {
            args.push(format!("--sysroot={}", sysroot.display()));
        }
        if let Some(target) = &target {
            args.push(format!("--target={}", target));
        }

        let cmdline = Cmdline::new(&args)?;
        run_with_cmdline_args(&cmdline)?;

        Ok(TestResult { h_path, rs_path, error_report_out_path })
    }
}

// TODO(lukasza): b/261074843 - Go back to exact string matching (and hardcoding thunk
// names) once we are using stable name mangling.
// ("Go back" = more or less revert cl/492292910 + manual review and
// tweaks.)
fn assert_body_matches(actual: &str, expected: &str) {
    fn build_regex(expected_body: &str) -> Regex {
        let patt = regex::escape(expected_body);
        let patt = format!("^{patt}"); // Not always matching $ enables prefix checks below.
        let patt = patt.replace("ANY_IDENTIFIER_CHARACTERS", "[a-zA-Z0-9_]*");
        RegexBuilder::new(&patt).multi_line(false).dot_matches_new_line(false).build().unwrap()
    }
    let is_whole_h_body_matching = {
        match build_regex(expected).shortest_match(&actual) {
            None => false,
            Some(len) => len == actual.len(),
        }
    };
    if !is_whole_h_body_matching {
        let longest_matching_expectation_len = (0..=expected.len())
            .rev() // Iterating from longest to shortest prefix
            .filter(|&len| {
                expected
                    .get(0..len) // Only valid UTF-8 boundaries
                    .filter(|prefix| build_regex(prefix).is_match(&actual))
                    .is_some()
            })
            .next() // Getting the first regex that matched
            .unwrap(); // We must get a match at least for 0-length expected body
        let longest_matching_regex = build_regex(&expected[0..longest_matching_expectation_len]);
        let len_of_longest_match = longest_matching_regex.shortest_match(&actual).unwrap(); // Again
                                                                                            // - we must
                                                                                            // get a
                                                                                            // match
                                                                                            // at least
                                                                                            // for 0-length
                                                                                            // expected
                                                                                            // body
        let mut marked_body = actual.to_string();
        marked_body.insert_str(len_of_longest_match, "!!!>>>");
        let mut marked_pattern = expected.to_string();
        marked_pattern.insert_str(longest_matching_expectation_len, "!!!>>>");
        panic!(
            "Mismatched expectations:\n\
                #### Actual body (first mismatch follows the \"!!!>>>\" marker):\n\
                {marked_body}\n\
                #### Mismatched pattern (mismatch follows the \"!!!>>>\" marker):\n\
                {marked_pattern}"
        );
    }
}

// TODO: b/455963829 - Replace this with golden test so it's easier to update.
#[gtest]
fn test_error_reporting_generation() -> Result<()> {
    let test_args =
        TestArgs::default_args()?.with_error_report_out("error_report.json").with_rs_input(
            r#"
            pub struct Unsupported<T> {
                pub field: T,
            }
            "#,
        );

    let test_result = test_args.run().expect("Error report generation should succeed");
    assert_that!(test_result.error_report_out_path.is_some(), eq(true));
    let error_report_out_path = test_result.error_report_out_path.as_ref().unwrap();
    assert_that!(error_report_out_path.exists(), eq(true));
    let error_report = std::fs::read_to_string(&error_report_out_path)?;
    let expected_error_report = r#"[
  {
    "source_language": "Rust",
    "name": "test_crate::Unsupported",
    "errors": [
      {
        "fmt": "crubit.rs/errors/unsupported_type: Generic types are not supported yet (b/259749095)"
      }
    ]
  }
]"#;
    assert_that!(error_report, eq(expected_error_report));
    Ok(())
}

#[gtest]
fn test_error_reporting_generation_const_generics() -> Result<()> {
    let test_args = TestArgs::default_args()?
        .with_extra_crubit_args(&["--crate-feature=self=generics"])
        .with_error_report_out("error_report.json")
        .with_rs_input(
            r#"
            pub struct Unsupported<const N: usize> {
                pub field: [u8; N],
            }
            "#,
        );

    let test_result = test_args.run().expect("Error report generation should succeed");
    assert_that!(test_result.error_report_out_path.is_some(), eq(true));
    let error_report_out_path = test_result.error_report_out_path.as_ref().unwrap();
    assert_that!(error_report_out_path.exists(), eq(true));
    let error_report = std::fs::read_to_string(&error_report_out_path)?;
    let expected_error_report = r#"[
  {
    "source_language": "Rust",
    "name": "test_crate::Unsupported",
    "errors": [
      {
        "fmt": "crubit.rs/errors/unsupported_type: `const`-generic ADTs are not supported yet (b/259749095)"
      }
    ]
  }
]"#;
    assert_that!(error_report, eq(expected_error_report));
    Ok(())
}

#[gtest]
fn test_with_include_guard() -> Result<()> {
    let test_args =
        TestArgs::default_args()?.with_include_guard("CRUBIT_GENERATED_HEADER_FOR_test_crate_");
    let test_result = test_args.run().expect("Customized include guard should succeed");

    assert_that!(test_result.h_path.exists(), eq(true));
    let temp_dir_str = test_args.tempdir.path().to_str().unwrap();
    let cc_api = std::fs::read_to_string(&test_result.h_path)?;
    #[rustfmt::skip]
    assert_body_matches(
        &cc_api,
        &format!(
            "{}\n{}\n{}",
r#"// Automatically @generated C++ bindings for the following Rust crate:
// test_crate
// Features: <none>

// clang-format off
#ifndef CRUBIT_GENERATED_HEADER_FOR_test_crate_
#define CRUBIT_GENERATED_HEADER_FOR_test_crate_

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"

namespace test_crate::public_module {
"#,
format!("// Generated from: {temp_dir_str}/test_crate.rs;l=2"),
r#"void public_function();

namespace __crubit_internal {
extern "C" void
__crubit_thunk_ANY_IDENTIFIER_CHARACTERS();
}
inline void public_function() {
  return __crubit_internal::
      __crubit_thunk_ANY_IDENTIFIER_CHARACTERS();
}

}  // namespace test_crate::public_module

#pragma clang diagnostic pop
#endif  // CRUBIT_GENERATED_HEADER_FOR_test_crate_
"#
        ),
    );
    Ok(())
}

#[gtest]
fn test_happy_path() -> Result<()> {
    let test_args = TestArgs::default_args()?;
    let test_result = test_args.run().expect("Default args should succeed");

    assert_that!(test_result.h_path.exists(), eq(true));
    let temp_dir_str = test_args.tempdir.path().to_str().unwrap();
    let cc_api = std::fs::read_to_string(&test_result.h_path)?;
    #[rustfmt::skip]
    assert_body_matches(
        &cc_api,
        &format!(
            "{}\n{}\n{}",
r#"// Automatically @generated C++ bindings for the following Rust crate:
// test_crate
// Features: <none>

// clang-format off
#pragma once

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"

namespace test_crate::public_module {
"#,
format!("// Generated from: {temp_dir_str}/test_crate.rs;l=2"),
r#"void public_function();

namespace __crubit_internal {
extern "C" void
__crubit_thunk_ANY_IDENTIFIER_CHARACTERS();
}
inline void public_function() {
  return __crubit_internal::
      __crubit_thunk_ANY_IDENTIFIER_CHARACTERS();
}

}  // namespace test_crate::public_module

#pragma clang diagnostic pop
"#
        ),
    );

    assert_that!(test_result.rs_path.exists(), eq(true));
    let cc_api_impl = std::fs::read_to_string(&test_result.rs_path)?;
    assert_body_matches(
        &cc_api_impl,
        r#"// Automatically @generated C++ bindings for the following Rust crate:
// test_crate
// Features: <none>

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_ANY_IDENTIFIER_CHARACTERS()
-> () {
    unsafe { ::test_crate::public_module::public_function() }
}
"#,
    );
    Ok(())
}

/// `test_cmdline_error_propagation` tests that errors from `Cmdline::new`
/// get propagated. More detailed test coverage of various specific
/// error types can be found in tests in `cmdline.rs`.
#[gtest]
fn test_cmdline_error_propagation() -> Result<()> {
    let err = TestArgs::default_args()?
        .with_extra_crubit_args(&["--unrecognized-crubit-flag"])
        .run()
        .expect_err("--unrecognized_crubit_flag should trigger an error");

    let msg = format!("{err:#}");
    assert_that!(msg, contains_substring("unexpected argument '--unrecognized-crubit-flag'"));
    Ok(())
}

/// `test_run_compiler_error_propagation` tests that errors from
/// `run_compiler` get propagated. More detailed test coverage of
/// various specific error types can be found in tests in `run_compiler.
/// rs`.
#[gtest]
fn test_run_compiler_error_propagation() -> Result<()> {
    let err = TestArgs::default_args()?
        .with_extra_rustc_args(&["--unrecognized-rustc-flag"])
        .run()
        .expect_err("--unrecognized-rustc-flag should trigger an error");

    let msg = format!("{err:#}");
    assert_that!(msg, eq("Errors reported by Rust compiler."));
    Ok(())
}

/// `test_rustc_with_panic_unwind` tests that `panic=unwind`
/// is supported at least nominally.
///
/// This is tested at the `cc_bindings_from_rs.rs` level instead of at the
/// `bindings.rs` level, because `run_compiler_test_support` doesn't
/// support specifying a custom panic mechanism.
#[gtest]
fn test_rustc_with_panic_unwind() -> Result<()> {
    let _ = TestArgs::default_args()?
        .with_panic_mechanism("unwind")
        .run()
        .expect("panic=unwind should not cause an error");
    Ok(())
}

#[gtest]
fn test_rustc_with_panic_abort() -> Result<()> {
    let _ = TestArgs::default_args()?
        .with_panic_mechanism("abort")
        .run()
        .expect("panic=abort should not cause an error");
    Ok(())
}

/// `test_invalid_h_out_path` tests not only the specific problem of an
/// invalid `--h-out` argument, but also tests that errors from
/// `run_with_tcx` are propagated.
#[gtest]
fn test_invalid_h_out_path() -> Result<()> {
    let err = TestArgs::default_args()?
        .with_h_path("../..")
        .run()
        .expect_err("Unwriteable --h-out should trigger an error");

    let msg = format!("{err:#}");
    assert_that!(
        msg,
        contains_substring("Error when writing to ../..: Is a directory (os error 21)")
    );
    Ok(())
}

#[gtest]
fn test_crate_features() -> Result<()> {
    let test_args = TestArgs::default_args()?.with_extra_crubit_args(&[
        "--default-features=supported",
        "--crate-feature=self=experimental",
    ]);
    let test_result = test_args.run()?;
    let cc_api = std::fs::read_to_string(&test_result.h_path)?;
    assert_that!(
        cc_api,
        starts_with(
            "// Automatically @generated C++ bindings for the following Rust crate:\n\
            // test_crate\n\
            // Features: experimental, impl_cxx_extern_type, supported, types"
        )
    );
    Ok(())
}

#[gtest]
fn test_crate_disabled_features() -> Result<()> {
    let test_args = TestArgs::default_args()?.with_extra_crubit_args(&[
        "--default-features=supported,experimental",
        "--crate-disabled-feature=self=experimental",
    ]);
    let test_result = test_args.run()?;
    let cc_api = std::fs::read_to_string(&test_result.h_path)?;
    assert_that!(
        cc_api,
        starts_with(
            "// Automatically @generated C++ bindings for the following Rust crate:\n\
            // test_crate\n\
            // Features: impl_cxx_extern_type, supported, types"
        )
    );
    Ok(())
}

#[gtest]
fn test_crate_default_features() -> Result<()> {
    let test_args =
        TestArgs::default_args()?.with_extra_crubit_args(&["--default-features=supported"]);
    let test_result = test_args.run()?;
    let cc_api = std::fs::read_to_string(&test_result.h_path)?;
    assert_that!(
        cc_api,
        starts_with(
            "// Automatically @generated C++ bindings for the following Rust crate:\n\
            // test_crate\n\
            // Features: impl_cxx_extern_type, supported, types"
        )
    );
    Ok(())
}

#[gtest]
fn test_crate_rename_reserved_keyword() -> Result<()> {
    let test_args =
        TestArgs::default_args()?.with_extra_crubit_args(&["--crate-rename=some_proto_crate=type"]);
    let test_result = test_args.run()?;
    let rs_body = std::fs::read_to_string(&test_result.rs_path)?;
    assert_that!(rs_body, contains_substring("extern crate some_proto_crate as r#type;"));
    Ok(())
}

#[gtest]
fn test_rustc_with_unstable_target_modifier_flags() -> Result<()> {
    let test_args = TestArgs::default_args()?
        .with_target("aarch64-unknown-linux-gnu")
        .with_extra_rustc_args(&["-Zbranch-protection=bti", "-Zfixed-x18"]);
    let test_result =
        test_args.run().expect("Unstable target modifier flags with '-' should not cause an error");
    let cc_api = std::fs::read_to_string(&test_result.h_path)?;
    assert_that!(cc_api, contains_substring("public_function"));
    Ok(())
}
