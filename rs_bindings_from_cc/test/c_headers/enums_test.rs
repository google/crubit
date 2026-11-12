// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_enums() {
    use enums::{Duplicates, LargeU64, Negative, ANON_ENUM_CONST};
    let _ = Negative::NEG_ONE;
    expect_that!(i32::from(Negative::NEG_TWO), eq(-2));
    expect_that!(core::mem::size_of::<LargeU64>(), eq(8));
    let _ = LargeU64::LARGE;

    expect_that!(Duplicates::DUP1, eq(Duplicates::DUP2));
    let _ = ANON_ENUM_CONST;
}
