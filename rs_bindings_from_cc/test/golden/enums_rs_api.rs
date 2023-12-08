// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:enums_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct Color(::core::ffi::c_uint);
impl !Send for Color {}
impl !Sync for Color {}
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

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct Empty(::core::ffi::c_uint);
impl !Send for Empty {}
impl !Sync for Empty {}
impl Empty {}
impl From<::core::ffi::c_uint> for Empty {
    fn from(value: ::core::ffi::c_uint) -> Empty {
        Empty(value)
    }
}
impl From<Empty> for ::core::ffi::c_uint {
    fn from(value: Empty) -> ::core::ffi::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct EmptyBool(bool);
impl !Send for EmptyBool {}
impl !Sync for EmptyBool {}
impl EmptyBool {}
impl From<bool> for EmptyBool {
    fn from(value: bool) -> EmptyBool {
        EmptyBool(value)
    }
}
impl From<EmptyBool> for bool {
    fn from(value: EmptyBool) -> bool {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct EmptyInt(::core::ffi::c_uint);
impl !Send for EmptyInt {}
impl !Sync for EmptyInt {}
impl EmptyInt {}
impl From<::core::ffi::c_uint> for EmptyInt {
    fn from(value: ::core::ffi::c_uint) -> EmptyInt {
        EmptyInt(value)
    }
}
impl From<EmptyInt> for ::core::ffi::c_uint {
    fn from(value: EmptyInt) -> ::core::ffi::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct EmptyChar(u8);
impl !Send for EmptyChar {}
impl !Sync for EmptyChar {}
impl EmptyChar {}
impl From<u8> for EmptyChar {
    fn from(value: u8) -> EmptyChar {
        EmptyChar(value)
    }
}
impl From<EmptyChar> for u8 {
    fn from(value: EmptyChar) -> u8 {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct NonEmptyBool(bool);
impl !Send for NonEmptyBool {}
impl !Sync for NonEmptyBool {}
impl NonEmptyBool {
    pub const kBool1: NonEmptyBool = NonEmptyBool(false);
    pub const kBool2: NonEmptyBool = NonEmptyBool(true);
}
impl From<bool> for NonEmptyBool {
    fn from(value: bool) -> NonEmptyBool {
        NonEmptyBool(value)
    }
}
impl From<NonEmptyBool> for bool {
    fn from(value: NonEmptyBool) -> bool {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct NonEmptyInt(::core::ffi::c_uint);
impl !Send for NonEmptyInt {}
impl !Sync for NonEmptyInt {}
impl NonEmptyInt {
    pub const kInt1: NonEmptyInt = NonEmptyInt(0);
    pub const kInt2: NonEmptyInt = NonEmptyInt(4294967295);
}
impl From<::core::ffi::c_uint> for NonEmptyInt {
    fn from(value: ::core::ffi::c_uint) -> NonEmptyInt {
        NonEmptyInt(value)
    }
}
impl From<NonEmptyInt> for ::core::ffi::c_uint {
    fn from(value: NonEmptyInt) -> ::core::ffi::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct NonEmptyChar(u8);
impl !Send for NonEmptyChar {}
impl !Sync for NonEmptyChar {}
impl NonEmptyChar {
    pub const kChar1: NonEmptyChar = NonEmptyChar(0);
    pub const kChar2: NonEmptyChar = NonEmptyChar(97);
}
impl From<u8> for NonEmptyChar {
    fn from(value: u8) -> NonEmptyChar {
        NonEmptyChar(value)
    }
}
impl From<NonEmptyChar> for u8 {
    fn from(value: NonEmptyChar) -> u8 {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct EmptyClass(::core::ffi::c_int);
impl !Send for EmptyClass {}
impl !Sync for EmptyClass {}
impl EmptyClass {}
impl From<::core::ffi::c_int> for EmptyClass {
    fn from(value: ::core::ffi::c_int) -> EmptyClass {
        EmptyClass(value)
    }
}
impl From<EmptyClass> for ::core::ffi::c_int {
    fn from(value: EmptyClass) -> ::core::ffi::c_int {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct EmptyBoolClass(bool);
impl !Send for EmptyBoolClass {}
impl !Sync for EmptyBoolClass {}
impl EmptyBoolClass {}
impl From<bool> for EmptyBoolClass {
    fn from(value: bool) -> EmptyBoolClass {
        EmptyBoolClass(value)
    }
}
impl From<EmptyBoolClass> for bool {
    fn from(value: EmptyBoolClass) -> bool {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct EmptyIntClass(::core::ffi::c_int);
impl !Send for EmptyIntClass {}
impl !Sync for EmptyIntClass {}
impl EmptyIntClass {}
impl From<::core::ffi::c_int> for EmptyIntClass {
    fn from(value: ::core::ffi::c_int) -> EmptyIntClass {
        EmptyIntClass(value)
    }
}
impl From<EmptyIntClass> for ::core::ffi::c_int {
    fn from(value: EmptyIntClass) -> ::core::ffi::c_int {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct EmptyCharClass(u8);
impl !Send for EmptyCharClass {}
impl !Sync for EmptyCharClass {}
impl EmptyCharClass {}
impl From<u8> for EmptyCharClass {
    fn from(value: u8) -> EmptyCharClass {
        EmptyCharClass(value)
    }
}
impl From<EmptyCharClass> for u8 {
    fn from(value: EmptyCharClass) -> u8 {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct NonEmptyBoolClass(bool);
impl !Send for NonEmptyBoolClass {}
impl !Sync for NonEmptyBoolClass {}
impl NonEmptyBoolClass {
    pub const k1: NonEmptyBoolClass = NonEmptyBoolClass(false);
    pub const k2: NonEmptyBoolClass = NonEmptyBoolClass(true);
}
impl From<bool> for NonEmptyBoolClass {
    fn from(value: bool) -> NonEmptyBoolClass {
        NonEmptyBoolClass(value)
    }
}
impl From<NonEmptyBoolClass> for bool {
    fn from(value: NonEmptyBoolClass) -> bool {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct NonEmptyIntClass(::core::ffi::c_uint);
impl !Send for NonEmptyIntClass {}
impl !Sync for NonEmptyIntClass {}
impl NonEmptyIntClass {
    pub const k1: NonEmptyIntClass = NonEmptyIntClass(0);
    pub const k2: NonEmptyIntClass = NonEmptyIntClass(4294967295);
}
impl From<::core::ffi::c_uint> for NonEmptyIntClass {
    fn from(value: ::core::ffi::c_uint) -> NonEmptyIntClass {
        NonEmptyIntClass(value)
    }
}
impl From<NonEmptyIntClass> for ::core::ffi::c_uint {
    fn from(value: NonEmptyIntClass) -> ::core::ffi::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct NonEmptyCharClass(u8);
impl !Send for NonEmptyCharClass {}
impl !Sync for NonEmptyCharClass {}
impl NonEmptyCharClass {
    pub const k1: NonEmptyCharClass = NonEmptyCharClass(0);
    pub const k2: NonEmptyCharClass = NonEmptyCharClass(97);
}
impl From<u8> for NonEmptyCharClass {
    fn from(value: u8) -> NonEmptyCharClass {
        NonEmptyCharClass(value)
    }
}
impl From<NonEmptyCharClass> for u8 {
    fn from(value: NonEmptyCharClass) -> u8 {
        value.0
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ENUMS_H_

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());
