// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
#[gtest]
fn test_bitfields_known_failure() {
    // See KNOWN_FAILURES.md: `_Bool` is a C99 keyword, which fails in C++ mode (it expects `bool`).
    use bitfields_known_failure::BitfieldsKnownFailure;
    let _ = core::mem::size_of::<BitfieldsKnownFailure>();
}
