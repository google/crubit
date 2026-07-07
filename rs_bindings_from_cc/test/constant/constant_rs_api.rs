// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/constant:constant

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[cfi_encoding = "3Foo"]
///CRUBIT_ANNOTATE: cpp_type=Foo
pub struct Foo(::ffi_11::c_uint);
impl Foo {
    pub const BAR: Foo = Foo(::ffi_11::new_c_uint(0));
}
impl From<::ffi_11::c_uint> for Foo {
    fn from(value: ::ffi_11::c_uint) -> Foo {
        Foo(value)
    }
}
impl From<Foo> for ::ffi_11::c_uint {
    fn from(value: Foo) -> ::ffi_11::c_uint {
        value.0
    }
}

// integer_constant_to_token_stream called with non-primitive underlying type:
//   crate::Foo
