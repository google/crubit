// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_support::{global_cpp, inline_cpp};
use inline_cpp_generated_bindings::Point;

// Declaring C++ headers and types:
global_cpp! {
    #include <algorithm>
    #include <cmath>
    #include "support/rs_std/str_ref.h"
    #include "third_party/absl/strings/str_cat.h"
    #include "third_party/absl/strings/string_view.h"

    struct Point {
        double x;
        double y;
    };

    template <typename T>
    struct MathHelper {
        static T Multiply(T a, T b) { return a * b; }
    };
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

/// Formats a greeting using C++ `absl::StrCat`.
pub fn format_greeting(name: &str) -> String {
    let greet = inline_cpp! {
        (rs_std::StrRef name) -> std::string {
            return absl::StrCat("Hello, ", name.to_string_view());
        }
    };
    let cpp_str = greet(name);
    cpp_str.to_string().expect("Valid UTF-8")
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

/// Calls an instantiated C++ standard library template.
pub fn clamp_value(val: f32, min: f32, max: f32) -> f32 {
    let clamp = inline_cpp! {
        (float val, float min, float max) -> float {
            return std::clamp<float>(val, min, max);
        }
    };
    clamp(val, min, max)
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
