// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use failed_template_instantiation_member_function_recursive::AForInt;
use googletest::{expect_false, expect_true, gtest};
use item_exists::type_exists;
use method_exists::method_exists;

/// Test to ensure that Crubit can import class template specialization with
/// un-instantiable member function that passes the type check but has an
/// incorrect function body.
#[gtest]
fn test_build() {
    expect_true!(type_exists!(failed_template_instantiation_member_function_recursive::AForInt));
    AForInt::default().NoOp();
    expect_true!(method_exists!(AForInt, NoOp));
    expect_false!(method_exists!(AForInt, Call_FailMethod));
    expect_false!(method_exists!(AForInt, FailMethod));
    expect_false!(method_exists!(AForInt, FailStaticMethod));
}
