// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::shared_ptr;
use cc_std::std::NonNull;
use crubit_annotate::must_bind;

#[must_bind]
pub fn roundtrip_shared_ptr(val: shared_ptr<i32>) -> shared_ptr<i32> {
    val
}

#[must_bind]
pub fn clone_shared_ptr(val: &shared_ptr<i32>) -> shared_ptr<i32> {
    val.clone()
}

#[must_bind]
pub fn consume_shared_ptr(_val: shared_ptr<i32>) {}

/// `NonNull<Ptr>` is spelled in C++ as `Ptr` plus the `crubit_nonnull` attribute.
#[must_bind]
pub fn roundtrip_nonnull_shared_ptr(val: NonNull<shared_ptr<i32>>) -> NonNull<shared_ptr<i32>> {
    val
}
