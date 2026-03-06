// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(no_core)]
#![no_core]

// Rename the crate to avoid conflicting with the alloc module in alloc.
extern crate alloc as foo;

use foo::string::String;

pub struct Test {
    s: String,
}

impl Test {
    pub fn new() -> Self {
        Test { s: String::new() }
    }

    pub fn s(&self) -> &str {
        &self.s
    }
}
