// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_unions() {
    use unions::{
        kUnionOfStructsAlign, kUnionOfStructsSize, kUnionWithArraysAlign, kUnionWithArraysSize,
        kUnionWithArrays_arr1Offset, kUnionWithArrays_arr2Offset, UnionOfStructs, UnionWithArrays,
    };
    expect_that!(core::mem::size_of::<UnionOfStructs>(), eq(unsafe { kUnionOfStructsSize }));
    expect_that!(core::mem::align_of::<UnionOfStructs>(), eq(unsafe { kUnionOfStructsAlign }));
    // s1 is opaque bytes
    // s2 is opaque bytes

    expect_that!(core::mem::size_of::<UnionWithArrays>(), eq(unsafe { kUnionWithArraysSize }));
    expect_that!(core::mem::align_of::<UnionWithArrays>(), eq(unsafe { kUnionWithArraysAlign }));
    expect_that!(
        core::mem::offset_of!(UnionWithArrays, arr1),
        eq(unsafe { kUnionWithArrays_arr1Offset })
    );
    expect_that!(
        core::mem::offset_of!(UnionWithArrays, arr2),
        eq(unsafe { kUnionWithArrays_arr2Offset })
    );
}
