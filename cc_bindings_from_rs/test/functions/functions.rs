// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `functions_test.cc`.

use std::sync::Mutex;

#[no_mangle]
pub extern "C" fn get_42_as_f64_via_no_mangle_extern_c() -> f64 {
    42.0
}

#[no_mangle]
pub extern "C" fn add_f64_via_no_mangle_extern_c(x: f64, y: f64) -> f64 {
    x + y
}

#[no_mangle]
pub extern "C" fn add_i32_via_no_mangle_extern_c(x: i32, y: i32) -> i32 {
    x + y
}

#[export_name = "custom_export_name_for_add_i32"]
pub extern "C" fn add_i32_via_extern_c_with_export_name(x: i32, y: i32) -> i32 {
    x + y
}

pub extern "C" fn add_i32_via_extern_c_with_mangling(x: i32, y: i32) -> i32 {
    x + y
}

pub fn add_i32_via_rust_abi(x: i32, y: i32) -> i32 {
    x + y
}

pub fn add_i32_via_rust_abi_with_duplicated_param_names(x: i32, y: i32, _: i32, _: i32) -> i32 {
    x + y
}

static G_I32: Mutex<i32> = Mutex::new(0);

// Presence of the API below tests how bindings handle functions returning
// `void`.
#[export_name = "custom_export_name_for_get_global_i32"]
pub extern "C" fn set_global_i32_via_extern_c_with_export_name(x: i32) {
    *G_I32.lock().unwrap() = x;
}

#[no_mangle]
pub extern "C" fn get_global_i32_via_extern_c_with_export_name() -> i32 {
    *G_I32.lock().unwrap()
}
