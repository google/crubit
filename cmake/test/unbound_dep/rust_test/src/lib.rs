// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[unsafe(no_mangle)]
pub extern "C" fn rust_test_main() -> i32 {
    assert_eq!(top_rs::Visible(), 4);
    0
}
