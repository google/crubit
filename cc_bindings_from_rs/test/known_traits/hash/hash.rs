// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `hash_test.cc`.

/// Test of a derived impl of `Hash` and `PartialEq` on a struct.
pub mod derived_struct {
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    pub fn create_point(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

/// Test of an explicit impl of `Hash` on a struct.
pub mod explicit_struct {
    use std::hash::{Hash, Hasher};

    #[derive(Clone, PartialEq, Eq)]
    pub struct CustomHashStruct {
        pub value: i32,
    }

    impl Hash for CustomHashStruct {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Write value * 2 to distinguish from derived hash
            (self.value * 2).hash(state);
        }
    }

    pub fn create_custom(value: i32) -> CustomHashStruct {
        CustomHashStruct { value }
    }
}

/// Test of a derived impl of `Hash` on an enum.
pub mod derived_enum {
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    pub fn create_red() -> Color {
        Color::Red
    }

    pub fn create_green() -> Color {
        Color::Green
    }

    pub fn create_blue() -> Color {
        Color::Blue
    }
}

/// Test of a derived impl of `Hash` on a tuple struct.
pub mod derived_tuple_struct {
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct TupleStruct(pub i32, pub i32);

    pub fn create_tuple(x: i32, y: i32) -> TupleStruct {
        TupleStruct(x, y)
    }
}
