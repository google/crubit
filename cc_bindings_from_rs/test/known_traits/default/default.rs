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

/// Test of a struct with 1) `impl Default` and 2) field that does *not* have
/// `impl Default`.  This is a regression test for b/288138612.
pub mod field_with_no_default {
    pub struct StructWithFieldWithNoDefault {
        field: StructWithoutDefault,
    }

    impl Default for StructWithFieldWithNoDefault {
        fn default() -> Self {
            Self { field: StructWithoutDefault(123) }
        }
    }

    /// It is important that `StructWithoutDefault` is `pub` so that `field`
    /// above is typed correctly in the C++ bindings and not replaced with a
    /// blob of bytes.
    pub struct StructWithoutDefault(i32);

    impl StructWithFieldWithNoDefault {
        pub fn extract_int(s: Self) -> i32 {
            s.field.0
        }
    }
}

/// Test of a missing impl of a trait.
pub mod no_impl {
    pub struct SomeStruct(i32);
}

pub mod transparent_struct {
    #[repr(transparent)]
    #[derive(Default)]
    pub struct SomeStruct(i32);

    impl SomeStruct {
        pub fn extract_int(&self) -> i32 {
            self.0
        }
    }
}
