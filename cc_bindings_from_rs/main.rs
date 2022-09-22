// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]
#![deny(rustc::internal)]

// TODO(lukasza): Remove the `extern crate` declarations - they shouldn't be
// needed once we switch to Bazel.
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

mod bindings_generation {

    use anyhow::Context;
    use rustc_middle::ty::TyCtxt;
    use std::fmt::Display;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use crate::cmdline::Cmdline;
    use crate::lib::GeneratedBindings;

    pub fn main(cmdline: &Cmdline, tcx: TyCtxt) -> anyhow::Result<()> {
        let bindings = GeneratedBindings::generate(tcx);

        // TODO(lukasza): Use `tokens_to_string` from
        // `../common.token_stream_printer.rs`.
        write_file(cmdline.h_out(), bindings.h_body)
    }

    fn write_file(path: &Path, content: impl Display) -> anyhow::Result<()> {
        File::create(path)
            .and_then(|mut f| write!(f, "{}", content))
            .with_context(|| format!("Error when writing to {}", path.display()))
    }
}

/// Glue that enables the top-level `main() -> anyhow::Result<()>` to call into
/// `fn main(cmdline: &Cmdline, tcx: TyCtxt) -> anyhow::Result<()>` in the
/// `bindings_generation` module.
mod callbacks {

    use rustc_interface::interface::Compiler;
    use rustc_interface::Queries;

    use crate::cmdline::Cmdline;
    use crate::lib::enter_tcx;

    /// When passed to `rustc_driver::RunCompiler::run`, the `CompilerCallbacks`
    /// below will wait until Rust compiler parsing and analysis are done,
    /// and then will invoke `bindings_generation::main` and stash/expose
    /// its result via `CompilerCallbacks::into_result`.
    pub struct CompilerCallbacks<'a> {
        cmdline: &'a Cmdline,
        result: anyhow::Result<()>,
    }

    impl<'a> CompilerCallbacks<'a> {
        pub fn new(cmdline: &'a Cmdline) -> Self {
            Self { cmdline, result: Ok(()) }
        }

        pub fn into_result(self) -> anyhow::Result<()> {
            self.result
        }
    }

    impl rustc_driver::Callbacks for CompilerCallbacks<'_> {
        fn after_analysis<'tcx>(
            &mut self,
            _compiler: &Compiler,
            queries: &'tcx Queries<'tcx>,
        ) -> rustc_driver::Compilation {
            let rustc_result =
                enter_tcx(queries, |tcx| crate::bindings_generation::main(self.cmdline, tcx));

            // `expect`ing no errors in `rustc_result`, because `after_analysis` is only
            // called by `rustc_driver` if earlier compiler analysis was successful
            // (which as the *last* compilation phase presumably covers *all*
            // errors).
            self.result = rustc_result
                .expect("Expecting no compile errors inside `after_analysis` callback.");

            rustc_driver::Compilation::Stop
        }
    }
}

/// Wrapper around `rustc_driver::RunCompiler::run` that returns
/// `anyhow::Result<()>` instead of either returning
/// `rustc_interface::interface::Result<()>` or panicking with a special
/// sentinel value.
fn run_compiler<T>(rustc_args: &[String], callbacks: &mut T) -> anyhow::Result<()>
where
    T: rustc_driver::Callbacks + Send,
{
    // Rust compiler unwinds with a special sentinel value to abort compilation on
    // fatal errors. We use `catch_fatal_errors` to 1) catch such panics and
    // translate them into a Result, and 2) resume and propagate other panics.
    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::RunCompiler::new(rustc_args, callbacks).run()
    });

    // Flatten `Result<Result<T, ...>>` into `Result<T, ...>`.
    let result = result.and_then(|result| result);

    // Translate `rustc_interface::interface::Result` into `anyhow::Result`.  (Can't
    // use `?` because the trait `std::error::Error` is not implemented for
    // `ErrorGuaranteed` which is required by the impl of
    // `From<ErrorGuaranteed>` for `anyhow::Error`.)
    result.map_err(|_err| {
        // We can ignore `_err` because it has no payload / because this type has only
        // one valid/possible value.
        anyhow::format_err!("Errors reported by Rust compiler.")
    })
}

// TODO(lukasza): Add end-to-end tests that verify that the exit code is
// non-zero when:
// * input contains syntax errors - test coverage for `run_compiler`.
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

    // Invoke the Rust compiler with Crubit-specific `callbacks`.
    let mut callbacks = callbacks::CompilerCallbacks::new(&cmdline);
    run_compiler(cmdline.rustc_args(), &mut callbacks)?;
    callbacks.into_result()
}
