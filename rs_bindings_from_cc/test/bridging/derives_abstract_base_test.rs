// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_true, gtest};

#[gtest]
fn test_derived_abstract_base_can_be_bridged_by_transmute_even_though_it_starts_with_vtable() {
    let optional = derives_abstract_base::get_optional();
    expect_true!(optional.is_some());
}
