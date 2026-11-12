// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_types() {
    use types::{kTypesAlign, kTypesSize, Types};
    expect_that!(core::mem::size_of::<Types>(), eq(unsafe { kTypesSize }));
    expect_that!(core::mem::align_of::<Types>(), eq(unsafe { kTypesAlign }));
}
