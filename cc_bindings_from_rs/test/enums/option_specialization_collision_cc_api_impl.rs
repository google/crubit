// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// option_specialization_collision_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () =
    assert!(::core::mem::size_of::<::option_specialization_collision_golden::Value>() == 4);
const _: () =
    assert!(::core::mem::align_of::<::option_specialization_collision_golden::Value>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get(
    __self: &'static ::option_specialization_collision_golden::Value,
    __ret_ptr: *mut ::core::option::Option<&'static i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::option_specialization_collision_golden::Value::get(__self));
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::option_specialization_collision_golden::Value, value) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_by_uref(
    value: &'static i32,
    __ret_ptr: *mut ::core::option::Option<&'static i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::option_specialization_collision_golden::by_ref(value));
    }
}
