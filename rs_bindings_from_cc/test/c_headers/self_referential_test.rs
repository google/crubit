// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_self_referential() {
    use self_referential::{kNodeAlign, kNodeSize, kNode_nextOffset, kNode_valueOffset, Node};
    expect_that!(core::mem::size_of::<Node>(), eq(unsafe { kNodeSize }));
    expect_that!(core::mem::align_of::<Node>(), eq(unsafe { kNodeAlign }));
    expect_that!(core::mem::offset_of!(Node, value), eq(unsafe { kNode_valueOffset }));
    expect_that!(core::mem::offset_of!(Node, next), eq(unsafe { kNode_nextOffset }));
}
