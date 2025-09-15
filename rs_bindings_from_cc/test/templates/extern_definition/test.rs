// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use actual_instantiation::*;
use googletest::prelude::*;

#[gtest]
fn test_member_function_of_class_template_defined_in_cc_file() {
    let s = actual_instantiation_ns::MyTypeAlias::Create(123);
    assert_eq!(123, *s.value());
}
