// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `impls_test.cc`.

/// Basic static method (e.g. the method only uses primitive types and doesn't
/// refer back to the struct).
pub mod basic_static_method {
    /// No-op `i32` placeholder is used, because ZSTs are not supported.
    pub struct Math(i32);

    impl Math {
        pub fn add_i32(x: i32, y: i32) -> i32 {
            x + y
        }
    }
}

pub mod static_method_taking_same_struct_by_value {
    pub struct Number(i32);

    impl Number {
        pub fn create(i: i32) -> Self {
            Self(i)
        }
        pub fn static_into_i32(s: Self) -> i32 {
            s.0
        }
    }
}

/// The main point of the test below is that (for both of the `add_structs`
/// static methods and both of the structs):
/// 1) method declarations depend on forward declarations of the structs
/// 2) method definitions depend on struct definitions.
///
/// The above means that in the generated C++ header file the struct definition
/// (with method declarations) cannot be "glued" together with its method
/// definitions.
///
/// And the above is why `format_fn` and `format_adt` in `bindings.rs` return
/// multiple snippets - this means that method declarations and method
/// definitions can be reordered independently.
pub mod non_contiguous_method_decls_and_defs {
    pub struct S1(i32);
    pub struct S2(i32);

    impl S1 {
        pub fn create(i: i32) -> Self {
            Self(i)
        }
        pub fn add_structs(x: S1, y: S2) -> i32 {
            x.0 + y.0
        }
    }

    impl S2 {
        pub fn create(i: i32) -> Self {
            Self(i)
        }
        pub fn add_structs(x: S1, y: S2) -> i32 {
            x.0 + y.0
        }
    }
}
