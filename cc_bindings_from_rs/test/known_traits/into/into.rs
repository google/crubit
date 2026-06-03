// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `into_test.cc`.

// We explicitly want to test `Into` here, so disable the clippy warning about it.
#![allow(clippy::from_over_into)]

use crubit_annotate::must_bind;

#[must_bind]
pub struct Convert(pub i32);

impl std::convert::Into<i32> for Convert {
    #[must_bind]
    fn into(self) -> i32 {
        self.0
    }
}

impl Into<i64> for Convert {
    #[must_bind]
    fn into(self) -> i64 {
        self.0 as i64
    }
}

impl Into<&'static str> for Convert {
    #[must_bind]
    fn into(self) -> &'static str {
        "Convert"
    }
}

use std::convert;
impl convert::Into<i16> for Convert {
    #[must_bind]
    fn into(self) -> i16 {
        self.0.try_into().unwrap()
    }
}

#[must_bind]
pub struct ConvertRef<'a>(&'a str);

impl<'a> ConvertRef<'a> {
    #[must_bind]
    pub fn create(s: &'a str) -> Self {
        Self(s)
    }

    #[must_bind]
    pub fn transmigrate(self) -> Convert {
        Convert(42)
    }
}

impl<'a> Into<&'a str> for ConvertRef<'a> {
    #[must_bind]
    fn into(self) -> &'a str {
        self.0
    }
}

impl Into<Convert> for ConvertRef<'_> {
    #[must_bind]
    fn into(self) -> Convert {
        Convert(42)
    }
}

// `Into` impls with non-C++-compatible types shouldn't be bound.
#[must_bind]
pub struct NotFfiSafe(fn());

impl Into<fn()> for NotFfiSafe {
    fn into(self) -> fn() {
        self.0
    }
}

#[must_bind]
pub struct ConvertModule(pub i32);

pub mod another_module {
    use super::ConvertModule;
    use crubit_annotate::must_bind;

    impl Into<i32> for ConvertModule {
        #[must_bind]
        fn into(self) -> i32 {
            self.0
        }
    }
}

mod yet_another_module {
    use super::ConvertModule;
    use crubit_annotate::must_bind;

    impl Into<i64> for ConvertModule {
        #[must_bind]
        fn into(self) -> i64 {
            self.0 as i64
        }
    }
}

#[must_bind]
#[derive(Clone, Copy, Default)]
pub struct CloneCopyType(pub i32);

#[must_bind]
#[derive(Clone, Copy)]
pub struct CloneCopyTarget(pub i32);

impl Into<CloneCopyTarget> for CloneCopyType {
    #[must_bind]
    fn into(self) -> CloneCopyTarget {
        CloneCopyTarget(self.0)
    }
}

#[must_bind]
#[derive(Clone)]
pub struct CloneAllocType {
    pub value: String,
}

impl CloneAllocType {
    #[must_bind]
    pub fn create(s: &str) -> Self {
        Self { value: s.to_string() }
    }

    #[must_bind]
    pub fn get_value(&self) -> &str {
        &self.value
    }
}

#[must_bind]
#[derive(Clone)]
pub struct CloneAllocTarget {
    pub value: String,
}

impl CloneAllocTarget {
    #[must_bind]
    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl Into<CloneAllocTarget> for CloneAllocType {
    #[must_bind]
    fn into(self) -> CloneAllocTarget {
        CloneAllocTarget { value: self.value }
    }
}

#[must_bind]
#[derive(Default)]
pub struct NoCloneDefaultType(pub i32);

#[must_bind]
#[derive(Default)]
pub struct NoCloneDefaultTarget(pub i32);

impl Into<NoCloneDefaultTarget> for NoCloneDefaultType {
    #[must_bind]
    fn into(self) -> NoCloneDefaultTarget {
        NoCloneDefaultTarget(self.0)
    }
}

#[must_bind]
pub struct NoCloneCopyDropType(pub i32);

#[must_bind]
pub struct NoCloneCopyDropTarget(pub i32);

impl Into<NoCloneCopyDropTarget> for NoCloneCopyDropType {
    #[must_bind]
    fn into(self) -> NoCloneCopyDropTarget {
        NoCloneCopyDropTarget(self.0)
    }
}

#[must_bind]
pub struct LoopA(pub i32);

#[must_bind]
pub struct LoopB(pub i32);

impl Into<LoopB> for LoopA {
    #[must_bind]
    fn into(self) -> LoopB {
        LoopB(self.0)
    }
}

impl Into<LoopA> for LoopB {
    #[must_bind]
    fn into(self) -> LoopA {
        LoopA(self.0)
    }
}
