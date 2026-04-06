// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn add_with_dep(left: u64, right: u64) -> u64 {
    test_dependency::dependency_add(left, right)
}
