// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// compare_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::compare_golden::MyOrd>() == 4);
const _: () = assert!(::std::mem::align_of::<::compare_golden::MyOrd>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_ucompare_ugolden_x0000003a_x0000003aMyOrd_ucompare_ugolden_x0000003a_x0000003aMyOrd(
    __self: &'static ::compare_golden::MyOrd,
    other: &'static ::compare_golden::MyOrd,
) -> bool {
    unsafe {
        <::compare_golden::MyOrd as ::core::cmp::PartialEq<::compare_golden::MyOrd>>::eq(
            __self, other,
        )
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Ord_ucmp_ucompare_ugolden_x0000003a_x0000003aMyOrd(
    lhs: &::compare_golden::MyOrd,
    rhs: &::compare_golden::MyOrd,
) -> i8 {
    <::compare_golden::MyOrd as ::core::cmp::Ord>::cmp(lhs, rhs) as i8
}
const _: () = assert!(::core::mem::offset_of!(::compare_golden::MyOrd, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::compare_golden::MyPartialOrd>() == 8);
const _: () = assert!(::std::mem::align_of::<::compare_golden::MyPartialOrd>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_ucompare_ugolden_x0000003a_x0000003aMyPartialOrd_ucompare_ugolden_x0000003a_x0000003aMyPartialOrd(
    __self: &'static ::compare_golden::MyPartialOrd,
    other: &'static ::compare_golden::MyPartialOrd,
) -> bool {
    unsafe {
        <::compare_golden::MyPartialOrd as::core::cmp::PartialEq<::compare_golden::MyPartialOrd>>::eq(__self,other)
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_PartialOrd_upartial_ucmp_ucompare_ugolden_x0000003a_x0000003aMyPartialOrd_ucompare_ugolden_x0000003a_x0000003aMyPartialOrd(
    lhs: &::compare_golden::MyPartialOrd,
    rhs: &::compare_golden::MyPartialOrd,
) -> i8 {
    match <::compare_golden::MyPartialOrd as ::core::cmp::PartialOrd<
        ::compare_golden::MyPartialOrd,
    >>::partial_cmp(lhs, rhs)
    {
        ::core::option::Option::Some(ordering) => ordering as i8,
        ::core::option::Option::None => 2,
    }
}
const _: () = assert!(::core::mem::offset_of!(::compare_golden::MyPartialOrd, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::compare_golden::MyUnordered>() == 4);
const _: () = assert!(::std::mem::align_of::<::compare_golden::MyUnordered>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_ucompare_ugolden_x0000003a_x0000003aMyUnordered_ucompare_ugolden_x0000003a_x0000003aMyUnordered(
    __self: &'static ::compare_golden::MyUnordered,
    other: &'static ::compare_golden::MyUnordered,
) -> bool {
    unsafe {
        <::compare_golden::MyUnordered as ::core::cmp::PartialEq<::compare_golden::MyUnordered>>::eq(
            __self, other,
        )
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_PartialOrd_upartial_ucmp_ucompare_ugolden_x0000003a_x0000003aMyUnordered_ucompare_ugolden_x0000003a_x0000003aMyUnordered(
    lhs: &::compare_golden::MyUnordered,
    rhs: &::compare_golden::MyUnordered,
) -> i8 {
    match <::compare_golden::MyUnordered as ::core::cmp::PartialOrd<
        ::compare_golden::MyUnordered,
    >>::partial_cmp(lhs, rhs)
    {
        ::core::option::Option::Some(ordering) => ordering as i8,
        ::core::option::Option::None => 2,
    }
}
const _: () = assert!(::core::mem::offset_of!(::compare_golden::MyUnordered, 0) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Ord_ucmp_ucompare_ugolden_x0000003a_x0000003aMyOrd(
    __self: &'static ::compare_golden::MyOrd,
    other: &'static ::compare_golden::MyOrd,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::compare_golden::MyOrd as ::core::cmp::Ord>::cmp(__self, other);
        (__ret_ptr as *mut ::core::cmp::Ordering).write(__rs_return_value);
    }
}
