// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[test]
fn test_bindings_from_generated_header() {
    assert_eq!(generated_header::ReturnsFortyTwo(), 42);
}
