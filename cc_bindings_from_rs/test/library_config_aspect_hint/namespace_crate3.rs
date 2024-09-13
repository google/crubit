// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct Z {
    pub field: i32,
}

impl Z {
    pub fn create(field: i32) -> Self {
        Self { field }
    }
}
