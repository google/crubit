// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use empty_cc_lib_with_additional_rust_srcs::additional_rust_srcs_test_stub::*;
use googletest::prelude::*;

#[gtest]
fn test_additional_rust_srcs() {
    assert_eq!(func_that_returns_1(), 1);
}
