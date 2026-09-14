// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// no_core_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::no_core_golden::Test>() == 24);
const _: () = assert!(::std::mem::align_of::<::no_core_golden::Test>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uno_ucore_ugolden_x0000003a_x0000003aTest(
    __self: *mut ::no_core_golden::Test,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::no_core_golden::Test::new();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_s(
    __self: &'static ::no_core_golden::Test,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::no_core_golden::Test::s(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
