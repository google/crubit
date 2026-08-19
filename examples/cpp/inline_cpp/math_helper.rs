// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_support::{global_cpp, inline_cpp};

global_cpp! {
    template <typename T>
    struct MathHelper {
        static T Multiply(T a, T b) { return a * b; }
    };
}

/// Calls a C++ helper template defined in `global_cpp!`.
pub fn multiply_ints(a: i32, b: i32) -> i32 {
    let mult = inline_cpp! {
        (int a, int b) -> int {
            return MathHelper<int>::Multiply(a, b);
        }
    };
    mult(a, b)
}
