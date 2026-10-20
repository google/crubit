// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use optional_nested_pair_lib::*;

#[gtest]
fn test_struct_with_optional_pair() {
    let s = MakeStructWithOptionalPair(3, 4);
    // Since it's inside `optional` which is inside a struct, `pair` is forced layout-compatible.
    let pair = s.opt_pair.as_ref().expect("should be engaged");
    expect_eq!(pair.first, 3);
    expect_eq!(pair.second, 4);
}

#[gtest]
fn test_plain_pair() {
    let p = MakePlainPair(5, 6);
    expect_eq!(p, (5, 6));
}
