// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// uses_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ux(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::uses_golden::return_x();
        (__ret_ptr as *mut ::extern_crate::X).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_private_ufn() -> i32 {
    unsafe { ::uses_golden::doc_hidden_test::visible::private_fn() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_private_umiddle_upath() -> i32 {
    unsafe { ::uses_golden::a::c::private_middle_path() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_private_ufn() -> i32 {
    unsafe { ::uses_golden::private_fn() }
}
const _: () = assert!(::std::mem::size_of::<::uses_golden::AliasOfExportedStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::uses_golden::AliasOfExportedStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(field: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::uses_golden::AliasOfExportedStruct::create(field);
        (__ret_ptr as *mut ::uses_golden::AliasOfExportedStruct).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::uses_golden::AliasOfExportedStruct, field) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uy(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::uses_golden::return_y();
        (__ret_ptr as *mut ::extern_crate::Y).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::uses_golden::Original>() == 4);
const _: () = assert!(::std::mem::align_of::<::uses_golden::Original>() == 4);
const _: () = assert!(::core::mem::offset_of!(::uses_golden::Original, field) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_f() -> i32 {
    unsafe { ::uses_golden::f() }
}
