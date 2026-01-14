// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub trait MyTrait {
    fn do_something(&self) -> i32;
}

pub struct MyStruct {
    y: i32,
}

impl MyTrait for MyStruct {
    fn do_something(&self) -> i32 {
        self.y
    }
}
