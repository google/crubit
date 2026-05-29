// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// bridged_types_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_test_uformat_ubridged_ufunc_uarg_uby_upointer(
    __param_0: *const core::ffi::c_void,
) -> () {
    unsafe {
        let __param_0 = {
            let mut __crubit_temp = ::core::mem::MaybeUninit::<
                ::bridged_types_golden::test_format_bridged_func_arg_by_pointer::RustTypeView,
            >::uninit();
            cpp_pointer_to_rust_struct(
                __param_0,
                __crubit_temp.as_mut_ptr() as *mut core::ffi::c_void,
            );
            __crubit_temp.assume_init()
        };
        ::bridged_types_golden::test_format_bridged_func_arg_by_pointer::test_format_bridged_func_arg_by_pointer(__param_0)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_test_uformat_ubridged_ufunc_uarg_uby_uvalue(
    _a: *const core::ffi::c_void,
) -> () {
    unsafe {
        let _a = {
            let mut __crubit_temp = ::core::mem::MaybeUninit::<
                ::bridged_types_golden::test_format_bridged_func_arg_by_value::RustType,
            >::uninit();
            convert_cpp_to_rust_type(_a, __crubit_temp.as_mut_ptr() as *mut core::ffi::c_void);
            __crubit_temp.assume_init()
        };
        ::bridged_types_golden::test_format_bridged_func_arg_by_value::test_format_bridged_func_arg_by_value(_a)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_test_uformat_ubridged_ureturn_utype_uby_upointer(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=::bridged_types_golden::test_format_bridged_return_type_by_pointer::test_format_bridged_return_type_by_pointer();
        rust_struct_to_cpp_pointer(
            std::ptr::from_ref(&__rs_return_value) as *const core::ffi::c_void,
            __ret_ptr,
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_test_uformat_ubridged_ureturn_utype_uby_uvalue(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=::bridged_types_golden::test_format_bridged_return_type_by_value::test_format_bridged_return_type_by_value();
        rust_to_cpp_converter(
            std::ptr::from_ref(&__rs_return_value) as *const core::ffi::c_void,
            __ret_ptr,
        );
    }
}
unsafe extern "C" {
    fn convert_cpp_to_rust_type(cpp_in: *const core::ffi::c_void, rs_out: *mut core::ffi::c_void);
    fn cpp_pointer_to_rust_struct(cpp_in: *const core::ffi::c_void, rs_out: *mut core::ffi::c_void);
    fn rust_struct_to_cpp_pointer(rs_in: *const core::ffi::c_void, cpp_out: *mut core::ffi::c_void);
    fn rust_to_cpp_converter(rs_in: *const core::ffi::c_void, cpp_out: *mut core::ffi::c_void);
}
