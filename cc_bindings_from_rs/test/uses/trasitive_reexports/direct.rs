// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
extern crate transitive;

pub use transitive::Transitive;

pub struct Direct {
    pub value: i32,
}

impl Direct {
    pub fn new(train: Transitive) -> Self {
        Direct { value: train.value }
    }
}
