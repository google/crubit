// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn roundtrip_virtual_unique_ptr(
    val: cc_std::std::virtual_unique_ptr<test_helpers::virtual_unique_ptr_test::Base>,
) -> cc_std::std::virtual_unique_ptr<test_helpers::virtual_unique_ptr_test::Base> {
    val
}

pub fn create_virtual_unique_ptr(
) -> cc_std::std::virtual_unique_ptr<test_helpers::virtual_unique_ptr_test::Base> {
    test_helpers::virtual_unique_ptr_test::create_virtual_base()
}

pub fn get_derived_destructor_count() -> i32 {
    test_helpers::virtual_unique_ptr_test::get_derived_destructor_count()
}

pub fn consume_virtual_unique_ptr(
    _val: cc_std::std::virtual_unique_ptr<test_helpers::virtual_unique_ptr_test::Base>,
) {
}
