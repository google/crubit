// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// unions_golden
// Features: infer_operator_lifetimes, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::unions_golden::repr_c::U>() == 4);
const _: () = assert!(::std::mem::align_of::<::unions_golden::repr_c::U>() == 4);
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_c::U, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_c::U, y) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::unions_golden::repr_c::create();
        (__ret_ptr as *mut ::unions_golden::repr_c::U).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::unions_golden::repr_rust::U>() == 4);
const _: () = assert!(::std::mem::align_of::<::unions_golden::repr_rust::U>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_ux(
    __self: &'static mut ::unions_golden::repr_rust::U,
    x: u32,
) -> () {
    unsafe { ::unions_golden::repr_rust::U::set_x(__self, x) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ux(__self: &'static ::unions_golden::repr_rust::U) -> u32 {
    unsafe { ::unions_golden::repr_rust::U::get_x(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_uy(
    __self: &'static mut ::unions_golden::repr_rust::U,
    y: u32,
) -> () {
    unsafe { ::unions_golden::repr_rust::U::set_y(__self, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uy(__self: &'static ::unions_golden::repr_rust::U) -> u32 {
    unsafe { ::unions_golden::repr_rust::U::get_y(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_rust::U, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_rust::U, y) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::unions_golden::repr_rust::create();
        (__ret_ptr as *mut ::unions_golden::repr_rust::U).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::unions_golden::repr_c_packed::U>() == 4);
const _: () = assert!(::std::mem::align_of::<::unions_golden::repr_c_packed::U>() == 1);
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_c_packed::U, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_c_packed::U, y) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::unions_golden::repr_c_packed::create();
        (__ret_ptr as *mut ::unions_golden::repr_c_packed::U).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::unions_golden::repr_rust_packed::U>() == 4);
const _: () = assert!(::std::mem::align_of::<::unions_golden::repr_rust_packed::U>() == 1);
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_rust_packed::U, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_rust_packed::U, y) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::unions_golden::repr_rust_packed::create();
        (__ret_ptr as *mut ::unions_golden::repr_rust_packed::U).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::unions_golden::repr_c_clone::U>() == 4);
const _: () = assert!(::std::mem::align_of::<::unions_golden::repr_c_clone::U>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::unions_golden::repr_c_clone::U,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::unions_golden::repr_c_clone::U as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::unions_golden::repr_c_clone::U).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::unions_golden::repr_c_clone::U,
    source: &'static ::unions_golden::repr_c_clone::U,
) -> () {
    unsafe {
        <::unions_golden::repr_c_clone::U as ::core::clone::Clone>::clone_from(__self, source)
    }
}
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_c_clone::U, x) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::unions_golden::repr_c_clone::create();
        (__ret_ptr as *mut ::unions_golden::repr_c_clone::U).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::unions_golden::repr_rust_clone::U>() == 4);
const _: () = assert!(::std::mem::align_of::<::unions_golden::repr_rust_clone::U>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::unions_golden::repr_rust_clone::U,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::unions_golden::repr_rust_clone::U as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::unions_golden::repr_rust_clone::U).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::unions_golden::repr_rust_clone::U,
    source: &'static ::unions_golden::repr_rust_clone::U,
) -> () {
    unsafe {
        <::unions_golden::repr_rust_clone::U as ::core::clone::Clone>::clone_from(__self, source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_ux(
    __self: &'static mut ::unions_golden::repr_rust_clone::U,
    x: u32,
) -> () {
    unsafe { ::unions_golden::repr_rust_clone::U::set_x(__self, x) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ux(
    __self: &'static ::unions_golden::repr_rust_clone::U,
) -> u32 {
    unsafe { ::unions_golden::repr_rust_clone::U::get_x(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_rust_clone::U, x) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::unions_golden::repr_rust_clone::create();
        (__ret_ptr as *mut ::unions_golden::repr_rust_clone::U).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::unions_golden::repr_c_drop::U>() == 8);
const _: () = assert!(::std::mem::align_of::<::unions_golden::repr_c_drop::U>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::unions_golden::repr_c_drop::U as ::core::default::Default>::default();
        (__ret_ptr as *mut ::unions_golden::repr_c_drop::U).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::unions_golden::repr_c_drop::U>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_c_drop::U, x) == 0);
const _: () = assert!(::std::mem::size_of::<::unions_golden::repr_rust_drop::U>() == 8);
const _: () = assert!(::std::mem::align_of::<::unions_golden::repr_rust_drop::U>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::unions_golden::repr_rust_drop::U as ::core::default::Default>::default();
        (__ret_ptr as *mut ::unions_golden::repr_rust_drop::U).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::unions_golden::repr_rust_drop::U>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_ux(
    __self: &'static mut ::unions_golden::repr_rust_drop::U,
    x: *mut i32,
) -> () {
    unsafe { ::unions_golden::repr_rust_drop::U::set_x(__self, x) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ux(
    __self: &'static ::unions_golden::repr_rust_drop::U,
) -> *mut i32 {
    unsafe { ::unions_golden::repr_rust_drop::U::get_x(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::unions_golden::repr_rust_drop::U, x) == 0);
