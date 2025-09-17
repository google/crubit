// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `modules_test.cc`.

pub mod basic_module {
    pub fn add_i32(x: i32, y: i32) -> i32 {
        x + y
    }
}

#[deprecated]
pub mod deprecated_module {
    pub fn add_i32(x: i32, y: i32) -> i32 {
        crate::basic_module::add_i32(x, y)
    }
}

// Ensure that attributes on the inner submodule trigger non-nested-namespace syntax, as nested
// namespace syntax does not accept attributes (see b/445613694).
pub mod outer {
    #[deprecated]
    pub mod inner_deprecated {
        pub struct SomeType;
    }

    pub mod middle {
        #[deprecated]
        pub mod innermost_deprecated {
            pub struct SomeType;
        }
    }
}

/// This is a regression test that verifies that thunk impls use the right
/// fully-qualified name when referring to methods:
///
/// * Incorrect:
///   `::modules::impl_in_separate_private_module::impl_mod::Foo::into_i32`
/// * Correct: `::modules::impl_in_separate_private_module::Foo::into_i32`
///
/// To some extent this test tries to mimic that arrangement in
/// `concrete_fft/v0_2/src/fft128` (where the sub-module is in a
/// separate `f128_impl.rs` file).
pub mod impl_in_separate_private_module {
    pub struct Foo(pub i32);

    /// Okay if `impl` is in a *private* module.  (Whether the module is
    /// public or private has no impact on the problem that this test
    /// verifies doesn't regress.)
    mod impl_mod {
        impl super::Foo {
            pub fn create(i: i32) -> Self {
                Self(i)
            }

            pub fn into_i32(s: Self) -> i32 {
                s.0
            }
        }
    }
}
