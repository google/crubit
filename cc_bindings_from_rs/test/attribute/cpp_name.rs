// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[crubit_annotate::cpp_name("Replaced")]
pub struct Original {
    pub x: i32,
}

impl Original {
    #[crubit_annotate::cpp_name("create")]
    pub fn new() -> Self {
        Self { x: 42 }
    }
}
