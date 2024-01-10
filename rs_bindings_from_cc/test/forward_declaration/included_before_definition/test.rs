// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use item_exists::type_exists;

// TODO(b/309614052): Currently Crubit cannot generate bindings if the forward
// declaration and definition are from different targets.
#[test]
fn test_forward_declarations_included_before_definition() {
    assert!(!type_exists!(definition::A));
    assert!(!type_exists!(definition::my_namespace::B));
}
