// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![no_std]
extern crate alloc;

pub struct NoStdStruct {
    pub test: alloc::string::String,
    // Uncommenting this line causes a compilation failure due to missing `std`.
    //pub should_fail_to_compile: std::collections::HashMap<i32, i32>,
}

impl NoStdStruct {
    pub fn new(x: i32, y: f32) -> Self {
        NoStdStruct { test: alloc::format!("({}, {})", x, y) }
    }

    pub fn display(&self) -> &str {
        &self.test
    }
}
