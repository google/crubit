// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cref::{CMut, CRef};
use googletest::{expect_eq, gtest};

#[gtest]
fn test_value_or_default() {
    let x = 42;
    expect_eq!(assumed_optional_ref::ValueOrDefault(Some(&x), -1), 42);
    expect_eq!(assumed_optional_ref::ValueOrDefault(None, -1), -1);
}

#[gtest]
fn test_increment_if_present() {
    let mut x = 10;
    assumed_optional_ref::IncrementIfPresent(Some(&mut x));
    expect_eq!(x, 11);
    assumed_optional_ref::IncrementIfPresent(None);
    expect_eq!(x, 11);
}

#[gtest]
fn test_identity_const_optional_ref() {
    let x = 99;
    let out: Option<CRef<'_, i32>> = assumed_optional_ref::IdentityConstOptionalRef(Some(&x));
    expect_eq!(out.map(CRef::as_ptr), Some(&raw const x));
    let none_out: Option<CRef<'_, i32>> = assumed_optional_ref::IdentityConstOptionalRef(None);
    expect_eq!(none_out.map(CRef::as_ptr), None);
}

#[gtest]
fn test_identity_mut_optional_ref() {
    let mut x = 77;
    let px = &raw const x;
    let out: Option<CMut<'_, i32>> = assumed_optional_ref::IdentityMutOptionalRef(Some(&mut x));
    expect_eq!(out.map(CMut::as_ptr), Some(px));
    let none_out: Option<CMut<'_, i32>> = assumed_optional_ref::IdentityMutOptionalRef(None);
    expect_eq!(none_out.map(CMut::as_ptr), None);
}

#[gtest]
fn test_filter_positive() {
    let pos = 5;
    let out = assumed_optional_ref::FilterPositive(Some(&pos));
    expect_eq!(out.map(CRef::as_ptr), Some(&raw const pos));

    let neg = -5;
    let none_out = assumed_optional_ref::FilterPositive(Some(&neg));
    expect_eq!(none_out.map(CRef::as_ptr), None);

    let empty_out = assumed_optional_ref::FilterPositive(None);
    expect_eq!(empty_out.map(CRef::as_ptr), None);
}
