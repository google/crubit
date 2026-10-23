// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// type_aliases_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_returns_uflipped_ualias(
    __ret_ptr: *mut ::core::result::Result<u32, i8>,
) -> () {
    unsafe {
        __ret_ptr.write(::type_aliases_golden::test_generics_matching::returns_flipped_alias());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_returns_umatching_ualias(
    __ret_ptr: *mut ::core::result::Result<i32, i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::type_aliases_golden::test_generics_matching::returns_matching_alias());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_returns_uspecialized(
    __ret_ptr: *mut ::core::result::Result<i32, i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::type_aliases_golden::test_generics_matching::returns_specialized());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_func_uusing_ualias() -> i32 {
    unsafe { ::type_aliases_golden::test_type_aliases::func_using_alias() }
}
