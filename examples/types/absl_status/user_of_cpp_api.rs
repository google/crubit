// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_returns_status() {
    let status = cpp_api::ReturnsStatus(true);
    expect_true!(status.is_ok());
}

#[gtest]
fn test_returns_status_or_int() {
    let status_or_int = cpp_api::ReturnsStatusOrInt(true);
    expect_eq!(status_or_int, Ok(42));
}
