// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
// Intentionally tests Crubit bindings for a C `__attribute__((deprecated))` function.
#[allow(deprecated)]
#[gtest]
fn test_attributes() {
    use attributes::deprecated_func;
    deprecated_func();
}
