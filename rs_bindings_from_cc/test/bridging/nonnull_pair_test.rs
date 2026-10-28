// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::{unique_ptr, Nullable, OptionLike};
use googletest::prelude::*;
use nonnull_pair_lib::{MakePairOfNonnull, MakePairOfNonnullSecond, MakePairOfNullNullable};

/// Nullability written on a template argument of a `std::pair` applies to that element.
#[gtest]
fn test_pair_of_nonnull_unique_ptr() {
    let (p, n): (unique_ptr<i32>, i32) = MakePairOfNonnull(7);
    expect_eq!(*p, 7);
    expect_eq!(n, 8);
}

#[gtest]
fn test_pair_of_nonnull_unique_ptr_second() {
    let (n, p): (i32, unique_ptr<i32>) = MakePairOfNonnullSecond(7);
    expect_eq!(n, 8);
    expect_eq!(*p, 7);
}

#[gtest]
fn test_pair_of_nullable_unique_ptr() {
    let (p, n): (Nullable<unique_ptr<i32>>, i32) = MakePairOfNullNullable();
    expect_true!(p.as_option().is_none());
    expect_eq!(n, 0);
}
