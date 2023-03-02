// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:enums_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct Empty(u32);
impl Empty {}
impl From<u32> for Empty {
    fn from(value: u32) -> Empty {
        Empty(value)
    }
}
impl From<Empty> for u32 {
    fn from(value: Empty) -> u32 {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
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
pub struct EmptyInt(u32);
impl EmptyInt {}
impl From<u32> for EmptyInt {
    fn from(value: u32) -> EmptyInt {
        EmptyInt(value)
    }
}
impl From<EmptyInt> for u32 {
    fn from(value: EmptyInt) -> u32 {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct EmptyChar(u8);
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
pub struct NonEmptyInt(u32);
impl NonEmptyInt {
    pub const kInt1: NonEmptyInt = NonEmptyInt(0);
    pub const kInt2: NonEmptyInt = NonEmptyInt(4294967295);
}
impl From<u32> for NonEmptyInt {
    fn from(value: u32) -> NonEmptyInt {
        NonEmptyInt(value)
    }
}
impl From<NonEmptyInt> for u32 {
    fn from(value: NonEmptyInt) -> u32 {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct NonEmptyChar(u8);
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
pub struct EmptyClass(i32);
impl EmptyClass {}
impl From<i32> for EmptyClass {
    fn from(value: i32) -> EmptyClass {
        EmptyClass(value)
    }
}
impl From<EmptyClass> for i32 {
    fn from(value: EmptyClass) -> i32 {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
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
pub struct EmptyIntClass(i32);
impl EmptyIntClass {}
impl From<i32> for EmptyIntClass {
    fn from(value: i32) -> EmptyIntClass {
        EmptyIntClass(value)
    }
}
impl From<EmptyIntClass> for i32 {
    fn from(value: EmptyIntClass) -> i32 {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct EmptyCharClass(u8);
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
pub struct NonEmptyIntClass(u32);
impl NonEmptyIntClass {
    pub const k1: NonEmptyIntClass = NonEmptyIntClass(0);
    pub const k2: NonEmptyIntClass = NonEmptyIntClass(4294967295);
}
impl From<u32> for NonEmptyIntClass {
    fn from(value: u32) -> NonEmptyIntClass {
        NonEmptyIntClass(value)
    }
}
impl From<NonEmptyIntClass> for u32 {
    fn from(value: NonEmptyIntClass) -> u32 {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct NonEmptyCharClass(u8);
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
