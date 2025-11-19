// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::{emplace, CtorNew};
use default_member_functions::{Uncopyable, UncopyableDespiteDecl};
use googletest::gtest;
use static_assertions::assert_not_impl_any;

#[gtest]
fn test_can_make_uncopyable_struct() {
    assert_not_impl_any!(Uncopyable: Clone, Copy, Default);
    let _ = emplace!(Uncopyable::ctor_new(()));
}

#[gtest]
fn test_can_make_uncopyable_despite_decl_struct() {
    assert_not_impl_any!(UncopyableDespiteDecl: Clone, Copy, Default);
    let _ = emplace!(UncopyableDespiteDecl::ctor_new(()));
}
