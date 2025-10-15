// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// str_golden
// Features: do_not_hardcode_status_bridge, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::str_golden::TypeWithStr>() == 16);
const _: () = assert!(::std::mem::align_of::<::str_golden::TypeWithStr>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <::str_golden::TypeWithStr as ::core::default::Default>::default();
        (__ret_ptr as *mut ::str_golden::TypeWithStr).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    s: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::str_golden::TypeWithStr::create(s);
        (__ret_ptr as *mut ::str_golden::TypeWithStr).write(__rs_return_value);
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
unsafe extern "C" fn __crubit_thunk_str_uchecked_uas_upotentially_ualiasing(
    __param_0: &'static str,
    __param_1: &'static mut u8,
) -> () {
    unsafe { ::str_golden::str_checked_as_potentially_aliasing(__param_0, __param_1) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustr_ulen(s: &'static str) -> usize {
    unsafe { ::str_golden::get_str_len(s) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustr_udata(s: &'static str) -> *const u8 {
    unsafe { ::str_golden::get_str_data(s) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_foo_uas_ustr() -> &'static str {
    unsafe { ::str_golden::foo_as_str() }
}
