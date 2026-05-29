// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:crefs

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
pub fn id_cmut<'x>(x: &'x mut ::ffi_11::c_int) -> ::cref::CMut<'x, ::ffi_11::c_int> {
    unsafe { crate::detail::__rust_thunk___Z7id_cmutRi(x) }
}

#[inline(always)]
pub fn id_cref<'x>(x: &'x ::ffi_11::c_int) -> ::cref::CRef<'x, ::ffi_11::c_int> {
    unsafe { crate::detail::__rust_thunk___Z7id_crefRKi(x) }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z7id_cmutRi"]
        pub(crate) unsafe fn __rust_thunk___Z7id_cmutRi<'x>(
            x: &'x mut ::ffi_11::c_int,
        ) -> ::cref::CMut<'x, ::ffi_11::c_int>;
        #[link_name = "_Z7id_crefRKi"]
        pub(crate) unsafe fn __rust_thunk___Z7id_crefRKi<'x>(
            x: &'x ::ffi_11::c_int,
        ) -> ::cref::CRef<'x, ::ffi_11::c_int>;
    }
}
