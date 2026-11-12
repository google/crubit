// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::assert_that;
use googletest::gtest;
use googletest::matchers::eq;
use types_const_generic::{MakeTaggedArray, RustTaggedArray, SumTaggedArrayWithTag};

#[gtest]
fn test_const_generic_usize_and_negative_i32_roundtrip() {
    let arr: RustTaggedArray<i32, 3, -5> = MakeTaggedArray();
    assert_that!(arr, eq(RustTaggedArray::<i32, 3, -5>([10, 20, 30])));
    assert_that!(SumTaggedArrayWithTag(arr), eq(55));
}
