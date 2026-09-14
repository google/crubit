// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// str_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::str_golden::TypeWithStr>() == 16);
const _: () = assert!(::std::mem::align_of::<::str_golden::TypeWithStr>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    s: *mut &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let s = s.read();
        let __rs_return_value = ::str_golden::TypeWithStr::create(s);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustr_ulen(
    __self: &'static ::str_golden::TypeWithStr,
) -> usize {
    unsafe { ::str_golden::TypeWithStr::get_str_len(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustr_udata(
    __self: &'static ::str_golden::TypeWithStr,
) -> *const u8 {
    unsafe { ::str_golden::TypeWithStr::get_str_data(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::str_golden::TypeWithStr, str_field) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_foo_uas_ustr(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::str_golden::foo_as_str();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustr_udata(s: *mut &'static str) -> *const u8 {
    unsafe {
        let s = s.read();
        ::str_golden::get_str_data(s)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustr_ulen(s: *mut &'static str) -> usize {
    unsafe {
        let s = s.read();
        ::str_golden::get_str_len(s)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_str_uchecked_uas_upotentially_ualiasing(
    __param_0: *mut &'static str,
    __param_1: &'static mut u8,
) -> () {
    unsafe {
        let __param_0 = __param_0.read();
        ::str_golden::str_checked_as_potentially_aliasing(__param_0, __param_1)
    }
}
