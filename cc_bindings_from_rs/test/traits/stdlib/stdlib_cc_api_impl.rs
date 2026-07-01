// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// stdlib_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::stdlib_golden::MyStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::stdlib_golden::MyStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::stdlib_golden::MyStruct as ::core::default::Default>::default();
        (__ret_ptr as *mut ::stdlib_golden::MyStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    __self: &'static mut ::core::mem::MaybeUninit<::stdlib_golden::MyStruct>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    __self: &'static ::stdlib_golden::MyStruct,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::stdlib_golden::MyStruct as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::stdlib_golden::MyStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    __self: &'static mut ::stdlib_golden::MyStruct,
    source: &'static ::stdlib_golden::MyStruct,
) -> () {
    unsafe { <::stdlib_golden::MyStruct as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::stdlib_golden::MyStruct::new(x);
        (__ret_ptr as *mut ::stdlib_golden::MyStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_From_ufrom_ustdlib_ugolden_x0000003a_x0000003aMyStruct_ui32(
    value: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::stdlib_golden::MyStruct as ::core::convert::From<i32>>::from(value);
        (__ret_ptr as *mut ::stdlib_golden::MyStruct).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::stdlib_golden::MyStruct, x) == 0);
const _: () = assert!(::std::mem::size_of::<::stdlib_golden::NonCloneableIterator>() == 4);
const _: () = assert!(::std::mem::align_of::<::stdlib_golden::NonCloneableIterator>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::stdlib_golden::NonCloneableIterator::new(x);
        (__ret_ptr as *mut ::stdlib_golden::NonCloneableIterator).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::stdlib_golden::NonCloneableIterator, x) == 0);
const _: () = assert!(::std::mem::size_of::<::stdlib_golden::NonCloneableValue>() == 4);
const _: () = assert!(::std::mem::align_of::<::stdlib_golden::NonCloneableValue>() == 4);
const _: () = assert!(::core::mem::offset_of!(::stdlib_golden::NonCloneableValue, x) == 0);
const _: () = assert!(::std::mem::size_of::<::stdlib_golden::RefIterator>() == 24);
const _: () = assert!(::std::mem::align_of::<::stdlib_golden::RefIterator>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    slice: &'static [i32],
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::stdlib_golden::RefIterator::new(slice);
        (__ret_ptr as *mut ::stdlib_golden::RefIterator<'static>).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::stdlib_golden::RefIterator, slice) == 0);
const _: () = assert!(::core::mem::offset_of!(::stdlib_golden::RefIterator, index) == 16);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    __self: &'static mut ::stdlib_golden::MyStruct,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value = <::stdlib_golden::MyStruct as ::core::iter::Iterator>::next(__self);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_ToString_uto_ustring_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    __self: &'static ::stdlib_golden::MyStruct,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::stdlib_golden::MyStruct as ::alloc::string::ToString>::to_string(__self);
        (__ret_ptr as *mut ::alloc::string::String).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aNonCloneableIterator(
    __self: &'static mut ::stdlib_golden::NonCloneableIterator,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value =
            <::stdlib_golden::NonCloneableIterator as ::core::iter::Iterator>::next(__self);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<
                    ::stdlib_golden::NonCloneableValue,
                >()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aRefIterator_x0000003c_x00000027a_x0000003e(
    __self: &'static mut ::stdlib_golden::RefIterator<'static>,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value =
            <::stdlib_golden::RefIterator as ::core::iter::Iterator>::next(__self);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<&'static i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
