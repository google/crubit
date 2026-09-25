// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, gtest};
use pointer_field_protection::TrivialAbiPointer;

#[gtest]
fn test_get_after_cpp_copy() {
    expect_eq!(TrivialAbiPointer::GetAfterCppCopy(), 42);
}

// TODO(b/565847600): This test fails on AArch64 with PFP tagged mode.  See the
// bug for repro steps.
#[gtest]
fn test_get_after_rust_move() {
    let p = TrivialAbiPointer::Create();

    // `TrivialAbiPointer` is `Unpin`, so this is a Rust move: a `memcpy` to
    // the heap, without a call to a C++ copy constructor.
    let p = Box::new(p);

    expect_eq!(p.Get(), 42);
}
