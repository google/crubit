// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `build_rs_out_dir_test.cc`.
//!
//! See the top-level comment in BUILD for a high-level description and
//! motivation of the test.

include!(concat!(env!("OUT_DIR"), "/include_me.rs"));

/// This function should always exist, because the
/// cfg(feature="cfg_set_by_build_rs") is set unconditionally in build.rs. If it
/// seems not to be defined, it's because something isn't correctly ingesting
/// the build script.
#[cfg(feature = "cfg_set_by_build_rs")]
fn function_guarded_by_cfg() {}

pub fn cfg_set_by_build_rs() -> bool {
    function_guarded_by_cfg();
    cfg!(feature = "cfg_set_by_build_rs")
}
