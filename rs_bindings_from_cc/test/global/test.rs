// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, gtest};

#[gtest]
fn test_extern() {
    expect_eq!(unsafe { global::extern_int }, 1);
    expect_eq!(global::GetIntVal(), 1);
    unsafe { global::extern_int = 10 };
    expect_eq!(global::GetIntVal(), 10);
}

#[gtest]
fn test_extern_const() {
    expect_eq!(unsafe { global::kExternConstInt }, 2);
}

#[gtest]
fn test_extern_namespaced() {
    expect_eq!(unsafe { global::foo::extern_int_namespaced }, 3);
    expect_eq!(global::GetNamespacedIntVal(), 3);
    unsafe { global::foo::extern_int_namespaced = 30 };
    expect_eq!(global::GetNamespacedIntVal(), 30);
}

#[gtest]
fn test_extern_c_namespaced() {
    expect_eq!(unsafe { global::foo::extern_c_int_namespaced }, 4);
    expect_eq!(global::GetCNamespacedIntVal(), 4);
    unsafe { global::foo::extern_c_int_namespaced = 40 };
    expect_eq!(global::GetCNamespacedIntVal(), 40);
}

#[gtest]
fn test_constexpr_and_inline_const_are_constants() {
    expect_eq!(global::kInlineConstInt, 6);
    expect_eq!(global::kConstexprInt, 7);
    expect_eq!(global::inline_int, 5);
    expect_eq!(global::foo::inline_int_namespaced, 5);
    expect_eq!(global::foo::inline_long_long_namespaced, ffi_11::new_c_longlong(24));
    expect_eq!(global::foo::inline_bool_namespaced, true);
}

#[gtest]
fn test_constexpr_arrays_are_rust_arrays() {
    let int_array: [ffi_11::c_int; 3] = global::kConstexprIntArray;
    expect_eq!(int_array, [1, -2, 3]);
    let bool_array: [bool; 2] = global::kConstexprBoolArray;
    expect_eq!(bool_array, [true, false]);
    let unsigned_char_array: [ffi_11::c_uchar; 4] = global::kUnsignedCharArrayWithFiller;
    expect_eq!(unsigned_char_array, [7, 8, 0, 0]);
    let multidimensional_int_array: [[ffi_11::c_int; 2]; 3] = global::kMultidimensionalIntArray;
    expect_eq!(multidimensional_int_array, [[1, 2], [3, 0], [0, 0]]);
}

#[gtest]
fn test_anonymous_enum_constants() {
    expect_eq!(global::kAnonEnumConst, 123);
    expect_eq!(global::foo::kAnonEnumNamespacedConst, 456);
    expect_eq!(global::StructWithAnonEnum::kAnonEnumInStructConst, 789);
}

#[gtest]
fn test_static_data_members() {
    use global::StructWithStaticMembers;
    // Mutable static data members have no associated-static equivalent in Rust,
    // so they live in the struct's snake-cased module.
    expect_eq!(unsafe { global::struct_with_static_members::static_int }, 8);
    expect_eq!(global::GetStaticIntVal(), 8);
    unsafe { global::struct_with_static_members::static_int = 80 };
    expect_eq!(global::GetStaticIntVal(), 80);

    // Constant static data members are associated consts.
    expect_eq!(StructWithStaticMembers::kStaticConstInt, 11);
    expect_eq!(StructWithStaticMembers::kStaticConstexprInt, 12);
    expect_eq!(StructWithStaticMembers::kStaticConstexprIntArray, [1, 2, 3]);
    expect_eq!(StructWithStaticMembers::static_inline_int, 13);
    expect_eq!(global::GetStaticInlineIntVal(), 13);
}
