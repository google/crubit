// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};

#[gtest]
fn test_get_hello_size() {
    let size =
        handwritten_cpp_with_support_deps_lib::GetHelloSizeFromHandwrittenCpp("hello".into());
    expect_that!(size, eq(5));
}
