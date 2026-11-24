// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
use item_exists::value_exists;

#[gtest]
fn test_get_hello_size() {
    expect_that!(embedded_cpp_with_support_deps_lib::get_hello_size("hello"), eq(5));
}

#[gtest]
fn test_cpp_deps_mappings_are_local() {
    expect_that!(dependent_without_cpp_deps_lib::DependentBoundFn(), eq(7));
    expect_that!(
        value_exists!(dependent_without_cpp_deps_lib::DependentUnboundStrRefFn),
        eq(false)
    );
}
