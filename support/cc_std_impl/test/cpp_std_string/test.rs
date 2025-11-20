// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use rstest::rstest;
use std::thread;
use test_helpers::cpp_std_string_test::RoundTrip;

#[googletest::test]
#[rstest]
#[case(b"Hello world")]
#[case(b"A super longggggggggggggggggggggg non sso string")]
#[case(b"")]
#[case(b"Hello\xffworld")]
fn test_ffi_round_trip_handle_non_utf8(#[case] input: &[u8]) {
    let s = cc_std::std::string::from(input);
    let s2 = RoundTrip(s.clone());
    expect_eq!(s.as_slice(), s2.as_slice());
}

#[gtest]
fn test_from_string() {
    let input: String = String::from("A string");
    let s = cc_std::std::string::from(&input);
    assert_eq!(s.as_slice(), b"A string");
}

#[gtest]
fn test_from_vec() {
    let input: Vec<u8> = vec![1, 2, 3, 4, 5];
    let s = cc_std::std::string::from(&input);
    assert_eq!(s.as_slice(), b"\x01\x02\x03\x04\x05");
}

#[gtest]
fn test_from_str() {
    let input: &str = "A string";
    let s = cc_std::std::string::from(input);
    assert_eq!(s.as_slice(), b"A string");
}

#[gtest]
fn test_from_slice() {
    let input: &[u8] = b"A string";
    let s = cc_std::std::string::from(input);
    assert_eq!(s.as_slice(), b"A string");
}

#[gtest]
fn test_deref() {
    let s: cc_std::std::string = "array".into();
    expect_that!(&*s, container_eq(*b"array"));
}

#[gtest]
fn test_as_ref() {
    let s: cc_std::std::string = "array".into();
    expect_that!(&*s.as_ref(), container_eq(*b"array"));
}

#[gtest]
fn test_display_success() {
    let utf8_str: cc_std::std::string = "array".into();
    let utf8_str_formatted = format!("{}", utf8_str.display());
    expect_that!(utf8_str_formatted, eq("array"));
}

#[gtest]
fn test_display_error() {
    let non_utf8_str: &[u8] = b"Hello \xF0\xF0World";
    let non_utf8_str_formatted = cc_std::std::string::from(non_utf8_str);
    expect_that!(format!("{}", non_utf8_str_formatted.display()), eq("Hello ��World"));
}

#[gtest]
fn test_debug() {
    let utf8_str: cc_std::std::string = "array".into();
    let utf8_str_formatted = format!("{:?}", utf8_str);
    expect_that!(utf8_str_formatted, eq("cc_std::string([97, 114, 114, 97, 121])"));
}

// It should be possible to send strings across threads.
#[gtest]
fn test_send() {
    let s = thread::spawn(|| cc_std::std::string::from("taco")).join().unwrap();
    expect_eq!("taco", s.to_string());
}

// It should be possible to send references to strings across threads.
#[gtest]
fn test_sync() {
    let s = cc_std::std::string::from("taco");
    let s = thread::scope(|scope| scope.spawn(|| s.to_string()).join().unwrap());
    expect_eq!("taco", s);
}
