// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:free_function
// Features: assume_lifetimes, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/free_function.h;l=8
#[inline(always)]
pub fn increment_int_ref<'a>(a: &'a mut ::ffi_11::c_int) -> &'a mut ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z17increment_int_refRi(a) }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z17increment_int_refRi"]
        pub(crate) unsafe fn __rust_thunk___Z17increment_int_refRi<'a>(
            a: &'a mut ::ffi_11::c_int,
        ) -> &'a mut ::ffi_11::c_int;
    }
}
