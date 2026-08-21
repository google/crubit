// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rs_hash_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::rs_hash_golden::derived_enum::Color>() == 1);
const _: () = assert!(::std::mem::align_of::<::rs_hash_golden::derived_enum::Color>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
    __self: &'static ::rs_hash_golden::derived_enum::Color,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::rs_hash_golden::derived_enum::Color as ::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
    __self: &'static mut ::rs_hash_golden::derived_enum::Color,
    source: &'static ::rs_hash_golden::derived_enum::Color,
) -> () {
    unsafe {
        <::rs_hash_golden::derived_enum::Color as ::core::clone::Clone>::clone_from(__self, source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
    __self: &'static ::rs_hash_golden::derived_enum::Color,
    other: &'static ::rs_hash_golden::derived_enum::Color,
) -> bool {
    unsafe {
        <::rs_hash_golden::derived_enum::Color as ::core::cmp::PartialEq<
            ::rs_hash_golden::derived_enum::Color,
        >>::eq(__self, other)
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_uenum_x0000003a_x0000003aColor(
    self_: &::rs_hash_golden::derived_enum::Color,
) -> u64 {
    ::bridge_rust::internal::hash_u64(self_)
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ublue(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::rs_hash_golden::derived_enum::create_blue();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ugreen(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::rs_hash_golden::derived_enum::create_green();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ured(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::rs_hash_golden::derived_enum::create_red();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::rs_hash_golden::derived_struct::Point>() == 8);
const _: () = assert!(::std::mem::align_of::<::rs_hash_golden::derived_struct::Point>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
    __self: &'static ::rs_hash_golden::derived_struct::Point,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::rs_hash_golden::derived_struct::Point as ::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
    __self: &'static mut ::rs_hash_golden::derived_struct::Point,
    source: &'static ::rs_hash_golden::derived_struct::Point,
) -> () {
    unsafe {
        <::rs_hash_golden::derived_struct::Point as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
    __self: &'static ::rs_hash_golden::derived_struct::Point,
    other: &'static ::rs_hash_golden::derived_struct::Point,
) -> bool {
    unsafe {
        <::rs_hash_golden::derived_struct::Point as ::core::cmp::PartialEq<
            ::rs_hash_golden::derived_struct::Point,
        >>::eq(__self, other)
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_ustruct_x0000003a_x0000003aPoint(
    self_: &::rs_hash_golden::derived_struct::Point,
) -> u64 {
    ::bridge_rust::internal::hash_u64(self_)
}
const _: () = assert!(::core::mem::offset_of!(::rs_hash_golden::derived_struct::Point, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::rs_hash_golden::derived_struct::Point, y) == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_upoint(
    x: i32,
    y: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::rs_hash_golden::derived_struct::create_point(x, y);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () =
    assert!(::std::mem::size_of::<::rs_hash_golden::derived_tuple_struct::TupleStruct>() == 8);
const _: () =
    assert!(::std::mem::align_of::<::rs_hash_golden::derived_tuple_struct::TupleStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
    __self: &'static ::rs_hash_golden::derived_tuple_struct::TupleStruct,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::rs_hash_golden::derived_tuple_struct::TupleStruct as ::core::clone::Clone>::clone(
                __self,
            );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
    __self: &'static mut ::rs_hash_golden::derived_tuple_struct::TupleStruct,
    source: &'static ::rs_hash_golden::derived_tuple_struct::TupleStruct,
) -> () {
    unsafe {
        <::rs_hash_golden::derived_tuple_struct::TupleStruct as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
    __self: &'static ::rs_hash_golden::derived_tuple_struct::TupleStruct,
    other: &'static ::rs_hash_golden::derived_tuple_struct::TupleStruct,
) -> bool {
    unsafe {
        <::rs_hash_golden::derived_tuple_struct::TupleStruct as ::core::cmp::PartialEq<
            ::rs_hash_golden::derived_tuple_struct::TupleStruct,
        >>::eq(__self, other)
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aderived_utuple_ustruct_x0000003a_x0000003aTupleStruct(
    self_: &::rs_hash_golden::derived_tuple_struct::TupleStruct,
) -> u64 {
    ::bridge_rust::internal::hash_u64(self_)
}
const _: () =
    assert!(::core::mem::offset_of!(::rs_hash_golden::derived_tuple_struct::TupleStruct, 0) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::rs_hash_golden::derived_tuple_struct::TupleStruct, 1) == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_utuple(
    x: i32,
    y: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::rs_hash_golden::derived_tuple_struct::create_tuple(x, y);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () =
    assert!(::std::mem::size_of::<::rs_hash_golden::explicit_struct::CustomHashStruct>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::rs_hash_golden::explicit_struct::CustomHashStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
    __self: &'static ::rs_hash_golden::explicit_struct::CustomHashStruct,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::rs_hash_golden::explicit_struct::CustomHashStruct as ::core::clone::Clone>::clone(
                __self,
            );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
    __self: &'static mut ::rs_hash_golden::explicit_struct::CustomHashStruct,
    source: &'static ::rs_hash_golden::explicit_struct::CustomHashStruct,
) -> () {
    unsafe {
        <::rs_hash_golden::explicit_struct::CustomHashStruct as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
    __self: &'static ::rs_hash_golden::explicit_struct::CustomHashStruct,
    other: &'static ::rs_hash_golden::explicit_struct::CustomHashStruct,
) -> bool {
    unsafe {
        <::rs_hash_golden::explicit_struct::CustomHashStruct as ::core::cmp::PartialEq<
            ::rs_hash_golden::explicit_struct::CustomHashStruct,
        >>::eq(__self, other)
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Hash_uhash_urs_uhash_ugolden_x0000003a_x0000003aexplicit_ustruct_x0000003a_x0000003aCustomHashStruct(
    self_: &::rs_hash_golden::explicit_struct::CustomHashStruct,
) -> u64 {
    ::bridge_rust::internal::hash_u64(self_)
}
const _: () = assert!(
    ::core::mem::offset_of!(::rs_hash_golden::explicit_struct::CustomHashStruct, value) == 0
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ucustom(
    value: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::rs_hash_golden::explicit_struct::create_custom(value);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
