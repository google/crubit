// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_basic_structs() {
    use basic_structs::{
        kDataAlign, kDataSize, kData_fOffset, kData_iOffset, kPointAlign, kPointSize,
        kPoint_xOffset, kPoint_yOffset, Data, Point,
    };

    let p = Point { x: 1, y: 2 };
    expect_that!(p.x, eq(1));
    expect_that!(core::mem::size_of::<Point>(), eq(unsafe { kPointSize }));
    expect_that!(core::mem::align_of::<Point>(), eq(unsafe { kPointAlign }));
    expect_that!(core::mem::offset_of!(Point, x), eq(unsafe { kPoint_xOffset }));
    expect_that!(core::mem::offset_of!(Point, y), eq(unsafe { kPoint_yOffset }));

    expect_that!(core::mem::size_of::<Data>(), eq(unsafe { kDataSize }));
    expect_that!(core::mem::align_of::<Data>(), eq(unsafe { kDataAlign }));
    expect_that!(core::mem::offset_of!(Data, i), eq(unsafe { kData_iOffset }));
    expect_that!(core::mem::offset_of!(Data, f), eq(unsafe { kData_fOffset }));
}
