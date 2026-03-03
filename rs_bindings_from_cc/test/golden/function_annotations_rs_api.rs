// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:function_annotations_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut, unused_unsafe)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/golden/function_annotations.h;l=8[355,377]
#[inline(always)]
pub fn function_returning_int() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z22function_returning_intv() }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z22function_returning_intv"]
        pub(crate) unsafe fn __rust_thunk___Z22function_returning_intv() -> ::ffi_11::c_int;
    }
}
