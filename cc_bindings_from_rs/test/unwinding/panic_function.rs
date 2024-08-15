// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn panic_rust() {
    panic!("this is a panic");
}

pub extern "C" fn panic_c() {
    panic!("this is a panic");
}
