#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;

pub type __builtin_ms_va_list = *mut u8;

/// The same as Derived from inheritance.h, but in a different build target.
///
/// This tests inheritance across library boundaries.
///
/// TODO(b/216195042): Correctly namespace base classes in generated Rust code.
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct Derived2 {
    __base_class_subobjects: [std::mem::MaybeUninit<u8>; 12],
    pub derived_1: u8,
}
impl<'a> From<&'a Derived2> for &'a Base0 {
    fn from(x: &'a Derived2) -> Self {
        unsafe { &*((x as *const _ as *const u8).offset(0) as *const Base0) }
    }
}
impl<'a> From<&'a Derived2> for &'a Base1 {
    fn from(x: &'a Derived2) -> Self {
        unsafe { &*((x as *const _ as *const u8).offset(0) as *const Base1) }
    }
}
impl<'a> From<&'a Derived2> for &'a Base2 {
    fn from(x: &'a Derived2) -> Self {
        unsafe { &*((x as *const _ as *const u8).offset(10) as *const Base2) }
    }
}

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=11
// Error while generating bindings for item 'Derived2::Derived2':
// Parameter type 'struct Derived2 &&' is not supported

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=11
// Error while generating bindings for item 'Derived2::operator=':
// Parameter type 'struct Derived2 &&' is not supported

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_BASE_CLASS_H_

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<Derived2>() == 16usize);
const _: () = assert!(std::mem::align_of::<Derived2>() == 8usize);
const _: () = assert!(offset_of!(Derived2, derived_1) * 8 == 96usize);
