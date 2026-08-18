// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// partial_eq_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::partial_eq_golden::basic_test::MyStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::partial_eq_golden::basic_test::MyStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: usize, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::partial_eq_golden::basic_test::MyStruct::new(val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_upartial_ueq_ugolden_x0000003a_x0000003abasic_utest_x0000003a_x0000003aMyStruct_upartial_ueq_ugolden_x0000003a_x0000003abasic_utest_x0000003a_x0000003aMyStruct(
    __self: &'static ::partial_eq_golden::basic_test::MyStruct,
    other: &'static ::partial_eq_golden::basic_test::MyStruct,
) -> bool {
    unsafe {
        <::partial_eq_golden::basic_test::MyStruct as ::core::cmp::PartialEq<
            ::partial_eq_golden::basic_test::MyStruct,
        >>::eq(__self, other)
    }
}
const _: () = assert!(::std::mem::size_of::<::partial_eq_golden::tuple_collision::MyStruct>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::partial_eq_golden::tuple_collision::MyStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: usize, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::partial_eq_golden::tuple_collision::MyStruct::new(val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_upartial_ueq_ugolden_x0000003a_x0000003atuple_ucollision_x0000003a_x0000003aMyStruct_u_x00000028usize_x0000002c_x00000020bool_x00000029(
    __self: &'static ::partial_eq_golden::tuple_collision::MyStruct,
    _other: &'static (usize, bool),
) -> bool {
    unsafe {
        <::partial_eq_golden::tuple_collision::MyStruct as::core::cmp::PartialEq<(usize,bool,)>>::eq(__self,_other)
    }
}
const _: () = assert!(::std::mem::size_of::<::partial_eq_golden::usize_rhs::MyStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::partial_eq_golden::usize_rhs::MyStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: usize, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::partial_eq_golden::usize_rhs::MyStruct::new(val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_upartial_ueq_ugolden_x0000003a_x0000003ausize_urhs_x0000003a_x0000003aMyStruct_uusize(
    __self: &'static ::partial_eq_golden::usize_rhs::MyStruct,
    other: &'static usize,
) -> bool {
    unsafe {
        <::partial_eq_golden::usize_rhs::MyStruct as ::core::cmp::PartialEq<usize>>::eq(
            __self, other,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_u_x00000028u32_x0000002c_x00000020bool_x00000029(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <(u32, bool) as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
