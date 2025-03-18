// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use item_exists::value_exists;

#[gtest]
fn test_extern() {
    assert_eq!(unsafe { global_lib::extern_int }, 1);
    assert_eq!(global_lib::GetIntVal(), 1);
    unsafe { global_lib::extern_int = 10 };
    assert_eq!(global_lib::GetIntVal(), 10);
}

#[gtest]
fn test_extern_const() {
    assert_eq!(unsafe { global_lib::kExternConstInt }, 2);
}

#[gtest]
fn test_extern_namespaced() {
    assert_eq!(unsafe { global_lib::foo::extern_int_namespaced }, 3);
    assert_eq!(global_lib::GetNamespacedIntVal(), 3);
    unsafe { global_lib::foo::extern_int_namespaced = 30 };
    assert_eq!(global_lib::GetNamespacedIntVal(), 30);
}

#[gtest]
fn test_extern_c_namespaced() {
    assert_eq!(unsafe { global_lib::foo::extern_c_int_namespaced }, 4);
    assert_eq!(global_lib::GetCNamespacedIntVal(), 4);
    unsafe { global_lib::foo::extern_c_int_namespaced = 40 };
    assert_eq!(global_lib::GetCNamespacedIntVal(), 40);
}

#[gtest]
fn test_inline_global() {
    assert_eq!(unsafe { global_lib::inline_int }, 5);
    assert_eq!(global_lib::GetInlineIntVal(), 5);
    unsafe { global_lib::inline_int = 50 };
    assert_eq!(global_lib::GetInlineIntVal(), 50);
}

#[gtest]
fn test_non_generated_items() {
    assert!(!value_exists!(global_lib::kInlineConstInt));
    assert!(!value_exists!(global_lib::kConstexprInt));
    assert!(!value_exists!(global_lib::templated_variable));
}
