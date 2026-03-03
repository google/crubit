// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn my_test() {
    let s = member_function::S::default();
    let int_field = s.int_accessor();
    assert_eq!(*int_field, 42);
}

#[gtest]
fn self_reference_test() {
    let mut s = member_function::S::default();
    unsafe {
        let s_ptr: *mut member_function::S =
            member_function::S::me(&mut s as *mut member_function::S);
        let int_field = (*s_ptr).int_accessor();
        assert_eq!(*int_field, 42);
    }
}
