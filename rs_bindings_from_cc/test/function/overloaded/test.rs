// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use googletest::prelude::*;
use item_exists::value_exists;

#[gtest]
fn test_ambiguous() {
    expect_false!(value_exists!(overloaded::Ambiguous));
    expect_false!(value_exists!(overloaded::AmbiguousDeprecated));
}

#[gtest]
fn test_canonical_nondeprecated() {
    overloaded::CanonicalNonDeprecated();
}
