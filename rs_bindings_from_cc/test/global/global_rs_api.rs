// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/global:global
// Features: supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

extern "C" {
    pub static mut extern_int: ::ffi_11::c_int;
}

extern "C" {
    pub static kExternConstInt: ::ffi_11::c_int;
}

// Check that duplicate extern declarations are handled correctly.

// namespace foo

pub const kInlineConstInt: ::ffi_11::c_int = ::ffi_11::new_c_int(6);

pub const kConstexprInt: ::ffi_11::c_int = ::ffi_11::new_c_int(7);

pub const inline_int: ::ffi_11::c_int = ::ffi_11::new_c_int(5);

pub mod foo {
    extern "C" {
        #[link_name = "_ZN3foo21extern_int_namespacedE"]
        pub static mut extern_int_namespaced: ::ffi_11::c_int;
    }

    extern "C" {
        pub static mut extern_c_int_namespaced: ::ffi_11::c_int;
    }

    pub const inline_int_namespaced: ::ffi_11::c_int = ::ffi_11::new_c_int(5);

    pub const inline_long_long_namespaced: ::ffi_11::c_longlong = ::ffi_11::new_c_longlong(24);

    pub const inline_bool_namespaced: bool = true;
}

// namespace foo

// Generated from: rs_bindings_from_cc/test/global/global.h;l=30
// Error while generating bindings for global variable 'templated_variable':
// templated variables are not supported

/// instantiate templated_variable<int>
///
/// Generated from: rs_bindings_from_cc/test/global/global.h;l=33
#[inline(always)]
pub fn Unused(arg: ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z6Unusedi(arg) }
}

/// Generated from: rs_bindings_from_cc/test/global/global.h;l=35
#[inline(always)]
pub fn GetIntVal() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z9GetIntValv() }
}

/// Generated from: rs_bindings_from_cc/test/global/global.h;l=36
#[inline(always)]
pub fn GetNamespacedIntVal() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z19GetNamespacedIntValv() }
}

/// Generated from: rs_bindings_from_cc/test/global/global.h;l=37
#[inline(always)]
pub fn GetCNamespacedIntVal() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z20GetCNamespacedIntValv() }
}

/// Generated from: rs_bindings_from_cc/test/global/global.h;l=38
#[inline(always)]
pub fn GetInlineIntVal() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z15GetInlineIntValv() }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z6Unusedi(arg: ::ffi_11::c_int);
        #[link_name = "_Z9GetIntValv"]
        pub(crate) unsafe fn __rust_thunk___Z9GetIntValv() -> ::ffi_11::c_int;
        #[link_name = "_Z19GetNamespacedIntValv"]
        pub(crate) unsafe fn __rust_thunk___Z19GetNamespacedIntValv() -> ::ffi_11::c_int;
        #[link_name = "_Z20GetCNamespacedIntValv"]
        pub(crate) unsafe fn __rust_thunk___Z20GetCNamespacedIntValv() -> ::ffi_11::c_int;
        #[link_name = "_Z15GetInlineIntValv"]
        pub(crate) unsafe fn __rust_thunk___Z15GetInlineIntValv() -> ::ffi_11::c_int;
    }
}
