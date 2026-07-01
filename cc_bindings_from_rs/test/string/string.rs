// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! End-to-end tests of `cc_bindings_from_rs`, focusing on `String` bindings.

pub fn roundtrip_rust_string(val: String) -> String {
    val
}

pub fn compute_rust_string_length(val: String) -> usize {
    val.len()
}

// The point of this test is to generate bindings for `&String` and not `&str`.
#[allow(clippy::ptr_arg)]
pub fn compute_rust_string_ref_length(val: &String) -> usize {
    val.len()
}

pub fn append_to_rust_string(val: &mut String, s: &str) {
    val.push_str(s);
}
