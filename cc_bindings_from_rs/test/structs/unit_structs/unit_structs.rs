// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// We omit the `Default` trait, since these structs should be default-constructible without it.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct UnitStruct;

impl UnitStruct {
    pub fn get_five(self) -> i32 {
        5
    }
}
