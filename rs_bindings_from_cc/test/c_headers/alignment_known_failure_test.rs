// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
#[gtest]
fn test_alignment_known_failure() {
    // See KNOWN_FAILURES.md: C11 `_Alignas` causes a parsing error because headers are parsed as
    // C++.
    use alignment_known_failure::AlignedStruct;
    let _ = core::mem::size_of::<AlignedStruct>();
}
