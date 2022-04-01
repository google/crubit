// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:inheritance_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;
use static_assertions::{assert_impl_all, assert_not_impl_all};
use std as rust_std;

pub type __builtin_ms_va_list = *mut u8;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Using classes to force these to be non-POD.
/// In the Itanium ABI, the tail padding of POD types cannot be reused by other
/// objects, even if the POD type is potentially-overlapping.
#[repr(C)]
pub struct Base0 {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: rust_std::mem::MaybeUninit<u8>,
}

impl !Unpin for Base0 {}

// rs_bindings_from_cc/test/golden/inheritance.h;l=13
// Error while generating bindings for item 'Base0::Base0':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=13
// Error while generating bindings for item 'Base0::Base0':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=13
// Error while generating bindings for item 'Base0::Base0':
// Parameter #0 is not supported: Unsupported type 'class Base0 &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/inheritance.h;l=13
// Error while generating bindings for item 'Base0::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=13
// Error while generating bindings for item 'Base0::operator=':
// Parameter #0 is not supported: Unsupported type 'class Base0 &&': Unsupported type: && without lifetime

#[repr(C)]
pub struct Base1 {
    b1_1_: i64,
    b1_2_: u8,
}

impl !Unpin for Base1 {}

// rs_bindings_from_cc/test/golden/inheritance.h;l=14
// Error while generating bindings for item 'Base1::Base1':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=14
// Error while generating bindings for item 'Base1::Base1':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=14
// Error while generating bindings for item 'Base1::Base1':
// Parameter #0 is not supported: Unsupported type 'class Base1 &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/inheritance.h;l=14
// Error while generating bindings for item 'Base1::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=14
// Error while generating bindings for item 'Base1::operator=':
// Parameter #0 is not supported: Unsupported type 'class Base1 &&': Unsupported type: && without lifetime

#[repr(C)]
pub struct Base2 {
    b2_1_: i16,
}

impl !Unpin for Base2 {}

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Base2::Base2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Base2::Base2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Base2::Base2':
// Parameter #0 is not supported: Unsupported type 'class Base2 &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Base2::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Base2::operator=':
// Parameter #0 is not supported: Unsupported type 'class Base2 &&': Unsupported type: && without lifetime

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct Derived {
    __base_class_subobjects: [rust_std::mem::MaybeUninit<u8>; 12],
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

// rs_bindings_from_cc/test/golden/inheritance.h;l=23
// Error while generating bindings for item 'Derived::Derived':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=23
// Error while generating bindings for item 'Derived::Derived':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=23
// Error while generating bindings for item 'Derived::Derived':
// Parameter #0 is not supported: Unsupported type 'struct Derived &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/inheritance.h;l=23
// Error while generating bindings for item 'Derived::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=23
// Error while generating bindings for item 'Derived::operator=':
// Parameter #0 is not supported: Unsupported type 'struct Derived &&': Unsupported type: && without lifetime

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITANCE_H_

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<Base0>() == 1usize);
const _: () = assert!(rust_std::mem::align_of::<Base0>() == 1usize);
const _: () = {
    assert_not_impl_all!(Base0: Copy);
};
const _: () = {
    assert_not_impl_all!(Base0: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<Base1>() == 16usize);
const _: () = assert!(rust_std::mem::align_of::<Base1>() == 8usize);
const _: () = {
    assert_not_impl_all!(Base1: Copy);
};
const _: () = {
    assert_not_impl_all!(Base1: Drop);
};
const _: () = assert!(offset_of!(Base1, b1_1_) * 8 == 0usize);
const _: () = assert!(offset_of!(Base1, b1_2_) * 8 == 64usize);

const _: () = assert!(rust_std::mem::size_of::<Base2>() == 2usize);
const _: () = assert!(rust_std::mem::align_of::<Base2>() == 2usize);
const _: () = {
    assert_not_impl_all!(Base2: Copy);
};
const _: () = {
    assert_not_impl_all!(Base2: Drop);
};
const _: () = assert!(offset_of!(Base2, b2_1_) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<Derived>() == 16usize);
const _: () = assert!(rust_std::mem::align_of::<Derived>() == 8usize);
const _: () = {
    assert_impl_all!(Derived: Clone);
};
const _: () = {
    assert_impl_all!(Derived: Copy);
};
const _: () = {
    assert_not_impl_all!(Derived: Drop);
};
const _: () = assert!(offset_of!(Derived, derived_1) * 8 == 96usize);
