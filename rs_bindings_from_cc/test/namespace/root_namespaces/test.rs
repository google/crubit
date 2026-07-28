// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::assert_that;
use googletest::gtest;
use googletest::matchers::eq;

#[gtest]
fn test_root_namespaces() {
    // Verify that Foo is available at the root of the crate.
    let mut foo = root_namespaces_cc::Foo { x: 42 };
    assert_that!(foo.x, eq(42));
    foo.x = 43;
    assert_that!(foo.x, eq(43));
}
