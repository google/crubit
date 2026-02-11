// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// option_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::option_golden::HasHasOptions>() == 4);
const _: () = assert!(::std::mem::align_of::<::option_golden::HasHasOptions>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(value: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasHasOptions::new(value);
        (__ret_ptr as *mut ::option_golden::HasHasOptions).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasHasOptions, me) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::HasOptions>() == 4);
const _: () = assert!(::std::mem::align_of::<::option_golden::HasOptions>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(value: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasOptions::new(value);
        (__ret_ptr as *mut ::option_golden::HasOptions).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_with_uoption(
    value: *const core::ffi::c_uchar,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let value = unsafe {
            ::bridge_rust::internal::decode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<u8>()),
                value,
            )
        };
        let __rs_return_value = ::option_golden::HasOptions::with_option(value);
        (__ret_ptr as *mut ::option_golden::HasOptions).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_with_unone(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasOptions::with_none();
        (__ret_ptr as *mut ::option_golden::HasOptions).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasOptions, c) == 0);
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasOptions, a) == 2);
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasOptions, b) == 3);
const _: () = assert!(::std::mem::size_of::<::option_golden::NonMaxU8>() == 1);
const _: () = assert!(::std::mem::align_of::<::option_golden::NonMaxU8>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_value(__self: &'static ::option_golden::NonMaxU8) -> u8 {
    unsafe { ::option_golden::NonMaxU8::value(__self) }
}
