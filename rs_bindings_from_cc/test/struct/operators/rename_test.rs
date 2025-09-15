// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use rename::*;

#[gtest]
fn test_bitwise_not_as_rust_not() {
    let s = BitwiseNotAsRustNot { i: 7 };
    assert_eq!(-8, (!&s).i);
}

#[gtest]
fn test_two_nots() {
    let s = TwoNots { i: 7 };
    assert_eq!(0, s.logical_not().i);
    assert_eq!(-8, (!&s).i);
}
