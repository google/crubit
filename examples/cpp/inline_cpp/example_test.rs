// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_compute_hypotenuse() {
    expect_eq!(example_lib::compute_hypotenuse(3.0, 4.0), 5.0);
}

#[gtest]
fn test_format_greeting() {
    expect_eq!(example_lib::format_greeting("World"), "Hello, World");
}

#[gtest]
fn test_clamp_value() {
    expect_eq!(example_lib::clamp_value(5.0, 0.0, 10.0), 5.0);
    expect_eq!(example_lib::clamp_value(-5.0, 0.0, 10.0), 0.0);
    expect_eq!(example_lib::clamp_value(15.0, 0.0, 10.0), 10.0);
}

#[gtest]
fn test_multiply_ints() {
    expect_eq!(example_lib::multiply_ints(6, 7), 42);
    expect_eq!(example_lib::multiply_ints(-3, 4), -12);
}
