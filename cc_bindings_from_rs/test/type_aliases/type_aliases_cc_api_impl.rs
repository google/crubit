// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// type_aliases_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_returns_uflipped_ualias(
    __ret_ptr: *mut ::core::result::Result<u32, i8>,
) -> () {
    unsafe {
        __ret_ptr.write(::type_aliases_golden::test_generics_matching::returns_flipped_alias());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_returns_umatching_ualias(
    __ret_ptr: *mut ::core::result::Result<i32, i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::type_aliases_golden::test_generics_matching::returns_matching_alias());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_returns_uspecialized(
    __ret_ptr: *mut ::core::result::Result<i32, i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::type_aliases_golden::test_generics_matching::returns_specialized());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Reexported(x: i32) -> i32 {
    unsafe { ::type_aliases_golden::test_same_name_type_alias_and_function::Reexported(x) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_SameAdt(
    value: i32,
    __ret_ptr: *mut ::type_aliases_golden::test_same_name_type_alias_and_function::Wrapped,
) -> () {
    unsafe {
        __ret_ptr
            .write(::type_aliases_golden::test_same_name_type_alias_and_function::SameAdt(value));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_SameName(x: i32) -> i32 {
    unsafe { ::type_aliases_golden::test_same_name_type_alias_and_function::SameName(x) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Secondary() -> i16 {
    unsafe { ::type_aliases_golden::test_same_name_type_alias_and_function::Secondary() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Secondary_u() -> i16 {
    unsafe { ::type_aliases_golden::test_same_name_type_alias_and_function::Secondary_() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Shared() -> u16 {
    unsafe { ::type_aliases_golden::test_same_name_type_alias_and_function::Shared() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Sibling() -> i32 {
    unsafe { ::type_aliases_golden::test_same_name_type_alias_and_function::Sibling() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Trailing_u(x: i64) -> i64 {
    unsafe { ::type_aliases_golden::test_same_name_type_alias_and_function::Trailing_(x) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_WithSubmod() -> i32 {
    unsafe { ::type_aliases_golden::test_same_name_type_alias_and_function::WithSubmod() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_inside() -> i32 {
    unsafe { ::type_aliases_golden::test_same_name_type_alias_and_function::WithSubmod_::inside() }
}
const _: () = assert!(
    ::core::mem::size_of::<::type_aliases_golden::test_same_name_type_alias_and_function::Wrapped>(
    ) == 4
);
const _: () = assert!(
    ::core::mem::align_of::<::type_aliases_golden::test_same_name_type_alias_and_function::Wrapped>(
    ) == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(
        ::type_aliases_golden::test_same_name_type_alias_and_function::Wrapped,
        value
    ) == 0
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_func_uusing_ualias() -> i32 {
    unsafe { ::type_aliases_golden::test_type_aliases::func_using_alias() }
}
