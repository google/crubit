// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Default)]
pub struct NonTrivialStruct {
    pub a: i32,
}

impl Drop for NonTrivialStruct {
    fn drop(&mut self) {
        println!("Dropping NonTrivialStruct");
    }
}
