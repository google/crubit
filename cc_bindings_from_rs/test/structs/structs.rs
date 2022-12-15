// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `structs_test.cc`.

/// Test for a `#[repr(C)` struct.
pub mod repr_c {

    #[repr(C)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    pub fn create(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn get_x(p: Point) -> i32 {
        p.x
    }
}

/// Test for a struct using default layout (i.e. one without an explicit
/// `#[repr(C)]` or similar attribute).  Among other things, it tests that
/// building generated `..._cc_api_impl.rs` will not warn about
/// `improper_ctypes_definitions` (search for this warning name in `bindings.rs`
/// for a longer explanation of why suppressing this warning is okay).
pub mod default_repr {

    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    pub fn create(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn get_x(p: Point) -> i32 {
        p.x
    }
}

/// This module provides test coverage for reordering the generated bindings in
/// a way that ensures that C++ structs are defined *before* being referring to
/// them when (say) declaring a function that returns the struct by value, or
/// takes it by value as an argument.
///
/// This module has been structured in a way that forces at least one submodule
/// to be broken up into 2 separate chunks.  Definition dependencies force
/// bindings from one of the structs to come first - let's assume that `m1::S1`
/// comes first (the case where `m2::S2` comes first is symmetrical -
/// all the same conclusions apply).  Before `m1::create_S2` can be declared,
/// `m1::S2` needs to be defined.  This means that the order will be: `m1::S1`,
/// ..., `m2::S2`, ..., `m1::create_S2` - the `m1` module has to be split into
/// two non-contiguous chunks (in the generated bindings):
///
///     ```cpp
///     namespace m1 {  // <- FIRST CHUNK OF `mod m1`
///         struct S1 { ... };
///     }
///
///     namespace m2 {
///         struct S2 { ... };
///     }
///
///     namespace m1 {  // <- SECOND CHUNK OF `mod m1`
///         S2 create_s2();
///     }
///     ```
pub mod reordering_defs {
    pub mod m1 {
        use super::m2::S2;
        pub struct S1(pub i32);
        pub fn create_s2() -> S2 {
            S2(123)
        }
        pub fn get_int_from_s2(s2: S2) -> i32 {
            s2.0
        }
    }
    pub mod m2 {
        use super::m1::S1;
        pub struct S2(pub i32);
        pub fn create_s1() -> S1 {
            S1(456)
        }
        pub fn get_int_from_s1(s1: S1) -> i32 {
            s1.0
        }
    }
}
