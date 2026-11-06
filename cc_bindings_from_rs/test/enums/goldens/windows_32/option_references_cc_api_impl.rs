// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// option_references_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::core::mem::size_of::<::option_references_golden::Value>() == 4);
const _: () = assert!(::core::mem::align_of::<::option_references_golden::Value>() == 4);
const _: () = assert!(::core::mem::offset_of!(::option_references_golden::Value, x) == 0);
const _: () = assert!(::core::mem::size_of::<::option_references_golden::Values>() == 12);
const _: () = assert!(::core::mem::align_of::<::option_references_golden::Values>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_uoption_ureferences_ugolden_x0000003a_x0000003aValues(
    __ret_ptr: *mut ::option_references_golden::Values,
) -> () {
    unsafe {
        __ret_ptr
            .write(<::option_references_golden::Values as ::core::default::Default>::default());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    a: i32,
    b: i32,
    c: i32,
    __ret_ptr: *mut ::option_references_golden::Values,
) -> () {
    unsafe {
        __ret_ptr.write(::option_references_golden::Values::new(a, b, c));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get(
    __self: &'static ::option_references_golden::Values,
    index: usize,
    __ret_ptr: *mut ::core::option::Option<&'static i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::option_references_golden::Values::get(__self, index));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_umut(
    __self: &'static mut ::option_references_golden::Values,
    index: usize,
    __ret_ptr: *mut ::core::option::Option<&'static mut i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::option_references_golden::Values::get_mut(__self, index));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_iter(
    __self: &'static ::option_references_golden::Values,
    __ret_ptr: *mut ::option_references_golden::ValuesIter<'static>,
) -> () {
    unsafe {
        __ret_ptr.write(::option_references_golden::Values::iter(__self));
    }
}
const _: () = assert!(::core::mem::size_of::<::option_references_golden::ValuesIter>() == 8);
const _: () = assert!(::core::mem::align_of::<::option_references_golden::ValuesIter>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_as_uptr(
    v: &'static ::option_references_golden::Value,
    __ret_ptr: *mut ::core::option::Option<*const ::option_references_golden::Value>,
) -> () {
    unsafe {
        __ret_ptr.write(::option_references_golden::as_ptr(v));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_if_upositive(
    v: &'static ::option_references_golden::Value,
    __ret_ptr: *mut ::core::option::Option<&'static ::option_references_golden::Value>,
) -> () {
    unsafe {
        __ret_ptr.write(::option_references_golden::if_positive(v));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_if_upositive_umut(
    v: &'static mut ::option_references_golden::Value,
    __ret_ptr: *mut ::core::option::Option<&'static mut ::option_references_golden::Value>,
) -> () {
    unsafe {
        __ret_ptr.write(::option_references_golden::if_positive_mut(v));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_increment(
    v: *mut ::core::option::Option<&'static mut ::option_references_golden::Value>,
) -> () {
    unsafe {
        let v = v.read();
        ::option_references_golden::increment(v)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_same_uvalue(
    a: *mut ::core::option::Option<&'static ::option_references_golden::Value>,
    b: *mut ::core::option::Option<&'static ::option_references_golden::Value>,
) -> bool {
    unsafe {
        let a = a.read();
        let b = b.read();
        ::option_references_golden::same_value(a, b)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_x_uor(
    v: *mut ::core::option::Option<&'static ::option_references_golden::Value>,
    default: i32,
) -> i32 {
    unsafe {
        let v = v.read();
        ::option_references_golden::x_or(v, default)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Iterator_unext_uoption_ureferences_ugolden_x0000003a_x0000003aValuesIter_x0000003c_x00000027a_x0000003e_u_x00000027a(
    __self: &'static mut ::option_references_golden::ValuesIter<'static>,
    __ret_ptr: *mut ::core::option::Option<&'static i32>,
) -> () {
    unsafe {
        __ret_ptr.write(
            <::option_references_golden::ValuesIter<'static> as ::core::iter::Iterator>::next(
                __self,
            ),
        );
    }
}
