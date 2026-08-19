// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_support::{global_cpp, inline_cpp};
pub use inline_cpp_generated_bindings::Point;

global_cpp! {
    #include <cmath>

    struct Point {
        double x;
        double y;
    };
}

/// Calculates Euclidean distance for a `Point`.
pub fn get_distance(p: &Point) -> f64 {
    let calc = inline_cpp! {
        (const Point& p) -> double {
            return std::sqrt(p.x * p.x + p.y * p.y);
        }
    };
    calc(p)
}
