// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_false, expect_true, gtest};
use inherited_operators::ns::Comparable;
use inherited_operators::Equatable;
use static_assertions::{assert_impl_all, assert_not_impl_any};

#[gtest]
fn test_eq_inherited_from_base() {
    assert_impl_all!(Equatable: PartialEq);
    // The C++ operators only compare `key`.
    let a = Equatable { key: 1, payload: 10 };
    let b = Equatable { key: 1, payload: 20 };
    let c = Equatable { key: 2, payload: 10 };
    expect_true!(a == b);
    expect_false!(a != b);
    expect_false!(a == c);
    expect_true!(a != c);
}

#[gtest]
fn test_no_partial_ord_without_inherited_lt() {
    assert_not_impl_any!(Equatable: PartialOrd);
}

#[gtest]
fn test_eq_and_lt_inherited_from_indirect_bases() {
    assert_impl_all!(Comparable: PartialEq, PartialOrd);
    // The C++ operators only compare `key`.
    let a = Comparable { key: 1, payload: 10 };
    let b = Comparable { key: 1, payload: 20 };
    let c = Comparable { key: 2, payload: 0 };
    expect_true!(a == b);
    expect_true!(a != c);
    expect_false!(a < b);
    expect_true!(a <= b);
    expect_true!(a < c);
    expect_true!(c > a);
    expect_true!(c >= b);
}
