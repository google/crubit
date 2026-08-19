// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use point_lib::Point;

fn main() {
    let hypot = math_lib::compute_hypotenuse(3.0, 4.0);
    println!("hypot = {hypot}");
    let greeting = greeting_lib::format_greeting("World");
    println!("greeting = {greeting}");
    let p = Point { x: 3.0, y: 4.0 };
    let distance = point_lib::get_distance(&p);
    println!("distance = {distance}");
    let clamped = clamp_lib::clamp_value(15.0, 0.0, 10.0);
    println!("clamped = {clamped}");
    let product = math_helper_lib::multiply_ints(6, 7);
    println!("product = {product}");
}
