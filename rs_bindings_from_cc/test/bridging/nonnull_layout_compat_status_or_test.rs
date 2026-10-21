// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::{unique_ptr, Nullable, OptionLike};
use googletest::prelude::*;
use nonnull_layout_compat_status_or_lib::{
    MakeLayoutCompatStatusOrOfNonnull, MakeLayoutCompatStatusOrOfNullNullable,
};

/// Nullability written on a template argument applies to that argument, including through a
/// layout-compatible `absl::StatusOr`.
#[gtest]
fn test_layout_compat_status_or_of_nonnull_unique_ptr() {
    let p: unique_ptr<i32> = MakeLayoutCompatStatusOrOfNonnull(7).ok().unwrap();
    expect_eq!(*p, 7);
}

#[gtest]
fn test_layout_compat_status_or_of_nullable_unique_ptr() {
    let p: Nullable<unique_ptr<i32>> = MakeLayoutCompatStatusOrOfNullNullable().ok().unwrap();
    expect_true!(p.as_option().is_none());
}
