// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_nested_anonymous() {
    use nested_anonymous::{kNestedAlign, kNestedSize, Nested};
    expect_that!(core::mem::size_of::<Nested>(), eq(unsafe { kNestedSize }));
    expect_that!(core::mem::align_of::<Nested>(), eq(unsafe { kNestedAlign }));
    // The anonymous fields are missing so we can't test field access on them.
}
