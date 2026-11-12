// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
#[gtest]
fn test_static_assert_known_failure() {

    // See KNOWN_FAILURES.md: C11 `_Static_assert` fails parsing in C++ mode.
}
