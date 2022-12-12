// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `modules_test.cc`.

pub mod basic_module {
    pub fn add_i32(x: i32, y: i32) -> i32 {
        x + y
    }
}
