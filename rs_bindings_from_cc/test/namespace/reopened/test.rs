// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use item_exists::value_exists;
use reopened_namespace::foo;

#[test]
fn test_not_present() {
    assert!(!value_exists!(foo::FunctionUsesNamespaceType));
}

#[test]
fn test_reopened_namespace() {
    assert_eq!(42, foo::Returns42());
}
