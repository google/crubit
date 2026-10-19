// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::assert_that;
use googletest::gtest;
use googletest::matchers::eq;

#[gtest]
fn test_types_feature() {
    let s = features_lib_types::SomeType { value: 42 };
    assert_that!(s.value, eq(42));
}

#[gtest]
fn test_multiple_features() {
    fn assert_send_sync_debug<T: Send + Sync + std::fmt::Debug>() {}
    assert_send_sync_debug::<features_lib_multi::SomeType>();
    let s = features_lib_multi::SomeType { value: 42 };
    assert_that!(s.value, eq(42));
    assert_that!(features_lib_multi::SomeFunction(), eq(42));
}

#[gtest]
fn test_types_with_extra_srcs() {
    let s = features_lib_srcs::SomeType { value: 42 };
    assert_that!(s.get_value_plus_one(), eq(43));
}
