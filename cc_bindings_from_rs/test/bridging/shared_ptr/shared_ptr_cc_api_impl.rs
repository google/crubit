// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// shared_ptr_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ushared_uptr(
    val: &'static ::cc_std::std::shared_ptr<i32>,
    __ret_ptr: *mut ::cc_std::std::shared_ptr<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::shared_ptr_golden::clone_shared_ptr(val));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_consume_ushared_uptr(
    _val: *mut ::cc_std::std::shared_ptr<i32>,
) -> () {
    unsafe {
        let _val = _val.read();
        ::shared_ptr_golden::consume_shared_ptr(_val)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_roundtrip_unonnull_ushared_uptr(
    val: *mut ::cc_std::std::NonNull<::cc_std::std::shared_ptr<i32>>,
    __ret_ptr: *mut ::cc_std::std::NonNull<::cc_std::std::shared_ptr<i32>>,
) -> () {
    unsafe {
        let val = val.read();
        __ret_ptr.write(::shared_ptr_golden::roundtrip_nonnull_shared_ptr(val));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_roundtrip_ushared_uptr(
    val: *mut ::cc_std::std::shared_ptr<i32>,
    __ret_ptr: *mut ::cc_std::std::shared_ptr<i32>,
) -> () {
    unsafe {
        let val = val.read();
        __ret_ptr.write(::shared_ptr_golden::roundtrip_shared_ptr(val));
    }
}
