// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use nested_items::same::AFunction;

#[gtest]
fn test_a_function() {
    assert_eq!(42, AFunction());
}
