// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;

#[gtest]
fn test_embedded_cpp() {
    assert_eq!(library_with_embedded_cpp::call_add_two_ints(5, 7), 12);
}

#[gtest]
fn test_get_magic_number() {
    assert_eq!(library_with_embedded_cpp::call_get_magic_number(), 42);
}

#[gtest]
fn test_multiply_ints() {
    assert_eq!(library_with_embedded_cpp::call_multiply_ints(3, 4), 12);
}

#[gtest]
fn test_make_point() {
    assert_eq!(library_with_embedded_cpp::call_make_point(10, 20), (10, 20));
}

#[gtest]
fn test_extracted_function() {
    assert_eq!(library_with_embedded_cpp::call_my_test_function(10), 15);
}

#[gtest]
fn test_static_namespace_method() {
    assert_eq!(library_with_embedded_cpp::call_static_method(), 3);
}

#[gtest]
fn test_char_with_brace() {
    assert_eq!(library_with_embedded_cpp::call_get_char_with_brace(), b'{');
}

#[gtest]
fn test_string_with_brace() {
    assert_eq!(
        library_with_embedded_cpp::call_get_string_with_brace(),
        "String with { and } in it!"
    );
}

#[gtest]
fn test_inline_thunk_sync() {
    assert_eq!(library_with_embedded_cpp::call_get_test_global_val(), 0);
    library_with_embedded_cpp::set_global_val_to_99();
    assert_eq!(library_with_embedded_cpp::call_get_test_global_val(), 99);
}
