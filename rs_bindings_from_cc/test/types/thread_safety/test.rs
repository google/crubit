// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use static_assertions::assert_not_impl_any;

#[test]
fn test_unsafe_struct() {
    assert_not_impl_any!(thread_unsafe_types::Struct: Send, Sync);
}

#[test]
fn test_unsafe_enum() {
    assert_not_impl_any!(thread_unsafe_types::Enum: Send, Sync);
}

#[test]
fn test_unsafe_union() {
    assert_not_impl_any!(thread_unsafe_types::Union: Send, Sync);
}
