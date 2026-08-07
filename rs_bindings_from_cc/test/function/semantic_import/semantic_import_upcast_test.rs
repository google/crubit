// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::emplace;
use ctor::CtorNew as _;
use googletest::gtest;

#[gtest]
fn test_int_accessor() {
    let mut s = semantic_import_upcast::S::from(42);
    assert_eq!(s.x(), 42);
    s.set_x(100);
    assert_eq!(s.x(), 100);
    assert_eq!(s.get_x(), 100);
}

#[gtest]
fn test_float_and_inherited_int_accessor() {
    let mut t = emplace!(semantic_import_upcast::T::ctor_new((42, 1.23f32)));
    assert_eq!(t.x(), 42);
    assert_eq!(t.y(), 1.23f32);
    t.set_x(100);
    assert_eq!(t.x(), 100);
    assert_eq!(t.get_x(), 100);
}
