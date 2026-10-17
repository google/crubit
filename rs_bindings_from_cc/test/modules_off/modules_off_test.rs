// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_that, gtest, matchers::eq};

fn dep(value: i32) -> dep_lib::DepStruct {
    dep_lib::MakeDepStruct(value)
}

#[gtest]
fn test_fno_modules() {
    expect_that!(fno_modules_lib::FnoModulesGetValue(dep(1)), eq(2));
}

#[gtest]
fn test_fno_modules_only() {
    expect_that!(fno_modules_only_lib::FnoModulesOnlyGetValue(dep(1)), eq(6));
}

#[gtest]
fn test_no_use_header_modules() {
    expect_that!(no_use_header_modules_lib::NoUseHeaderModulesGetValue(dep(1)), eq(3));
}

#[gtest]
fn test_no_module_maps() {
    expect_that!(no_module_maps_lib::NoModuleMapsGetValue(dep(1)), eq(4));
}

#[gtest]
fn test_no_header_modules() {
    expect_that!(no_header_modules_lib::NoHeaderModulesGetValue(dep(1)), eq(5));
}

#[gtest]
fn test_uses_fno_modules() {
    expect_that!(uses_fno_modules_lib::UsesFnoModulesGetValue(dep(1)), eq(12));
}
