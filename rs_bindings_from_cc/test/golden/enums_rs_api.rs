// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:enums_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, register_tool)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=Color
pub struct Color(::ffi_11::c_uint);
impl Color {
    pub const kRed: Color = Color(0);
    pub const kBlue: Color = Color(1);
    pub const kGreen: Color = Color(2);
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

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=EnumToRename
pub struct RenamedEnum(::ffi_11::c_uint);
impl RenamedEnum {}
impl From<::ffi_11::c_uint> for RenamedEnum {
    fn from(value: ::ffi_11::c_uint) -> RenamedEnum {
        RenamedEnum(value)
    }
}
impl From<RenamedEnum> for ::ffi_11::c_uint {
    fn from(value: RenamedEnum) -> ::ffi_11::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=EnumWithRenamedVariants
pub struct EnumWithRenamedVariants(::ffi_11::c_uint);
impl EnumWithRenamedVariants {
    pub const FOO: EnumWithRenamedVariants = EnumWithRenamedVariants(0);
    pub const BAR: EnumWithRenamedVariants = EnumWithRenamedVariants(1);
}
impl From<::ffi_11::c_uint> for EnumWithRenamedVariants {
    fn from(value: ::ffi_11::c_uint) -> EnumWithRenamedVariants {
        EnumWithRenamedVariants(value)
    }
}
impl From<EnumWithRenamedVariants> for ::ffi_11::c_uint {
    fn from(value: EnumWithRenamedVariants) -> ::ffi_11::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=Empty
pub struct Empty(::ffi_11::c_uint);
impl Empty {}
impl From<::ffi_11::c_uint> for Empty {
    fn from(value: ::ffi_11::c_uint) -> Empty {
        Empty(value)
    }
}
impl From<Empty> for ::ffi_11::c_uint {
    fn from(value: Empty) -> ::ffi_11::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=EmptyBool
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
///CRUBIT_ANNOTATE: cpp_type=EmptyInt
pub struct EmptyInt(::ffi_11::c_uint);
impl EmptyInt {}
impl From<::ffi_11::c_uint> for EmptyInt {
    fn from(value: ::ffi_11::c_uint) -> EmptyInt {
        EmptyInt(value)
    }
}
impl From<EmptyInt> for ::ffi_11::c_uint {
    fn from(value: EmptyInt) -> ::ffi_11::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=EmptyChar
pub struct EmptyChar(::ffi_11::c_char);
impl EmptyChar {}
impl From<::ffi_11::c_char> for EmptyChar {
    fn from(value: ::ffi_11::c_char) -> EmptyChar {
        EmptyChar(value)
    }
}
impl From<EmptyChar> for ::ffi_11::c_char {
    fn from(value: EmptyChar) -> ::ffi_11::c_char {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=NonEmptyBool
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
///CRUBIT_ANNOTATE: cpp_type=NonEmptyInt
pub struct NonEmptyInt(::ffi_11::c_uint);
impl NonEmptyInt {
    pub const kInt1: NonEmptyInt = NonEmptyInt(0);
    pub const kInt2: NonEmptyInt = NonEmptyInt(4294967295);
}
impl From<::ffi_11::c_uint> for NonEmptyInt {
    fn from(value: ::ffi_11::c_uint) -> NonEmptyInt {
        NonEmptyInt(value)
    }
}
impl From<NonEmptyInt> for ::ffi_11::c_uint {
    fn from(value: NonEmptyInt) -> ::ffi_11::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=NonEmptyChar
pub struct NonEmptyChar(::ffi_11::c_char);
impl NonEmptyChar {
    pub const kChar1: NonEmptyChar = NonEmptyChar(ffi_11::c_char::new(0 as u8));
    pub const kChar2: NonEmptyChar = NonEmptyChar(ffi_11::c_char::new(97 as u8));
}
impl From<::ffi_11::c_char> for NonEmptyChar {
    fn from(value: ::ffi_11::c_char) -> NonEmptyChar {
        NonEmptyChar(value)
    }
}
impl From<NonEmptyChar> for ::ffi_11::c_char {
    fn from(value: NonEmptyChar) -> ::ffi_11::c_char {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=EmptyClass
pub struct EmptyClass(::ffi_11::c_int);
impl EmptyClass {}
impl From<::ffi_11::c_int> for EmptyClass {
    fn from(value: ::ffi_11::c_int) -> EmptyClass {
        EmptyClass(value)
    }
}
impl From<EmptyClass> for ::ffi_11::c_int {
    fn from(value: EmptyClass) -> ::ffi_11::c_int {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=EmptyBoolClass
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
///CRUBIT_ANNOTATE: cpp_type=EmptyIntClass
pub struct EmptyIntClass(::ffi_11::c_int);
impl EmptyIntClass {}
impl From<::ffi_11::c_int> for EmptyIntClass {
    fn from(value: ::ffi_11::c_int) -> EmptyIntClass {
        EmptyIntClass(value)
    }
}
impl From<EmptyIntClass> for ::ffi_11::c_int {
    fn from(value: EmptyIntClass) -> ::ffi_11::c_int {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=EmptyCharClass
pub struct EmptyCharClass(::ffi_11::c_char);
impl EmptyCharClass {}
impl From<::ffi_11::c_char> for EmptyCharClass {
    fn from(value: ::ffi_11::c_char) -> EmptyCharClass {
        EmptyCharClass(value)
    }
}
impl From<EmptyCharClass> for ::ffi_11::c_char {
    fn from(value: EmptyCharClass) -> ::ffi_11::c_char {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=NonEmptyBoolClass
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
///CRUBIT_ANNOTATE: cpp_type=NonEmptyIntClass
pub struct NonEmptyIntClass(::ffi_11::c_uint);
impl NonEmptyIntClass {
    pub const k1: NonEmptyIntClass = NonEmptyIntClass(0);
    pub const k2: NonEmptyIntClass = NonEmptyIntClass(4294967295);
}
impl From<::ffi_11::c_uint> for NonEmptyIntClass {
    fn from(value: ::ffi_11::c_uint) -> NonEmptyIntClass {
        NonEmptyIntClass(value)
    }
}
impl From<NonEmptyIntClass> for ::ffi_11::c_uint {
    fn from(value: NonEmptyIntClass) -> ::ffi_11::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=NonEmptyCharClass
pub struct NonEmptyCharClass(::ffi_11::c_char);
impl NonEmptyCharClass {
    pub const k1: NonEmptyCharClass = NonEmptyCharClass(ffi_11::c_char::new(0 as u8));
    pub const k2: NonEmptyCharClass = NonEmptyCharClass(ffi_11::c_char::new(97 as u8));
}
impl From<::ffi_11::c_char> for NonEmptyCharClass {
    fn from(value: ::ffi_11::c_char) -> NonEmptyCharClass {
        NonEmptyCharClass(value)
    }
}
impl From<NonEmptyCharClass> for ::ffi_11::c_char {
    fn from(value: NonEmptyCharClass) -> ::ffi_11::c_char {
        value.0
    }
}

// Error while generating bindings for enum 'ForwardDeclared':
// Can't generate bindings for ForwardDeclared, because it is unsupported: b/322391132: Forward-declared (opaque) enums are not implemented yet

// Error while generating bindings for function 'do_not_generate_bindings_for_me':
// Cannot use an error type by value: Can't generate bindings for ForwardDeclared, because it is unsupported
