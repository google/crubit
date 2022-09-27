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

// TODO(lukasza): Add end-to-end tests that verify that the exit code is
// non-zero when:
// * input contains syntax errors - test coverage for `RunCompiler::run`.
// * mandatory parameters (e.g. `--h_out`) are missing - test coverage for how
//   `main` calls `Cmdline::new`.
// * `--h_out` cannot be written to (in this case, the error message should
//   include the os-level error + Crubit-level error that includes the file
//   name) - test coverage for `write_file`.
fn main() -> anyhow::Result<()> {
    rustc_driver::init_env_logger("CRUBIT_LOG");

    // TODO: Investigate if we should install a signal handler here.  See also how
    // compiler/rustc_driver/src/lib.rs calls `signal_handler::install()`.

    rustc_driver::install_ice_hook();

    // Parse Crubit's cmdline arguments.
    let cmdline = {
        // `std::env::args()` will panic if any of the cmdline arguments are not valid
        // Unicode.  This seems okay.
        let args = std::env::args().collect_vec();
        Cmdline::new(&args)?
    };

    // Invoke the Rust compiler and call `bindings_main::main` after parsing and
    // analysis are done.
    bindings_driver::RunCompiler::new(&cmdline).run()
}
