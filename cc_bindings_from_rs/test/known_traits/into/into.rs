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
