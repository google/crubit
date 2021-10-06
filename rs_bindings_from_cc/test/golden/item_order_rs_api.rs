#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, const_raw_ptr_deref)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FirstStruct {
    pub field: i32,
}
const_assert_eq!(std::mem::size_of::<FirstStruct>(), 4usize);
const_assert_eq!(std::mem::align_of::<FirstStruct>(), 4usize);
const_assert_eq!(offset_of!(FirstStruct, field) * 8, 0usize);
#[inline(always)]
pub fn first_func() -> i32 {
    unsafe { crate::detail::__rust_thunk__first_func() }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SecondStruct {
    pub field: i32,
}
const_assert_eq!(std::mem::size_of::<SecondStruct>(), 4usize);
const_assert_eq!(std::mem::align_of::<SecondStruct>(), 4usize);
const_assert_eq!(offset_of!(SecondStruct, field) * 8, 0usize);
#[inline(always)]
pub fn second_func() -> i32 {
    unsafe { crate::detail::__rust_thunk__second_func() }
}
mod detail {
    extern "C" {
        pub(crate) fn __rust_thunk__first_func() -> i32;
        pub(crate) fn __rust_thunk__second_func() -> i32;
    }
}
