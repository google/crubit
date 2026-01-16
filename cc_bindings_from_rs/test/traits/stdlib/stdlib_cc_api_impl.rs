// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// stdlib_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::stdlib_golden::MyStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::stdlib_golden::MyStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <::stdlib_golden::MyStruct as ::core::default::Default>::default();
        (__ret_ptr as *mut ::stdlib_golden::MyStruct).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::stdlib_golden::MyStruct, x) == 0);
