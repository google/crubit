// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn niif_use_test() {
    let i: named_instantiation::NiIF = (1, 2f32).into();
    unsafe {
        named_instantiation::SomeApi(&i);
    }
}
