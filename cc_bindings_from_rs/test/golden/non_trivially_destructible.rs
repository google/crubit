// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Default, Clone)]
pub struct NonTriviallyDestructable {
    pub field: i32,
}

impl Drop for NonTriviallyDestructable {
    fn drop(&mut self) {
        self.field = 123;
    }
}

pub fn take_by_value(_x: NonTriviallyDestructable) {}

pub fn return_by_value() -> NonTriviallyDestructable {
    NonTriviallyDestructable { field: 123 }
}
