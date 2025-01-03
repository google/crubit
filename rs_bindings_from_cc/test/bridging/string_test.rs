// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std;
use googletest::prelude::*;

#[gtest]
fn test_string_type_as_function_arg() {
    let s = cc_std::std::string::from("hello");
    let len = s.len();
    let ffi_len = string_test_lib::GetStringSize(s);
    assert_eq!(len, ffi_len.try_into().unwrap());
}

#[gtest]
fn test_string_type_as_return_value() {
    let s1: cc_std::std::string =
        unsafe { string_test_lib::CreateString("hello".as_ptr() as _, 5) };
    let s2 = cc_std::std::string::from("hello");
    assert_eq!(s1, s2);
}
// TODO(b/351976622): Support basic_string in supported.
// #[gtest]
// fn test_basic_string_as_return_value() {
//    let s1: cc_std::std::string =
//        unsafe { string_test_lib::CreateStringAsBasicString("hello".as_ptr()
// as _, 5) };    let s2 = cc_std::std::string::from("hello");
//    assert_eq!(s1, s2);
//}
