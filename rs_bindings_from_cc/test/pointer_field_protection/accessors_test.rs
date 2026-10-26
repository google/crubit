// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use accessors::PointerHolder;
use googletest::{expect_eq, gtest};

#[gtest]
fn test_set_from_rust_get_from_cpp() {
    let mut holder = PointerHolder::default();
    let mut x: i32 = 7;
    // SAFETY: `x` outlives `holder`.
    unsafe {
        holder.set_ptr(&mut x);
    }
    expect_eq!(holder.ptr(), &raw mut x);
    expect_eq!(holder.GetFromCpp(), 7);
}

#[gtest]
fn test_set_from_cpp_get_from_rust() {
    let mut holder = PointerHolder::default();
    expect_eq!(holder.ptr(), core::ptr::null_mut());
    holder.SetFromCpp();
    // SAFETY: `SetFromCpp` makes `ptr` point to a global `int`.
    expect_eq!(unsafe { *holder.ptr() }, 42);
}
