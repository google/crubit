// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;

#[gtest]
fn test_extern() {
    assert_eq!(unsafe { global::extern_int }, 1);
    assert_eq!(global::GetIntVal(), 1);
    unsafe { global::extern_int = 10 };
    assert_eq!(global::GetIntVal(), 10);
}

#[gtest]
fn test_extern_const() {
    assert_eq!(unsafe { global::kExternConstInt }, 2);
}

#[gtest]
fn test_extern_namespaced() {
    assert_eq!(unsafe { global::foo::extern_int_namespaced }, 3);
    assert_eq!(global::GetNamespacedIntVal(), 3);
    unsafe { global::foo::extern_int_namespaced = 30 };
    assert_eq!(global::GetNamespacedIntVal(), 30);
}

#[gtest]
fn test_extern_c_namespaced() {
    assert_eq!(unsafe { global::foo::extern_c_int_namespaced }, 4);
    assert_eq!(global::GetCNamespacedIntVal(), 4);
    unsafe { global::foo::extern_c_int_namespaced = 40 };
    assert_eq!(global::GetCNamespacedIntVal(), 40);
}

#[gtest]
fn test_constexpr_and_inline_const_are_constants() {
    assert_eq!(global::kInlineConstInt, 6);
    assert_eq!(global::kConstexprInt, 7);
    assert_eq!(global::inline_int, 5);
    assert_eq!(global::foo::inline_int_namespaced, 5);
    assert_eq!(global::foo::inline_long_long_namespaced, ffi_11::new_c_longlong(24));
    assert_eq!(global::foo::inline_bool_namespaced, true);
}
#[gtest]
fn test_anonymous_enum_constants() {
    assert_eq!(global::kAnonEnumConst, 123);
    assert_eq!(global::foo::kAnonEnumNamespacedConst, 456);
    assert_eq!(global::struct_with_anon_enum::kAnonEnumInStructConst, 789);
}
