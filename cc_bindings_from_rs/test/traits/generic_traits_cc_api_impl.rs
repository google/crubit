// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// generic_traits_golden
// Features: assume_lifetimes, assume_this_lifetimes, callables, check_default_initialized, experimental, fmt, supported, types, unsafe_view, wrapper

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::generic_traits_golden::AnotherStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::generic_traits_golden::AnotherStruct>() == 4);
const _: () = assert!(::core::mem::offset_of!(::generic_traits_golden::AnotherStruct, y) == 0);
const _: () = assert!(::std::mem::size_of::<::generic_traits_golden::StructGeneric>() == 4);
const _: () = assert!(::std::mem::align_of::<::generic_traits_golden::StructGeneric>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::generic_traits_golden::StructGeneric::new(x);
        (__ret_ptr as *mut ::generic_traits_golden::StructGeneric).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::generic_traits_golden::StructGeneric, x) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_TraitWithGeneric_ufoo(
    __self: &'static ::generic_traits_golden::StructGeneric,
    t: i32,
) -> i32 {
    unsafe {
        <::generic_traits_golden::StructGeneric as::generic_traits_golden::TraitWithGeneric<i32>>::foo(__self,t)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_TraitWithTwoGenerics_ubar(
    __self: &'static ::generic_traits_golden::StructGeneric,
    t: i32,
    u: i32,
) -> i32 {
    unsafe {
        <::generic_traits_golden::StructGeneric as ::generic_traits_golden::TraitWithTwoGenerics<
            i32,
            i32,
        >>::bar(__self, t, u)
    }
}
