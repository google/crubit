// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// dep_crate_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::dep_crate_golden::ExpectedName>() == 4);
const _: () = assert!(::std::mem::align_of::<::dep_crate_golden::ExpectedName>() == 4);
const _: () = assert!(::core::mem::offset_of!(::dep_crate_golden::ExpectedName, x) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_use_us(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::dep_crate_golden::use_s();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
