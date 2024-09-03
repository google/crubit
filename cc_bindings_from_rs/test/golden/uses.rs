// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![allow(dead_code)]

pub mod test_use_glob {
    pub fn f1() -> i32 {
        42
    }

    pub fn f2() -> i32 {
        43
    }

    fn f3() -> i32 {
        44
    }

    pub struct X1 {
        x: i32,
    }

    struct X2 {
        x: i32,
    }
}

pub use test_use_glob::*;
