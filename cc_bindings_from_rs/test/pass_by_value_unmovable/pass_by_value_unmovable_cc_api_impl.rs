// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// pass_by_value_unmovable_golden
// Features: callables, fmt, supported, types

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::pass_by_value_unmovable_golden::CppMovable>() == 4);
const _: () = assert!(::std::mem::align_of::<::pass_by_value_unmovable_golden::CppMovable>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default() -> ::pass_by_value_unmovable_golden::CppMovable {
    unsafe { <::pass_by_value_unmovable_golden::CppMovable as ::core::default::Default>::default() }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::pass_by_value_unmovable_golden::CppMovable>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () =
    assert!(::core::mem::offset_of!(::pass_by_value_unmovable_golden::CppMovable, 0) == 0);
const _: () =
    assert!(::std::mem::size_of::<::pass_by_value_unmovable_golden::NotCppMovable>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::pass_by_value_unmovable_golden::NotCppMovable>() == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::pass_by_value_unmovable_golden::NotCppMovable>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () =
    assert!(::core::mem::offset_of!(::pass_by_value_unmovable_golden::NotCppMovable, 0) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_takes_uval_umovable(
    _val: ::pass_by_value_unmovable_golden::CppMovable,
) -> () {
    unsafe { ::pass_by_value_unmovable_golden::takes_val_movable(_val) }
}
