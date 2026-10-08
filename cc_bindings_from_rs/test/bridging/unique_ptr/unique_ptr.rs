// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::shared_ptr;
use cc_std::std::unique_ptr;
use cc_std::std::virtual_unique_ptr;
use cc_std::std::NonNull;
use crubit_annotate::must_bind;
use test_helpers::unique_ptr_test::Base;
use test_helpers::unique_ptr_test::Target;

#[must_bind]
pub fn roundtrip_unique_ptr(val: unique_ptr<Target>) -> unique_ptr<Target> {
    val
}

#[must_bind]
pub fn create_unique_ptr(token: shared_ptr<i32>) -> unique_ptr<Target> {
    test_helpers::unique_ptr_test::create_target(token)
}

#[must_bind]
pub fn consume_unique_ptr(_val: unique_ptr<Target>) {}

#[must_bind]
pub fn roundtrip_virtual_unique_ptr(val: virtual_unique_ptr<Base>) -> virtual_unique_ptr<Base> {
    val
}

#[must_bind]
pub fn create_virtual_unique_ptr(token: shared_ptr<i32>) -> virtual_unique_ptr<Base> {
    test_helpers::unique_ptr_test::create_virtual_base(token)
}

#[must_bind]
pub fn consume_virtual_unique_ptr(_val: virtual_unique_ptr<Base>) {}

#[must_bind]
pub fn accept_unique_ptr_tuple(val: unique_ptr<(i32, i32)>) -> unique_ptr<(i32, i32)> {
    val
}

#[must_bind]
pub fn accept_unique_ptr_option(val: unique_ptr<Option<i32>>) -> unique_ptr<Option<i32>> {
    val
}

/// `NonNull<Ptr>` is spelled in C++ as `Ptr` plus the `crubit_nonnull` attribute, so passing one
/// by value exercises the wrapped pointer's own movability rather than `NonNull`'s.
#[must_bind]
pub fn roundtrip_nonnull_unique_ptr(
    val: NonNull<unique_ptr<Target>>,
) -> NonNull<unique_ptr<Target>> {
    val
}

#[must_bind]
pub fn roundtrip_nonnull_virtual_unique_ptr(
    val: NonNull<virtual_unique_ptr<Base>>,
) -> NonNull<virtual_unique_ptr<Base>> {
    val
}
