// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use item_exists::{type_exists, value_exists};
use no_bindings::crubit::no_bindings;

#[gtest]
fn test_deprecated_alias() {
    assert!(!type_exists!(no_bindings::DeprecatedAlias));
}

// vectorcall attribute is outright ignored on e.g. ARM -- so on that platform,
// this isn't actually a different calling convention, and we'd expect bindings
// to exist after all.
#[cfg(target_arch = "x86_64")]
#[gtest]
fn test_vectorcall() {
    assert!(!value_exists!(no_bindings::crubit_vectorcall));
}

#[gtest]
fn test_parameter_lifetimebound() {
    assert!(!value_exists!(no_bindings::crubit_parameter_lifetimebound));
}

#[gtest]
fn test_noreturn() {
    assert!(!value_exists!(no_bindings::crubit_noreturn));
}

#[gtest]
fn test_nodiscard() {
    assert!(!value_exists!(no_bindings::crubit_nodiscard));
}

#[gtest]
fn test_deprecated() {
    assert!(!value_exists!(no_bindings::crubit_deprecated));
}

#[gtest]
fn test_enable_if() {
    assert!(!value_exists!(no_bindings::crubit_enable_if));
}

#[gtest]
fn test_unknown_attr_struct() {
    assert!(!type_exists!(no_bindings::UnknownAttrStruct));
}

#[gtest]
fn test_unknown_attr_enum() {
    assert!(!type_exists!(no_bindings::UnknownAttrEnum));
}

#[gtest]
fn test_templates() {
    assert!(!type_exists!(no_bindings::TemplatedStruct));
    assert!(!type_exists!(no_bindings::InstantiatedTemplatedStruct));
}

/// Function pointers, like most supported types, are only supported if their
/// type dependencies are.
#[gtest]
fn test_function_pointers() {
    assert!(!value_exists!(no_bindings::crubit_invoke_callback));
}

#[gtest]
fn test_type_attributes() {
    assert!(!type_exists!(no_bindings::UnknownTypeAttribute));
    assert!(!value_exists!(no_bindings::crubit_unknown_type_attribute));
}

#[gtest]
fn test_incomplete_type() {
    assert!(!value_exists!(no_bindings::crubit_incomplete_type));
}

#[gtest]
fn test_std_vector() {
    assert!(!value_exists!(no_bindings::UseSetByValue));
    assert!(!value_exists!(no_bindings::UseSetByReference));
    assert!(!value_exists!(no_bindings::UseSetByPointer));
}

#[gtest]
fn test_consteval() {
    assert!(!value_exists!(no_bindings::consteval_add));
}

#[gtest]
fn test_variadic() {
    assert!(!value_exists!(no_bindings::variadic_function));
}
