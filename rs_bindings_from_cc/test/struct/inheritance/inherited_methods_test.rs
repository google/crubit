// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use inherited_methods::{Base, Derived};

#[gtest]
fn test_inherits_has_bindings() {
    let _base = Base::default();
    let _derived = Derived::default();
    assert!(_base.has_bindings());
    assert!(_derived.has_bindings());
}
