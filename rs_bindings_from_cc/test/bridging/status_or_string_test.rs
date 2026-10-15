// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::{emplace, CtorNew};
use googletest::matchers::{anything, err as is_err, ok as is_ok};
use googletest::{expect_eq, expect_that, gtest};
use status_or_string_lib::{
    MakeOptionalOrError, MakeOptionalStringOrError, MakeStringOrError, NonTrivialToken, TokenHelper,
};

#[gtest]
fn test_status_or_string_without_layout_compat_string_feature() {
    let ok = emplace!(MakeStringOrError(true));
    expect_that!(&*ok, is_ok(anything()));
    let res = &*ok;
    let s = res.as_ref().unwrap();
    expect_eq!(s.as_slice(), &b"hello"[..]);

    let err = emplace!(MakeStringOrError(false));
    expect_that!(&*err, is_err(anything()));
}

#[gtest]
fn test_status_or_optional_without_layout_compat_optional_feature() {
    let ok = emplace!(MakeOptionalOrError(true));
    expect_that!(&*ok, is_ok(anything()));
    let res = &*ok;
    let opt = res.as_ref().unwrap();
    expect_eq!(opt.as_ref(), Some(&42));

    let err = emplace!(MakeOptionalOrError(false));
    expect_that!(&*err, is_err(anything()));
}

#[gtest]
fn test_status_or_optional_string_nested() {
    let ok = emplace!(MakeOptionalStringOrError(true));
    expect_that!(&*ok, is_ok(anything()));
    let res = &*ok;
    let opt = res.as_ref().unwrap();
    let s = opt.as_ref().unwrap();
    expect_eq!(s.as_slice(), &b"world"[..]);

    let err = emplace!(MakeOptionalStringOrError(false));
    expect_that!(&*err, is_err(anything()));
}

#[gtest]
fn test_method_with_ctor_param_returning_status_or_string() {
    let helper = TokenHelper::default();
    let ok = emplace!(helper.GetStringWithToken(NonTrivialToken::ctor_new(1)));
    expect_that!(&*ok, is_ok(anything()));
    let res = &*ok;
    let s = res.as_ref().unwrap();
    expect_eq!(s.as_slice(), &b"token_ok"[..]);

    let err = emplace!(helper.GetStringWithToken(NonTrivialToken::ctor_new(-1)));
    expect_that!(&*err, is_err(anything()));
}
