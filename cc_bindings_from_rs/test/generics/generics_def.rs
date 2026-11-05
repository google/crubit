// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct TransitiveGeneric<T> {
    pub value: T,
}

impl<T> TransitiveGeneric<T> {
    pub fn get_value(&self) -> &T {
        &self.value
    }
}
