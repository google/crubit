// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_support::{global_cpp, inline_cpp};

global_cpp! {
    #include <cmath>
}

/// Computes the hypotenuse using C++ `std::hypot`.
pub fn compute_hypotenuse(a: f64, b: f64) -> f64 {
    let hypot_fn = inline_cpp! {
        (double a, double b) -> double {
            return std::hypot(a, b);
        }
    };
    hypot_fn(a, b)
}
