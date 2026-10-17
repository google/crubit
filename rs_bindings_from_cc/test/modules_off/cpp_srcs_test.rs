// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_that, gtest, matchers::eq};

#[gtest]
fn test_cpp_srcs_transitive_include() {
    expect_that!(cpp_srcs_lib::transitive_get_value(7), eq(7));
    expect_that!(cpp_srcs_lib::CppSrcsLibGetValue(1), eq(12));
}
