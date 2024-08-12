// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/function:example_lib
// Features: supported

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

// Crubit only supports extern C functions right now. As a consequence, the
// functions need a unique name. (Including the namespace name in the symbol,
// e.g., `gshoe`, below, is one approach to this.)

/// Generated from: examples/cpp/function/example.h;l=14
#[inline(always)]
pub fn gshoe_add_two_integers(x: i32, y: i32) -> i32 {
    unsafe { crate::detail::__rust_thunk__gshoe_add_two_integers(x, y) }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk__gshoe_add_two_integers(x: i32, y: i32) -> i32;
    }
}
