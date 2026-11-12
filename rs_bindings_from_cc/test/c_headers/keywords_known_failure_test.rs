// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
#[gtest]
fn test_keywords_known_failure() {
    // See KNOWN_FAILURES.md: C identifiers like `class` fail because they are C++ keywords.
    use keywords_known_failure::KeywordsKnownFailure;
    let _ = core::mem::size_of::<KeywordsKnownFailure>();
}
