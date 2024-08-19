// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_round_trip() {
    let value = leaf_rs_lib::wrap(4);
    assert_eq!(middle_cc_lib::crubit::Unwrap(value), 4);

    let value = middle_cc_lib::crubit::Wrap(2);
    assert_eq!(leaf_rs_lib::unwrap(value), 2);
}

#[gtest]
fn test_enum_round_trip() {
    let value = leaf_rs_lib::wrap_enum(2);
    assert_eq!(middle_cc_lib::crubit::UnwrapEnum(value), 2);

    let value = middle_cc_lib::crubit::WrapEnum(1);
    assert_eq!(leaf_rs_lib::unwrap_enum(value), 1);
}
