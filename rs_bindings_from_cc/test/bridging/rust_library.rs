// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct MyStruct {
    pub x: i32,
}

impl Default for MyStruct {
    fn default() -> Self {
        MyStruct { x: 42 }
    }
}
