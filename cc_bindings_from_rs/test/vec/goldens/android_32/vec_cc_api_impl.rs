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
const _: () = assert!(::std::mem::size_of::<::vec_golden::RustVecOwner>() == 12);
const _: () = assert!(::std::mem::align_of::<::vec_golden::RustVecOwner>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_uvec_ugolden_x0000003a_x0000003aRustVecOwner(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::vec_golden::RustVecOwner as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uvec_ugolden_x0000003a_x0000003aRustVecOwner(
    __self: *mut ::vec_golden::RustVecOwner,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::vec_golden::RustVecOwner::new();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_umut_uvec(
    __self: &'static mut ::vec_golden::RustVecOwner,
) -> &'static mut ::alloc::vec::Vec<i32> {
    unsafe { ::vec_golden::RustVecOwner::get_mut_vec(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ulen(__self: &'static ::vec_golden::RustVecOwner) -> usize {
    unsafe { ::vec_golden::RustVecOwner::get_len(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uelement(
    __self: &'static ::vec_golden::RustVecOwner,
    index: usize,
) -> i32 {
    unsafe { ::vec_golden::RustVecOwner::get_element(__self, index) }
}
const _: () = assert!(::std::mem::size_of::<::vec_golden::StructWithVec>() == 12);
const _: () = assert!(::std::mem::align_of::<::vec_golden::StructWithVec>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::vec_golden::StructWithVec::new(val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::vec_golden::StructWithVec, v) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_drop_uvec(_v: *mut ::alloc::vec::Vec<i32>) -> () {
    unsafe {
        let _v = _v.read();
        ::vec_golden::drop_vec(_v)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ugrown_uvec(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::vec_golden::return_grown_vec();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uu8_uvec(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::vec_golden::return_u8_vec();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uvec(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::vec_golden::return_vec();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_rust_uadd_uelements(
    v: &'static mut ::alloc::vec::Vec<i32>,
) -> () {
    unsafe { ::vec_golden::rust_add_elements(v) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_uvec(v: *mut ::alloc::vec::Vec<i32>) -> i32 {
    unsafe {
        let v = v.read();
        ::vec_golden::take_vec(v)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::alloc::vec::Vec<i32> as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    __self: &'static ::alloc::vec::Vec<i32>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::alloc::vec::Vec<i32> as ::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    __self: &'static mut ::alloc::vec::Vec<i32>,
    source: &'static ::alloc::vec::Vec<i32>,
) -> () {
    unsafe { <::alloc::vec::Vec<i32> as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::alloc::vec::Vec<u8> as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
    __self: &'static ::alloc::vec::Vec<u8>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::alloc::vec::Vec<u8> as ::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
    __self: &'static mut ::alloc::vec::Vec<u8>,
    source: &'static ::alloc::vec::Vec<u8>,
) -> () {
    unsafe { <::alloc::vec::Vec<u8> as ::core::clone::Clone>::clone_from(__self, source) }
}
