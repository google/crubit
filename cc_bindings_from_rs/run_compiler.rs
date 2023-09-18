// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! The `run_compiler` crate mostly wraps and simplifies a subset of APIs
//! from the `rustc_driver` module, providing an easy way to `run_compiler`.

use anyhow::anyhow;
use either::Either;
use rustc_interface::interface::Compiler;
use rustc_interface::Queries;
use rustc_middle::ty::TyCtxt; // See also <internal link>/ty.html#import-conventions
use rustc_session::config::ErrorOutputType;
use rustc_session::EarlyErrorHandler;

/// Wrapper around `rustc_driver::RunCompiler::run` that exposes a
/// simplified API:
/// - Takes a `callback` that will be invoked from within Rust compiler, after
///   parsing and analysis are done,
/// - Compilation will stop after parsing, analysis, and the `callback` are
///   done,
/// - Returns the combined results from the Rust compiler *and* the `callback`.
/// - Is safe to run from unit tests (which may run in parallel / on multiple
///   threads).
pub fn run_compiler<F, R>(rustc_args: &[String], callback: F) -> anyhow::Result<R>
where
    F: FnOnce(TyCtxt) -> anyhow::Result<R> + Send,
    R: Send,
{
    // Calling `init_env_logger` 1) here and 2) via `sync::Lazy` helps to ensure
    // that logging is intialized exactly once, even if the `run_compiler`
    // function is invoked by mutliple unit tests running in parallel on
    // separate threads.  This is important for avoiding flaky/racy
    // panics related to 1) multiple threads entering
    // `!tracing::dispatcher::has_been_set()` code in `rustc_driver_impl/src/
    // lib.rs` and 2) `rustc_log/src/lib.rs` assumming that
    // `tracing::subscriber::set_global_default` always succeeds.
    use once_cell::sync::Lazy;
    static ENV_LOGGER_INIT: Lazy<()> = Lazy::new(|| {
        let early_error_handler = EarlyErrorHandler::new(ErrorOutputType::default());
        rustc_driver::init_env_logger(&early_error_handler, "CRUBIT_LOG");
    });
    Lazy::force(&ENV_LOGGER_INIT);

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
            anyhow!("Errors reported by Rust compiler.")
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
    /// Configures how `rustc` internals work when invoked via `run_compiler`.
    /// Note that `run_compiler_for_testing` uses a separate `Config`.
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        // Silence warnings in the target crate to avoid reporting them twice: once when
        // compiling the crate via `rustc` and once when "compiling" the crate
        // via `cc_bindings_from_rs` (the `config` here affects the latter one).
        config.opts.lint_opts.push(("warnings".to_string(), rustc_lint_defs::Level::Allow));
    }

    fn after_analysis<'tcx>(
        &mut self,
        // TODO(b/300606577): Remove after stable picks this up.
        #[cfg(not(
            google3_internal_rustc_contains_commit_2eca717a240a37e4e996d727b6506d2f2e990b74
        ))]
        _handler: &EarlyErrorHandler,
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

/// Helper (used by `run_compiler` and `run_compiler_for_testing`) for invoking
/// functions operating on `TyCtxt`.
fn enter_tcx<'tcx, F, T>(
    queries: &'tcx Queries<'tcx>,
    f: F,
) -> rustc_interface::interface::Result<T>
where
    F: FnOnce(TyCtxt<'tcx>) -> T + Send,
    T: Send,
{
    let mut query_context = queries.global_ctxt()?;
    Ok(query_context.enter(f))
}

#[cfg(test)]
pub mod tests {
    use super::run_compiler;

    use rustc_middle::ty::TyCtxt; // See also <internal link>/ty.html#import-conventions
    use std::path::PathBuf;
    use tempfile::tempdir;

    const DEFAULT_RUST_SOURCE_FOR_TESTING: &'static str = r#" pub mod public_module {
                    pub fn public_function() {
                        private_function()
                    }

                    fn private_function() {}
                }
            "#;

    #[test]
    fn test_run_compiler_rustc_error_propagation() -> anyhow::Result<()> {
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
    fn test_run_compiler_no_args_except_argv0() -> anyhow::Result<()> {
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
    fn test_run_compiler_help() -> anyhow::Result<()> {
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
    fn test_run_compiler_no_output_file() -> anyhow::Result<()> {
        let tmpdir = tempdir()?;

        let rs_path = tmpdir.path().join("input_crate.rs");
        std::fs::write(&rs_path, DEFAULT_RUST_SOURCE_FOR_TESTING)?;

        let out_path = tmpdir.path().join("unexpected_output.o");

        let rustc_args = vec![
            // Default parameters.
            "run_compiler_unittest_executable".to_string(),
            "--crate-type=lib".to_string(),
            format!("--sysroot={}", get_sysroot_for_testing().display()),
            rs_path.display().to_string(),
            // Test-specific parameter: asking for after-analysis output
            "-o".to_string(),
            out_path.display().to_string(),
        ];

        run_compiler(&rustc_args, |_tcx| Ok(()))?;

        // Verify that compilation didn't continue after the initial analysis.
        assert!(!out_path.exists());
        Ok(())
    }

    #[cfg(llvm_unstable)]
    const CROSSTOOL_VERSION: &str = "llvm_unstable";
    #[cfg(stable)]
    const CROSSTOOL_VERSION: &str = "stable";
    #[cfg(oss)]
    const CROSSTOOL_VERSION: &str = "oss";

    /// Returns the `rustc` sysroot that is suitable for the environment where
    /// unit tests run.
    ///
    /// The sysroot is used internally by `run_compiler_for_testing`, but it may
    /// also be passed as `--sysroot=...` in `rustc_args` argument of
    /// `run_compiler`
    pub fn get_sysroot_for_testing() -> PathBuf {
        let runfiles = runfiles::Runfiles::create().unwrap();
        let loc = runfiles.rlocation(if CROSSTOOL_VERSION == "oss" {
            "rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools/rust_toolchain/lib/rustlib/x86_64-unknown-linux-gnu".into()
        } else {
            format!("google3/third_party/crosstool/v18/{CROSSTOOL_VERSION}/rust/main_sysroot")
        });
        assert!(loc.exists(), "Sysroot directory '{}' doesn't exist", loc.display());
        assert!(loc.is_dir(), "Provided sysroot '{}' is not a directory", loc.display());
        loc
    }

    #[test]
    #[should_panic(expected = "Test inputs shouldn't cause compilation errors")]
    fn test_run_compiler_for_testing_panic_when_test_input_contains_syntax_errors() {
        run_compiler_for_testing("syntax error here", |_tcx| panic!("This part shouldn't execute"))
    }

    #[test]
    #[should_panic(expected = "Test inputs shouldn't cause compilation errors")]
    fn test_run_compiler_for_testing_panic_when_test_input_triggers_analysis_errors() {
        run_compiler_for_testing("#![feature(no_such_feature)]", |_tcx| {
            panic!("This part shouldn't execute")
        })
    }

    #[test]
    #[should_panic(expected = "Test inputs shouldn't cause compilation errors")]
    fn test_run_compiler_for_testing_panic_when_test_input_triggers_warnings() {
        run_compiler_for_testing("pub fn foo(unused_parameter: i32) {}", |_tcx| {
            panic!("This part shouldn't execute")
        })
    }

    #[test]
    fn test_run_compiler_for_testing_nightly_features_ok_in_test_input() {
        // This test arbitrarily picks `yeet_expr` as an example of a feature that
        // hasn't yet been stabilized.
        let test_src = r#"
                // This test is supposed to test that *nightly* features are ok
                // in the test input.  The `forbid` directive below helps to
                // ensure that we'll realize in the future when the `yeet_expr`
                // feature gets stabilized, making it not quite fitting for use
                // in this test.
                #![forbid(stable_features)]

                #![feature(yeet_expr)]
            "#;
        run_compiler_for_testing(test_src, |_tcx| ())
    }

    #[test]
    fn test_run_compiler_for_testing_stabilized_features_ok_in_test_input() {
        // This test arbitrarily picks `const_ptr_offset_from` as an example of a
        // feature that has been already stabilized.
        run_compiler_for_testing("#![feature(const_ptr_offset_from)]", |_tcx| ())
    }

    /// `run_compiler_for_testing` is similar to `run_compiler`: it invokes the
    /// `callback` after parsing and analysis are done, but instead of taking
    /// `rustc_args` it:
    ///
    /// * Invokes the Rust compiler on the given Rust `source`
    /// * Hardcodes other compiler flags (e.g. picks Rust 2021 edition, and opts
    ///   into treating all warnings as errors).
    pub fn run_compiler_for_testing<F, T>(source: impl Into<String>, callback: F) -> T
    where
        F: for<'tcx> FnOnce(TyCtxt<'tcx>) -> T + Send,
        T: Send,
    {
        use rustc_session::config::{
            CodegenOptions, CrateType, Input, Options, OutputType, OutputTypes,
        };

        const TEST_FILENAME: &str = "crubit_unittests.rs";

        // Setting `output_types` that will trigger code gen - otherwise some parts of
        // the analysis will be missing (e.g. `tcx.exported_symbols()`).
        // The choice of `Bitcode` is somewhat arbitrary (e.g. `Assembly`,
        // `Mir`, etc. would also trigger code gen).
        let output_types = OutputTypes::new(&[(OutputType::Bitcode, None /* PathBuf */)]);

        let opts = Options {
            crate_types: vec![CrateType::Rlib], // Test inputs simulate library crates.
            maybe_sysroot: Some(get_sysroot_for_testing()),
            output_types,
            edition: rustc_span::edition::Edition::Edition2021,
            unstable_features: rustc_feature::UnstableFeatures::Allow,
            lint_opts: vec![
                ("warnings".to_string(), rustc_lint_defs::Level::Deny),
                ("stable_features".to_string(), rustc_lint_defs::Level::Allow),
            ],
            cg: CodegenOptions {
                // As pointed out in `panics_and_exceptions.md` the tool only supports `-C
                // panic=abort` and therefore we explicitly opt into this config for tests.
                panic: Some(rustc_target::spec::PanicStrategy::Abort),
                ..Default::default()
            },
            ..Default::default()
        };

        let config = rustc_interface::interface::Config {
            opts,
            crate_cfg: Default::default(),
            crate_check_cfg: Default::default(),
            input: Input::Str {
                name: rustc_span::FileName::Custom(TEST_FILENAME.to_string()),
                input: source.into(),
            },
            output_file: None,
            output_dir: None,
            file_loader: None,
            lint_caps: Default::default(),
            parse_sess_created: None,
            register_lints: None,
            override_queries: None,
            make_codegen_backend: None,
            registry: rustc_errors::registry::Registry::new(rustc_error_codes::DIAGNOSTICS),
            locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES,
            ice_file: None,
        };

        rustc_interface::interface::run_compiler(config, |compiler| {
            compiler.enter(|queries| {
                use rustc_interface::interface::Result;
                let result: Result<Result<()>> = super::enter_tcx(queries, |tcx| {
                    // Explicitly force full `analysis` stage to detect compilation
                    // errors that the earlier stages might miss.  This helps ensure that the
                    // test inputs are valid Rust (even if `callback` wouldn't
                    // have triggered full analysis).
                    tcx.analysis(())
                });

                // Flatten the outer and inner results into a single result.  (outer result
                // comes from `enter_tcx`; inner result comes from `analysis`).
                //
                // TODO(lukasza): Use `Result::flatten` API when it gets stabilized.  See also
                // https://github.com/rust-lang/rust/issues/70142
                let result: Result<()> = result.and_then(|result| result);

                // `analysis` might succeed even if there are some lint / warning errors.
                // Detecting these requires explicitly checking `compile_status`.
                let result: Result<()> = result.and_then(|()| compiler.session().compile_status());

                // Run the provided callback.
                let result: Result<T> = result.and_then(|()| super::enter_tcx(queries, callback));
                result.expect("Test inputs shouldn't cause compilation errors")
            })
        })
    }
}
