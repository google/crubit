// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:noescape_pointers_feature_off_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `p`: raw pointer
#[inline(always)]
pub unsafe fn takes_noescape_nonnull_feature_off(p: *mut ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z34takes_noescape_nonnull_feature_offPi(p) }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z34takes_noescape_nonnull_feature_offPi"]
        pub(crate) unsafe fn __rust_thunk___Z34takes_noescape_nonnull_feature_offPi(
            p: *mut ::ffi_11::c_int,
        );
    }
}
