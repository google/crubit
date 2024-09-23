// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[path = "../../rs_bindings_from_cc.rs"]
mod real;
// Only needed for library crates.
// pub use real::*;

fn main() -> std::process::ExitCode {
    real::main()
}

// Rust dependencies on C++ libraries.
//
// Rust/Cargo needs to see all Rust dependencies in the build graph to avoid
// pruning them from the link line. By making them a explicit dependency of
// this crate, we ensure the C++ libraries are included in the link step.
extern crate rs_bindings_from_cc_sys;
