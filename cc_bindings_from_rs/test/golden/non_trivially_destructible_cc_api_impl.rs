// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// non_trivially_destructible_rust_golden
// Features: callables, custom_ffi_types, experimental, fmt, non_unpin_ctor, std_unique_ptr, std_vector, supported, wrapper

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(
    ::std::mem::size_of::<::non_trivially_destructible_rust_golden::NonTriviallyDestructable>()
        == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::non_trivially_destructible_rust_golden::NonTriviallyDestructable>()
        == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value=<::non_trivially_destructible_rust_golden::NonTriviallyDestructable as::core::default::Default>::default();
        (__ret_ptr as *mut ::non_trivially_destructible_rust_golden::NonTriviallyDestructable)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::non_trivially_destructible_rust_golden::NonTriviallyDestructable,
    >,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::non_trivially_destructible_rust_golden::NonTriviallyDestructable,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<::non_trivially_destructible_rust_golden::NonTriviallyDestructable as::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::non_trivially_destructible_rust_golden::NonTriviallyDestructable)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::non_trivially_destructible_rust_golden::NonTriviallyDestructable,
    source: &'static ::non_trivially_destructible_rust_golden::NonTriviallyDestructable,
) -> () {
    unsafe {
        <::non_trivially_destructible_rust_golden::NonTriviallyDestructable as::core::clone::Clone>::clone_from(__self,source)
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::non_trivially_destructible_rust_golden::NonTriviallyDestructable,
        field
    ) == 0
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uby_uvalue(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::non_trivially_destructible_rust_golden::return_by_value();
        (__ret_ptr as *mut ::non_trivially_destructible_rust_golden::NonTriviallyDestructable)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_uby_uvalue(
    _x: &'static mut ::core::mem::MaybeUninit<
        ::non_trivially_destructible_rust_golden::NonTriviallyDestructable,
    >,
) -> () {
    unsafe {
        let _x = _x.assume_init_read();
        ::non_trivially_destructible_rust_golden::take_by_value(_x)
    }
}
