// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// pass_by_value_unmovable_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::pass_by_value_unmovable_golden::CppMovable>() == 4);
const _: () = assert!(::std::mem::align_of::<::pass_by_value_unmovable_golden::CppMovable>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_upass_uby_uvalue_uunmovable_ugolden_x0000003a_x0000003aCppMovable(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::pass_by_value_unmovable_golden::CppMovable as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_upass_uby_uvalue_uunmovable_ugolden_x0000003a_x0000003aCppMovable(
    __self: *mut ::pass_by_value_unmovable_golden::CppMovable,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
const _: () =
    assert!(::core::mem::offset_of!(::pass_by_value_unmovable_golden::CppMovable, 0) == 0);
const _: () =
    assert!(::std::mem::size_of::<::pass_by_value_unmovable_golden::NotCppMovable>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::pass_by_value_unmovable_golden::NotCppMovable>() == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_upass_uby_uvalue_uunmovable_ugolden_x0000003a_x0000003aNotCppMovable(
    __self: *mut ::pass_by_value_unmovable_golden::NotCppMovable,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
const _: () =
    assert!(::core::mem::offset_of!(::pass_by_value_unmovable_golden::NotCppMovable, 0) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_takes_uval_umovable(
    _val: *mut ::pass_by_value_unmovable_golden::CppMovable,
) -> () {
    unsafe {
        let _val = _val.read();
        ::pass_by_value_unmovable_golden::takes_val_movable(_val)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_takes_uval_uunmovable(
    _val: *mut ::pass_by_value_unmovable_golden::NotCppMovable,
) -> () {
    unsafe {
        let _val = _val.read();
        ::pass_by_value_unmovable_golden::takes_val_unmovable(_val)
    }
}
