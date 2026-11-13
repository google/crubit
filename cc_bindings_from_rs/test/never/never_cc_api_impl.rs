// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// never_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::core::mem::size_of::<::never_golden::NeverStruct>() == 4);
const _: () = assert!(::core::mem::align_of::<::never_golden::NeverStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_unever_ugolden_x0000003a_x0000003aNeverStruct(
    __ret_ptr: *mut ::never_golden::NeverStruct,
) -> () {
    unsafe {
        __ret_ptr.write(<::never_golden::NeverStruct as ::core::default::Default>::default());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_associated_ufn_unever_ureturn() -> ! {
    unsafe { ::never_golden::NeverStruct::associated_fn_never_return() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_method_unever_ureturn(
    __self: &'static ::never_golden::NeverStruct,
) -> ! {
    unsafe { ::never_golden::NeverStruct::method_never_return(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_never_ureturn() -> ! {
    unsafe { ::never_golden::never_return() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_option_unever(__ret_ptr: *mut ::core::option::Option<!>) -> () {
    unsafe {
        __ret_ptr.write(::never_golden::option_never());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_result_ui32_unever(
    __ret_ptr: *mut ::core::result::Result<i32, !>,
) -> () {
    unsafe {
        __ret_ptr.write(::never_golden::result_i32_never());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_result_unever_ui32(
    __ret_ptr: *mut ::core::result::Result<!, i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::never_golden::result_never_i32());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_result_unever_unever(
    __ret_ptr: *mut ::core::result::Result<!, !>,
) -> () {
    unsafe {
        __ret_ptr.write(::never_golden::result_never_never());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_result_unever_uunit(
    __ret_ptr: *mut ::core::result::Result<!, ()>,
) -> () {
    unsafe {
        __ret_ptr.write(::never_golden::result_never_unit());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_result_ustring_unever(
    __ret_ptr: *mut ::core::result::Result<::alloc::string::String, !>,
) -> () {
    unsafe {
        __ret_ptr.write(::never_golden::result_string_never());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_result_uunit_unever(
    __ret_ptr: *mut ::core::result::Result<(), !>,
) -> () {
    unsafe {
        __ret_ptr.write(::never_golden::result_unit_never());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000002c_x00000020_x00000021_x0000003e(
    __self: &'static ::core::result::Result<::alloc::string::String, !>,
    __ret_ptr: *mut ::core::result::Result<::alloc::string::String, !>,
) -> () {
    unsafe {
        __ret_ptr.write(
            <core::result::Result<::alloc::string::String, !> as ::core::clone::Clone>::clone(
                __self,
            ),
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000002c_x00000020_x00000021_x0000003e(
    __self: &'static mut ::core::result::Result<::alloc::string::String, !>,
    source: &'static ::core::result::Result<::alloc::string::String, !>,
) -> () {
    unsafe {
        <core::result::Result<::alloc::string::String, !> as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
