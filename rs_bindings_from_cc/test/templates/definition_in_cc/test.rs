// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cref::CRef;
use googletest::gtest;

#[gtest]
fn test_member_function_of_class_template_defined_in_cc_file() {
    let s = definition_in_cc::MyTypeAlias::Create(123);
    // SAFETY: s.value() is alive because `s` is alive; it is also the only reference to s.value().
    assert_eq!(123, *unsafe { CRef::unchanging(s.value()) });
}
