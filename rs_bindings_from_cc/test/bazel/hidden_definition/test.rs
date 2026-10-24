// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// The main thing this test checks is that `lib`'s generated C++ code compiles:
// it must not refer to declarations (such as `hidden_ns::Payload`) that are
// only reachable through the definition of `hidden_ns::Holder`, which isn't
// visible there.

use googletest::{expect_eq, gtest};

#[gtest]
fn test_lib() {
    expect_eq!(lib::lib_ns::ReturnsOne(), 1);
}
