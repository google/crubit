// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Default, Clone)]
pub struct SomeStruct {
    _unused: i32,
}

impl std::fmt::Display for SomeStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SomeStruct")
    }
}

impl SomeStruct {
    pub fn method(&self) -> &'static str {
        "SomeStruct method"
    }
    pub fn assoc_function() -> &'static str {
        "SomeStruct assoc"
    }
}

pub fn free_function() -> &'static str {
    "SomeStruct free"
}
