// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

fn main() {
    let pt_carcinized = point_rs::geometry::Point { x: 10, y: 20 };
    println!("point_rs GetX: {}", point_rs::geometry::GetX(&pt_carcinized));

    let pt_refactored = point_refactored::geometry::Point { x: 30, y: 40 };
    println!("point_refactored GetX: {}", point_refactored::geometry::GetX(&pt_refactored));

    let v1 = math_utils_rs::math::Vector2 { x: 1, y: 2 };
    let v2 = math_utils_rs::math::Vector2 { x: 3, y: 4 };
    println!("DotProduct: {}", math_utils_rs::math::DotProduct(&v1, &v2));

    println!("clamp generic: {}", math_utils_rs::math::clamp(15, 0, 10));
    println!("clamp_i32: {}", math_utils_rs::math::clamp_i32(15, 0, 10));
}
