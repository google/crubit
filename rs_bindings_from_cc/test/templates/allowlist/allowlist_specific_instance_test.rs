// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn int_float_test() {
    let mut i: allowlist_specific_instance::crubit_bind_instantiation_0 = (1, 2f32).into();
    unsafe {
        allowlist_specific_instance::__CcTemplateInst2TsIifE::Member(&raw mut i);
        allowlist_specific_instance::IntFloatCaller(i);
    }
}

#[gtest]
fn short_double_test() {
    let i: allowlist_specific_instance::__CcTemplateInst2TsIsdE = (1i16, 2f64).into();
    allowlist_specific_instance::ShortDoubleCaller(i);
}
