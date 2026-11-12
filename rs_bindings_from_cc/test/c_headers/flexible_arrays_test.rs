// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_flexible_arrays() {
    use flexible_arrays::{kFlexAlign, kFlexSize, kFlex_lengthOffset, Flex};
    expect_that!(core::mem::size_of::<Flex>(), eq(unsafe { kFlexSize }));
    expect_that!(core::mem::align_of::<Flex>(), eq(unsafe { kFlexAlign }));
    expect_that!(core::mem::offset_of!(Flex, length), eq(unsafe { kFlex_lengthOffset }));
    // data is skipped because flexible array is unsupported
}
