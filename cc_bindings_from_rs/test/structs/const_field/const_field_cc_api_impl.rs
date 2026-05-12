// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// const_field_golden
// Features: fmt, leading_colons_for_cpp_type, supported, types

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustruct_uwith_uconst_ufield_uby_uvalue_uin_uoption(
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value =
            ::const_field_golden::return_struct_with_const_field_by_value_in_option();
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<
                    ::cc_struct::struct_with_const_field,
                >()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustruct_uwith_uconst_ufield_uby_uvalue_uin_uresult(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::const_field_golden::return_struct_with_const_field_by_value_in_result();
        (__ret_ptr as *mut ::core::result::Result<::cc_struct::struct_with_const_field, u8>)
            .write(__rs_return_value);
    }
}
