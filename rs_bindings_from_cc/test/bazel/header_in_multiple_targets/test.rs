// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// The main thing this test checks is that `lib`'s bindings compile at all: if a
// declaration from `shared.h` were bound twice, rustc would reject the crate
// with E0428 ("the name `...` is defined multiple times").

use googletest::{expect_eq, expect_pred, gtest};
use item_exists::{type_exists, value_exists};

#[gtest]
fn test_shared_header_items_are_bound() {
    expect_pred!(type_exists!(lib::shared_ns::Info));
    expect_pred!(type_exists!(lib::shared_ns::Callback));
    expect_pred!(type_exists!(lib::shared_ns::IntAlias));
    expect_pred!(type_exists!(lib::shared_ns::InfoTypedef));
    expect_pred!(value_exists!(lib::shared_ns::kSharedConstant));
    expect_pred!(value_exists!(lib::shared_ns::kAnonymousEnumerator));
    expect_pred!(type_exists!(lib::shared_ns::Color));
    expect_pred!(value_exists!(lib::shared_ns::SharedFunction));
    expect_pred!(type_exists!(lib::other_ns::Info));
}

#[gtest]
fn test_shared_header_items_are_usable() {
    let x: lib::shared_ns::IntAlias = 1;
    expect_eq!(lib::lib_ns::LibFunction(x), 2);
    expect_eq!(lib::shared_ns::kSharedConstant, 3);
    expect_eq!(lib::shared_ns::SharedFunction(), 7);
    let info: lib::shared_ns::InfoTypedef = Default::default();
    expect_eq!(info.value(), 42);
    let other_info: lib::other_ns::Info = info;
    expect_eq!(other_info.value(), 42);
}
