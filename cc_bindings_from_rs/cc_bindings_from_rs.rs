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
extern crate rustc_lint_defs;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

// TODO(b/254679226): `bindings` and `cmdline` should be separate crates.
mod bindings;
mod cmdline;

use anyhow::Context;
use itertools::Itertools;
use rustc_middle::ty::TyCtxt;
use std::path::Path;

use bindings::GeneratedBindings;
use cmdline::Cmdline;
use token_stream_printer::{
    cc_tokens_to_formatted_string, rs_tokens_to_formatted_string, RustfmtConfig,
};

/// This mostly wraps and simplifies a subset of APIs from the `rustc_driver`
/// module.
mod bindings_driver {

    use anyhow::anyhow;
    use either::Either;
    use rustc_interface::interface::Compiler;
    use rustc_interface::Queries;
    use rustc_middle::ty::TyCtxt;

    use crate::bindings::enter_tcx;

    /// Wrapper around `rustc_driver::RunCompiler::run` that exposes a
    /// simplified API:
    /// - Takes a `callback` that will be invoked from within Rust compiler,
    ///   after parsing and analysis are done,
    /// - Compilation will stop after parsing, analysis, and the `callback` are
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

        /// Runs Rust compiler, and then invokes the stored callback (with
        /// `TyCtxt` of the parsed+analyzed Rust crate as the callback's
        /// argument), and then finally returns the combined results
        /// from Rust compiler *and* the callback.
        fn run(mut self) -> anyhow::Result<R> {
            // Rust compiler unwinds with a special sentinel value to abort compilation on
            // fatal errors. We use `catch_fatal_errors` to 1) catch such panics and
            // translate them into a Result, and 2) resume and propagate other panics.
            use rustc_interface::interface::Result;
            let rustc_result: Result<Result<()>> = rustc_driver::catch_fatal_errors(|| {
                rustc_driver::RunCompiler::new(self.args, &mut self).run()
            });

            // Flatten `Result<Result<T, ...>>` into `Result<T, ...>` (i.e. combine the
            // result from `RunCompiler::run` and `catch_fatal_errors`).
            //
            // TODO(lukasza): Use `Result::flatten` API when it gets stabilized.  See also
            // https://github.com/rust-lang/rust/issues/70142
            let rustc_result: Result<()> = rustc_result.and_then(|result| result);

            // Translate `rustc_interface::interface::Result` into `anyhow::Result`.  (Can't
            // use `?` because the trait `std::error::Error` is not implemented for
            // `ErrorGuaranteed` which is required by the impl of
            // `From<ErrorGuaranteed>` for `anyhow::Error`.)
            let rustc_result: anyhow::Result<()> = rustc_result.map_err(|_err| {
                // We can ignore `_err` because it has no payload / because this type has only
                // one valid/possible value.
                anyhow::format_err!("Errors reported by Rust compiler.")
            });

            // Return either `rustc_result` or `self.callback_result` or a new error.
            rustc_result.and_then(|()| {
                self.callback_or_result.right_or_else(|_left| {
                    // When rustc cmdline arguments (i.e. `self.args`) are empty (or contain
                    // `--help`) then the `after_analysis` callback won't be invoked.  Handle
                    // this case by emitting an explicit error at the Crubit level.
                    Err(anyhow!("The Rust compiler had no crate to compile and analyze"))
                })
            })
        }
    }

    impl<'a, F, R> rustc_driver::Callbacks for AfterAnalysisCallback<'a, F, R>
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
    let bindings = GeneratedBindings::generate(tcx)?;

    {
        let h_body = cc_tokens_to_formatted_string(bindings.h_body)?;
        write_file(&cmdline.h_out, &h_body)?;
    }

    {
        let rustfmt_config =
            RustfmtConfig::new(&cmdline.rustfmt_exe_path, cmdline.rustfmt_config_path.as_deref());
        let rs_body = rs_tokens_to_formatted_string(bindings.rs_body, &rustfmt_config)?;
        write_file(&cmdline.rs_out, &rs_body)?;
    }

    Ok(())
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

fn main() -> anyhow::Result<()> {
    rustc_driver::init_env_logger("CRUBIT_LOG");

    // TODO: Investigate if we should install a signal handler here.  See also how
    // compiler/rustc_driver/src/lib.rs calls `signal_handler::install()`.

    // TODO(b/254689400): Provide Crubit-specific panic hook message (we shouldn't use
    // `rustc_driver::install_ice_hook` because it's message asks to file bugs at
    // https://github.com/rust-lang/rust/issues/new.

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
    use token_stream_printer::RUSTFMT_EXE_PATH_FOR_TESTING;

    /// Test data builder (see also
    /// https://testing.googleblog.com/2018/02/testing-on-toilet-cleanly-create-test.html).
    struct TestArgs {
        h_path: Option<String>,
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
    }

    impl TestArgs {
        fn default_args() -> anyhow::Result<Self> {
            Ok(Self {
                h_path: None,
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

        /// Replaces the default `--codegen=panic=abort` with the specified
        /// `panic_mechanism`.
        fn with_panic_mechanism(mut self, panic_mechanism: &str) -> Self {
            self.panic_mechanism = panic_mechanism.to_string();
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
            let rs_path = self.tempdir.path().join("test_crate_cc_api_impl.rs");

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
                format!("--rs-out={}", rs_path.display()),
                format!("--rustfmt-exe-path={RUSTFMT_EXE_PATH_FOR_TESTING}"),
            ];
            args.extend(self.extra_crubit_args.iter().cloned());
            args.extend([
                "--".to_string(),
                format!("--codegen=panic={}", &self.panic_mechanism),
                // See comments about `SymbolManglingVersion::V0`in `test::run_compiler` in
                // `bindings.rs` for rationale behind using `symbol-mangling-version=v0`.
                "--codegen=symbol-mangling-version=v0".to_string(),
                "--crate-type=lib".to_string(),
                format!("--sysroot={}", get_sysroot_for_testing().display()),
                rs_input_path.display().to_string(),
            ]);
            args.extend(self.extra_rustc_args.iter().cloned());

            run_with_cmdline_args(&args)?;

            Ok(TestResult { h_path, rs_path })
        }
    }

    #[test]
    fn test_happy_path() -> anyhow::Result<()> {
        let test_args = TestArgs::default_args()?;
        let test_result = test_args.run().expect("Default args should succeed");

        assert!(test_result.h_path.exists());
        let h_body = std::fs::read_to_string(&test_result.h_path)?;
        assert_eq!(
            h_body,
r#"// Automatically @generated C++ bindings for the following Rust crate:
// test_crate

#pragma once

namespace test_crate {
namespace __crubit_internal {
extern "C" void
__crubit_thunk__RNvCslKXfKXPWofF_10test_crate15public_function();
}
inline void public_function() {
  return __crubit_internal::
      __crubit_thunk__RNvCslKXfKXPWofF_10test_crate15public_function();
}
}  // namespace test_crate"#
        );

        assert!(test_result.rs_path.exists());
        let rs_body = std::fs::read_to_string(&test_result.rs_path)?;
        assert_eq!(
            rs_body,
            r#"// Automatically @generated C++ bindings for the following Rust crate:
// test_crate

#![allow(improper_ctypes_definitions)]

#[no_mangle]
extern "C" fn __crubit_thunk__RNvCslKXfKXPWofF_10test_crate15public_function() -> () {
    test_crate::public_function()
}
"#
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

        let msg = format!("{err:#}");
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

        let msg = format!("{err:#}");
        assert_eq!("Errors reported by Rust compiler.", msg);
        Ok(())
    }

    #[test]
    fn test_rustc_help() -> anyhow::Result<()> {
        // Tests that we gracefully handle scenarios where `rustc` doesn't compile
        // anything (e.g. when there are no rustc cmdline arguments, or when
        // `--help` is present).
        let err = TestArgs::default_args()?
            .with_extra_rustc_args(["--help"])
            .run()
            .expect_err("--help passed to rustc should trigger Crubit-level error");

        let msg = format!("{err:#}");
        assert_eq!("The Rust compiler had no crate to compile and analyze", msg);
        Ok(())
    }

    #[test]
    fn test_rustc_unsupported_panic_mechanism() -> anyhow::Result<()> {
        // Tests that `panic=unwind` results in an error.
        let err = TestArgs::default_args()?
            .with_panic_mechanism("unwind")
            .run()
            .expect_err("panic=unwind should trigger an error");

        let msg = format!("{err:#}");
        assert_eq!("No support for panic=unwind strategy (b/254049425)", msg);
        Ok(())
    }

    #[test]
    fn test_invalid_h_out_path() -> anyhow::Result<()> {
        // Tests not only the specific problem of an invalid `--h-out` argument, but
        // also tests that errors from `run_with_tcx` are propagated.
        let err = TestArgs::default_args()?
            .with_h_path("../..")
            .run()
            .expect_err("Unwriteable --h-out should trigger an error");

        let msg = format!("{err:#}");
        assert_eq!("Error when writing to ../..: Is a directory (os error 21)", msg);
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
