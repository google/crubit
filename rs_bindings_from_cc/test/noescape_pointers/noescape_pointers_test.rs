// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_that, gtest, matchers::eq};
use noescape_pointers_cc::takes_noescape_nonnull;

#[gtest]
fn test_noescape_nonnull_param_is_safe_mut_ref() {
    let mut x = 41;
    // No `unsafe` block: the `_Nonnull` noescape pointer binds as `&mut c_int`.
    let res = takes_noescape_nonnull(&mut x);
    expect_that!(res, eq(42));
    expect_that!(x, eq(42));
}
