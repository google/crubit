// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_lib_with_additional_rust_srcs_try_trait::check_try_trait_available;
use googletest::assert_that;
use googletest::gtest;
use googletest::matchers::eq;

#[gtest]
fn test_try_trait() {
    assert_that!(check_try_trait_available(Some(1)), eq(true));
}
