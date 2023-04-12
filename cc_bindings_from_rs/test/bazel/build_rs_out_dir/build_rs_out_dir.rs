// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `build_rs_out_dir_test.cc`.
//!
//! See the top-level comment in BUILD for a high-level description and
//! motivation of the test.

include!(concat!(env!("OUT_DIR"), "/include_me.rs"));
