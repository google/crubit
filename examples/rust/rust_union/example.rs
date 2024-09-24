// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub union ReprRustUnion {
    a: i32,
    b: f64,
}

impl Default for ReprRustUnion {
    fn default() -> Self {
        ReprRustUnion { a: 0 }
    }
}

// Because repr(Rust) unions do not have their fields exposed natively in C++,
// (and because ReprRustUnion's fields are all private),
// we need to provide accessor methods or functions for each field.

impl ReprRustUnion {
    pub fn set_a(&mut self, a: i32) {
        self.a = a;
    }

    pub fn set_b(&mut self, b: f64) {
        self.b = b;
    }
}
