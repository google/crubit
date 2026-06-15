// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;

#[gtest]
fn test_extracted_function() {
    assert_eq!(test_extracted_cc::MyTestFunction(10), 15);
}

#[gtest]
fn test_static_namespace_method() {
    assert_eq!(test_extracted_cc::my_test_namespace::TestClass::StaticMethod(), 3);
}

#[gtest]
fn test_char_with_brace() {
    let c = test_extracted_cc::my_test_namespace::GetCharWithBrace();
    assert_eq!(u8::from(c), b'{');
}

#[gtest]
fn test_string_with_brace() {
    unsafe {
        let ptr = test_extracted_cc::my_test_namespace::GetStringWithBrace();
        let c_str = std::ffi::CStr::from_ptr(ptr as *const std::os::raw::c_char);
        assert_eq!(c_str.to_str().unwrap(), "String with { and } in it!");
    }
}
