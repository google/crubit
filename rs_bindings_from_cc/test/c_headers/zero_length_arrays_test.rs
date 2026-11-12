// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_zero_length_arrays() {
    use zero_length_arrays::{
        kMultiDimAlign, kMultiDimSize, kMultiDim_dataOffset, kZeroLengthAlign, kZeroLengthSize,
        kZeroLength_lengthOffset, MultiDim, ZeroLength,
    };
    expect_that!(core::mem::size_of::<ZeroLength>(), eq(unsafe { kZeroLengthSize }));
    expect_that!(core::mem::align_of::<ZeroLength>(), eq(unsafe { kZeroLengthAlign }));
    expect_that!(
        core::mem::offset_of!(ZeroLength, length),
        eq(unsafe { kZeroLength_lengthOffset })
    );
    // data is skipped because zero-length array is unsupported
    expect_that!(core::mem::size_of::<MultiDim>(), eq(unsafe { kMultiDimSize }));
    expect_that!(core::mem::align_of::<MultiDim>(), eq(unsafe { kMultiDimAlign }));
    expect_that!(core::mem::offset_of!(MultiDim, data), eq(unsafe { kMultiDim_dataOffset }));
}
