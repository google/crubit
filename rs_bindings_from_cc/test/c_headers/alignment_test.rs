// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_alignment() {
    use alignment::{
        kPackedStructAlign, kPackedStructSize, kPackedStruct_cOffset, kPackedStruct_iOffset,
        kPragmaPackedAlign, kPragmaPackedSize, kPragmaPacked_cOffset, kPragmaPacked_iOffset,
        PackedStruct, PragmaPacked,
    };
    expect_that!(core::mem::size_of::<PackedStruct>(), eq(unsafe { kPackedStructSize }));
    expect_that!(core::mem::align_of::<PackedStruct>(), eq(unsafe { kPackedStructAlign }));
    expect_that!(core::mem::offset_of!(PackedStruct, c), eq(unsafe { kPackedStruct_cOffset }));
    expect_that!(core::mem::offset_of!(PackedStruct, i), eq(unsafe { kPackedStruct_iOffset }));
    expect_that!(core::mem::size_of::<PragmaPacked>(), eq(unsafe { kPragmaPackedSize }));
    expect_that!(core::mem::align_of::<PragmaPacked>(), eq(unsafe { kPragmaPackedAlign }));
    expect_that!(core::mem::offset_of!(PragmaPacked, c), eq(unsafe { kPragmaPacked_cOffset }));
    expect_that!(core::mem::offset_of!(PragmaPacked, i), eq(unsafe { kPragmaPacked_iOffset }));
}
