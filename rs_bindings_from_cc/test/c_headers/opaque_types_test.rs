// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
#[gtest]
fn test_opaque_types() {
    use opaque_types::{process_opaque, Opaque};
    // process_opaque(ptr)
    let o: *mut Opaque = core::ptr::null_mut();
    unsafe { process_opaque(o) };
}
