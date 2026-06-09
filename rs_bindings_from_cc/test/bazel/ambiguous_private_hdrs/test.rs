// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_pred, gtest};
use item_exists::value_exists;

#[gtest]
fn test_gets_bindings() {
    expect_pred!(value_exists!(lib::AlsoInPrivateHeader));
    expect_pred!(value_exists!(lib::AlsoInTextualHeader));
}
