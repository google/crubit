// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::string_view as ccsv;
use ctor::{emplace, CtorNew};
use googletest::prelude::*;

#[gtest]
fn type_alias_ctor_test() {
    let a: ccsv = "hello".into();
    let _test_struct = emplace!(type_alias::TypeAliasCtor::ctor_new(a));
}
