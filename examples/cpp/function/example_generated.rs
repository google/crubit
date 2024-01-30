// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/function:example_lib
// Features: extern_c

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(warnings)]

/// Generated from: examples/cpp/function/example.h;l=11
#[inline(always)]
pub fn crubit_add_two_integers(x: i32, y: i32) -> i32 {
    unsafe { crate::detail::__rust_thunk___Z23crubit_add_two_integers(x, y) }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___Z23crubit_add_two_integers(x: i32, y: i32) -> i32;
    }
}
