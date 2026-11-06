// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, gtest};

#[gtest]
fn test_add() {
    expect_eq!(my_lib::add(10, 20), 30);
}
