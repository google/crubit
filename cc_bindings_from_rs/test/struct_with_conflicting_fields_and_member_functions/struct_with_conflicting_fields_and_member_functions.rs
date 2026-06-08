// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![allow(dead_code)]
pub struct X {
    pub a: i32,
    b: i32,
    c: i32,
}

impl X {
    pub fn a(&self) -> i32 {
        self.a
    }
    pub fn b(&self) -> i32 {
        self.b
    }
}
