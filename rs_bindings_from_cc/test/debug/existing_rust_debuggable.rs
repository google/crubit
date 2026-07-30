// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[crubit_annotate::must_bind]
#[derive(Debug)]
pub struct ExistingRustDebuggable {
    pub value: char,
}
