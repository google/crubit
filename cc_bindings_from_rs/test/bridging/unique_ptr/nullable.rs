// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests that `Nullable<Ptr>` is bridged to C++ as `Ptr crubit_nullable`.
//!
//! This crate deliberately does not enable `nonnull_smart_pointers`: unlike `NonNull`, the
//! `Nullable` bridge is not gated on it.

use cc_std::std::shared_ptr;
use cc_std::std::unique_ptr;
use cc_std::std::vector;
use cc_std::std::virtual_unique_ptr;
use cc_std::std::Nullable;
use cc_std::std::OptionLike;
use crubit_annotate::must_bind;
use test_helpers::unique_ptr_test::Base;
use test_helpers::unique_ptr_test::Target;

#[must_bind]
pub fn roundtrip_nullable_unique_ptr(
    val: Nullable<unique_ptr<Target>>,
) -> Nullable<unique_ptr<Target>> {
    val
}

#[must_bind]
pub fn roundtrip_nullable_virtual_unique_ptr(
    val: Nullable<virtual_unique_ptr<Base>>,
) -> Nullable<virtual_unique_ptr<Base>> {
    val
}

#[must_bind]
pub fn roundtrip_nullable_shared_ptr(val: Nullable<shared_ptr<i32>>) -> Nullable<shared_ptr<i32>> {
    val
}

#[must_bind]
pub fn is_null_unique_ptr(val: &Nullable<unique_ptr<Target>>) -> bool {
    val.as_option().is_none()
}

#[must_bind]
pub fn make_null_unique_ptr() -> Nullable<unique_ptr<Target>> {
    Nullable::default()
}

// `Nullable<Ptr>` as a template argument: bridged as `std::vector<Ptr crubit_nullable>`.

#[must_bind]
pub fn roundtrip_vector_of_nullable_unique_ptr(
    val: vector<Nullable<unique_ptr<Target>>>,
) -> vector<Nullable<unique_ptr<Target>>> {
    val
}

#[must_bind]
pub fn count_null_unique_ptrs(val: vector<Nullable<unique_ptr<Target>>>) -> i32 {
    val.iter().filter(|p| p.as_option().is_none()).count() as i32
}

#[must_bind]
pub fn make_vector_of_null_unique_ptr() -> vector<Nullable<unique_ptr<Target>>> {
    let mut v = vector::new();
    v.push(Nullable::default());
    v
}

#[must_bind]
pub fn roundtrip_vector_of_nullable_shared_ptr(
    val: vector<Nullable<shared_ptr<i32>>>,
) -> vector<Nullable<shared_ptr<i32>>> {
    val
}
