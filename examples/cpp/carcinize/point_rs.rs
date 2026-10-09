// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![allow(non_snake_case, clippy::needless_lifetimes)]

use inline_cpp_macro::global_cpp;

global_cpp! {
    // Declarations for geometry::point
}

pub mod geometry {
    use inline_cpp_macro::inline_cpp;

    #[derive(Clone, Copy, Default)]
    #[repr(C)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    #[inline(always)]
    pub fn GetX<'p>(p: &'p Point) -> i32 {
        (inline_cpp! {
            (int x) -> int {
                return x;
            }
        })(p.x)
    }
}
