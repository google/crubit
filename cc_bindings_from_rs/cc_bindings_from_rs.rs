// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(never_type)]
#![feature(rustc_private)]

use arc_anyhow::Result;
use cmdline::Cmdline;
use itertools::Itertools;

fn main() -> Result<()> {
    // TODO: Investigate if we should install a signal handler here.  See also how
    // compiler/rustc_driver/src/lib.rs calls `signal_handler::install()`.

    // TODO(b/254689400): Provide Crubit-specific panic hook message (we shouldn't
    // use `rustc_driver::install_ice_hook` because it's message asks to file
    // bugs at https://github.com/rust-lang/rust/issues/new.

    // `std::env::args()` will panic if any of the cmdline arguments are not valid
    // Unicode.  This seems okay.
    let raw_args = std::env::args().collect_vec();

    Cmdline::new(&raw_args)
        .map_err(|err| err.into())
        .and_then(|args| cpp_api_from_rust_lib::run_with_cmdline_args(&args))
        .map_err(|err| match err.downcast_ref::<clap::Error>() {
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
