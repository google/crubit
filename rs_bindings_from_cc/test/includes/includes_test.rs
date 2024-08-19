// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_access_to_declaration_from_header_using_system_include_directory() {
    assert_eq!(using_includes::ReturnsFortyTwo(), 42);
}
