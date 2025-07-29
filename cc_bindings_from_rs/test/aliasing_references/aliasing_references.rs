// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `aliasing_references_test.cc`.

pub fn mut_refs(_: &mut i32, _: &mut i32) {}
pub fn mut_ref_and_shared_refs(_: &mut i32, _: &i32, _: &i32) {}

#[derive(Copy, Clone, Default)]
pub struct SomeStruct {
    pub field: i32,
}

impl SomeStruct {
    pub fn mut_self_and_mut_ref(&mut self, _: &mut i32) {}
    pub fn mut_self_and_shared_ref(&mut self, _: &i32) {}
    pub fn shared_self_and_mut_ref(&self, _: &mut i32) {}
    pub fn shared_self_and_shared_ref_allows_alias(&self, _: &i32) {}
}

#[derive(Default)]
pub struct NonFreezeType(std::cell::UnsafeCell<i32>);

impl NonFreezeType {
    /// # Safety
    ///
    /// This function must not be called while an outstanding reference to the underlying
    /// `i32` is held.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_mut_unchecked(&self) -> &mut i32 {
        unsafe { &mut *self.0.get() }
    }
    pub fn shared_self_mut_ref_allows_alias(&self, _: &mut i32) {}
}
