// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]
#![deny(rustc::internal)]

extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

// TODO(lukasza): Make `cmdline` and `lib` a separate crate (once we move to
// Bazel).
mod cmdline;
mod lib;

use cmdline::Cmdline;
use itertools::Itertools;

mod bindings_main {

    use anyhow::Context;
    use rustc_middle::ty::TyCtxt;
    use std::path::Path;

    use crate::cmdline::Cmdline;
    use crate::lib::GeneratedBindings;
    use token_stream_printer::tokens_to_string;

    pub fn main(cmdline: &Cmdline, tcx: TyCtxt) -> anyhow::Result<()> {
        let bindings = GeneratedBindings::generate(tcx);
        write_file(cmdline.h_out(), &tokens_to_string(bindings.h_body)?)
    }

    fn write_file(path: &Path, content: &str) -> anyhow::Result<()> {
        std::fs::write(path, content)
            .with_context(|| format!("Error when writing to {}", path.display()))
    }
}

/// Glue that enables the top-level `fn main() -> anyhow::Result<()>` to call
/// into `fn main(cmdline: &Cmdline, tcx: TyCtxt) -> anyhow::Result<()>` in the
/// `bindings_main` module.  This mostly wraps and simplifies a subset of APIs
/// from the `rustc_driver` module.
mod bindings_driver {

    use rustc_interface::interface::Compiler;
    use rustc_interface::Queries;

    use crate::cmdline::Cmdline;
    use crate::lib::enter_tcx;

    /// Wrapper around `rustc_driver::RunCompiler` that exposes a simplified API
    /// (e.g. doesn't take arbitrary `Callbacks` but always calls into
    /// `bindings_main::main`).
    pub struct RunCompiler<'a>(BindingsCallbacks<'a>);

    impl<'a> RunCompiler<'a> {
        /// Creates new Rust compiler runner that will
        /// - pass `cmdline.rustc_args()` to the Rust compiler
        /// - pass `cmdline` to `bindings_main::main` (in addition to passing
        ///   `TyCtxt` - see the doc comment of `RunCompiler::run` below).
        pub fn new(cmdline: &'a Cmdline) -> Self {
            Self(BindingsCallbacks { cmdline, bindings_main_result: None })
        }

        /// Runs Rust compiler and then passes the `TyCtxt` of the
        /// parsed+analyzed Rust crate into `bindings_main::main`.
        /// Returns the combined results from Rust compiler *and*
        /// `bindings_main::main`.
        pub fn run(mut self) -> anyhow::Result<()> {
            // Rust compiler unwinds with a special sentinel value to abort compilation on
            // fatal errors. We use `catch_fatal_errors` to 1) catch such panics and
            // translate them into a Result, and 2) resume and propagate other panics.
            let rustc_result = rustc_driver::catch_fatal_errors(|| {
                rustc_driver::RunCompiler::new(self.0.cmdline.rustc_args(), &mut self.0).run()
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

            // Return either `rustc_result` or `self.0.bindings_main_result`.
            rustc_result.and_then(|()| {
                assert!(
                    self.0.bindings_main_result.is_some(),
                    "BindingsCallbacks::run_main should have been called by now"
                );
                self.0.bindings_main_result.unwrap()
            })
        }
    }

    /// Non-`pub` to avoid exposing `impl rustc_driver::Callbacks`.
    struct BindingsCallbacks<'a> {
        cmdline: &'a Cmdline,
        bindings_main_result: Option<anyhow::Result<()>>,
    }

    impl rustc_driver::Callbacks for BindingsCallbacks<'_> {
        fn after_analysis<'tcx>(
            &mut self,
            _compiler: &Compiler,
            queries: &'tcx Queries<'tcx>,
        ) -> rustc_driver::Compilation {
            let rustc_result = enter_tcx(queries, |tcx| {
                assert!(
                    self.bindings_main_result.is_none(),
                    "after_analysis should only run once"
                );
                self.bindings_main_result =
                    Some(crate::bindings_main::main(self.cmdline, tcx))
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

/// Main entrypoint that (unlike `main`) doesn't do any intitializations that
/// should only happen once for the binary (e.g. it doesn't call
/// `install_ice_hook`) and therefore can be used from the tests module below.
fn run_with_cmdline_args(args: &[String]) -> anyhow::Result<()> {
    let cmdline = Cmdline::new(args)?;
    bindings_driver::RunCompiler::new(&cmdline).run()
}

// TODO(lukasza): Add end-to-end shell tests that invoke our executable
// and verify 1) the happy path (zero exit code) and 2) any random
// error path (non-zero exit code).
fn main() -> anyhow::Result<()> {
    rustc_driver::init_env_logger("CRUBIT_LOG");

    // TODO: Investigate if we should install a signal handler here.  See also how
    // compiler/rustc_driver/src/lib.rs calls `signal_handler::install()`.

    rustc_driver::install_ice_hook();

    // `std::env::args()` will panic if any of the cmdline arguments are not valid
    // Unicode.  This seems okay.
    let args = std::env::args().collect_vec();

    run_with_cmdline_args(&args)
}

#[cfg(test)]
mod tests {
    use super::run_with_cmdline_args;

    use crate::lib::tests::get_sysroot_for_testing;
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
        fn run(&self) -> anyhow::Result<PathBuf> {
            let h_path = match self.h_path.as_ref() {
                None => self.tempdir.path().join("test_crate.rs"),
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
                format!("--h_out={}", h_path.display()),
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

            Ok(h_path)
        }
    }

    #[test]
    fn test_happy_path() -> anyhow::Result<()> {
        let test_args = TestArgs::default_args()?;
        let h_path = test_args.run().expect("Default args should succeed");

        assert!(h_path.exists());
        let h_body = std::fs::read_to_string(&h_path)?;
        assert_eq!(
            h_body,
            "// Automatically @generated C++ bindings for the following Rust crate:\n\
             // test_crate\n\
             \n\
             // List of public functions:\n\
             // public_function\n"
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
        assert_eq!("Unrecognized option: 'unrecognized-crubit-flag'", msg);
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
        // Tests not only the specific problem of an invalid `--h_out` argument, but
        // also tests that errors from `bindings_main::main` are propagated.
        let err = TestArgs::default_args()?
            .with_h_path("../..")
            .run()
            .expect_err("Unwriteable --h_out should trigger an error");

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
