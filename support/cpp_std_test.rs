// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::expect_eq;
use googletest::expect_gt;
use googletest::gtest;

#[gtest]
fn test_cpp_std_vector() {
    let mut v = cpp_std::vector::<i32>::new();
    v.push(42);
    expect_eq!(v.len(), 1);
    expect_eq!(v[0], 42);
}

#[gtest]
fn test_cpp_std_string_view() {
    let sv = cpp_std::string_view::from("hello world");
    expect_eq!(sv.as_bytes(), b"hello world");
}

#[gtest]
fn test_cpp_std_unique_ptr_type() {
    // Verify unique_ptr is exposed and has the expected size.
    let size = size_of::<cpp_std::unique_ptr<i32>>();
    expect_gt!(size, 0);
}

#[gtest]
fn test_cpp_std_string_wrapper() {
    let s = cpp_std::string_wrapper::from("hello string");
    expect_eq!(s.as_slice(), b"hello string");
}
