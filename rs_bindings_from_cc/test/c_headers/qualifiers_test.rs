// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
#[gtest]
fn test_qualifiers() {
    use qualifiers::{process_ptr_to_const, process_ptr_to_ptr};
    unsafe { process_ptr_to_ptr(core::ptr::null_mut()) };
    unsafe { process_ptr_to_const(core::ptr::null()) };
}
