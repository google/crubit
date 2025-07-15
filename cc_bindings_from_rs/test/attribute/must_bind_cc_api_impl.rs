// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// must_bind_golden
// Features: infer_operator_lifetimes, supported, unsafe_types

#![allow(unused_unsafe)]
#![allow(improper_ctypes_definitions)]

const _: () = assert!(::std::mem::size_of::<::must_bind_golden::Original>() == 4);
const _: () = assert!(::std::mem::align_of::<::must_bind_golden::Original>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::must_bind_golden::Original::new();
        (__ret_ptr as *mut ::must_bind_golden::Original).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::must_bind_golden::Original, x) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_bar() -> () {
    unsafe { ::must_bind_golden::bar() }
}
