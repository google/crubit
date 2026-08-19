// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![allow(non_snake_case)]

pub mod geometry {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    pub fn GetX(p: &Point) -> i32 {
        p.x
    }
}
