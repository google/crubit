// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Regression test for b/504720727
#[crubit_annotate::must_bind]
pub fn return_struct_with_const_field_by_value_in_result(
) -> Result<cc_struct::struct_with_const_field, u8> {
    Ok(cc_struct::struct_with_const_field { num_page_locations: 42 })
}

#[crubit_annotate::must_bind]
pub fn return_struct_with_const_field_by_value_in_option(
) -> Option<cc_struct::struct_with_const_field> {
    Some(cc_struct::struct_with_const_field { num_page_locations: 42 })
}
