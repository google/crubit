// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cref::{CMut, CRef};
use googletest::prelude::*;

#[gtest]
fn my_test() {
    let s = member_function::S::default();
    let int_field = s.int_accessor();
    // SAFETY: int_field refers to s; there are no other references to s.
    assert_eq!(*unsafe { CRef::unchanging(int_field) }, 42);
}

#[gtest]
fn self_reference_test() {
    let mut s = member_function::S::default();
    let s_ref: CMut<'_, member_function::S> = member_function::S::me(&mut s);
    // SAFETY: s is alive through the entire unsafe block and is not mutated.
    unsafe {
        let int_field = (*CMut::as_ptr(s_ref)).int_accessor();
        assert_eq!(*CRef::unchanging(int_field), 42);
    }
}
