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
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::shared_ptr_golden::clone_shared_ptr(val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_consume_ushared_uptr(_val: *const core::ffi::c_void) -> () {
    unsafe {
        let _val = {
            let mut __crubit_temp =
                ::core::mem::MaybeUninit::<::cc_std::std::shared_ptr<i32>>::uninit();
            __crubit_temp.write((_val as *const ::cc_std::std::shared_ptr<i32>).read());
            __crubit_temp.assume_init()
        };
        ::shared_ptr_golden::consume_shared_ptr(_val)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_roundtrip_ushared_uptr(
    val: *const core::ffi::c_void,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let val = {
            let mut __crubit_temp =
                ::core::mem::MaybeUninit::<::cc_std::std::shared_ptr<i32>>::uninit();
            __crubit_temp.write((val as *const ::cc_std::std::shared_ptr<i32>).read());
            __crubit_temp.assume_init()
        };
        let __rs_return_value = ::shared_ptr_golden::roundtrip_shared_ptr(val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
