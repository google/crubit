// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// `Box<int>` and `Box<int16_t>` are each instantiated in the precompiled
// modules of both `:a` and `:b`, but only one of the modules declares the
// implicit copy constructor. Clang keeps the definition from whichever module
// is loaded first, so for one of the two types, the copy constructor is
// declared only in the demoted definition. The same can happen to the implicit
// default constructor. Both types must get all their special members,
// regardless of the order the modules are loaded in.

use googletest::{expect_eq, gtest};

fn assert_copy<T: Copy>(_: &T) {}
fn assert_default<T: Default>(_: &T) {}

#[gtest]
fn test_int_box() {
    let box1 = lib::lib_ns::MakeIntBox(42);
    assert_copy(&box1);
    assert_default(&box1);
    let box2 = box1;
    expect_eq!(box1.value, 42);
    expect_eq!(box2.value, 42);
    expect_eq!(box1.GetOne(), 1);
}

#[gtest]
fn test_int16_box() {
    let box1 = lib::lib_ns::MakeInt16Box(42);
    assert_copy(&box1);
    assert_default(&box1);
    let box2 = box1;
    expect_eq!(box1.value, 42);
    expect_eq!(box2.value, 42);
    expect_eq!(box1.GetOne(), 1);
}
