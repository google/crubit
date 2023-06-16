// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `default_test.cc`.

/// Test of an explicit impl of a trait: `impl Default for SomeStruct`.
pub mod explicit_impl {
    pub struct SomeStruct(i32);

    impl Default for SomeStruct {
        fn default() -> Self {
            Self(42)
        }
    }

    impl SomeStruct {
        pub fn extract_int(s: Self) -> i32 {
            s.0
        }
    }
}

/// Test of a derived impl of a trait: `#[derive(Default)]`.
pub mod derived_impl {
    #[derive(Default)]
    pub struct SomeStruct(i32);

    impl SomeStruct {
        pub fn extract_int(s: Self) -> i32 {
            s.0
        }
    }
}

/// Test of a missing impl of a trait.
pub mod no_impl {
    pub struct SomeStruct(i32);
}
