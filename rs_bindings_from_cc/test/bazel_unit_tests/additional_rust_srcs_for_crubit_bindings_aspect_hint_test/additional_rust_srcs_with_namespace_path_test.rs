// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_lib_with_additional_rust_srcs_with_namespace_path::*;
use googletest::prelude::*;

#[gtest]
fn test_additional_rust_srcs_with_namespace_path() {
    assert_eq!(a::b::c::f(), 42);
    assert_eq!(a::b::c::a::b::c::g(), 53);
}
