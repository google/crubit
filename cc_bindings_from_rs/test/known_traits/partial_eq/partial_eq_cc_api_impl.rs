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
const _: () = assert!(::std::mem::size_of::<::partial_eq_golden::basic_test::MyStruct>() == 8);
const _: () = assert!(::std::mem::align_of::<::partial_eq_golden::basic_test::MyStruct>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: usize, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::partial_eq_golden::basic_test::MyStruct::new(val);
        (__ret_ptr as *mut ::partial_eq_golden::basic_test::MyStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq(
    __self: &'static ::partial_eq_golden::basic_test::MyStruct,
    other: &'static ::partial_eq_golden::basic_test::MyStruct,
) -> bool {
    unsafe {
        <::partial_eq_golden::basic_test::MyStruct as ::core::cmp::PartialEq<
            ::partial_eq_golden::basic_test::MyStruct,
        >>::eq(__self, other)
    }
}
const _: () = assert!(::std::mem::size_of::<::partial_eq_golden::tuple_collision::MyStruct>() == 8);
const _: () =
    assert!(::std::mem::align_of::<::partial_eq_golden::tuple_collision::MyStruct>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: usize, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::partial_eq_golden::tuple_collision::MyStruct::new(val);
        (__ret_ptr as *mut ::partial_eq_golden::tuple_collision::MyStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq(
    __self: &'static ::partial_eq_golden::tuple_collision::MyStruct,
    _other: &'static (usize, bool),
) -> bool {
    unsafe {
        <::partial_eq_golden::tuple_collision::MyStruct as::core::cmp::PartialEq<(usize,bool,)>>::eq(__self,_other)
    }
}
const _: () = assert!(::std::mem::size_of::<::partial_eq_golden::usize_rhs::MyStruct>() == 8);
const _: () = assert!(::std::mem::align_of::<::partial_eq_golden::usize_rhs::MyStruct>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: usize, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::partial_eq_golden::usize_rhs::MyStruct::new(val);
        (__ret_ptr as *mut ::partial_eq_golden::usize_rhs::MyStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq(
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
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <(usize, bool) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut usize).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut bool).write(__rs_return_value_1);
    }
}
