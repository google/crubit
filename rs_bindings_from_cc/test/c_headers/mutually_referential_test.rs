// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_mutually_referential() {
    use mutually_referential::{kAAlign, kASize, kA_bOffset, kBAlign, kBSize, kB_aOffset, A, B};
    expect_that!(core::mem::size_of::<A>(), eq(unsafe { kASize }));
    expect_that!(core::mem::align_of::<A>(), eq(unsafe { kAAlign }));
    expect_that!(core::mem::offset_of!(A, b), eq(unsafe { kA_bOffset }));
    expect_that!(core::mem::size_of::<B>(), eq(unsafe { kBSize }));
    expect_that!(core::mem::align_of::<B>(), eq(unsafe { kBAlign }));
    expect_that!(core::mem::offset_of!(B, a), eq(unsafe { kB_aOffset }));
}
