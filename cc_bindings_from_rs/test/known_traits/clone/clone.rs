// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `clone_test.cc`.

/// Test of an explicit impl of a trait: `impl Clone for SomeStruct`.
/// Only `clone` is provided, `clone_from` uses the default implementation.
pub mod explicit_impl_of_mandatory_method {
    pub struct SomeStruct(i32);

    impl Clone for SomeStruct {
        fn clone(&self) -> Self {
            // Adding 10000 when cloning to aid with test verification.
            Self(self.0 + 10000)
        }
    }

    impl SomeStruct {
        pub fn create_struct(i: i32) -> Self {
            Self(i)
        }

        pub fn extract_int(s: Self) -> i32 {
            s.0
        }
    }
}

/// Test of an explicit impl of a trait: `impl Clone for SomeStruct`.
/// Both `clone` and `clone_from` are implemented.
pub mod explicit_impl_of_all_methods {
    pub struct SomeStruct(i32);

    impl Clone for SomeStruct {
        fn clone(&self) -> Self {
            // Adding 10000 when cloning to aid with test verification.
            // Note that `clone_from` adds a different amount.
            Self(self.0 + 10000)
        }

        fn clone_from(&mut self, source: &Self) {
            // Adding 20000 when cloning to aid with test verification.
            // Note that `clone` adds a different amount.
            self.0 = source.0 + 20000;
        }
    }

    impl SomeStruct {
        pub fn create_struct(i: i32) -> Self {
            Self(i)
        }

        pub fn extract_int(s: Self) -> i32 {
            s.0
        }
    }
}

/// Test of a derived impl of a trait: `#[derive(Clone)]`.
pub mod derived_impl {
    #[derive(Clone)]
    pub struct SomeStruct(i32);

    impl SomeStruct {
        pub fn create_struct(i: i32) -> Self {
            Self(i)
        }

        pub fn extract_int(s: Self) -> i32 {
            s.0
        }
    }
}

/// Test of a derived impl, where one of the fields is a struct that
/// implements `Clone` but doesn't implement `Default`.  This is a regression
/// test for b/288138612.
pub mod derived_impl_with_non_default_field {
    /// It is important that `InnerStruct` is `pub` so that `SomeStruct.0` field
    /// is typed correctly in the C++ bindings and not replaced with a blob
    /// of bytes.
    #[derive(Clone)]
    pub struct InnerStruct(i32);

    #[derive(Clone)]
    pub struct SomeStruct(InnerStruct);

    impl SomeStruct {
        pub fn create_struct(i: i32) -> Self {
            Self(InnerStruct(i))
        }

        pub fn extract_int(s: Self) -> i32 {
            s.0.0
        }
    }
}

/// Test of a missing impl of a trait.
pub mod no_impl {
    pub struct SomeStruct(i32);
}
