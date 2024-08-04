// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(register_tool)]
#![register_tool(__crubit)]

#[__crubit::annotate(cpp_name = "Replaced")]
pub struct Original {
    pub x: i32,
}

impl Original {
    pub fn create() -> Self {
        Self { x: 42 }
    }
}
