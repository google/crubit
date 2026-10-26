// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_twice_int() {
    expect_eq!(widget::twice_int(21), 42);
}

#[gtest]
fn test_doubled() {
    let w = widget::Widget { size: 21 };
    expect_eq!(widget::doubled(&w).size, 42);
}
