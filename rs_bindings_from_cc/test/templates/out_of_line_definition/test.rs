// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cref::CRef;
use googletest::gtest;

#[gtest]
fn test_member_function_of_class_template_defined_out_of_line_in_h_file() {
    let s = out_of_line_definition::MyTypeAlias::Create(123);
    // SAFETY: s is alive while s.value() is alive, and no other references to s exist.
    assert_eq!(123, *unsafe { CRef::unchanging(s.value()) });
}
