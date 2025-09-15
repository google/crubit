// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use explicit_specialization::*;
use googletest::prelude::*;

#[gtest]
fn test_explicit_specialization_works_correctly() {
    let x = ReturnX();
    assert_eq!(42, x.val);
}
