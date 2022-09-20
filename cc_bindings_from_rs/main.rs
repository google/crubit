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

use itertools::Itertools;
use rustc_interface::interface::Compiler;
use rustc_interface::Queries;

#[derive(Default)]
struct CompilerCallbacks {}

impl rustc_driver::Callbacks for CompilerCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        let rustc_result = lib::enter_tcx(queries, |tcx| {
            // TODO(lukasza): Replace this with actually generating C++ bindings.
            for fn_name in lib::get_names_of_exported_fns(tcx) {
                println!("EXPORTED FN: {}", fn_name);
            }
        });

        // Expecting no rustc errors here, because `after_analysis` is only called by
        // `rustc_driver` if earlier compiler analysis was successful (which as the
        // *last* compilation phase presumably covers *all* errors).
        rustc_result.expect("Expecting no compile errors inside `after_analysis` callback.");

        rustc_driver::Compilation::Stop
    }
}

fn main() {
    rustc_driver::init_env_logger("CRUBIT_LOG");

    // TODO: Investigate if we should install a signal handler here.  See also how
    // compiler/rustc_driver/src/lib.rs calls `signal_handler::install()`.

    rustc_driver::install_ice_hook();

    // `std::env::args()` will panic if any of the cmdline arguments are not valid
    // Unicode.  This seems okay.
    let args = std::env::args().collect_vec();

    // Rust compiler unwinds with a special sentinel value to abort compilation on
    // fatal errors. We use `catch_with_exit_code` to 1) catch such panics and
    // translate them into an exit code, and 2) resume and propagate other
    // panics.
    let exit_code = rustc_driver::catch_with_exit_code(|| {
        let mut callbacks = CompilerCallbacks::default();
        rustc_driver::RunCompiler::new(&args, &mut callbacks).run()
    });
    std::process::exit(exit_code);
}

// TODO(lukasza): Make `lib` a separate crate (once we move to Bazel).
mod lib;
