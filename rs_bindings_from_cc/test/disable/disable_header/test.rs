// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use item_exists::value_exists;

use googletest::prelude::*;

#[gtest]
fn test_disabled_header() {
    expect_pred!(value_exists!(test_lib::foo));
    expect_pred!(!value_exists!(test_lib::must_not_get_bindings));
    expect_pred!(value_exists!(test_lib2::bar));
    expect_pred!(!value_exists!(test_lib2::must_not_get_bindings));
}
