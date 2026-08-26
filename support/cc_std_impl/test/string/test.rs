// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::new_string;
use cc_std::std::string_view;
use cc_std::std::string_wrapper;
use ctor::{emplace, CtorNew};
use googletest::{expect_eq, expect_ne, expect_that, gtest, matchers::container_eq};
use rstest::rstest;
use test_helpers::cpp_std_string_test::RoundTrip;

// The type should implement Send and Sync.
static_assertions::assert_impl_all!(cc_std::std::string_wrapper : Send, Sync);

#[googletest::test]
#[rstest]
#[case(b"Hello world")]
#[case(b"A super longggggggggggggggggggggg non sso string")]
#[case(b"")]
#[case(b"Hello\xffworld")]
fn test_ffi_round_trip_handle_non_utf8(#[case] input: &[u8]) {
    let s = string_wrapper::from(input);
    let s2 = RoundTrip(s.clone());
    expect_eq!(s.as_slice(), s2.as_slice());
}

#[gtest]
#[rstest]
#[case("")]
#[case("foo")]
fn utf8_compares_equal_to_str_and_bytes(#[case] input: &str) {
    let view: string_view<'_> = input.into();
    let bytes = input.as_bytes();
    expect_eq!(view, *input);
    expect_eq!(view, *bytes);
    expect_eq!(view, view);
}

#[gtest]
fn non_utf8_compares_equal_to_bytes() {
    let view: string_view<'_> = b"\x80\x81".into();
    let bytes: &[u8] = b"\x80\x81";
    expect_eq!(view, *bytes);
}

#[gtest]
fn different_utf8_strings_compare_unequal() {
    let foo: string_view<'_> = "foo".into();
    let bar: &[u8] = b"bar";
    expect_ne!(foo, *bar);
    expect_ne!(foo, *"bar");
}

#[gtest]
fn test_from_string() {
    let input: String = String::from("A string");
    let s = string_wrapper::from(&input);
    assert_eq!(s.as_slice(), b"A string");
}

#[gtest]
fn test_from_vec() {
    let input: Vec<u8> = vec![1, 2, 3, 4, 5];
    let s = string_wrapper::from(&input);
    assert_eq!(s.as_slice(), b"\x01\x02\x03\x04\x05");
}

#[gtest]
fn test_from_str() {
    let input: &str = "A string";
    let s = string_wrapper::from(input);
    assert_eq!(s.as_slice(), b"A string");
}

#[gtest]
fn test_from_slice() {
    let input: &[u8] = b"A string";
    let s = string_wrapper::from(input);
    assert_eq!(s.as_slice(), b"A string");
}

#[gtest]
fn test_len_and_empty() {
    let s: string_wrapper = "".into();
    assert_eq!(s.len(), 0);
    assert!(s.is_empty());

    let s: string_wrapper = "12345".into();
    assert_eq!(s.len(), 5);
    assert_eq!(s.is_empty(), false);
}

#[gtest]
fn test_deref() {
    let s: string_wrapper = "array".into();
    expect_that!(&*s, container_eq(*b"array"));
}

#[gtest]
fn test_as_ref() {
    let s: string_wrapper = "array".into();
    expect_that!(&*s.as_ref(), container_eq(*b"array"));
}

#[gtest]
fn test_contains() {
    let s: string_wrapper = "12345".into();
    assert!(s.contains(&b'1'));
    assert!(s.contains(&b'5'));
    assert!(!s.contains(&b'0'));
}

#[gtest]
fn test_display_success() {
    let utf8_str: string_wrapper = "array".into();
    let utf8_str_formatted = format!("{}", utf8_str.display());
    expect_eq!(utf8_str_formatted, "array");
}

#[gtest]
fn test_display_error() {
    let non_utf8_str: &[u8] = b"Hello \xF0\xF0World";
    let non_utf8_str_formatted = string_wrapper::from(non_utf8_str);
    expect_eq!(format!("{}", non_utf8_str_formatted.display()), "Hello \u{FFFD}\u{FFFD}World");
}

#[gtest]
fn test_debug() {
    let utf8_str: string_wrapper = "array".into();
    let utf8_str_formatted = format!("{:?}", utf8_str);
    expect_eq!(utf8_str_formatted, "cc_std::string_wrapper([97, 114, 114, 97, 121])");
}

#[gtest]
fn test_new_string_construction() {
    let s = emplace!(new_string::ctor_new("Hello new_string"));
    expect_eq!(s.as_slice(), b"Hello new_string");
    expect_eq!(s.to_str().unwrap(), "Hello new_string");
}

#[gtest]
fn test_new_string_as_mut_slice() {
    let mut s = emplace!(new_string::ctor_new("hello"));
    let mut_slice = s.as_mut().as_mut_slice();
    mut_slice[0] = b'H';
    expect_eq!(s.as_slice(), b"Hello");
}

#[gtest]
fn test_new_string_comparisons() {
    let s1 = emplace!(new_string::ctor_new("abc"));
    let s2 = emplace!(new_string::ctor_new("abc"));
    let s3 = emplace!(new_string::ctor_new("def"));

    // PartialEq, Eq
    expect_eq!(*s1, *s2);
    expect_ne!(*s1, *s3);

    // PartialOrd, Ord
    assert!(*s1 < *s3);
    assert!(*s3 > *s1);
    expect_eq!((*s1).cmp(&*s2), core::cmp::Ordering::Equal);
    expect_eq!((*s1).cmp(&*s3), core::cmp::Ordering::Less);

    // Hash
    use core::hash::{Hash, Hasher};
    let mut h1 = std::collections::hash_map::DefaultHasher::new();
    let mut h2 = std::collections::hash_map::DefaultHasher::new();
    (*s1).hash(&mut h1);
    (*s2).hash(&mut h2);
    expect_eq!(h1.finish(), h2.finish());

    // PartialEq with other types
    let view: string_view = "abc".into();
    expect_eq!(*s1, view);
    expect_eq!(*s1, "abc");
    expect_eq!(*s1, *"abc");
    expect_eq!(*s1, &b"abc"[..]);
}

#[gtest]
fn test_new_string_display() {
    let s = emplace!(new_string::ctor_new("hello"));
    expect_eq!(format!("{}", s.display()), "hello");

    let s_invalid = emplace!(new_string::ctor_new(b"hello \xffworld" as &[u8]));
    expect_eq!(format!("{}", s_invalid.display()), "hello \u{FFFD}world");
}

#[gtest]
fn test_new_string_debug() {
    let s = emplace!(new_string::ctor_new("hello"));
    expect_eq!(format!("{:?}", s), "\"hello\"");

    let s_invalid = emplace!(new_string::ctor_new(b"hello \xffworld" as &[u8]));
    expect_eq!(format!("{:?}", s_invalid), "\"hello \\xffworld\"");
}
