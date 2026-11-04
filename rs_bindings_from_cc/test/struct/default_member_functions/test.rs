// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::{emplace, CtorNew};
use default_member_functions::{Uncopyable, UncopyableDespiteDecl};
use googletest::gtest;
use static_assertions::{assert_impl_all, assert_not_impl_any};

#[gtest]
fn test_can_make_uncopyable_struct() {
    assert_not_impl_any!(Uncopyable: Clone, Copy, Default);
    let _ = emplace!(Uncopyable::ctor_new(()));
}

#[gtest]
fn test_can_make_uncopyable_despite_decl_struct() {
    assert_not_impl_any!(UncopyableDespiteDecl: Clone, Copy);
    // `std::vector` is bitwise-relocatable regardless of its element type, so
    // the struct is Rust-movable and gets a `Default` impl, but its copy
    // constructor would fail to instantiate, so it is still not `Clone`.
    assert_impl_all!(UncopyableDespiteDecl: Default, Unpin);
    let _ = emplace!(UncopyableDespiteDecl::ctor_new(()));
    let _ = UncopyableDespiteDecl::default();
}
