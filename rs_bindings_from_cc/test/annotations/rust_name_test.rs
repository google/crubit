// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
use rust_name::crubit::test::SomeStruct;

#[gtest]
fn test_renamed_free_fn() {
    rust_name::crubit::test::free_fn_new_name();
}

#[gtest]
fn test_renamed_struct() {
    rust_name::crubit::test::StructNewName::default();
}

#[gtest]
fn test_renamed_constructor() {
    let x = SomeStruct::ConstructorNewName(1, 2, 3);
    assert_eq!(x.field_new_name, 1 + 2 + 3);
}

#[gtest]
fn test_renamed_method() {
    let x = SomeStruct::default();
    unsafe { SomeStruct::MethodNewName(&raw const x) };
}

#[gtest]
fn test_renamed_field() {
    let x = SomeStruct::default();
    assert_eq!(x.field_new_name, 24601);
}
