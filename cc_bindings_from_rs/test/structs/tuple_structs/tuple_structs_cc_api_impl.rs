// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// tuple_structs_golden
// Features: supported, unsafe_types

#![allow(unused_unsafe)]
#![allow(improper_ctypes_definitions)]

const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructOnePublicArg>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::TupleStructOnePublicArg>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(arg: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuple_structs_golden::TupleStructOnePublicArg::create(arg);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructOnePublicArg)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<::tuple_structs_golden::TupleStructOnePublicArg>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructOnePublicArg::get_arg(__self)
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::tuple_structs_golden::TupleStructOnePublicArg, 0) == 0);
const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructOnePrivateArg>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::TupleStructOnePrivateArg>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(arg: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuple_structs_golden::TupleStructOnePrivateArg::create(arg);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructOnePrivateArg)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<::tuple_structs_golden::TupleStructOnePrivateArg>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructOnePrivateArg::get_arg(__self)
    }
}
const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructTwoPublicArgs>() == 8);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::TupleStructTwoPublicArgs>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    first_arg: i32,
    second_arg: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::tuple_structs_golden::TupleStructTwoPublicArgs::create(first_arg, second_arg);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructTwoPublicArgs)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ufirst_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<::tuple_structs_golden::TupleStructTwoPublicArgs>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructTwoPublicArgs::get_first_arg(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_usecond_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<::tuple_structs_golden::TupleStructTwoPublicArgs>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructTwoPublicArgs::get_second_arg(__self)
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::tuple_structs_golden::TupleStructTwoPublicArgs, 0) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::tuple_structs_golden::TupleStructTwoPublicArgs, 1) == 4);
const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructTwoPrivateArgs>() == 8);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::TupleStructTwoPrivateArgs>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    first_arg: i32,
    second_arg: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::tuple_structs_golden::TupleStructTwoPrivateArgs::create(first_arg, second_arg);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructTwoPrivateArgs)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ufirst_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::tuple_structs_golden::TupleStructTwoPrivateArgs,
    >,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructTwoPrivateArgs::get_first_arg(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_usecond_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::tuple_structs_golden::TupleStructTwoPrivateArgs,
    >,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructTwoPrivateArgs::get_second_arg(__self)
    }
}
