// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_has_dependency() {
    let dependency = dependency::Dependency { magic: 42 };
    assert_eq!(uses_dependency::UseDependency(dependency).magic, dependency.magic);
}
