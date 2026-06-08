// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/namespace/reexported_namespaces:reexported_namespaces

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(rust_2024_compatibility)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

pub use crate::absl::*;
pub use crate::base::*;
pub mod absl {
    #[inline(always)]
    pub fn MyAbslFunction() {
        unsafe { crate::detail::__rust_thunk___ZN4absl14MyAbslFunctionEv() }
    }
}

// namespace absl

pub mod base {
    #[inline(always)]
    pub fn MyBaseFunction() {
        unsafe { crate::detail::__rust_thunk___ZN4base14MyBaseFunctionEv() }
    }
}

// namespace base

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN4absl14MyAbslFunctionEv();
        pub(crate) unsafe fn __rust_thunk___ZN4base14MyBaseFunctionEv();
    }
}
