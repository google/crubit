// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::emplace;
use ctor::CtorNew as _;
use googletest::gtest;
use item_exists::value_exists;

#[gtest]
fn test_int_accessor() {
    let mut s = semantic_import::S::from(42);
    assert_eq!(s.x(), 42);
    s.set_x(100);
    assert_eq!(s.x(), 100);
    assert_eq!(s.get_x(), 100);
}

#[gtest]
fn test_float_and_inherited_int_accessor() {
    let t = emplace!(semantic_import::T::ctor_new((42, 1.23f32)));
    assert_eq!(t.y(), 1.23f32);
    assert!(!value_exists!(semantic_import::t::x));
    assert!(!value_exists!(semantic_import::t::get_x));
}

#[gtest]
fn test_char_accessors() {
    let mut chars = semantic_import::Chars::default();
    assert_eq!(chars.c(), b'c');
    assert_eq!(chars.sc(), b's' as i8);
    assert_eq!(chars.uc(), b'u');
    chars.set_c(b'C'.into());
    chars.set_sc(b'S' as i8);
    chars.set_uc(b'U');
    assert_eq!(chars.c(), b'C');
    assert_eq!(chars.sc(), b'S' as i8);
    assert_eq!(chars.uc(), b'U');
}

#[gtest]
fn test_bool_accessors() {
    let mut bools = semantic_import::Bools::default();
    assert_eq!(bools.b(), true);
    bools.set_b(false);
    assert_eq!(bools.b(), false);
}
