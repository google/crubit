// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use failed_template_instantiation::*;
use googletest::prelude::*;

#[gtest]
fn test_build() {
    let ok = Ok::default();
    Func1(ok);
    let c = CSpecializedForInt::default();
    Func2(c);
}
