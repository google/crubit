// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `functions_test.cc`.

#[no_mangle]
pub extern "C" fn get_42_as_f64_via_no_mangle_extern_c() -> f64 {
    42.0
}

#[no_mangle]
pub extern "C" fn add_f64_via_no_mangle_extern_c(x: f64, y: f64) -> f64 {
    x + y
}
