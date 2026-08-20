// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::{eq, ok as is_ok};
use googletest::{expect_that, expect_true, gtest};

#[gtest]
fn test_returns_status() {
    let status = cpp_api::ReturnsStatus(true);
    expect_true!(status.is_ok());

    let status = cpp_api::ReturnsStatus(false);
    expect_true!(status.is_err());
}

#[gtest]
fn test_returns_status_or_int() {
    let status_or_int = cpp_api::ReturnsStatusOrInt(true);
    expect_that!(status_or_int, is_ok(eq(&42)));

    let status_or_int = cpp_api::ReturnsStatusOrInt(false);
    expect_true!(status_or_int.is_err());
}
