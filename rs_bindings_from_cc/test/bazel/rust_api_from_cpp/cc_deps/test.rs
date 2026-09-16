// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, gtest};

#[gtest]
fn test_hello() {
    expect_eq!(lib::hello_from_cpp(), 42);
}

#[gtest]
fn test_extra() {
    expect_eq!(lib::call_dep_double(21), 42);
}
