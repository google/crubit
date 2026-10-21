// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// reexport_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue_uvia_ufree_ufn(
    value: &'static ::cc_type::cc_ns::CcType,
) -> i32 {
    unsafe { ::reexport_golden::get_value_via_free_fn(value) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uvalue(
    value: i32,
    __ret_ptr: *mut ::cc_type::cc_ns::CcType,
) -> () {
    unsafe {
        __ret_ptr.write(::reexport_golden::make_value(value));
    }
}
