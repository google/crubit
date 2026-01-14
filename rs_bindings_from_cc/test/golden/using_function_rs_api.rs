// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:using_function_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod A {
    #[inline(always)]
    pub fn Foo() {
        unsafe { crate::detail::__rust_thunk___ZN1A3FooEv() }
    }
}

// namespace A

pub mod B { // Error while generating bindings for item 'B::Foo':
            // Function aliases are not yet supported.
}

// namespace B

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_ZN1A3FooEv"]
        pub(crate) unsafe fn __rust_thunk___ZN1A3FooEv();
    }
}
