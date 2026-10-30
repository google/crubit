// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests that a function declared as a `friend` in one target and redeclared in another target
//! can be used when both targets' bindings are linked into the same binary, which used to fail
//! with duplicate thunk symbols.

use googletest::prelude::*;

#[gtest]
fn test_friend_redeclared_in_other_target() {
    let b = friend::Befriending::default();
    expect_eq!(b.Value(), 42);
    expect_eq!(redeclaration::Describe(&b).to_string(), Ok("42".to_string()));
}
