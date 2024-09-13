// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct Y {
    pub field: i32,
}

impl Y {
    pub fn create(field: i32) -> Self {
        Self { field }
    }
}

pub fn f(y: &Y) -> namespace_crate1::X {
    namespace_crate1::X { field: y.field }
}

pub fn g(z: &namespace_crate3::Z) -> namespace_crate1::X {
    namespace_crate1::X { field: z.field }
}
