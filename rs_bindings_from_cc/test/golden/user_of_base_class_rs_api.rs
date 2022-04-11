// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;
use static_assertions::{assert_impl_all, assert_not_impl_all};

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// The same as Derived from inheritance.h, but in a different build target.
///
/// This tests inheritance across library boundaries.
///
/// TODO(b/216195042): Correctly namespace base classes in generated Rust code.
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct Derived2 {
    __base_class_subobjects: [rust_std::mem::MaybeUninit<u8>; 12],
    pub derived_1: u8,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Derived2"), Derived2);
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

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=15
// Error while generating bindings for item 'Derived2::Derived2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=15
// Error while generating bindings for item 'Derived2::Derived2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=15
// Error while generating bindings for item 'Derived2::Derived2':
// Parameter #0 is not supported: Unsupported type 'struct Derived2 &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=15
// Error while generating bindings for item 'Derived2::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=15
// Error while generating bindings for item 'Derived2::operator=':
// Parameter #0 is not supported: Unsupported type 'struct Derived2 &&': Unsupported type: && without lifetime

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_BASE_CLASS_H_

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<Derived2>() == 16usize);
const _: () = assert!(rust_std::mem::align_of::<Derived2>() == 8usize);
const _: () = {
    assert_impl_all!(Derived2: Clone);
};
const _: () = {
    assert_impl_all!(Derived2: Copy);
};
const _: () = {
    assert_not_impl_all!(Derived2: Drop);
};
const _: () = assert!(offset_of!(Derived2, derived_1) * 8 == 96usize);
