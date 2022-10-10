// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(never_type)]
#![feature(rustc_private)]
#![deny(rustc::internal)]

extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_feature;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

// TODO(lukasza): Make `bindings` and `cmdline` separate crates (once we move to
// Bazel).  This hasn't been done that yet, because:
// * Today it would require replicating `rustc_driver`-related `BUILD` hacks
//   into additional targets.  And since this particular problem will go away on
//   its own in Q4 2022 or Q1 2023, maybe for now we can ignore having
//   multi-source-file crate?
// * To avoid the ickyness above, one may want to start by making `cmdline.rs`
//   `rustc_driver`-agnostic first (switching `@herefile` to a `clap`-based,
//   manually coded behavior).  But this refactoring feels a bit arbitrary and
//   needs more discussion (maybe we want to keep `@file` support from rustc?).
mod bindings;
mod cmdline;

use anyhow::Context;
use itertools::Itertools;
use rustc_middle::ty::TyCtxt;
use std::path::Path;

use bindings::GeneratedBindings;
use cmdline::Cmdline;
use token_stream_printer::cc_tokens_to_formatted_string;

/// This mostly wraps and simplifies a subset of APIs from the `rustc_driver`
/// module.
mod bindings_driver {

    use either::Either;
    use rustc_interface::interface::Compiler;
    use rustc_interface::Queries;
    use rustc_middle::ty::TyCtxt;

    use crate::bindings::enter_tcx;

    /// Wrapper around `rustc_driver::RunCompiler::run` that exposes a
    /// simplified API:
    /// - Takes a `callback` that will be invoked from within Rust compiler,
    ///   after parsing and analysis are done,
    /// - Compilation will stop after parsing, analysis, and the `callback are
    ///   done,
    /// - Returns the combined results from the Rust compiler *and* the
    ///   `callback`.
    pub fn run_after_analysis_and_stop<F, R>(
        rustc_args: &[String],
        callback: F,
    ) -> anyhow::Result<R>
    where
        F: FnOnce(TyCtxt) -> anyhow::Result<R> + Send,
        R: Send,
    {
        AfterAnalysisCallback::new(rustc_args, callback).run()
    }

    struct AfterAnalysisCallback<'a, F, R>
    where
        F: FnOnce(TyCtxt) -> anyhow::Result<R> + Send,
        R: Send,
    {
        args: &'a [String],
        callback_or_result: Either<F, anyhow::Result<R>>,
    }

    impl<'a, F, R> AfterAnalysisCallback<'a, F, R>
    where
        F: FnOnce(TyCtxt) -> anyhow::Result<R> + Send,
        R: Send,
    {
        fn new(args: &'a [String], callback: F) -> Self {
            Self { args, callback_or_result: Either::Left(callback) }
        }

        /// Runs Rust compiler and then passes the `TyCtxt` of the
        /// parsed+analyzed Rust crate into `bindings_main::main`.
        /// Returns the combined results from Rust compiler *and*
        /// `bindings_main::main`.
        fn run(mut self) -> anyhow::Result<R> {
            // Rust compiler unwinds with a special sentinel value to abort compilation on
            // fatal errors. We use `catch_fatal_errors` to 1) catch such panics and
            // translate them into a Result, and 2) resume and propagate other panics.
            let rustc_result = rustc_driver::catch_fatal_errors(|| {
                rustc_driver::RunCompiler::new(self.args, &mut self).run()
            });

            // Flatten `Result<Result<T, ...>>` into `Result<T, ...>` (i.e. get the Result
            // from `RunCompiler::run` rather than the Result from
            // `catch_fatal_errors`).
            let rustc_result = rustc_result.and_then(|result| result);

            // Translate `rustc_interface::interface::Result` into `anyhow::Result`.  (Can't
            // use `?` because the trait `std::error::Error` is not implemented for
            // `ErrorGuaranteed` which is required by the impl of
            // `From<ErrorGuaranteed>` for `anyhow::Error`.)
            let rustc_result = rustc_result.map_err(|_err| {
                // We can ignore `_err` because it has no payload / because this type has only
                // one valid/possible value.
                anyhow::format_err!("Errors reported by Rust compiler.")
            });

            // Return either `rustc_result` or `self.callback_result`.
            rustc_result.and_then(|()| {
                self.callback_or_result.right_or_else(|_| panic!("The callback should have been called by now"))
            })
        }
    }

    impl<'a, F, R> rustc_driver::Callbacks for AfterAnalysisCallback<'_, F, R>
    where
        F: FnOnce(TyCtxt) -> anyhow::Result<R> + Send,
        R: Send,
    {
        fn after_analysis<'tcx>(
            &mut self,
            _compiler: &Compiler,
            queries: &'tcx Queries<'tcx>,
        ) -> rustc_driver::Compilation {
            let rustc_result = enter_tcx(queries, |tcx| {
                let callback = {
                    let temporary_placeholder = Either::Right(Err(anyhow::anyhow!("unused")));
                    std::mem::replace(&mut self.callback_or_result, temporary_placeholder)
                        .left_or_else(|_| panic!("`after_analysis` should only run once"))
                };
                self.callback_or_result = Either::Right(callback(tcx));
            });

            // `expect`ing no errors in `rustc_result`, because `after_analysis` is only
            // called by `rustc_driver` if earlier compiler analysis was successful
            // (which as the *last* compilation phase presumably covers *all*
            // errors).
            rustc_result.expect("Expecting no compile errors inside `after_analysis` callback.");

            rustc_driver::Compilation::Stop
        }
    }
}

fn write_file(path: &Path, content: &str) -> anyhow::Result<()> {
    std::fs::write(path, content)
        .with_context(|| format!("Error when writing to {}", path.display()))
}

fn run_with_tcx(cmdline: &Cmdline, tcx: TyCtxt) -> anyhow::Result<()> {
    let bindings = GeneratedBindings::generate(tcx);
    write_file(&cmdline.h_out, cc_tokens_to_formatted_string(bindings.h_body)?.as_str())
}

/// Main entrypoint that (unlike `main`) doesn't do any intitializations that
/// should only happen once for the binary (e.g. it doesn't call
/// `install_ice_hook`) and therefore can be used from the tests module below.
fn run_with_cmdline_args(args: &[String]) -> anyhow::Result<()> {
    let cmdline = Cmdline::new(args)?;
    bindings_driver::run_after_analysis_and_stop(&cmdline.rustc_args, |tcx| {
        run_with_tcx(&cmdline, tcx)
    })
}

// TODO(lukasza): Add end-to-end shell tests that invoke our executable
// and verify:
//
// 1) the happy path (verify zero exit code + contents of the generated .h)
//
// 2) `clap` error path (verify non-zero exit code + error output
//    [not sure if *colored* output can be verified])
//
// 3) `clap` --help path (verify *zero* exit code;  the error message is
//    already verified in unit tests under `cmdline.rs`)
//
// 4) other error path (verify non-zero exit code + error output)
fn main() -> anyhow::Result<()> {
    rustc_driver::init_env_logger("CRUBIT_LOG");

    // TODO: Investigate if we should install a signal handler here.  See also how
    // compiler/rustc_driver/src/lib.rs calls `signal_handler::install()`.

    rustc_driver::install_ice_hook();

    // `std::env::args()` will panic if any of the cmdline arguments are not valid
    // Unicode.  This seems okay.
    let args = std::env::args().collect_vec();

    run_with_cmdline_args(&args)
        .map_err(|anyhow_err| match anyhow_err.downcast::<clap::Error>() {
            // Explicitly call `clap::Error::exit`, because 1) it results in *colored* output and
            // 2) it uses a zero exit code for specific "errors" (e.g. for `--help` output).
            Ok(clap_err) => {
                let _ : ! = clap_err.exit();
            },

            // Return `other_err` from `main`.  This will print the error message (no color codes
            // though) and terminate the process with a non-zero exit code.
            Err(other_err) => other_err,
        })
}

#[cfg(test)]
mod tests {
    use super::run_with_cmdline_args;

    use crate::bindings::tests::get_sysroot_for_testing;
    use itertools::Itertools;
    use std::path::PathBuf;
    use tempfile::{tempdir, TempDir};

    /// Test data builder (see also
    /// https://testing.googleblog.com/2018/02/testing-on-toilet-cleanly-create-test.html).
    struct TestArgs {
        h_path: Option<String>,
        extra_crubit_args: Vec<String>,
        extra_rustc_args: Vec<String>,
        tempdir: TempDir,
    }

    /// Result of `TestArgs::run` that helps tests access test outputs (e.g. the
    /// internally generated `h_path` and/or `rs_input_path`).
    #[derive(Debug)]
    struct TestResult {
        h_path: PathBuf,
        rs_input_path: PathBuf,
    }

    impl TestArgs {
        fn default_args() -> anyhow::Result<Self> {
            Ok(Self {
                h_path: None,
                extra_crubit_args: vec![],
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

        /// Appends `extra_rustc_args` at the end of the cmdline (i.e. as
        /// additional rustc args, in addition to `--sysroot`,
        /// `--crate-type=...`, etc.).
        fn with_extra_rustc_args<T>(mut self, extra_rustc_args: T) -> Self
        where
            T: IntoIterator,
            T::Item: Into<String>,
        {
            self.extra_rustc_args = extra_rustc_args.into_iter().map(|t| t.into()).collect_vec();
            self
        }

        /// Appends `extra_crubit_args` before the first `--`.
        fn with_extra_crubit_args<T>(mut self, extra_crubit_args: T) -> Self
        where
            T: IntoIterator,
            T::Item: Into<String>,
        {
            self.extra_crubit_args = extra_crubit_args.into_iter().map(|t| t.into()).collect_vec();
            self
        }

        /// Invokes `super::run_with_cmdline_args` with default `test_crate.rs`
        /// input (and with other default args + args gathered by
        /// `self`).
        ///
        /// Returns the path to the `h_out` file.  The file's lifetime is the
        /// same as `&self`.
        fn run(&self) -> anyhow::Result<TestResult> {
            let h_path = match self.h_path.as_ref() {
                None => self.tempdir.path().join("test_crate_cc_api.h"),
                Some(s) => PathBuf::from(s),
            };

            let rs_input_path = self.tempdir.path().join("test_crate.rs");
            std::fs::write(
                &rs_input_path,
                r#" pub fn public_function() {
                        private_function()
                    }

                    fn private_function() {}
                "#,
            )?;

            let mut args = vec![
                "cc_bindings_from_rs_unittest_executable".to_string(),
                format!("--h-out={}", h_path.display()),
            ];
            args.extend(self.extra_crubit_args.iter().cloned());
            args.extend([
                "--".to_string(),
                "--crate-type=lib".to_string(),
                format!("--sysroot={}", get_sysroot_for_testing().display()),
                rs_input_path.display().to_string(),
            ]);
            args.extend(self.extra_rustc_args.iter().cloned());

            run_with_cmdline_args(&args)?;

            Ok(TestResult { h_path, rs_input_path })
        }
    }

    #[test]
    fn test_happy_path() -> anyhow::Result<()> {
        let test_args = TestArgs::default_args()?;
        let test_result = test_args.run().expect("Default args should succeed");

        assert!(test_result.h_path.exists());
        let h_body = std::fs::read_to_string(&test_result.h_path)?;
        let rs_input_path = test_result.rs_input_path.display().to_string();
        assert_eq!(
            h_body,
            format!(
r#"// Automatically @generated C++ bindings for the following Rust crate:
// test_crate

#pragma once

namespace test_crate {{

// Error while generating bindings for `public_function` defined at
// {rs_input_path}:1:2: 1:26: Nothing works yet!

}}"#
            )
        );
        Ok(())
    }

    #[test]
    fn test_cmdline_error_propagation() -> anyhow::Result<()> {
        // Tests that errors from `Cmdline::new` get propagated.  Broader coverage of
        // various error types can be found in tests in `cmdline.rs`.
        let err = TestArgs::default_args()?
            .with_extra_crubit_args(["--unrecognized-crubit-flag"])
            .run()
            .expect_err("--unrecognized_crubit_flag should trigger an error");

        let msg = err.to_string();
        assert!(
            msg.contains("Found argument '--unrecognized-crubit-flag' which wasn't expected"),
            "msg = {}",
            msg,
        );
        Ok(())
    }

    #[test]
    fn test_rustc_error_propagation() -> anyhow::Result<()> {
        // Tests that `rustc` errors are propagated.
        let err = TestArgs::default_args()?
            .with_extra_rustc_args(["--unrecognized-rustc-flag"])
            .run()
            .expect_err("--unrecognized-rustc-flag should trigger an error");

        let msg = err.to_string();
        assert_eq!("Errors reported by Rust compiler.", msg);
        Ok(())
    }

    #[test]
    fn test_invalid_h_out_path() -> anyhow::Result<()> {
        // Tests not only the specific problem of an invalid `--h-out` argument, but
        // also tests that errors from `bindings_main::main` are propagated.
        let err = TestArgs::default_args()?
            .with_h_path("../..")
            .run()
            .expect_err("Unwriteable --h-out should trigger an error");

        let msg = err.to_string();
        assert_eq!("Error when writing to ../..", msg);
        Ok(())
    }

    #[test]
    fn test_no_output_file() -> anyhow::Result<()> {
        // Tests that we stop the compilation midway.
        let tmpdir = tempdir()?;
        let out_path = tmpdir.path().join("unexpected_output.o");
        TestArgs::default_args()?
            .with_extra_rustc_args(vec!["-o", &out_path.display().to_string()])
            .run()
            .expect("No rustc or Crubit errors are expected in this test");

        assert!(!out_path.exists());
        Ok(())
    }
}
