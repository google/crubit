// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct Bar {
    pub bar: Option<i32>,
}

impl Bar {
    pub fn new(i: i32) -> Self {
        Self { bar: Some(i) }
    }
}
