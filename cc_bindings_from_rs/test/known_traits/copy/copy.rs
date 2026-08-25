// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![allow(clippy::manual_non_exhaustive)]

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `copy_test.cc`.

/// Test of an explicit impl of a trait: `impl Clone for SomeStruct`.
pub mod explicit_impl {
    pub struct SomeStruct {
        pub field: i32,
        _private: (),
    }

    impl Clone for SomeStruct {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl Copy for SomeStruct {}

    impl SomeStruct {
        pub fn create_struct(i: i32) -> Self {
            SomeStruct { field: i, _private: () }
        }

        pub fn extract_int(s: Self) -> i32 {
            s.field
        }
    }
}

/// Test of a derived impl of a trait: `#[derive(..., Copy)]`.
pub mod derived_impl {
    #[derive(Clone, Copy)]
    pub struct SomeStruct {
        pub field: i32,
        _private: (),
    }

    impl SomeStruct {
        pub fn create_struct(i: i32) -> Self {
            SomeStruct { field: i, _private: () }
        }

        pub fn extract_int(s: Self) -> i32 {
            s.field
        }
    }
}

/// Test of a missing impl of a trait.
pub mod no_impl {
    pub struct SomeStruct {
        pub field: i32,
        _private: (),
    }
}
