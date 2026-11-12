// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_nested_scope() {
    use nested_scope::{kOuterScopeAlign, kOuterScopeSize, kOuterScope_innerOffset, OuterScope};
    expect_that!(core::mem::size_of::<OuterScope>(), eq(unsafe { kOuterScopeSize }));
    expect_that!(core::mem::align_of::<OuterScope>(), eq(unsafe { kOuterScopeAlign }));
    expect_that!(core::mem::offset_of!(OuterScope, inner), eq(unsafe { kOuterScope_innerOffset }));
}
