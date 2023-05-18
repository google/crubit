// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `include_str_test.cc`.

pub fn get_the_answer() -> i32 {
    let s = include_str!("the_answer.md");
    let s = s.trim_end(); // Remove trailing '\n' and other whitespace.
    s.parse().unwrap()
}
