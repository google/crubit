// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:using_function_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod A {
    #[inline(always)]
    pub fn Foo() {
        unsafe { crate::detail::__rust_thunk__d6cf3509__ZN1A3FooEv() }
    }

    #[inline(always)]
    pub fn Add(a: ::ffi_11::c_int, b: ::ffi_11::c_int) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk__e4bd4550__ZN1A3AddEii(a, b) }
    }

    // error: function `A::Overloaded` could not be bound
    //   Cannot generate bindings for overloaded function

    // error: function `A::Overloaded` could not be bound
    //   Cannot generate bindings for overloaded function
}

// namespace A

pub mod B {
    #[inline(always)]
    pub fn Foo() {
        unsafe { crate::detail::__rust_thunk__2e00d1b3__ZN1A3FooEv() }
    }

    /// A doc comment on the using-declaration.
    #[inline(always)]
    pub fn Add(a: ::ffi_11::c_int, b: ::ffi_11::c_int) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk__beecbe97__ZN1A3AddEii(a, b) }
    }

    // error: function `B::Overloaded` could not be bound
    //   Cannot generate bindings for overloaded function

    // error: function `B::Overloaded` could not be bound
    //   Cannot generate bindings for overloaded function
}

// namespace B

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk__d6cf3509__ZN1A3FooEv();
        pub(crate) unsafe fn __rust_thunk__e4bd4550__ZN1A3AddEii(
            a: ::ffi_11::c_int,
            b: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__2e00d1b3__ZN1A3FooEv();
        pub(crate) unsafe fn __rust_thunk__beecbe97__ZN1A3AddEii(
            a: ::ffi_11::c_int,
            b: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
    }
}
