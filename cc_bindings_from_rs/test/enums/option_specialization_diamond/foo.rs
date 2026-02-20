// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct Foo {
    pub foo: Option<i32>,
}

impl Foo {
    pub fn new(i: i32) -> Self {
        Self { foo: Some(i) }
    }

    pub fn set_field(&mut self, x: &Option<i32>) {
        self.foo = *x;
    }
}
