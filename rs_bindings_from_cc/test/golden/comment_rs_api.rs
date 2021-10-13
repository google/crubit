#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, const_raw_ptr_deref)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;

// File comment

// TODO(b/202929091)
// namespace ns {
// a

/// Foo
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Foo {
    /// A field
    pub i: i32,
    /// Another field
    pub j: i32,
}

// b

// }  // namespace ns

// c

/// foo
#[inline(always)]
pub fn foo() -> () {
    unsafe { crate::detail::__rust_thunk__foo() }
}

/// Bar
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Bar {
    pub i: i32,
}

/// d
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HasNoComments {
    pub i: i32,
}

// e

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMMENT_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk__foo() -> ();
    }
}

const_assert_eq!(std::mem::size_of::<Foo>(), 8usize);
const_assert_eq!(std::mem::align_of::<Foo>(), 4usize);
const_assert_eq!(offset_of!(Foo, i) * 8, 0usize);
const_assert_eq!(offset_of!(Foo, j) * 8, 32usize);

const_assert_eq!(std::mem::size_of::<Bar>(), 4usize);
const_assert_eq!(std::mem::align_of::<Bar>(), 4usize);
const_assert_eq!(offset_of!(Bar, i) * 8, 0usize);

const_assert_eq!(std::mem::size_of::<HasNoComments>(), 4usize);
const_assert_eq!(std::mem::align_of::<HasNoComments>(), 4usize);
const_assert_eq!(offset_of!(HasNoComments, i) * 8, 0usize);
