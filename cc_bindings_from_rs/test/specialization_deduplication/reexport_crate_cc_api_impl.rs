// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// reexport_crate_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_use_us(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::reexport_crate_golden::use_s();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_use_us_ualias(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::reexport_crate_golden::use_s_alias();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
