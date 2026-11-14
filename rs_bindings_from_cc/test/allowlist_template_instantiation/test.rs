// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![deny(unused_unsafe)]

use googletest::prelude::*;

#[gtest]
fn test_string_view() {
    let s = string_view_test_lib::PopACharFromStringView("foo".into());
    let bytes = unsafe { &*s.as_raw_bytes() };
    assert_eq!(bytes, b"oo");
}

#[gtest]
fn test_unsafe_view() {
    let x: i32 = 42;
    let v = unsafe {
        string_view_test_lib::IdUnsafeView(string_view_test_lib::MyUnsafeIntView { ptr: &x })
    };
    assert_eq!(unsafe { *v.ptr }, 42);
}
