// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// vec_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::vec_golden::StructWithVec>() == 24);
const _: () = assert!(::std::mem::align_of::<::vec_golden::StructWithVec>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::vec_golden::StructWithVec>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::vec_golden::StructWithVec::new(val);
        (__ret_ptr as *mut ::vec_golden::StructWithVec).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::vec_golden::StructWithVec, v) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uu8_uvec(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::vec_golden::return_u8_vec();
        (__ret_ptr as *mut ::alloc::vec::Vec<u8>).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uvec(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::vec_golden::return_vec();
        (__ret_ptr as *mut ::alloc::vec::Vec<i32>).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_uvec(
    v: &'static mut ::core::mem::MaybeUninit<::alloc::vec::Vec<i32>>,
) -> i32 {
    unsafe {
        let v = v.assume_init_read();
        ::vec_golden::take_vec(v)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <::alloc::vec::Vec<i32> as ::core::default::Default>::default();
        (__ret_ptr as *mut ::alloc::vec::Vec<i32>).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::alloc::vec::Vec<i32>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::alloc::vec::Vec<i32> as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::alloc::vec::Vec<i32>).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::alloc::vec::Vec<i32>,
    source: &'static ::alloc::vec::Vec<i32>,
) -> () {
    unsafe { <::alloc::vec::Vec<i32> as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_drop_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e(
    vec: *mut ::alloc::vec::Vec<i32>,
) {
    unsafe { ::core::ptr::drop_in_place(vec) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <::alloc::vec::Vec<u8> as ::core::default::Default>::default();
        (__ret_ptr as *mut ::alloc::vec::Vec<u8>).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::alloc::vec::Vec<u8>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::alloc::vec::Vec<u8> as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::alloc::vec::Vec<u8>).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::alloc::vec::Vec<u8>,
    source: &'static ::alloc::vec::Vec<u8>,
) -> () {
    unsafe { <::alloc::vec::Vec<u8> as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_drop_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e(
    vec: *mut ::alloc::vec::Vec<u8>,
) {
    unsafe { ::core::ptr::drop_in_place(vec) };
}
