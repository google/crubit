// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_point_rs() {
    let p = point_rs::geometry::Point { x: 42, y: 100 };
    expect_eq!(point_rs::geometry::GetX(&p), 42);
}

#[gtest]
fn test_point_refactored() {
    let p = point_refactored::geometry::Point { x: 42, y: 100 };
    expect_eq!(point_refactored::geometry::GetX(&p), 42);
}
