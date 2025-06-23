// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use item_exists::{type_exists, value_exists};

#[gtest]
fn test_forward_declared_type() {
    expect_that!(pub_crate_types::create_and_consume_forward_declared(), eq(42));
}

#[gtest]
fn test_forward_declared_type_visibility() {
    // can't really test the field visibility, but we can test the visibility of the types and
    // functions.
    expect_pred!(!type_exists!(pub_crate_types::ForwardDeclared));
    expect_pred!(!type_exists!(pub_crate_types::ForwardDeclaredAlias));
    expect_pred!(!value_exists!(pub_crate_types::CreateForwardDeclared));

    // The compound data type does exist, but its field is private.
    expect_pred!(value_exists!(pub_crate_types::ConsumeCompoundDataType));
}

#[gtest]
fn test_other_pub_crate_types() {
    expect_pred!(!value_exists!(pub_crate_types::OtherPubCrateTypes));
}

#[gtest]
fn test_templated_type() {
    expect_pred!(!type_exists!(pub_crate_types::Template));

    expect_that!(pub_crate_types::get_int_from_template_int(), eq(42));
}

#[gtest]
fn test_other_templated_type() {
    expect_that!(pub_crate_types::get_int_from_template2_int(), eq(42));
}
