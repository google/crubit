// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub trait MyTrait {
    fn do_something(&self) -> i32;
}

// This should not generate bindings because i32 is from `std`.
impl MyTrait for i32 {
    fn do_something(&self) -> i32 {
        *self + 1
    }
}

pub struct MyStruct {
    pub x: i32,
}

// This should not generate bindings because Default is from `std`.
impl Default for MyStruct {
    fn default() -> Self {
        MyStruct { x: 42 }
    }
}
