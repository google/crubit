// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// layout_equivalent_pointers_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_test_uformat_ufunc_uarg_upointer_ulike(
    __param_0: *const core::ffi::c_void,
) -> () {
    unsafe {
        let __param_0 = {
            let mut __crubit_temp=::core::mem::MaybeUninit::<::layout_equivalent_pointers_golden::test_format_func_arg_pointer_like::RustTypeView>::uninit();
            __crubit_temp.write(::core::mem::transmute(__param_0));
            __crubit_temp.assume_init()
        };
        ::layout_equivalent_pointers_golden::test_format_func_arg_pointer_like::test_format_func_arg_pointer_like(__param_0)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_test_uformat_ureturn_utype_upointer_ulike(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=::layout_equivalent_pointers_golden::test_format_return_type_pointer_like::test_format_return_type_pointer_like();
        (__ret_ptr as*mut::layout_equivalent_pointers_golden::test_format_return_type_pointer_like::RustTypeOwned).write(__rs_return_value);
    }
}
