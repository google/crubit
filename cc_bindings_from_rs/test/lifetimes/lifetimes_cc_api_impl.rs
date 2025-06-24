// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// lifetimes_golden
// Features: supported, unsafe_types

#![allow(unused_unsafe)]
#![allow(improper_ctypes_definitions)]

const _: () = assert!(::std::mem::size_of::<::lifetimes_golden::StructWithLifetime>() == 8);
const _: () = assert!(::std::mem::align_of::<::lifetimes_golden::StructWithLifetime>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_value(
    __self: &'static mut ::core::mem::MaybeUninit<::lifetimes_golden::StructWithLifetime>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::lifetimes_golden::StructWithLifetime::value(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_ustatic_u42(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::lifetimes_golden::StructWithLifetime::make_static_42();
        (__ret_ptr as *mut ::lifetimes_golden::StructWithLifetime).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ustatic_uref_uwhere_ubound(
    field_with_lifetime: &'static i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::lifetimes_golden::StructWithLifetime::from_static_ref_where_bound(
            field_with_lifetime,
        );
        (__ret_ptr as *mut ::lifetimes_golden::StructWithLifetime).write(__rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(::lifetimes_golden::StructWithLifetime, field_with_lifetime) == 0
);
