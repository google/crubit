// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `from_test.cc`.

use crubit_annotate::must_bind;

#[must_bind]
pub struct Opaque(pub i32);

impl std::convert::From<Opaque> for i32 {
    fn from(value: Opaque) -> Self {
        value.0
    }
}

impl From<Opaque> for i64 {
    fn from(value: Opaque) -> i64 {
        value.0 as i64
    }
}

impl From<Opaque> for &'static str {
    fn from(_: Opaque) -> &'static str {
        "Opaque"
    }
}

use std::convert;
impl convert::From<Opaque> for i16 {
    fn from(value: Opaque) -> i16 {
        value.0.try_into().unwrap()
    }
}

impl From<Opaque> for OpaqueRef<'static> {
    fn from(value: Opaque) -> Self {
        Self(value.into())
    }
}

#[must_bind]
pub struct OpaqueRef<'a>(&'a str);

impl<'a> OpaqueRef<'a> {
    #[must_bind]
    pub fn create(s: &'a str) -> Self {
        Self(s)
    }

    #[must_bind]
    pub fn get_arg(&self) -> &'a str {
        self.0
    }
}

impl<'a> From<OpaqueRef<'a>> for &'a str {
    fn from(value: OpaqueRef<'a>) -> &'a str {
        value.get_arg()
    }
}

// `From` impls with non-C++-compatible types shouldn't be bound.
#[must_bind]
pub struct NotFfiSafe(fn());

#[allow(dead_code)]
fn test() {}

impl NotFfiSafe {
    #[must_bind]
    pub fn create() -> Self {
        Self(test)
    }
}
impl From<NotFfiSafe> for i32 {
    fn from(_: NotFfiSafe) -> i32 {
        42
    }
}
impl From<NotFfiSafe> for fn() {
    fn from(value: NotFfiSafe) -> fn() {
        value.0
    }
}
