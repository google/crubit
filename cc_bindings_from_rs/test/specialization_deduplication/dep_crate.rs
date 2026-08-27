// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct ExpectedName {
    pub x: i32,
}

pub fn use_s() -> Result<ExpectedName, i32> {
    Err(11)
}
