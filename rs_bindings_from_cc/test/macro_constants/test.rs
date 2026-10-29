// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, gtest};

/// `expect_eq!` requires both sides to have the same type, so this checks that the constants
/// have the same types as in C++, too.
#[gtest]
fn test_values_match_cc() {
    expect_eq!(macro_constants::INT_CONSTANT, macro_constants::IntConstant());
    expect_eq!(macro_constants::UNSIGNED_CONSTANT, macro_constants::UnsignedConstant());
    expect_eq!(macro_constants::HEX_CONSTANT, macro_constants::HexConstant());
    expect_eq!(macro_constants::CHAR_CONSTANT, macro_constants::CharConstant());
}

#[gtest]
fn test_textual_header_constant() {
    expect_eq!(macro_constants::TEXTUAL_HEADER_CONSTANT, 5);
}
