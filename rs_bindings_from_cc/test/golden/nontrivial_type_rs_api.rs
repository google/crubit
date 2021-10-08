#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, const_raw_ptr_deref, negative_impls)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;

#[repr(C)]
pub struct Nontrivial {
    pub field: i32,
}

impl !Unpin for Nontrivial {}

const_assert_eq!(std::mem::size_of::<Nontrivial>(), 4usize);
const_assert_eq!(std::mem::align_of::<Nontrivial>(), 4usize);
const_assert_eq!(offset_of!(Nontrivial, field) * 8, 0usize);
