// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
use parity_decls::parity::{IsEven, IsOdd};

#[gtest]
fn test_is_even() {
    expect_that!(IsEven(0), eq(true));
    expect_that!(IsEven(7), eq(false));
    expect_that!(IsEven(10), eq(true));
}

#[gtest]
fn test_is_odd_via_cpp() {
    expect_that!(IsOdd(0), eq(false));
    expect_that!(IsOdd(7), eq(true));
    expect_that!(IsOdd(10), eq(false));
}

#[gtest]
fn test_is_odd_via_rust() {
    expect_that!(is_odd::is_odd(0), eq(false));
    expect_that!(is_odd::is_odd(7), eq(true));
    expect_that!(is_odd::is_odd(10), eq(false));
}
