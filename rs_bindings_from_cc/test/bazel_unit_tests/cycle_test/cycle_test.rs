// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_my_type_adder() {
    // other_func() returns 42, m.Add(0, 0) returns 0.
    expect_eq!(original_library::MyType::add(0, 0), 42);
}

#[gtest]
fn test_my_struct_adder() {
    let s = original_library::MyStruct { a: 1, b: 2 };
    expect_eq!(original_library::MyStructAdder(s), 3);
}
