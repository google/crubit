// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use enums::{EnumWithCollide, EnumWithOneCollide, ScopedEnum, UnscopedEnum};
use googletest::{expect_that, gtest, matchers::eq};

#[gtest]
fn test_unscoped_enum() {
    const ZERO: UnscopedEnum = UnscopedEnum::from_raw(0);
    const ONE: UnscopedEnum = UnscopedEnum::from_raw(1);

    expect_that!(ZERO, eq(UnscopedEnum::kZero));
    expect_that!(ONE, eq(UnscopedEnum::kOne));
    expect_that!(ZERO.into_raw(), eq(0));
    expect_that!(ONE.into_raw(), eq(1));
}

#[gtest]
fn test_scoped_enum() {
    const ZERO: ScopedEnum = ScopedEnum::from_raw(0);
    const ONE: ScopedEnum = ScopedEnum::from_raw(1);

    expect_that!(ZERO, eq(ScopedEnum::kZero));
    expect_that!(ONE, eq(ScopedEnum::kOne));
    expect_that!(ZERO.into_raw(), eq(0));
    expect_that!(ONE.into_raw(), eq(1));
}

#[gtest]
fn test_collide() {
    // There shouldn't be `from_raw` or `into_raw` methods because of collision.
    // We can only check the associated consts.
    let _a = EnumWithCollide::from_raw;
    let _b = EnumWithCollide::into_raw;
}

#[gtest]
fn test_one_collide() {
    // `from_raw` collides, so we shouldn't have `from_raw` method.
    // But `into_raw` should still exist.
    const OTHER: EnumWithOneCollide = EnumWithOneCollide::kOther;
    expect_that!(OTHER.into_raw(), eq(1));
}
