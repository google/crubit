// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Regression test for b/555410712: types with
//! `ABSL_REQUIRE_EXPLICIT_INIT` fields must not get `Default`/`CtorNew`
//! bindings, because the generated C++ thunk would not compile.

use googletest::prelude::*;
use require_explicit_init::*;
use static_assertions::{assert_impl_all, assert_not_impl_any};

assert_impl_all!(NoExplicitInitField: Default);
// The attribute is ignored in non-aggregates, so bindings must be unaffected.
assert_impl_all!(NonAggregateWithIgnoredAttr: Default);

assert_not_impl_any!(HasExplicitInitField: Default);
assert_not_impl_any!(ContainsExplicitInitField: Default);
assert_not_impl_any!(DerivesExplicitInitField: Default);

#[gtest]
fn control_type_is_default_constructible() {
    let x = NoExplicitInitField::default();
    expect_eq!(x.a, 0);
}

#[gtest]
fn explicit_init_types_are_still_constructible_field_wise() {
    let x = HasExplicitInitField { a: 1, b: 2 };
    expect_eq!(x.b, 2);
}
