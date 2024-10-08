// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use static_assertions::{assert_impl_all, assert_not_impl_any};

#[gtest]
fn test_unsafe_struct() {
    assert_not_impl_any!(thread_unsafe_types::Struct: Send, Sync);
}

#[gtest]
fn test_unsafe_union() {
    assert_not_impl_any!(thread_unsafe_types::Union: Send, Sync);
}

#[gtest]
fn test_enum() {
    assert_impl_all!(thread_safe_types::Enum: Send, Sync);
}
