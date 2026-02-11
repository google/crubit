// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! The `run_compiler` crate mostly wraps and simplifies a subset of APIs
//! from the `rustc_driver` crate, providing an easy way to `run_compiler`.
#![feature(rustc_private, cfg_accessible)]
#![deny(rustc::internal)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_lint_defs;
extern crate rustc_log;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use arc_anyhow::{anyhow, bail, Result};
use either::Either;
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt; // See also <internal link>/ty.html#import-conventions
use rustc_session::config::ErrorOutputType;
use rustc_session::EarlyDiagCtxt;

/// Wrapper around `rustc_driver::RunCompiler::run` that exposes a
/// simplified API:
/// - Takes a `callback` that will be invoked from within Rust compiler, after
///   parsing and analysis are done,
/// - Compilation will stop after parsing, analysis, and the `callback` are
///   done,
/// - Returns the combined results from the Rust compiler *and* the `callback`.
/// - Is safe to run from unit tests (which may run in parallel / on multiple
///   threads).
pub fn run_compiler<F, R>(rustc_args: &[String], callback: F) -> Result<R>
where
    F: FnOnce(TyCtxt) -> Result<R> + Send,
    R: Send,
{
    // Calling `init_logger` 1) here and 2) via `sync::LazyLock` helps to ensure
    // that logging is intialized exactly once, even if the `run_compiler`
    // function is invoked by mutliple unit tests running in parallel on
    // separate threads.  This is important for avoiding flaky/racy
    // panics related to 1) multiple threads entering
    // `!tracing::dispatcher::has_been_set()` code in `rustc_driver_impl/src/
    // lib.rs` and 2) `rustc_log/src/lib.rs` assumming that
    // `tracing::subscriber::set_global_default` always succeeds.
    use std::sync::LazyLock;
    static ENV_LOGGER_INIT: LazyLock<()> = LazyLock::new(|| {
        let early_error_handler = EarlyDiagCtxt::new(ErrorOutputType::default());
        rustc_driver::init_logger(
            &early_error_handler,
            rustc_log::LoggerConfig::from_env("CRUBIT_LOG"),
        );
    });
    LazyLock::force(&ENV_LOGGER_INIT);

    AfterAnalysisCallback::new(rustc_args, callback).run()
}

struct AfterAnalysisCallback<'a, F, R>
where
    F: FnOnce(TyCtxt) -> Result<R> + Send,
    R: Send,
{
    args: &'a [String],
    callback_or_result: Either<F, Result<R>>,
}

impl<'a, F, R> AfterAnalysisCallback<'a, F, R>
where
    F: FnOnce(TyCtxt) -> Result<R> + Send,
    R: Send,
{
    fn new(args: &'a [String], callback: F) -> Self {
        Self { args, callback_or_result: Either::Left(callback) }
    }

    #[cfg_accessible(rustc_driver::run_compiler)]
    fn run_internal(&mut self) -> () {
        rustc_driver::run_compiler(self.args, self)
    }

    #[cfg_accessible(rustc_driver::RunCompiler)]
    fn run_internal(&mut self) -> () {
        rustc_driver::RunCompiler::new(self.args, self).run()
    }

    /// Runs Rust compiler, and then invokes the stored callback (with
    /// `TyCtxt` of the parsed+analyzed Rust crate as the callback's
    /// argument), and then finally returns the combined results
    /// from Rust compiler *and* the callback.
    fn run(mut self) -> Result<R> {
        // Rust compiler unwinds with a special sentinel value to abort compilation on
        // fatal errors. We use `catch_fatal_errors` to 1) catch such panics and
        // translate them into a Result, and 2) resume and propagate other panics.
        let catch_fatal_errors_result: std::result::Result<
            (),
            rustc_span::fatal_error::FatalError,
        > = rustc_driver::catch_fatal_errors(|| {
            self.run_internal();
        });

        match catch_fatal_errors_result {
            Ok(()) => {}
            // We can ignore the `Err` payloads because the error types have only one value.
            _ => bail!("Errors reported by Rust compiler."),
        };

        self.callback_or_result.right_or_else(|_left| {
            // When rustc cmdline arguments (i.e. `self.args`) are empty (or contain
            // `--help`) then the `after_analysis` callback won't be invoked.  Handle
            // this case by emitting an explicit error at the Crubit level.
            bail!("The Rust compiler had no crate to compile and analyze")
        })
    }
}

impl<'a, F, R> rustc_driver::Callbacks for AfterAnalysisCallback<'a, F, R>
where
    F: FnOnce(TyCtxt) -> Result<R> + Send,
    R: Send,
{
    /// Configures how `rustc` internals work when invoked via `run_compiler`.
    /// Note that `run_compiler_test_support` uses a separate `Config`.
    #[allow(rustc::internal)]
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        // Silence warnings in the target crate to avoid reporting them twice: once when
        // compiling the crate via `rustc` and once when "compiling" the crate
        // via `cc_bindings_from_rs` (the `config` here affects the latter one).
        config.opts.lint_opts.push(("warnings".to_string(), rustc_lint_defs::Level::Allow));
        // Needed for when using a target.json; avoids:
        // error loading target specification: custom targets are unstable and require `-Zunstable-options`
        // TODO: use `Session::unstable_options` instead of
        // `unstable_opts.unstable_options` and remove the function #[allow(rustc::internal)].
        config.opts.unstable_opts.unstable_options = true;
    }

    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        let callback = {
            let temporary_placeholder = Either::Right(Err(anyhow!("unused")));
            std::mem::replace(&mut self.callback_or_result, temporary_placeholder)
                .left_or_else(|_| panic!("`after_analysis` should only run once"))
        };
        self.callback_or_result = Either::Right(callback(tcx));

        rustc_driver::Compilation::Stop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use run_compiler_test_support::setup_rustc_target_for_testing;
    use run_compiler_test_support::sysroot_path;
    use tempfile::tempdir;

    const DEFAULT_RUST_SOURCE_FOR_TESTING: &'static str = r#" pub mod public_module {
                    pub fn public_function() {
                        private_function()
                    }

                    fn private_function() {}
                }
            "#;

    #[test]
    fn test_run_compiler_rustc_error_propagation() -> Result<()> {
        let rustc_args = vec![
            "run_compiler_unittest_executable".to_string(),
            "--unrecognized-rustc-flag".to_string(),
        ];
        let err = run_compiler(&rustc_args, |_tcx| Ok(()))
            .expect_err("--unrecognized-rustc-flag should trigger an error");

        let msg = format!("{err:#}");
        assert_eq!("Errors reported by Rust compiler.", msg);
        Ok(())
    }

    /// `test_run_compiler_empty_args` tests that we gracefully handle scenarios
    /// where `rustc` doesn't compile anything (e.g. when there are no
    /// cmdline args).
    #[test]
    fn test_run_compiler_no_args_except_argv0() -> Result<()> {
        let rustc_args = vec!["run_compiler_unittest_executable".to_string()];
        let err = run_compiler(&rustc_args, |_tcx| Ok(()))
            .expect_err("Empty `rustc_args` should trigger an error");

        let msg = format!("{err:#}");
        assert_eq!("The Rust compiler had no crate to compile and analyze", msg);
        Ok(())
    }

    /// `test_run_compiler_help` tests that we gracefully handle scenarios where
    /// `rustc` doesn't compile anything (e.g. when passing `--help`).
    #[test]
    fn test_run_compiler_help() -> Result<()> {
        let rustc_args = vec!["run_compiler_unittest_executable".to_string(), "--help".to_string()];
        let err = run_compiler(&rustc_args, |_tcx| Ok(()))
            .expect_err("--help passed to rustc should trigger an error");

        let msg = format!("{err:#}");
        assert_eq!("The Rust compiler had no crate to compile and analyze", msg);
        Ok(())
    }

    /// `test_run_compiler_no_output_file` tests that we stop the compilation
    /// midway (i.e. that we return `Stop` from `after_analysis`).
    #[test]
    fn test_run_compiler_no_output_file() -> Result<()> {
        let tmpdir = tempdir()?;

        let rs_path = tmpdir.path().join("input_crate.rs");
        std::fs::write(&rs_path, DEFAULT_RUST_SOURCE_FOR_TESTING)?;

        let out_path = tmpdir.path().join("unexpected_output.o");
        let mut rustc_args = vec![
            // Default parameters.
            "run_compiler_unittest_executable".to_string(),
            "--crate-type=lib".to_string(),
            rs_path.display().to_string(),
            // Test-specific parameter: asking for after-analysis output
            "-o".to_string(),
            out_path.display().to_string(),
        ];
        if let Some(sysroot) = sysroot_path() {
            rustc_args.push(format!("--sysroot={}", sysroot.display()));
        }
        if let Some(target_arg) = setup_rustc_target_for_testing(tmpdir.path()) {
            rustc_args.push(format!("--target={}", target_arg));
        }

        run_compiler(&rustc_args, |_tcx| Ok(()))?;

        // Verify that compilation didn't continue after the initial analysis.
        assert!(!out_path.exists());
        Ok(())
    }
}
