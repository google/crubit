// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_get_string() {
    let x = wrapper_library::get_string();
    expect_that!(&*x, eq(b"hello, world!"));
}

#[gtest]
fn test_copy_string() {
    let x = "hello, world!".into();
    let y = wrapper_library::copy_string(&x);
    expect_that!(&*x, eq(&*y));
}
