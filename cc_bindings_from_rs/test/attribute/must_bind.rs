// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// TODO: b/402989591 - Change this test to a binding generation test once those are supported.
#[crubit_annotate::must_bind]
pub struct Original {
    pub x: i32,
}

impl Original {
    #[crubit_annotate::must_bind]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { x: 42 }
    }
}

#[crubit_annotate::must_bind]
pub fn bar() {}
