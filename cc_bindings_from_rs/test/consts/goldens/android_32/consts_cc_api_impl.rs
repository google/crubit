// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// consts_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::consts_golden::NestedStruct>() == 16);
const _: () = assert!(::std::mem::align_of::<::consts_golden::NestedStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aNestedStruct_uconsts_ugolden_x0000003a_x0000003aNestedStruct(
    __self: &'static ::consts_golden::NestedStruct,
    other: &'static ::consts_golden::NestedStruct,
) -> bool {
    unsafe {
        <::consts_golden::NestedStruct as ::core::cmp::PartialEq<::consts_golden::NestedStruct>>::eq(
            __self, other,
        )
    }
}
const _: () = assert!(::core::mem::offset_of!(::consts_golden::NestedStruct, point) == 0);
const _: () = assert!(::core::mem::offset_of!(::consts_golden::NestedStruct, tuple) == 8);
const _: () = assert!(::std::mem::size_of::<::consts_golden::Point>() == 8);
const _: () = assert!(::std::mem::align_of::<::consts_golden::Point>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aPoint_uconsts_ugolden_x0000003a_x0000003aPoint(
    __self: &'static ::consts_golden::Point,
    other: &'static ::consts_golden::Point,
) -> bool {
    unsafe {
        <::consts_golden::Point as ::core::cmp::PartialEq<::consts_golden::Point>>::eq(
            __self, other,
        )
    }
}
const _: () = assert!(::core::mem::offset_of!(::consts_golden::Point, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::consts_golden::Point, y) == 4);
const _: () = assert!(::std::mem::size_of::<::consts_golden::StructWithArray>() == 8);
const _: () = assert!(::std::mem::align_of::<::consts_golden::StructWithArray>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aStructWithArray_uconsts_ugolden_x0000003a_x0000003aStructWithArray(
    __self: &'static ::consts_golden::StructWithArray,
    other: &'static ::consts_golden::StructWithArray,
) -> bool {
    unsafe {
        <::consts_golden::StructWithArray as ::core::cmp::PartialEq<
            ::consts_golden::StructWithArray,
        >>::eq(__self, other)
    }
}
const _: () = assert!(::core::mem::offset_of!(::consts_golden::StructWithArray, values) == 0);
const _: () = assert!(::std::mem::size_of::<::consts_golden::StructWithStr>() == 12);
const _: () = assert!(::std::mem::align_of::<::consts_golden::StructWithStr>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aStructWithStr_x0000003c_x00000027a_x0000003e_uconsts_ugolden_x0000003a_x0000003aStructWithStr_x0000003c_x00000027a_x0000003e(
    __self: &'static ::consts_golden::StructWithStr<'static>,
    other: &'static ::consts_golden::StructWithStr<'static>,
) -> bool {
    unsafe {
        <::consts_golden::StructWithStr as ::core::cmp::PartialEq<
            ::consts_golden::StructWithStr<'static>,
        >>::eq(__self, other)
    }
}
const _: () = assert!(::core::mem::offset_of!(::consts_golden::StructWithStr, msg) == 0);
const _: () = assert!(::core::mem::offset_of!(::consts_golden::StructWithStr, count) == 8);
const _: () = assert!(::std::mem::size_of::<::consts_golden::TupleStruct>() == 8);
const _: () = assert!(::std::mem::align_of::<::consts_golden::TupleStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_uconsts_ugolden_x0000003a_x0000003aTupleStruct_uconsts_ugolden_x0000003a_x0000003aTupleStruct(
    __self: &'static ::consts_golden::TupleStruct,
    other: &'static ::consts_golden::TupleStruct,
) -> bool {
    unsafe {
        <::consts_golden::TupleStruct as ::core::cmp::PartialEq<::consts_golden::TupleStruct>>::eq(
            __self, other,
        )
    }
}
const _: () = assert!(::core::mem::offset_of!(::consts_golden::TupleStruct, 0) == 0);
const _: () = assert!(::core::mem::offset_of!(::consts_golden::TupleStruct, 1) == 4);
const _: () = assert!(::std::mem::size_of::<::consts_golden::TyWithAssocConsts>() == 1);
const _: () = assert!(::std::mem::align_of::<::consts_golden::TyWithAssocConsts>() == 1);
