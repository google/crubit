// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
use item_exists::type_exists;

#[gtest]
fn test_map_cc_library_named_core() {
    assert!(type_exists!(core_cpp_reserved_target_name::StructInCore));
}
