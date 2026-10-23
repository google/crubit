// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// no_core_golden

#![no_std]
#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::core::mem::size_of::<::no_core_golden::Test>() == 12);
const _: () = assert!(::core::mem::align_of::<::no_core_golden::Test>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(__ret_ptr: *mut ::no_core_golden::Test) -> () {
    unsafe {
        __ret_ptr.write(::no_core_golden::Test::new());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_s(
    __self: &'static ::no_core_golden::Test,
    __ret_ptr: *mut &'static str,
) -> () {
    unsafe {
        __ret_ptr.write(::no_core_golden::Test::s(__self));
    }
}
const _: () = assert!(::core::mem::offset_of!(::no_core_golden::Test, s) == 0);
