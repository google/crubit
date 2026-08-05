// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use a::A_Struct;

pub fn hello_from_extra() -> i32 {
    let s = A_Struct { x: 42 };
    s.x
}
