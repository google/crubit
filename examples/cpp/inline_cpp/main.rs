// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

fn main() {
    let hypot = example_lib::compute_hypotenuse(3.0, 4.0);
    println!("hypot = {hypot}");
    let greeting = example_lib::format_greeting("World");
    println!("greeting = {greeting}");
    let clamped = example_lib::clamp_value(15.0, 0.0, 10.0);
    println!("clamped = {clamped}");
    let product = example_lib::multiply_ints(6, 7);
    println!("product = {product}");
}
