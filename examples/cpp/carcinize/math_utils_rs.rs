// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![allow(non_snake_case, clippy::needless_lifetimes)]

use crubit_support::global_cpp;

global_cpp! {
    namespace math {
    template <typename T>
    T Clamp(T val, T min, T max) {
      return val < min ? min : (val > max ? max : val);
    }
    }  // namespace math
}

pub mod math {
    use crubit_support::inline_cpp;

    #[derive(Clone, Copy, Default)]
    #[repr(C)]
    pub struct Vector2 {
        pub x: i32,
        pub y: i32,
    }

    #[inline(always)]
    pub fn DotProduct<'a, 'b>(a: &'a Vector2, b: &'b Vector2) -> i32 {
        (inline_cpp! {
            (int ax, int ay, int bx, int by) -> int {
                return ax * bx + ay * by;
            }
        })(a.x, a.y, b.x, b.y)
    }

    /// Strategy 1: Pure Rust generic implementation.
    pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
        if val < min {
            min
        } else if val > max {
            max
        } else {
            val
        }
    }

    /// Strategy 2: Wrap specific template instantiation with inline_cpp!.
    pub fn clamp_i32(val: i32, min: i32, max: i32) -> i32 {
        (inline_cpp! {
            (int val, int min, int max) -> int {
                return math::Clamp(val, min, max);
            }
        })(val, min, max)
    }
}
