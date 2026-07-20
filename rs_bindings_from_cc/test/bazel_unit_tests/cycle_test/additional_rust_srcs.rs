// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

impl crate::MyType {
    pub fn add(a: i32, b: i32) -> i32 {
        crate::MyTypeAdder(a, b)
    }
}
