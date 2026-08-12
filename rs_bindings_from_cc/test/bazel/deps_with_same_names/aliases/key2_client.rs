// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn call_key2() -> i32 {
    // Use the alias name of key2_cc bindings
    key2_cc_rust::key2_func()
}
