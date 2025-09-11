// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
use inline::namespaced::forward_declared_doubler;
use inline::{double_unsigned_int, hello_world_inline, take_struct_by_const_ptr, SomeStruct};

#[gtest]
fn test_hello_world() {
    assert_eq!(hello_world_inline(), 42);
}

#[gtest]
fn test_take_struct_by_const_ptr() {
    let s = SomeStruct { int_field: 789 };
    assert_eq!(789, unsafe { take_struct_by_const_ptr(&raw const s) });
}

#[gtest]
fn test_double_unsigned_int() {
    assert_eq!(double_unsigned_int(123), 246);
}

#[gtest]
fn test_forward_declared_doubler() {
    assert_eq!(forward_declared_doubler(124), 248);
}
