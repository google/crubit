// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/enum:example_lib
// Features: supported

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=Color
pub struct Color(::ffi_11::c_uint);
impl Color {
    pub const kRed: Color = Color(::ffi_11::new_c_uint(0));
    pub const kBlue: Color = Color(::ffi_11::new_c_uint(1));
    pub const kGreen: Color = Color(::ffi_11::new_c_uint(2));
}
impl From<::ffi_11::c_uint> for Color {
    fn from(value: ::ffi_11::c_uint) -> Color {
        Color(value)
    }
}
impl From<Color> for ::ffi_11::c_uint {
    fn from(value: Color) -> ::ffi_11::c_uint {
        value.0
    }
}
