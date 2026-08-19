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
const _: () = assert!(::core::mem::offset_of!(::result_golden::CloneNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::result_golden::CloneNoDefaultResult>() == 4);
const _: () = assert!(::std::mem::align_of::<::result_golden::CloneNoDefaultResult>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::CloneNoDefaultResult::new(val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
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
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
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
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::GetsResult, value) == 0);
const _: () = assert!(::std::mem::size_of::<::result_golden::HasDefault>() == 12);
const _: () = assert!(::std::mem::align_of::<::result_golden::HasDefault>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    val: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::HasDefault::new(val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_val(
    __self: &'static ::result_golden::HasDefault,
) -> &'static str {
    unsafe { ::result_golden::HasDefault::val(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::result_golden::HasDefaultResult>() == 24);
const _: () = assert!(::std::mem::align_of::<::result_golden::HasDefaultResult>() == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uresult_ugolden_x0000003a_x0000003aHasDefaultResult(
    __self: *mut ::result_golden::HasDefaultResult,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    val: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::HasDefaultResult::new(val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasDefaultResult, in_ok) == 0);
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasDefaultResult, in_err) == 12);
const _: () = assert!(::std::mem::size_of::<::result_golden::HasNoDefault>() == 12);
const _: () = assert!(::std::mem::align_of::<::result_golden::HasNoDefault>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_val(
    __self: &'static ::result_golden::HasNoDefault,
) -> &'static str {
    unsafe { ::result_golden::HasNoDefault::val(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::result_golden::HasNoDefaultResult>() == 24);
const _: () = assert!(::std::mem::align_of::<::result_golden::HasNoDefaultResult>() == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uresult_ugolden_x0000003a_x0000003aHasNoDefaultResult(
    __self: *mut ::result_golden::HasNoDefaultResult,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    val: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::HasNoDefaultResult::new(val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasNoDefaultResult, in_ok) == 0);
const _: () = assert!(::core::mem::offset_of!(::result_golden::HasNoDefaultResult, in_err) == 12);
const _: () = assert!(::std::mem::size_of::<::result_golden::NestedResult>() == 16);
const _: () = assert!(::std::mem::align_of::<::result_golden::NestedResult>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::NestedResult::new(val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::result_golden::NestedResult, in_ok) == 0);
const _: () = assert!(::core::mem::offset_of!(::result_golden::NestedResult, in_err) == 8);
const _: () = assert!(::std::mem::size_of::<::result_golden::ResultWithSizeTypes>() == 32);
const _: () = assert!(::std::mem::align_of::<::result_golden::ResultWithSizeTypes>() == 4);
const _: () =
    assert!(::core::mem::offset_of!(::result_golden::ResultWithSizeTypes, uval_in_ok) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::result_golden::ResultWithSizeTypes, uval_in_err) == 8);
const _: () =
    assert!(::core::mem::offset_of!(::result_golden::ResultWithSizeTypes, ival_in_ok) == 16);
const _: () =
    assert!(::core::mem::offset_of!(::result_golden::ResultWithSizeTypes, ival_in_err) == 24);
const _: () = assert!(::std::mem::size_of::<::result_golden::ZStream>() == 8);
const _: () = assert!(::std::mem::align_of::<::result_golden::ZStream>() == 4);
const _: () = assert!(::core::mem::offset_of!(::result_golden::ZStream, zfree) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uresult_uby_uvalue(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::return_result_by_value();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uresult_uunit_uerr(
    has_err: bool,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::return_result_unit_err(has_err);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uresult_uunit_uok(
    has_val: bool,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::result_golden::return_result_unit_ok(has_val);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_uresult_uby_uvalue(
    r: *mut ::core::result::Result<u8, u8>,
) -> u8 {
    unsafe {
        let r = r.read();
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
unsafe extern "C" fn __crubit_thunk_take_uresult_uunit_uerr(
    val: *mut ::core::result::Result<u8, ()>,
) -> bool {
    unsafe {
        let val = val.read();
        ::result_golden::take_result_unit_err(val)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_uresult_uunit_uok(
    val: *mut ::core::result::Result<(), u8>,
) -> bool {
    unsafe {
        let val = val.read();
        ::result_golden::take_result_unit_ok(val)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cresult_ugolden_x0000003a_x0000003aCloneNoDefault_x0000002c_x00000020u8_x0000003e(
    __self: &'static ::core::result::Result<::result_golden::CloneNoDefault, u8>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<std::result::Result<::result_golden::CloneNoDefault,u8>as::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cresult_ugolden_x0000003a_x0000003aCloneNoDefault_x0000002c_x00000020u8_x0000003e(
    __self: &'static mut ::core::result::Result<::result_golden::CloneNoDefault, u8>,
    source: &'static ::core::result::Result<::result_golden::CloneNoDefault, u8>,
) -> () {
    unsafe {
        <std::result::Result<::result_golden::CloneNoDefault,u8>as::core::clone::Clone>::clone_from(__self,source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cu8_x0000002c_x00000020result_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
    __self: &'static ::core::result::Result<u8, ::result_golden::CloneNoDefault>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<std::result::Result<u8,::result_golden::CloneNoDefault>as::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cu8_x0000002c_x00000020result_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
    __self: &'static mut ::core::result::Result<u8, ::result_golden::CloneNoDefault>,
    source: &'static ::core::result::Result<u8, ::result_golden::CloneNoDefault>,
) -> () {
    unsafe {
        <std::result::Result<u8,::result_golden::CloneNoDefault>as::core::clone::Clone>::clone_from(__self,source)
    }
}
