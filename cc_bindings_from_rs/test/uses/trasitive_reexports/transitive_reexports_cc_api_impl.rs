// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// transitive_reexports_golden
// Features: supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_direct_uto_utransitive(
    direct: &'static ::direct::Direct,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::transitive_reexports_golden::direct_to_transitive(direct);
        (__ret_ptr as *mut ::direct::Transitive).write(__rs_return_value);
    }
}
