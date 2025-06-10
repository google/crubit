// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_wrapped_library() {
    let x = wrapper_library::get_global();
    x.set(100);
    expect_that!(x.get(), eq(100));
}
