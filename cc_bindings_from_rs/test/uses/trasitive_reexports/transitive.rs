// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
pub struct Transitive {
    pub value: i32,
}

impl Transitive {
    pub fn new(value: i32) -> Self {
        Transitive { value }
    }
}
