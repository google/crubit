// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(never_type)]
#![feature(rustc_private)]
#![deny(rustc::internal)]

extern crate rustc_middle;

use arc_anyhow::{Context, Result};
use itertools::Itertools;
use rustc_middle::ty::TyCtxt; // See also <internal link>/ty.html#import-conventions
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use bindings::Database;
use cmdline::Cmdline;
use code_gen_utils::CcInclude;
use error_report::{ErrorReport, ErrorReporting, IgnoreErrors};
use run_compiler::run_compiler;
use token_stream_printer::{
    cc_tokens_to_formatted_string, rs_tokens_to_formatted_string, RustfmtConfig,
};

fn write_file(path: &Path, content: &str) -> Result<()> {
    std::fs::write(path, content)
        .with_context(|| format!("Error when writing to {}", path.display()))
}

fn new_db<'tcx>(
    cmdline: &Cmdline,
    tcx: TyCtxt<'tcx>,
    errors: Rc<dyn ErrorReporting>,
) -> Database<'tcx> {
    let crubit_support_path_format = cmdline.crubit_support_path_format.as_str().into();

    let mut crate_name_to_include_paths = <HashMap<Rc<str>, Vec<CcInclude>>>::new();
    for (crate_name, include_path) in &cmdline.crate_headers {
        let paths = crate_name_to_include_paths.entry(crate_name.as_str().into()).or_default();
        paths.push(CcInclude::user_header(include_path.as_str().into()));
    }

    let mut crate_name_to_features =
        <HashMap<Rc<str>, flagset::FlagSet<crubit_feature::CrubitFeature>>>::new();
    for (crate_name, features) in &cmdline.crate_features {
        let accumulated_features =
            crate_name_to_features.entry(crate_name.as_str().into()).or_default();
        *accumulated_features |= *features
    }

    Database::new(
        tcx,
        crubit_support_path_format,
        crate_name_to_include_paths.into(),
        crate_name_to_features.into(),
        errors,
        cmdline.no_thunk_name_mangling,
    )
}

fn run_with_tcx(cmdline: &Cmdline, tcx: TyCtxt) -> Result<()> {
    use bindings::{generate_bindings, Output};

    let errors: Rc<dyn ErrorReporting> = if cmdline.error_report_out.is_some() {
        Rc::new(ErrorReport::new())
    } else {
        Rc::new(IgnoreErrors)
    };

    let Output { h_body, rs_body } = {
        let db = new_db(cmdline, tcx, errors.clone());
        generate_bindings(&db)?
    };

    {
        let h_body = cc_tokens_to_formatted_string(h_body, &cmdline.clang_format_exe_path)?;
        write_file(&cmdline.h_out, &h_body)?;
    }

    {
        let rustfmt_config =
            RustfmtConfig::new(&cmdline.rustfmt_exe_path, cmdline.rustfmt_config_path.as_deref());
        let rs_body = rs_tokens_to_formatted_string(rs_body, &rustfmt_config)?;
        write_file(&cmdline.rs_out, &rs_body)?;
    }

    if let Some(error_report_out) = &cmdline.error_report_out {
        write_file(error_report_out, &errors.serialize_to_string().unwrap())?;
    }

    Ok(())
}

/// Main entrypoint that (unlike `main`) doesn't do any intitializations that
/// should only happen once for the binary (e.g. it doesn't call
/// `init_env_logger`) and therefore can be used from the tests module below.
fn run_with_cmdline_args(args: &[String]) -> Result<()> {
    let cmdline = Cmdline::new(args)?;
    run_compiler(&cmdline.rustc_args, |tcx| run_with_tcx(&cmdline, tcx))
}

fn main() -> Result<()> {
    // TODO: Investigate if we should install a signal handler here.  See also how
    // compiler/rustc_driver/src/lib.rs calls `signal_handler::install()`.

    // TODO(b/254689400): Provide Crubit-specific panic hook message (we shouldn't
    // use `rustc_driver::install_ice_hook` because it's message asks to file
    // bugs at https://github.com/rust-lang/rust/issues/new.

    // `std::env::args()` will panic if any of the cmdline arguments are not valid
    // Unicode.  This seems okay.
    let args = std::env::args().collect_vec();

    run_with_cmdline_args(&args).map_err(|err| match err.downcast_ref::<clap::Error>() {
        // Explicitly call `clap::Error::exit`, because 1) it results in *colored* output and
        // 2) it uses a zero exit code for specific "errors" (e.g. for `--help` output).
        Some(clap_err) => {
            let _: ! = clap_err.exit();
        }

        // Return `err` from `main`.  This will print the error message (no color codes
        // though) and terminate the process with a non-zero exit code.
        None => err,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use itertools::Itertools;
    use regex::{Regex, RegexBuilder};
    use run_compiler_test_support::get_sysroot_for_testing;
    use run_compiler_test_support::setup_rustc_target_for_testing;
    use std::path::PathBuf;
    use tempfile::{tempdir, TempDir};
    use token_stream_printer::{CLANG_FORMAT_EXE_PATH_FOR_TESTING, RUSTFMT_EXE_PATH_FOR_TESTING};

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

        /// Replaces the default `--codegen=panic=abort` with the specified
        /// `panic_mechanism`.
        fn with_panic_mechanism(mut self, panic_mechanism: &str) -> Self {
            self.panic_mechanism = panic_mechanism.to_string();
            self
        }

        /// Appends `extra_rustc_args` at the end of the cmdline (i.e. as
        /// additional rustc args, in addition to `--sysroot`,
        /// `--crate-type=...`, etc.).
        fn with_extra_rustc_args(mut self, extra_rustc_args: &[&str]) -> Self {
            self.extra_rustc_args = extra_rustc_args.iter().map(|t| t.to_string()).collect_vec();
            self
        }

        /// Appends `extra_crubit_args` before the first `--`.
        fn with_extra_crubit_args(mut self, extra_crubit_args: &[&str]) -> Self {
            self.extra_crubit_args = extra_crubit_args.iter().map(|t| t.to_string()).collect_vec();
            self
        }

        /// Invokes `super::run_with_cmdline_args` with default `test_crate.rs`
        /// input (and with other default args + args gathered by
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
                format!("--clang-format-exe-path={CLANG_FORMAT_EXE_PATH_FOR_TESTING}"),
                format!("--rustfmt-exe-path={RUSTFMT_EXE_PATH_FOR_TESTING}"),
            ];

            let mut error_report_out_path = None;
            if let Some(error_report_out) = self.error_report_out.as_ref() {
                error_report_out_path = Some(self.tempdir.path().join(error_report_out));
                args.push(format!(
                    "--error-report-out={}",
                    error_report_out_path.as_ref().unwrap().display()
                ));
            }
            args.extend(self.extra_crubit_args.iter().cloned());
            args.extend([
                "--".to_string(),
                format!("--codegen=panic={}", &self.panic_mechanism),
                "--crate-type=lib".to_string(),
                format!("--sysroot={}", get_sysroot_for_testing().display()),
                rs_input_path.display().to_string(),
            ]);

            if let Some(target) = &setup_rustc_target_for_testing(self.tempdir.path()) {
                args.push(format!("--target={}", target));
            }

            args.extend(self.extra_rustc_args.iter().cloned());

            run_with_cmdline_args(&args)?;

            Ok(TestResult { h_path, rs_path, error_report_out_path })
        }
    }

    // TODO(b/261074843): Go back to exact string matching (and hardcoding thunk
    // names) once we are using stable name mangling (which may be coming in Q1
    // 2023).  ("Go back" = more or less revert cl/492292910 + manual review and
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
            let longest_matching_regex =
                build_regex(&expected[0..longest_matching_expectation_len]);
            let len_of_longest_match = longest_matching_regex.shortest_match(&actual).unwrap(); // Again - we must get a match at least for 0-length expected body
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

    #[test]
    fn test_error_reporting_generation() -> Result<()> {
        let test_args =
            TestArgs::default_args()?.with_error_report_out("error_report.json").with_rs_input(
                r#"
                pub use std::collections;
                pub use std::path;
                "#,
            );

        let test_result = test_args.run().expect("Error report generation should succeed");
        assert!(test_result.error_report_out_path.is_some());
        let error_report_out_path = test_result.error_report_out_path.as_ref().unwrap();
        assert!(error_report_out_path.exists());
        let error_report = std::fs::read_to_string(&error_report_out_path)?;
        let expected_error_report = r#"{
  "Unsupported use statement that refers to this type of the entity: {:#?}": {
    "count": 2,
    "sample_message": "Unsupported use statement that refers to this type of the entity: Def(\n    Mod,\n    DefId(std[46ff]::collections),\n)"
  }
}"#;
        assert_eq!(expected_error_report, error_report);
        Ok(())
    }

    #[test]
    fn test_happy_path() -> Result<()> {
        let test_args = TestArgs::default_args()?;
        let test_result = test_args.run().expect("Default args should succeed");

        assert!(test_result.h_path.exists());
        let temp_dir_str = test_args.tempdir.path().to_str().unwrap();
        let h_body = std::fs::read_to_string(&test_result.h_path)?;
        #[rustfmt::skip]
        assert_body_matches(
            &h_body,
            &format!(
                "{}\n{}\n{}",
r#"// Automatically @generated C++ bindings for the following Rust crate:
// test_crate

#pragma once

namespace test_crate {

namespace public_module {
"#,
 // TODO(b/261185414): Avoid assuming that all source code paths are google3 paths.
format!("// Generated from: google3/{temp_dir_str}/test_crate.rs;l=2"),
r#"void public_function();

namespace __crubit_internal {
extern "C" void
__crubit_thunk__ANY_IDENTIFIER_CHARACTERS();
}
inline void public_function() {
  return __crubit_internal::
      __crubit_thunk__ANY_IDENTIFIER_CHARACTERS();
}

}  // namespace public_module

}  // namespace test_crate
"#
            ),
        );

        assert!(test_result.rs_path.exists());
        let rs_body = std::fs::read_to_string(&test_result.rs_path)?;
        assert_body_matches(
            &rs_body,
            r#"// Automatically @generated C++ bindings for the following Rust crate:
// test_crate

#![allow(improper_ctypes_definitions)]

#[no_mangle]
extern "C" fn __crubit_thunk__ANY_IDENTIFIER_CHARACTERS()
-> () {
    ::test_crate::public_module::public_function()
}
"#,
        );
        Ok(())
    }

    /// `test_cmdline_error_propagation` tests that errors from `Cmdline::new`
    /// get propagated. More detailed test coverage of various specific
    /// error types can be found in tests in `cmdline.rs`.
    #[test]
    fn test_cmdline_error_propagation() -> Result<()> {
        let err = TestArgs::default_args()?
            .with_extra_crubit_args(&["--unrecognized-crubit-flag"])
            .run()
            .expect_err("--unrecognized_crubit_flag should trigger an error");

        let msg = format!("{err:#}");
        assert!(msg.contains("unexpected argument '--unrecognized-crubit-flag'"), "msg = {}", msg,);
        Ok(())
    }

    /// `test_run_compiler_error_propagation` tests that errors from
    /// `run_compiler` get propagated. More detailed test coverage of
    /// various specific error types can be found in tests in `run_compiler.
    /// rs`.
    #[test]
    fn test_run_compiler_error_propagation() -> Result<()> {
        let err = TestArgs::default_args()?
            .with_extra_rustc_args(&["--unrecognized-rustc-flag"])
            .run()
            .expect_err("--unrecognized-rustc-flag should trigger an error");

        let msg = format!("{err:#}");
        assert_eq!("Errors reported by Rust compiler.", msg);
        Ok(())
    }

    /// `test_rustc_with_panic_unwind` tests that `panic=unwind`
    /// is supported at least nominally.
    ///
    /// This is tested at the `cc_bindings_from_rs.rs` level instead of at the
    /// `bindings.rs` level, because `run_compiler_test_support` doesn't
    /// support specifying a custom panic mechanism.
    #[test]
    fn test_rustc_with_panic_unwind() -> Result<()> {
        let _ = TestArgs::default_args()?
            .with_panic_mechanism("unwind")
            .run()
            .expect("panic=unwind should not cause an error");
        Ok(())
    }

    #[test]
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
    #[test]
    fn test_invalid_h_out_path() -> Result<()> {
        let err = TestArgs::default_args()?
            .with_h_path("../..")
            .run()
            .expect_err("Unwriteable --h-out should trigger an error");

        let msg = format!("{err:#}");
        assert_eq!("Error when writing to ../..: Is a directory (os error 21)", msg);
        Ok(())
    }
}
