// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/enum:example_lib
// Features: supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(warnings)]

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[__crubit::annotate(cpp_type = "Color")]
pub struct Color(::core::ffi::c_uint);
impl Color {
    pub const kRed: Color = Color(0);
    pub const kBlue: Color = Color(1);
    pub const kGreen: Color = Color(2);
}
impl From<::core::ffi::c_uint> for Color {
    fn from(value: ::core::ffi::c_uint) -> Color {
        Color(value)
    }
}
impl From<Color> for ::core::ffi::c_uint {
    fn from(value: Color) -> ::core::ffi::c_uint {
        value.0
    }
}
