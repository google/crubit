// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::vector;
use test_helpers::vector_test::Target;

pub fn roundtrip_vector(val: vector<Target>) -> vector<Target> {
    val
}

pub fn create_vector(size: i32) -> vector<Target> {
    test_helpers::vector_test::create_vector(size)
}

pub fn get_destructor_count() -> i32 {
    Target::get_destructor_count()
}

pub fn reset_destructor_count() {
    Target::reset_destructor_count();
}

pub fn consume_vector(_val: vector<Target>) {}
