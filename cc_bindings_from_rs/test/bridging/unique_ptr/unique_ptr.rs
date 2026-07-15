// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::unique_ptr;
use cc_std::std::virtual_unique_ptr;
use crubit_annotate::must_bind;
use test_helpers::unique_ptr_test::Base;
use test_helpers::unique_ptr_test::Derived;
use test_helpers::unique_ptr_test::Target;

#[must_bind]
pub fn roundtrip_unique_ptr(val: unique_ptr<Target>) -> unique_ptr<Target> {
    val
}

#[must_bind]
pub fn create_unique_ptr() -> unique_ptr<Target> {
    test_helpers::unique_ptr_test::create_target()
}

#[must_bind]
pub fn get_destructor_count() -> i32 {
    Target::get_destructor_count()
}

#[must_bind]
pub fn consume_unique_ptr(_val: unique_ptr<Target>) {}

#[must_bind]
pub fn roundtrip_virtual_unique_ptr(val: virtual_unique_ptr<Base>) -> virtual_unique_ptr<Base> {
    val
}

#[must_bind]
pub fn create_virtual_unique_ptr() -> virtual_unique_ptr<Base> {
    test_helpers::unique_ptr_test::create_virtual_base()
}

#[must_bind]
pub fn get_derived_destructor_count() -> i32 {
    Derived::get_derived_destructor_count()
}

#[must_bind]
pub fn consume_virtual_unique_ptr(_val: virtual_unique_ptr<Base>) {}

#[must_bind]
pub fn accept_unique_ptr_tuple(val: unique_ptr<(i32, i32)>) -> unique_ptr<(i32, i32)> {
    val
}

#[must_bind]
pub fn accept_unique_ptr_option(val: unique_ptr<Option<i32>>) -> unique_ptr<Option<i32>> {
    val
}
