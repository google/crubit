// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//TODO(rosica): We need namespaces in order to be able to test more here.

use cc_std::*;
use googletest::prelude::*;

#[gtest]
fn test_limits_inline() {
    // Tests of items from the `<limits>` header.
    // https://en.cppreference.com/w/cpp/types/numeric_limits/float_round_style:
    expect_eq!(i32::from(std::float_round_style::round_toward_zero), 0);
    expect_eq!(i32::from(std::float_round_style::round_to_nearest), 1);
    expect_eq!(i32::from(std::float_round_style::round_toward_infinity), 2);
    expect_eq!(i32::from(std::float_round_style::round_toward_neg_infinity), 3);
    expect_eq!(i32::from(std::float_round_style::round_indeterminate), -1);
}
