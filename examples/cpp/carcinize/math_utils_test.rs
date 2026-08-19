// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_dot_product() {
    let a = math_utils_rs::math::Vector2 { x: 2, y: 3 };
    let b = math_utils_rs::math::Vector2 { x: 4, y: 5 };
    expect_eq!(math_utils_rs::math::DotProduct(&a, &b), 23);
}

#[gtest]
fn test_clamp_generic() {
    expect_eq!(math_utils_rs::math::clamp(5, 0, 10), 5);
    expect_eq!(math_utils_rs::math::clamp(-5, 0, 10), 0);
    expect_eq!(math_utils_rs::math::clamp(15, 0, 10), 10);
    expect_eq!(math_utils_rs::math::clamp(2.5, 0.0, 1.0), 1.0);
}

#[gtest]
fn test_clamp_i32() {
    expect_eq!(math_utils_rs::math::clamp_i32(5, 0, 10), 5);
    expect_eq!(math_utils_rs::math::clamp_i32(-5, 0, 10), 0);
    expect_eq!(math_utils_rs::math::clamp_i32(15, 0, 10), 10);
}
