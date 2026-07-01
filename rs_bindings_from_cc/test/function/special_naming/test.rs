// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, gtest};

#[gtest]
fn test_llvm_no_mangle_marker() {
    use special_naming::llvm_no_mangle_marker;
    expect_eq!(llvm_no_mangle_marker(), 42);
}

#[gtest]
fn asm_name_with_dollar_sign() {
    use special_naming::asm_name_with_dollar_sign;
    expect_eq!(asm_name_with_dollar_sign(), 42);
}
