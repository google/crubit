// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use operator_and::*;

#[gtest]
fn test() {
    unsafe {
        MyBadClass::Accepts(MyBadClass::Returns());
    }
}
