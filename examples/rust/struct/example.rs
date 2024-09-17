// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct Struct {
    pub a: i32,
}

// Default trait gives the struct a default constructor in C++.
#[derive(Default)]
pub struct StructWithDefault {
    pub a: i32,
}

// Clone trait gives the struct a copy constructor in C++.
#[derive(Default, Clone)]
pub struct StructWithClone {
    pub a: i32,
}
