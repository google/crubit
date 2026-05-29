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
    opt: *const core::ffi::c_void,
) -> i32 {
    unsafe {
        let opt = {
            let mut __crubit_temp = ::core::mem::MaybeUninit::<
                ::layout_equivalent_generics_golden::MyOptional<i32>,
            >::uninit();
            __crubit_temp
                .write((opt as *const ::layout_equivalent_generics_golden::MyOptional<i32>).read());
            __crubit_temp.assume_init()
        };
        ::layout_equivalent_generics_golden::accept_optional_by_value(opt)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_ustatus(status: *const core::ffi::c_void) -> bool {
    unsafe {
        let status = {
            let mut __crubit_temp = ::core::mem::MaybeUninit::<
                ::layout_equivalent_generics_golden::MyStatusOr<()>,
            >::uninit();
            __crubit_temp.write(
                (status as *const ::layout_equivalent_generics_golden::MyStatusOr<()>).read(),
            );
            __crubit_temp.assume_init()
        };
        ::layout_equivalent_generics_golden::accept_status(status)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_ustatus_uor_uunit(
    status: *const core::ffi::c_void,
) -> bool {
    unsafe {
        let status = {
            let mut __crubit_temp = ::core::mem::MaybeUninit::<
                ::layout_equivalent_generics_golden::MyStatusOr<()>,
            >::uninit();
            __crubit_temp.write(
                (status as *const ::layout_equivalent_generics_golden::MyStatusOr<()>).read(),
            );
            __crubit_temp.assume_init()
        };
        ::layout_equivalent_generics_golden::accept_status_or_unit(status)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ubool_ubool_upair(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::layout_equivalent_generics_golden::create_bool_bool_pair();
        (__ret_ptr as *mut ::layout_equivalent_generics_golden::MyPair<bool, bool>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_uint_ubool_upair(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::layout_equivalent_generics_golden::create_int_bool_pair();
        (__ret_ptr as *mut ::layout_equivalent_generics_golden::MyPair<i32, bool>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ustatus_uwith_uprivate_usecret(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::layout_equivalent_generics_golden::create_status_with_private_secret();
        (__ret_ptr as *mut ::layout_equivalent_generics_golden::MyStatusOr<i32>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ustatus_uwith_usecret(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::layout_equivalent_generics_golden::create_status_with_secret();
        (__ret_ptr as *mut ::layout_equivalent_generics_golden::MyStatusOr<i32>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ustatus_uwith_usecret_ualias(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::layout_equivalent_generics_golden::create_status_with_secret_alias();
        (__ret_ptr as *mut ::layout_equivalent_generics_golden::MyStatusOr<i32>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_uok_usecret(status: *const core::ffi::c_void) -> bool {
    unsafe {
        let status = {
            let mut __crubit_temp = ::core::mem::MaybeUninit::<
                ::layout_equivalent_generics_golden::MyStatusOr<i32>,
            >::uninit();
            __crubit_temp.write(
                (status as *const ::layout_equivalent_generics_golden::MyStatusOr<i32>).read(),
            );
            __crubit_temp.assume_init()
        };
        ::layout_equivalent_generics_golden::is_ok_secret(status)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uoptional_uby_uvalue(
    x: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::layout_equivalent_generics_golden::return_optional_by_value(x);
        (__ret_ptr as *mut ::layout_equivalent_generics_golden::MyOptional<i32>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustatus(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::layout_equivalent_generics_golden::return_status();
        (__ret_ptr as *mut ::layout_equivalent_generics_golden::MyStatusOr<()>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustatus_ualias(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::layout_equivalent_generics_golden::return_status_alias();
        (__ret_ptr as *mut ::layout_equivalent_generics_golden::MyStatusOr<()>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustatus_unon_uunit(
    status: *const core::ffi::c_void,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let status = {
            let mut __crubit_temp = ::core::mem::MaybeUninit::<
                ::layout_equivalent_generics_golden::MyStatusOr<u32>,
            >::uninit();
            __crubit_temp.write(
                (status as *const ::layout_equivalent_generics_golden::MyStatusOr<u32>).read(),
            );
            __crubit_temp.assume_init()
        };
        let __rs_return_value = ::layout_equivalent_generics_golden::return_status_non_unit(status);
        (__ret_ptr as *mut ::layout_equivalent_generics_golden::MyStatusOr<u64>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustatus_uor_uunit(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::layout_equivalent_generics_golden::return_status_or_unit();
        (__ret_ptr as *mut ::layout_equivalent_generics_golden::MyStatusOr<()>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustatus_uor_uunit_ualias(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::layout_equivalent_generics_golden::return_status_or_unit_alias();
        (__ret_ptr as *mut ::layout_equivalent_generics_golden::MyStatusOr<()>)
            .write(__rs_return_value);
    }
}
