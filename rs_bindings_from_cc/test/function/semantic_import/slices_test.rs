// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;

/// `rs_std::SliceRef` maps to a Rust slice pointer. The getter reads the field directly, while
/// the setter keeps using a thunk, so check that the two agree.
#[gtest]
fn test_slice_accessors() {
    let mut slices = slices::Slices::default();
    let xs: [i32; 3] = [1, 2, 3];
    unsafe {
        slices.set_s(&raw const xs[..]);
        assert_eq!(&*slices.s(), &xs[..]);
    }
}
