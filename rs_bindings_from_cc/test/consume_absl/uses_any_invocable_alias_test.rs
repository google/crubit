// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_that, gtest, matchers::eq};
use uses_any_invocable_alias::*;

#[gtest]
fn test_call_aliased_int_mapper() {
    expect_that!(CallAliasedIntMapper(Box::new(|x: i32| -> i32 { x + 1 }), 41), eq(42));
}

#[gtest]
fn test_return_aliased_int_mapper() {
    let f = ReturnAliasedIntMapper();
    expect_that!(f(21), eq(42));
}
