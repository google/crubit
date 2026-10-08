// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests `nonnull_smart_pointers` (crubit.rs-nullable): a smart pointer known to be non-null is a
//! bare Rust `Ptr`, and any other is `Nullable<Ptr>`.

use cc_std::std::{shared_ptr, unique_ptr, vector, virtual_unique_ptr, Nullable, OptionLike};
use common::OverloadedDelete;
use googletest::prelude::*;
use nonnull_smart_pointer_lib::{
    MakeDefaultNonnullUniquePtr, MakeExplicitlyNullableUniquePtr, MakeNonnullSharedPtr,
    MakeNonnullUniquePtr, MakeNonnullVirtualUniquePtr, MakeSharedPtr, MakeUniquePtr,
    MakeVectorOfNonnull, MakeVectorOfNullable, UseNonnullSharedPtrByValue,
    UseNonnullUniquePtrByValue, UseUniquePtrByValue,
};

#[gtest]
fn test_nonnull_unique_ptr_round_trips() {
    let p: unique_ptr<i32> = MakeNonnullUniquePtr(1);
    expect_eq!(*p, 1);
    expect_eq!(UseNonnullUniquePtrByValue(p), 1);
}

#[gtest]
fn test_nonnull_shared_ptr_round_trips() {
    let p: shared_ptr<i32> = MakeNonnullSharedPtr(2);
    expect_eq!(*p, 2);
    expect_eq!(UseNonnullSharedPtrByValue(p), 2);
}

/// An element type that overloads `operator delete` produces `virtual_unique_ptr`, which is a
/// separate branch in the code that spells the type.
#[gtest]
fn test_nonnull_virtual_unique_ptr() {
    let _: virtual_unique_ptr<OverloadedDelete> = MakeNonnullVirtualUniquePtr();
}

/// Without an annotation, a smart pointer may be null.
#[gtest]
fn test_unannotated_unique_ptr_is_nullable() {
    let p: Nullable<unique_ptr<i32>> = MakeUniquePtr(3);
    expect_eq!(p.as_option().map(|p| **p), Some(3));
    expect_eq!(UseUniquePtrByValue(p), 3);
    expect_eq!(UseUniquePtrByValue(Nullable::default()), -1);
}

#[gtest]
fn test_unannotated_shared_ptr_is_nullable() {
    let p: Nullable<shared_ptr<i32>> = MakeSharedPtr(3);
    expect_eq!(p.as_option().map(|p| **p), Some(3));
}

/// Under `ABSL_POINTERS_DEFAULT_NONNULL`, unannotated smart pointers are non-null.
#[gtest]
fn test_default_nonnull_unique_ptr_is_bare() {
    let p: unique_ptr<i32> = MakeDefaultNonnullUniquePtr(4);
    expect_eq!(*p, 4);
}

/// An explicit `absl_nullable` overrides `ABSL_POINTERS_DEFAULT_NONNULL`.
#[gtest]
fn test_explicitly_nullable_unique_ptr_is_nullable() {
    let p: Nullable<unique_ptr<i32>> = MakeExplicitlyNullableUniquePtr(5);
    expect_eq!(p.as_option().map(|p| **p), Some(5));
}

/// Nullability written on a template argument applies to that argument.
#[gtest]
fn test_vector_of_nonnull_unique_ptr() {
    let v: vector<unique_ptr<i32>> = MakeVectorOfNonnull(6);
    expect_eq!(v.len(), 1);
    expect_eq!(*v[0], 6);
}

#[gtest]
fn test_vector_of_nullable_unique_ptr() {
    let v: vector<Nullable<unique_ptr<i32>>> = MakeVectorOfNullable(7);
    expect_eq!(v.len(), 2);
    expect_eq!(v[0].as_option().map(|p| **p), Some(7));
    expect_true!(v[1].as_option().is_none());
}
