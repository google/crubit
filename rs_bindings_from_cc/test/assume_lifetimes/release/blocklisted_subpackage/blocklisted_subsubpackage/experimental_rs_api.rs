// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes/release/blocklisted_subpackage/blocklisted_subsubpackage:experimental

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(rust_2024_compatibility)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

#[inline(always)]
pub fn f<'a>(a: &'a mut ::ffi_11::c_int) -> ::cref::CMut<'a, ::ffi_11::c_int> {
    unsafe { crate::detail::__rust_thunk___Z1fRi(a) }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z1fRi"]
        pub(crate) unsafe fn __rust_thunk___Z1fRi<'a>(
            a: &'a mut ::ffi_11::c_int,
        ) -> ::cref::CMut<'a, ::ffi_11::c_int>;
    }
}
