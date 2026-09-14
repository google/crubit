// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::TryDeref;
use common::OverloadedDelete;
use googletest::prelude::*;
use nonnull_smart_pointer_lib::{
    MakeNonnullSharedPtr, MakeNonnullUniquePtr, MakeNonnullVirtualUniquePtr, MakeUniquePtr,
    UseNonnullSharedPtrByValue, UseNonnullUniquePtrByValue,
};

#[gtest]
fn test_nonnull_unique_ptr_round_trips() {
    let p: cc_std::std::NonNull<cc_std::std::unique_ptr<i32>> = MakeNonnullUniquePtr(1);
    // `NonNull` derefs straight through to the pointee: no unwrapping, and no null check.
    expect_eq!(*p, 1);
    assert_eq!(UseNonnullUniquePtrByValue(p), 1);
}

#[gtest]
fn test_nonnull_shared_ptr_round_trips() {
    let p: cc_std::std::NonNull<cc_std::std::shared_ptr<i32>> = MakeNonnullSharedPtr(2);
    expect_eq!(*p, 2);
    expect_eq!(UseNonnullSharedPtrByValue(p), 2);
}

/// An element type that overloads `operator delete` produces `virtual_unique_ptr`, which is a
/// separate branch in the code that spells the type.
#[gtest]
fn test_nonnull_virtual_unique_ptr() {
    let _: cc_std::std::NonNull<cc_std::std::virtual_unique_ptr<OverloadedDelete>> =
        MakeNonnullVirtualUniquePtr();
}

/// Without the annotation the type is unchanged, so `NonNull` only ever appears where the C++
/// author asked for it.
#[gtest]
fn test_unannotated_unique_ptr_is_not_wrapped() {
    let p: cc_std::std::unique_ptr<i32> = MakeUniquePtr(3);
    expect_eq!(p.try_deref(), Some(&3));
}
