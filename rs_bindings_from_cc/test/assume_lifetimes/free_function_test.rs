// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cref::CMut;
use googletest::prelude::*;

#[gtest]
fn increment_int_ref_test() {
    let mut x = 1;
    let y = free_function::increment_int_ref(&mut x);
    // SAFETY: y is a reference to x, and after increment_int_ref, x is alive but never mutated.
    expect_eq!(*unsafe { CMut::unchanging(y) }, 2);
    expect_eq!(x, 2);
}
