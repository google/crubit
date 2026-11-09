// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
extern crate defining_crate;

pub mod inner {
    pub use defining_crate::Braced;
}

pub struct Local {
    pub value: i32,
}
