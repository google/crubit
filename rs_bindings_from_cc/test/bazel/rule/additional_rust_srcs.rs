// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn add(a: u32, b: u32) -> u32 {
    crate::CppAdd(a, b)
}

impl crate::math_dependency::MyStruct {
    pub fn do_something(&self) {}
}
