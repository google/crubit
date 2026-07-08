// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ::int128::absl::int128;
use ::int128::absl::uint128;
use googletest::prelude::*;

#[gtest]
fn test_uint128() {
    let i = uint128::from(40) + uint128::from(2) + uint128::default();
    expect_pred!(i == uint128::from(42u64));
}

#[gtest]
fn test_int128() {
    let i = int128::from(40) + int128::from(2) + int128::default();
    expect_pred!(i == int128::from(42i64));
}
