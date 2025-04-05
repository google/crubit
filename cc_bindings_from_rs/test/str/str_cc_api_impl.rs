// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// str_golden
// Features: supported

#![allow(unused_unsafe)]
#![allow(improper_ctypes_definitions)]

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
unsafe extern "C" fn __crubit_thunk_get_ustr_ulen<'__anon1>(
    __self: &'__anon1 ::str_golden::TypeWithStr,
) -> usize {
    unsafe { ::str_golden::TypeWithStr::get_str_len(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustr_udata<'__anon1>(
    __self: &'__anon1 ::str_golden::TypeWithStr,
) -> *const u8 {
    unsafe { ::str_golden::TypeWithStr::get_str_data(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::str_golden::TypeWithStr, str_field) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustr_ulen<'__anon1>(s: &'__anon1 str) -> usize {
    unsafe { ::str_golden::get_str_len(s) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustr_udata<'__anon1>(s: &'__anon1 str) -> *const u8 {
    unsafe { ::str_golden::get_str_data(s) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_foo_uas_ustr() -> &'static str {
    unsafe { ::str_golden::foo_as_str() }
}
