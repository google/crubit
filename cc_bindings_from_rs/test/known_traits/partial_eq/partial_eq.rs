// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `partial_eq_test.cc`.

pub mod basic_test {
    pub struct MyStruct(usize);

    impl MyStruct {
        pub fn new(val: usize) -> Self {
            Self(val)
        }
    }

    impl PartialEq<MyStruct> for MyStruct {
        fn eq(&self, other: &MyStruct) -> bool {
            self.0 == other.0
        }
    }
}

pub mod usize_rhs {
    pub struct MyStruct(usize);

    impl MyStruct {
        pub fn new(val: usize) -> Self {
            Self(val)
        }
    }

    impl PartialEq<usize> for MyStruct {
        fn eq(&self, other: &usize) -> bool {
            self.0 == *other
        }
    }
}

/// Regression test for b/516804442.  The main verification is whether the generated bindings
/// compile without any errors.
pub mod tuple_collision {
    #[allow(dead_code)]
    pub struct MyStruct(usize);

    impl MyStruct {
        pub fn new(val: usize) -> Self {
            Self(val)
        }
    }

    impl PartialEq<(u64, bool)> for MyStruct {
        fn eq(&self, _other: &(u64, bool)) -> bool {
            true
        }
    }
    impl PartialEq<(usize, bool)> for MyStruct {
        fn eq(&self, _other: &(usize, bool)) -> bool {
            true
        }
    }
}
