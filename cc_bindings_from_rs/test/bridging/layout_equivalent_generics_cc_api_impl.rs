// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// layout_equivalent_generics_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_uoptional_uby_ureference(
    opt: &'static ::layout_equivalent_generics_golden::MyOptional<i32>,
) -> i32 {
    unsafe { ::layout_equivalent_generics_golden::accept_optional_by_reference(opt) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_uoptional_uby_uvalue(
    opt: *mut ::layout_equivalent_generics_golden::MyOptional<i32>,
) -> i32 {
    unsafe {
        let opt = opt.read();
        ::layout_equivalent_generics_golden::accept_optional_by_value(opt)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_uoptional_uisize(
    opt: *mut ::layout_equivalent_generics_golden::MyOptional<isize>,
) -> isize {
    unsafe {
        let opt = opt.read();
        ::layout_equivalent_generics_golden::accept_optional_isize(opt)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_uoptional_uusize(
    opt: *mut ::layout_equivalent_generics_golden::MyOptional<usize>,
) -> usize {
    unsafe {
        let opt = opt.read();
        ::layout_equivalent_generics_golden::accept_optional_usize(opt)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_ustatus(
    status: *mut ::layout_equivalent_generics_golden::MyStatusOr<()>,
) -> bool {
    unsafe {
        let status = status.read();
        ::layout_equivalent_generics_golden::accept_status(status)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_ustatus_uor_uunit(
    status: *mut ::layout_equivalent_generics_golden::MyStatusOr<()>,
) -> bool {
    unsafe {
        let status = status.read();
        ::layout_equivalent_generics_golden::accept_status_or_unit(status)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ubool_ubool_upair(
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyPair<bool, bool>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::create_bool_bool_pair());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_uint_ubool_upair(
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyPair<i32, bool>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::create_int_bool_pair());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ustatus_uwith_uprivate_usecret(
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyStatusOr<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::create_status_with_private_secret());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ustatus_uwith_usecret(
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyStatusOr<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::create_status_with_secret());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ustatus_uwith_usecret_ualias(
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyStatusOr<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::create_status_with_secret_alias());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_uok_usecret(
    status: *mut ::layout_equivalent_generics_golden::MyStatusOr<i32>,
) -> bool {
    unsafe {
        let status = status.read();
        ::layout_equivalent_generics_golden::is_ok_secret(status)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uoptional_uby_uvalue(
    x: i32,
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyOptional<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::return_optional_by_value(x));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uoptional_uisize(
    x: isize,
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyOptional<isize>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::return_optional_isize(x));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uoptional_uusize(
    x: usize,
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyOptional<usize>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::return_optional_usize(x));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustatus(
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyStatusOr<()>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::return_status());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustatus_ualias(
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyStatusOr<()>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::return_status_alias());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustatus_unon_uunit(
    status: *mut ::layout_equivalent_generics_golden::MyStatusOr<u32>,
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyStatusOr<u64>,
) -> () {
    unsafe {
        let status = status.read();
        __ret_ptr.write(::layout_equivalent_generics_golden::return_status_non_unit(status));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustatus_uor_uunit(
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyStatusOr<()>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::return_status_or_unit());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustatus_uor_uunit_ualias(
    __ret_ptr: *mut ::layout_equivalent_generics_golden::MyStatusOr<()>,
) -> () {
    unsafe {
        __ret_ptr.write(::layout_equivalent_generics_golden::return_status_or_unit_alias());
    }
}
