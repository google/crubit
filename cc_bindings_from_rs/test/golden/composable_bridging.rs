// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn returns_some_int() -> Option<i32> {
    Some(42)
}

pub fn returns_no_int() -> Option<i32> {
    None
}

pub fn unwrap_or_zero(x: Option<i32>) -> i32 {
    x.unwrap_or(0)
}

pub fn option_increments(x: Option<i32>) -> Option<i32> {
    x.map(|x| x + 1)
}
