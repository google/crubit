// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[unsafe(no_mangle)]
pub extern "C" fn rust_test_main() -> i32 {
    assert_eq!(basic_rs::Sum(basic_rs::Point { x: 2, y: 3 }), 5);
    assert_eq!(basic_rs::Always(), 1);
    0
}
