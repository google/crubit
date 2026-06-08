// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;

#[gtest]
fn test_reexported_namespaces() {
    // Without the module prefix `absl::`, this should compile because it's re-exported natively.
    reexported_namespaces::MyAbslFunction();
    reexported_namespaces::MyBaseFunction();
}
