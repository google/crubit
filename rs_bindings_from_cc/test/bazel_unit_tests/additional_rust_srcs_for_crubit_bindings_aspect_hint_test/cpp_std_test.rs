// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{assert_that, gtest};

#[gtest]
fn test_implicit() {
    assert_that!(empty_cc_lib_with_implicit_cpp_std::implicit_cpp_std_works(), eq(true));
}

#[gtest]
fn test_explicit() {
    assert_that!(empty_cc_lib_with_explicit_cpp_std::explicit_cpp_std_works(), eq(true));
}
