// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[test]
fn test_disabled_layering_check() {
    assert_eq!(top_lib::GetValFromMyStruct(my_lib::MyStruct { val: 42 }), 42);
}
