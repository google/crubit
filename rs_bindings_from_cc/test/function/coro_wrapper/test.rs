// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;

#[gtest]
fn test_coro_wrapper() {
    // We just need to verify that the function can be called.
    // Since it returns a fake type, we don't need to check the return value.
    coro_wrapper_lib::coro_test::my_coro_wrapper_function();
}
