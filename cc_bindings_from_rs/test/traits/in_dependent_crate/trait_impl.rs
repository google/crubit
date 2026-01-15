// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use trait_definition::MyTrait;

pub struct MyStruct {
    pub x: i32,
}

impl MyStruct {
    pub fn new(x: i32) -> Self {
        Self { x }
    }
}

impl MyTrait for MyStruct {
    fn do_something(&self) -> i32 {
        self.x
    }
}

pub struct NotImplemented {
    pub foo: String,
}
