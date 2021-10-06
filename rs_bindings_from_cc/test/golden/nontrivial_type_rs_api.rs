#![feature(negative_impls, const_ptr_offset_from, const_maybe_uninit_as_ptr, const_raw_ptr_deref)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;
#[derive()]
#[repr(C)]
pub struct Nontrivial {
    pub field: i32,
}
const_assert_eq!(std::mem::size_of::<Nontrivial>(), 4usize);
const_assert_eq!(std::mem::align_of::<Nontrivial>(), 4usize);
const_assert_eq!(offset_of!(Nontrivial, field) * 8, 0usize);
impl !Unpin for Nontrivial {}
