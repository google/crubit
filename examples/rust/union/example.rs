// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C)]
pub union ReprCUnion {
    a: i32,
    b: f64,
}

impl Default for ReprCUnion {
    fn default() -> Self {
        ReprCUnion { a: 0 }
    }
}
