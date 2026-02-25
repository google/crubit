// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct MyStruct {
    pub left: u64,
    pub right: u64,
}

impl MyStruct {
    pub fn new(left: u64, right: u64) -> Self {
        MyStruct { left, right }
    }

    pub fn add(&self) -> u64 {
        self.left + self.right
    }
}
