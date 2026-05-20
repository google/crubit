// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:crefs
// Features: assume_lifetimes, assume_this_lifetimes, callables, check_default_initialized, experimental, leading_colons_for_cpp_type, supported, template_instantiation, types, unsafe_view, wrapper

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/crefs.h;l=8
#[inline(always)]
pub fn id_cmut<'x>(x: &'x mut ::ffi_11::c_int) -> ::cref::CMut<'x, ::ffi_11::c_int> {
    unsafe { crate::detail::__rust_thunk___Z7id_cmutRi(x) }
}

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/crefs.h;l=9
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
