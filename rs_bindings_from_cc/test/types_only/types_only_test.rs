// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::{emplace, CtorNew};
use functions_using_types::{PassCloneable, PassCopyable, PassMovable};
use googletest::gtest;
use std::pin::Pin;
use types_only::{Cloneable, Copyable, Movable};

#[gtest]
fn test_types_are_generated_and_usable_with_downstream_functions() {
    let copyable = Copyable::default();
    let cloneable = Cloneable::ctor_new(1);
    let movable = Movable::ctor_new(2);

    let _: Copyable = PassCopyable(copyable);
    let _: Pin<&mut Cloneable> = emplace!(PassCloneable(cloneable));
    let _: Pin<&mut Movable> = emplace!(PassMovable(movable));
}
