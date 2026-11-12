// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_same_name_tag() {
    use same_name_tag::{kTagSameNameAlign, kTagSameNameSize, kTagSameName_xOffset, TagSameName};
    expect_that!(core::mem::size_of::<TagSameName>(), eq(unsafe { kTagSameNameSize }));
    expect_that!(core::mem::align_of::<TagSameName>(), eq(unsafe { kTagSameNameAlign }));
    expect_that!(core::mem::offset_of!(TagSameName, x), eq(unsafe { kTagSameName_xOffset }));
}
