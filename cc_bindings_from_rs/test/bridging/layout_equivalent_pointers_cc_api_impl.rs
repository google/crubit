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
    __param_0: ::layout_equivalent_pointers_golden::test_format_func_arg_pointer_like::RustTypeView,
) -> () {
    unsafe {
        ::layout_equivalent_pointers_golden::test_format_func_arg_pointer_like::test_format_func_arg_pointer_like(__param_0)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_test_uformat_ureturn_utype_upointer_ulike(
) -> ::layout_equivalent_pointers_golden::test_format_return_type_pointer_like::RustTypeOwned {
    unsafe {
        ::layout_equivalent_pointers_golden::test_format_return_type_pointer_like::test_format_return_type_pointer_like()
    }
}
