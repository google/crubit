// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn always_bound_test() {
    let mut i: allowlist_specific_template::__CcTemplateInst13AlwaysBoundTsIifE = (1, 2f32).into();
    unsafe {
        allowlist_specific_template::__CcTemplateInst13AlwaysBoundTsIifE::Member(&raw mut i);
        allowlist_specific_template::IntFloatCaller(i);
    }
}

#[gtest]
fn not_bound_test() {
    let i: allowlist_specific_template::__CcTemplateInst10NotBoundTsIifE = (1, 2f32).into();
    allowlist_specific_template::NotBoundCaller(i);
}
