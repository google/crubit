// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![deny(unused_unsafe)]

use googletest::prelude::*;

#[gtest]
fn test_string_view() {
    let s = unsafe { string_view_test_lib::PopACharFromStringView("foo".into()) };
    let bytes = unsafe { &*s.as_raw_bytes() };
    assert_eq!(bytes, b"oo");
}
