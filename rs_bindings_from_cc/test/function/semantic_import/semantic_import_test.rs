// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;

#[gtest]
fn test_int_accessor() {
    let mut s = semantic_import::S::from(42);
    assert_eq!(s.x(), 42);
    s.set_x(100);
    assert_eq!(s.x(), 100);
}
