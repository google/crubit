// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:enums_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
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

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[__crubit::annotate(cpp_type = "EnumToRename")]
pub struct RenamedEnum(::core::ffi::c_uint);
impl RenamedEnum {}
impl From<::core::ffi::c_uint> for RenamedEnum {
    fn from(value: ::core::ffi::c_uint) -> RenamedEnum {
        RenamedEnum(value)
    }
}
impl From<RenamedEnum> for ::core::ffi::c_uint {
    fn from(value: RenamedEnum) -> ::core::ffi::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[__crubit::annotate(cpp_type = "Empty")]
pub struct Empty(::core::ffi::c_uint);
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
#[__crubit::annotate(cpp_type = "EmptyBool")]
pub struct EmptyBool(bool);
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
#[__crubit::annotate(cpp_type = "EmptyInt")]
pub struct EmptyInt(::core::ffi::c_uint);
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
#[__crubit::annotate(cpp_type = "EmptyChar")]
pub struct EmptyChar(::core::ffi::c_char);
impl EmptyChar {}
impl From<::core::ffi::c_char> for EmptyChar {
    fn from(value: ::core::ffi::c_char) -> EmptyChar {
        EmptyChar(value)
    }
}
impl From<EmptyChar> for ::core::ffi::c_char {
    fn from(value: EmptyChar) -> ::core::ffi::c_char {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[__crubit::annotate(cpp_type = "NonEmptyBool")]
pub struct NonEmptyBool(bool);
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
#[__crubit::annotate(cpp_type = "NonEmptyInt")]
pub struct NonEmptyInt(::core::ffi::c_uint);
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
#[__crubit::annotate(cpp_type = "NonEmptyChar")]
pub struct NonEmptyChar(::core::ffi::c_char);
impl NonEmptyChar {
    pub const kChar1: NonEmptyChar = NonEmptyChar(0);
    pub const kChar2: NonEmptyChar = NonEmptyChar(97);
}
impl From<::core::ffi::c_char> for NonEmptyChar {
    fn from(value: ::core::ffi::c_char) -> NonEmptyChar {
        NonEmptyChar(value)
    }
}
impl From<NonEmptyChar> for ::core::ffi::c_char {
    fn from(value: NonEmptyChar) -> ::core::ffi::c_char {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[__crubit::annotate(cpp_type = "EmptyClass")]
pub struct EmptyClass(::core::ffi::c_int);
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
#[__crubit::annotate(cpp_type = "EmptyBoolClass")]
pub struct EmptyBoolClass(bool);
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
#[__crubit::annotate(cpp_type = "EmptyIntClass")]
pub struct EmptyIntClass(::core::ffi::c_int);
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
#[__crubit::annotate(cpp_type = "EmptyCharClass")]
pub struct EmptyCharClass(::core::ffi::c_char);
impl EmptyCharClass {}
impl From<::core::ffi::c_char> for EmptyCharClass {
    fn from(value: ::core::ffi::c_char) -> EmptyCharClass {
        EmptyCharClass(value)
    }
}
impl From<EmptyCharClass> for ::core::ffi::c_char {
    fn from(value: EmptyCharClass) -> ::core::ffi::c_char {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[__crubit::annotate(cpp_type = "NonEmptyBoolClass")]
pub struct NonEmptyBoolClass(bool);
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
#[__crubit::annotate(cpp_type = "NonEmptyIntClass")]
pub struct NonEmptyIntClass(::core::ffi::c_uint);
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
#[__crubit::annotate(cpp_type = "NonEmptyCharClass")]
pub struct NonEmptyCharClass(::core::ffi::c_char);
impl NonEmptyCharClass {
    pub const k1: NonEmptyCharClass = NonEmptyCharClass(0);
    pub const k2: NonEmptyCharClass = NonEmptyCharClass(97);
}
impl From<::core::ffi::c_char> for NonEmptyCharClass {
    fn from(value: ::core::ffi::c_char) -> NonEmptyCharClass {
        NonEmptyCharClass(value)
    }
}
impl From<NonEmptyCharClass> for ::core::ffi::c_char {
    fn from(value: NonEmptyCharClass) -> ::core::ffi::c_char {
        value.0
    }
}

// Error while generating bindings for item 'ForwardDeclared':
// b/322391132: Forward-declared (opaque) enums are not supported yet: ForwardDeclared

// Error while generating bindings for item 'do_not_generate_bindings_for_me':
// Failed to format return type: Can't generate bindings for ForwardDeclared due to missing bindings for its dependency: b/322391132: Forward-declared (opaque) enums are not supported yet: ForwardDeclared
