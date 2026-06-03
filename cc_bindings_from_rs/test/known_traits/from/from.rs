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

#[must_bind]
#[derive(Clone, Copy, Default)]
pub struct CloneCopyType(pub i32);

#[must_bind]
#[derive(Clone, Copy)]
pub struct CloneCopySource(pub i32);

impl From<CloneCopySource> for CloneCopyType {
    fn from(src: CloneCopySource) -> Self {
        Self(src.0)
    }
}

#[must_bind]
#[derive(Clone)]
pub struct CloneAllocType {
    pub value: String,
}

impl CloneAllocType {
    #[must_bind]
    pub fn get_value(&self) -> &str {
        &self.value
    }
}

#[must_bind]
#[derive(Clone)]
pub struct CloneAllocSource {
    pub value: String,
}

impl CloneAllocSource {
    #[must_bind]
    pub fn create(s: &str) -> Self {
        Self { value: s.to_string() }
    }

    #[must_bind]
    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl From<CloneAllocSource> for CloneAllocType {
    fn from(src: CloneAllocSource) -> Self {
        Self { value: src.value }
    }
}

#[must_bind]
#[derive(Default)]
pub struct NoCloneDefaultType(pub i32);

#[must_bind]
#[derive(Default)]
pub struct NoCloneDefaultSource(pub i32);

impl From<NoCloneDefaultSource> for NoCloneDefaultType {
    fn from(src: NoCloneDefaultSource) -> Self {
        Self(src.0)
    }
}

#[must_bind]
pub struct NoCloneCopyDropType(pub i32);

#[must_bind]
pub struct NoCloneCopyDropSource(pub i32);

impl From<NoCloneCopyDropSource> for NoCloneCopyDropType {
    fn from(src: NoCloneCopyDropSource) -> Self {
        Self(src.0)
    }
}

#[must_bind]
pub struct LoopA(pub i32);

#[must_bind]
pub struct LoopB(pub i32);

impl From<LoopA> for LoopB {
    fn from(src: LoopA) -> Self {
        Self(src.0)
    }
}

impl From<LoopB> for LoopA {
    fn from(src: LoopB) -> Self {
        Self(src.0)
    }
}
