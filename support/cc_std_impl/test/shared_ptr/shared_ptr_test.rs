// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::shared_ptr_const;
use googletest::prelude::*;

#[gtest]
fn test_layout() {
    // Testing that the layout matches C++ shared_ptr<const int32_t>
    assert_eq!(
        core::mem::size_of::<shared_ptr_const<i32>>(),
        test_helpers::shared_ptr_test::get_shared_ptr_size() as usize
    );
    assert_eq!(
        core::mem::align_of::<shared_ptr_const<i32>>(),
        test_helpers::shared_ptr_test::get_shared_ptr_alignment() as usize
    );
}

#[gtest]
fn test_polymorphic_destructor() {
    // Initial destructor count should be 0.
    assert_eq!(test_helpers::shared_ptr_test::get_derived_destructor_count(), 0);
    {
        let _shared_base = test_helpers::shared_ptr_test::create_virtual_base();
        assert_eq!(core::mem::size_of_val(&_shared_base), 16);
    }
    // After dropping shared_ptr<const Base> which points to Derived, count should be 1.
    assert_eq!(test_helpers::shared_ptr_test::get_derived_destructor_count(), 1);
}

#[gtest]
fn test_deref() {
    let shared = test_helpers::shared_ptr_test::create_shared_ptr();
    assert_eq!(*shared.as_ref().unwrap(), 1);
}
