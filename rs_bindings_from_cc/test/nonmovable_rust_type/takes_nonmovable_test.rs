// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn takes_by_value() {
    let x = nonmovable::NonMovable::from_byte(42);
    expect_eq!(takes_nonmovable::crubit_test::TakesByValue(x), 42);
}
