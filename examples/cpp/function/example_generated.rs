// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/function:example_lib
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod gshoe {
    /// Generated from: examples/cpp/function/example.h;l=12
    #[inline(always)]
    pub fn add_two_integers(x: i32, y: i32) -> i32 {
        unsafe { crate::detail::__rust_thunk___ZN5gshoe16add_two_integersEii(x, y) }
    }
}

// namespace gshoe

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN5gshoe16add_two_integersEii(x: i32, y: i32) -> i32;
    }
}
