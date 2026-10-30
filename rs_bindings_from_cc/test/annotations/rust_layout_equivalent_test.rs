// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{assert_that, gtest};
use rust_layout_equivalent::{crubit::test, SameCrateRustType, TaggedValue};
use rust_layout_equivalent_other_crate::ExternalRustType;

#[gtest]
fn test_same_crate_layout_equivalent() {
    let out = test::AddOneToSameCrate(SameCrateRustType { x: 41 });
    assert_that!(out, eq(SameCrateRustType { x: 42 }));
}

#[gtest]
fn test_generic_and_const_generic_layout_equivalent() {
    let sum = test::AddTag(TaggedValue { value: 32 });
    assert_that!(sum, eq(42));
}

#[gtest]
fn test_external_crate_with_label_hint() {
    let out = test::DoubleExternal(ExternalRustType { value: 21 });
    assert_that!(out, eq(ExternalRustType { value: 42 }));
}
