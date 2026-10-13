// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
use send_sync_inference::crubit::test::*;
use static_assertions::{assert_impl_all, assert_not_impl_any};

#[gtest]
fn test_plain_data_is_send_sync() {
    assert_impl_all!(Plain: Send, Sync);
    assert_impl_all!(Empty: Send, Sync);
    assert_impl_all!(ContainsPlain: Send, Sync);
}

#[gtest]
fn test_pointers_are_not_send_sync() {
    assert_not_impl_any!(WithPointer: Send, Sync);
    assert_not_impl_any!(ContainsWithPointer: Send, Sync);
}

#[gtest]
fn test_opaque_bytes_are_not_send_sync() {
    assert_not_impl_any!(WithPrivateField: Send, Sync);
    assert_not_impl_any!(WithUnsupportedField: Send, Sync);
    assert_not_impl_any!(Derived: Send, Sync);
}

#[gtest]
fn test_explicit_opt_outs() {
    assert_not_impl_any!(NotSend: Send);
    assert_impl_all!(NotSend: Sync);

    assert_impl_all!(NotSync: Send);
    assert_not_impl_any!(NotSync: Sync);

    assert_not_impl_any!(NotSendNotSync: Send, Sync);
}

#[gtest]
fn test_explicit_unsafe_impl() {
    assert_impl_all!(ExplicitSendSync: Send, Sync);
}
