// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `lifetimes_test.cc`.

#[derive(Copy, Clone)]
pub struct StructWithLifetime<'a> {
    pub field_with_lifetime: &'a i32,
}

impl<'a> From<&'a i32> for StructWithLifetime<'a> {
    fn from(field_with_lifetime: &'a i32) -> Self {
        Self { field_with_lifetime }
    }
}

impl<'a> Into<&'a i32> for StructWithLifetime<'a> {
    fn into(self) -> &'a i32 {
        self.field_with_lifetime
    }
}

impl Into<i32> for StructWithLifetime<'_> {
    fn into(self) -> i32 {
        *self.field_with_lifetime
    }
}

impl<'a> StructWithLifetime<'a> {
    pub fn from_ref(field_with_lifetime: &'a i32) -> Self {
        Self { field_with_lifetime }
    }

    pub fn into_ref(self) -> &'a i32 {
        self.field_with_lifetime
    }

    pub fn value(self) -> i32 {
        *self.field_with_lifetime
    }
}

impl StructWithLifetime<'static> {
    pub fn make_static_42() -> Self {
        Self { field_with_lifetime: &42 }
    }

    pub fn from_static_ref(field_with_lifetime: &'static i32) -> Self {
        Self { field_with_lifetime }
    }

    pub fn from_static_ref_where_bound<'a>(field_with_lifetime: &'a i32) -> Self
    where
        'a: 'static,
    {
        Self { field_with_lifetime }
    }
}
