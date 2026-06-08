// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate std;
use std::ops::Try;

pub fn check_try_trait_available<T: Try>(_x: T) -> bool {
    true
}
// touched
