// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `enums_test.cc`.

pub mod classless_enum {
    #[crubit_annotate::cpp_enum(kind = "enum")]
    #[repr(transparent)]
    pub struct Color(i32);

    impl Color {
        pub const RED: Color = Color(0);
        pub const BLUE: Color = Color(2);
    }
}

pub mod cpp_enum {
    #[crubit_annotate::cpp_enum(kind = "enum class")]
    #[repr(transparent)]
    pub struct Color(i32);

    impl Color {
        pub const RED: Color = Color(0);
        pub const BLUE: Color = Color(2);
    }
}

pub mod deprecated_enum {
    #![allow(deprecated)]
    #![allow(unused)]

    #[crubit_annotate::cpp_enum(kind = "enum class")]
    #[repr(transparent)]
    #[deprecated(note = "Use NewColor")]
    #[must_use]
    pub struct Color(i32);

    impl Color {
        pub const RED: Color = Color(0);
        pub const BLUE: Color = Color(2);
    }
}
