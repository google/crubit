// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::TryDeref;
use common::OverloadedDelete;
use googletest::prelude::*;
use nonnull_smart_pointer_lib::{
    MakeDefaultNonnullUniquePtr, MakeExplicitlyNullableUniquePtr, MakeNonnullSharedPtr,
    MakeNonnullUniquePtr, MakeNonnullVirtualUniquePtr, MakeUniquePtr, MakeVectorOfNonnull,
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

/// Under `ABSL_POINTERS_DEFAULT_NONNULL`, unannotated smart pointers are non-null.
#[gtest]
fn test_default_nonnull_unique_ptr_is_wrapped() {
    let p: cc_std::std::NonNull<cc_std::std::unique_ptr<i32>> = MakeDefaultNonnullUniquePtr(4);
    expect_eq!(*p, 4);
}

/// An explicit `absl_nullable` overrides `ABSL_POINTERS_DEFAULT_NONNULL`.
#[gtest]
fn test_explicitly_nullable_unique_ptr_is_not_wrapped() {
    let p: cc_std::std::unique_ptr<i32> = MakeExplicitlyNullableUniquePtr(5);
    expect_eq!(p.try_deref(), Some(&5));
}

/// Nullability written on a template argument applies to that argument.
#[gtest]
fn test_vector_of_nonnull_unique_ptr() {
    let v: cc_std::std::vector<cc_std::std::NonNull<cc_std::std::unique_ptr<i32>>> =
        MakeVectorOfNonnull(6);
    expect_eq!(v.len(), 1);
    expect_eq!(*v[0], 6);
}
