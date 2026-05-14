// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// stdlib_golden
// Features: fmt, supported, types

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::stdlib_golden::MyStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::stdlib_golden::MyStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <::stdlib_golden::MyStruct as ::core::default::Default>::default();
        (__ret_ptr as *mut ::stdlib_golden::MyStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::stdlib_golden::MyStruct>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::stdlib_golden::MyStruct,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::stdlib_golden::MyStruct as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::stdlib_golden::MyStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::stdlib_golden::MyStruct,
    source: &'static ::stdlib_golden::MyStruct,
) -> () {
    unsafe { <::stdlib_golden::MyStruct as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::stdlib_golden::MyStruct::new(x);
        (__ret_ptr as *mut ::stdlib_golden::MyStruct).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::stdlib_golden::MyStruct, x) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Iterator_unext(
    __self: &'static mut ::stdlib_golden::MyStruct,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value = <::stdlib_golden::MyStruct as ::core::iter::Iterator>::next(__self);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
