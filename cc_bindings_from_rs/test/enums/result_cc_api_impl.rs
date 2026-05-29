// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// result_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::result_golden::CloneNoDefault>() == 1);
const _: () = assert!(::std::mem::align_of::<::result_golden::CloneNoDefault>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::result_golden::CloneNoDefault,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::result_golden::CloneNoDefault as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::result_golden::CloneNoDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::result_golden::CloneNoDefault,
    source: &'static ::result_golden::CloneNoDefault,
) -> () {
    unsafe { <::result_golden::CloneNoDefault as ::core::clone::Clone>::clone_from(__self, source) }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::CloneNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::result_golden::CloneNoDefaultResult>() == 4);
const _: () = assert!(::std::mem::align_of::<::result_golden::CloneNoDefaultResult>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::CloneNoDefaultResult::new(val);
        (__ret_ptr as *mut ::result_golden::CloneNoDefaultResult).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::CloneNoDefaultResult, in_ok) == 0);
const _: () = assert!(::core::mem::offset_of!(::result_golden::CloneNoDefaultResult, in_err) == 2);
const _: () = assert!(::std::mem::size_of::<::result_golden::CopyNoDefault>() == 1);
const _: () = assert!(::std::mem::align_of::<::result_golden::CopyNoDefault>() == 1);
const _: () = assert!(::core::mem::offset_of!(::result_golden::CopyNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::result_golden::CopyNoDefaultResult>() == 4);
const _: () = assert!(::std::mem::align_of::<::result_golden::CopyNoDefaultResult>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::CopyNoDefaultResult::new(val);
        (__ret_ptr as *mut ::result_golden::CopyNoDefaultResult).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::CopyNoDefaultResult, in_ok) == 0);
const _: () = assert!(::core::mem::offset_of!(::result_golden::CopyNoDefaultResult, in_err) == 2);
const _: () = assert!(::std::mem::size_of::<::result_golden::GetsResult>() == 8);
const _: () = assert!(::std::mem::align_of::<::result_golden::GetsResult>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::GetsResult::new(val);
        (__ret_ptr as *mut ::result_golden::GetsResult).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::GetsResult, value) == 0);
const _: () = assert!(::std::mem::size_of::<::result_golden::HasDefault>() == 24);
const _: () = assert!(::std::mem::align_of::<::result_golden::HasDefault>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::result_golden::HasDefault as ::core::default::Default>::default();
        (__ret_ptr as *mut ::result_golden::HasDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::result_golden::HasDefault>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    val: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::HasDefault::new(val);
        (__ret_ptr as *mut ::result_golden::HasDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_val(
    __self: &'static ::result_golden::HasDefault,
) -> &'static str {
    unsafe { ::result_golden::HasDefault::val(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::result_golden::HasDefaultResult>() == 48);
const _: () = assert!(::std::mem::align_of::<::result_golden::HasDefaultResult>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::result_golden::HasDefaultResult>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    val: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::HasDefaultResult::new(val);
        (__ret_ptr as *mut ::result_golden::HasDefaultResult).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasDefaultResult, in_ok) == 0);
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasDefaultResult, in_err) == 24);
const _: () = assert!(::std::mem::size_of::<::result_golden::HasNoDefault>() == 24);
const _: () = assert!(::std::mem::align_of::<::result_golden::HasNoDefault>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::result_golden::HasNoDefault>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_val(
    __self: &'static ::result_golden::HasNoDefault,
) -> &'static str {
    unsafe { ::result_golden::HasNoDefault::val(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::result_golden::HasNoDefaultResult>() == 48);
const _: () = assert!(::std::mem::align_of::<::result_golden::HasNoDefaultResult>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::result_golden::HasNoDefaultResult>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    val: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::HasNoDefaultResult::new(val);
        (__ret_ptr as *mut ::result_golden::HasNoDefaultResult).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasNoDefaultResult, in_ok) == 0);
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasNoDefaultResult, in_err) == 24);
const _: () = assert!(::std::mem::size_of::<::result_golden::NestedResult>() == 16);
const _: () = assert!(::std::mem::align_of::<::result_golden::NestedResult>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::NestedResult::new(val);
        (__ret_ptr as *mut ::result_golden::NestedResult).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::NestedResult, in_ok) == 0);
const _: () = assert!(::core::mem::offset_of!(::result_golden::NestedResult, in_err) == 8);
const _: () = assert!(::std::mem::size_of::<::result_golden::ResultWithSizeTypes>() == 64);
const _: () = assert!(::std::mem::align_of::<::result_golden::ResultWithSizeTypes>() == 8);
const _: () =
    assert!(::core::mem::offset_of!(::result_golden::ResultWithSizeTypes, uval_in_ok) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::result_golden::ResultWithSizeTypes, uval_in_err) == 16);
const _: () =
    assert!(::core::mem::offset_of!(::result_golden::ResultWithSizeTypes, ival_in_ok) == 32);
const _: () =
    assert!(::core::mem::offset_of!(::result_golden::ResultWithSizeTypes, ival_in_err) == 48);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uresult_uby_uvalue(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::return_result_by_value();
        (__ret_ptr as *mut ::core::result::Result<u8, u8>).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_uresult_uby_uvalue(
    r: &'static mut ::core::mem::MaybeUninit<::core::result::Result<u8, u8>>,
) -> u8 {
    unsafe {
        let r = r.assume_init_read();
        ::result_golden::take_result_by_value(r)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_uresult_uclone_uno_udefault_uerr(
    r: &'static ::core::result::Result<u8, ::result_golden::CloneNoDefault>,
) -> u8 {
    unsafe { ::result_golden::take_result_clone_no_default_err(r) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_uresult_ucopy_uno_udefault_uok(
    r: &'static ::core::result::Result<::result_golden::CopyNoDefault, u8>,
) -> u8 {
    unsafe { ::result_golden::take_result_copy_no_default_ok(r) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_uresult_uhas_udefault(
    r: &'static ::core::result::Result<::result_golden::HasDefault, u8>,
) -> &'static str {
    unsafe { ::result_golden::take_result_has_default(r) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::core::result::Result<::result_golden::CloneNoDefault, u8>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<std::result::Result<::result_golden::CloneNoDefault,u8>as::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::core::result::Result<::result_golden::CloneNoDefault, u8>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::core::result::Result<::result_golden::CloneNoDefault, u8>,
    source: &'static ::core::result::Result<::result_golden::CloneNoDefault, u8>,
) -> () {
    unsafe {
        <std::result::Result<::result_golden::CloneNoDefault,u8>as::core::clone::Clone>::clone_from(__self,source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::core::result::Result<u8, ::result_golden::CloneNoDefault>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<std::result::Result<u8,::result_golden::CloneNoDefault>as::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::core::result::Result<u8, ::result_golden::CloneNoDefault>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::core::result::Result<u8, ::result_golden::CloneNoDefault>,
    source: &'static ::core::result::Result<u8, ::result_golden::CloneNoDefault>,
) -> () {
    unsafe {
        <std::result::Result<u8,::result_golden::CloneNoDefault>as::core::clone::Clone>::clone_from(__self,source)
    }
}
