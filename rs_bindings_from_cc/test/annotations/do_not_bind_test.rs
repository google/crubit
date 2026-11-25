// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use do_not_bind::crubit::test::{ArgumentToBoundOverload, StructWithDoNotBindTemplateMembers};
use googletest::gtest;

#[gtest]
fn test_do_not_bind_on_function_templates() {
    let _ = StructWithDoNotBindTemplateMembers::from(ArgumentToBoundOverload::default());
}
