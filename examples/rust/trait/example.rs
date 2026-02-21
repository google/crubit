// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub trait MyTrait {
    fn add_with(&self, y: i32) -> i32;

    fn describe(&self) -> &'static str;
}

#[derive(Clone, Copy, Default)]
pub struct MyStruct {
    x: i32,
}

impl MyStruct {
    pub fn new(x: i32) -> Self {
        Self { x }
    }
}

impl MyTrait for MyStruct {
    fn add_with(&self, y: i32) -> i32 {
        self.x + y
    }

    fn describe(&self) -> &'static str {
        "MyStruct"
    }
}
