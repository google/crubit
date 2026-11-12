// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_typedef_anon() {
    use typedef_anon::{
        kPointTypedefAlign, kPointTypedefSize, kPointTypedef_xOffset, kPointTypedef_yOffset,
        ColorTypedef, PointAlias, PointChainAlias, PointTypedef,
    };
    expect_that!(core::mem::size_of::<PointTypedef>(), eq(unsafe { kPointTypedefSize }));
    expect_that!(core::mem::align_of::<PointTypedef>(), eq(unsafe { kPointTypedefAlign }));
    expect_that!(core::mem::offset_of!(PointTypedef, x), eq(unsafe { kPointTypedef_xOffset }));
    expect_that!(core::mem::offset_of!(PointTypedef, y), eq(unsafe { kPointTypedef_yOffset }));
    expect_that!(core::mem::size_of::<PointAlias>(), eq(unsafe { kPointTypedefSize }));
    expect_that!(core::mem::size_of::<PointChainAlias>(), eq(unsafe { kPointTypedefSize }));
    let _ = ColorTypedef::RED;
}
