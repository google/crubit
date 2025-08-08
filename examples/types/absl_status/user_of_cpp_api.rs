// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_returns_status() {
    let status = cpp_api::ReturnsStatus(true);
    expect_true!(status.is_ok());
}
