#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;

pub type __builtin_ms_va_list = *mut u8;

/// Using classes to force these to be non-POD.
/// In the Itanium ABI, the tail padding of POD types cannot be reused by other
/// objects, even if the POD type is potentially-overlapping.
#[repr(C)]
pub struct Base0 {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: std::mem::MaybeUninit<u8>,
}

impl !Unpin for Base0 {}

// rs_bindings_from_cc/test/golden/inheritance.h;l=9
// Error while generating bindings for item 'Base0::Base0':
// Parameter type 'class Base0 &&' is not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=9
// Error while generating bindings for item 'Base0::operator=':
// Parameter type 'class Base0 &&' is not supported

#[repr(C)]
pub struct Base1 {
    b1_1_: i64,
    b1_2_: u8,
}

impl !Unpin for Base1 {}

// rs_bindings_from_cc/test/golden/inheritance.h;l=10
// Error while generating bindings for item 'Base1::Base1':
// Parameter type 'class Base1 &&' is not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=10
// Error while generating bindings for item 'Base1::operator=':
// Parameter type 'class Base1 &&' is not supported

#[repr(C)]
pub struct Base2 {
    b2_1_: i16,
}

impl !Unpin for Base2 {}

// rs_bindings_from_cc/test/golden/inheritance.h;l=15
// Error while generating bindings for item 'Base2::Base2':
// Parameter type 'class Base2 &&' is not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=15
// Error while generating bindings for item 'Base2::operator=':
// Parameter type 'class Base2 &&' is not supported

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct Derived {
    __base_class_subobjects: [std::mem::MaybeUninit<u8>; 12],
    pub derived_1: u8,
}
impl<'a> From<&'a Derived> for &'a Base0 {
    fn from(x: &'a Derived) -> Self {
        unsafe { &*((x as *const _ as *const u8).offset(0) as *const Base0) }
    }
}
impl<'a> From<&'a Derived> for &'a Base1 {
    fn from(x: &'a Derived) -> Self {
        unsafe { &*((x as *const _ as *const u8).offset(0) as *const Base1) }
    }
}
impl<'a> From<&'a Derived> for &'a Base2 {
    fn from(x: &'a Derived) -> Self {
        unsafe { &*((x as *const _ as *const u8).offset(10) as *const Base2) }
    }
}

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Derived::Derived':
// Parameter type 'struct Derived &&' is not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Derived::operator=':
// Parameter type 'struct Derived &&' is not supported

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITANCE_H_

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<Base0>() == 1usize);
const _: () = assert!(std::mem::align_of::<Base0>() == 1usize);

const _: () = assert!(std::mem::size_of::<Base1>() == 16usize);
const _: () = assert!(std::mem::align_of::<Base1>() == 8usize);
const _: () = assert!(offset_of!(Base1, b1_1_) * 8 == 0usize);
const _: () = assert!(offset_of!(Base1, b1_2_) * 8 == 64usize);

const _: () = assert!(std::mem::size_of::<Base2>() == 2usize);
const _: () = assert!(std::mem::align_of::<Base2>() == 2usize);
const _: () = assert!(offset_of!(Base2, b2_1_) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<Derived>() == 16usize);
const _: () = assert!(std::mem::align_of::<Derived>() == 8usize);
const _: () = assert!(offset_of!(Derived, derived_1) * 8 == 96usize);
