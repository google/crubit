// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// The forward-declared type is only usable from this create.
pub fn create_and_consume_template_int() -> core::ffi::c_int {
    let template_int: crate::TemplateIntAlias = crate::GetTemplateInt();
    crate::ConsumeCompoundDataType(crate::CompoundDataType { template_int })
}

pub fn get_int_from_template_int() -> i32 {
    ctor::emplace!(crate::GetTemplateInt()).value
}

pub fn get_int_from_template2_int() -> i32 {
    ctor::emplace!(crate::GetOtherPubCrateTemplate2Int()).value
}
