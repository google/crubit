// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::shared_ptr;
use googletest::prelude::*;

#[gtest]
fn test_layout() {
    // Testing that the layout matches C++ shared_ptr<const int32_t>
    expect_eq!(
        core::mem::size_of::<shared_ptr<i32>>(),
        test_helpers::shared_ptr_test::get_shared_ptr_size() as usize
    );
    expect_eq!(
        core::mem::align_of::<shared_ptr<i32>>(),
        test_helpers::shared_ptr_test::get_shared_ptr_alignment() as usize
    );
}

#[gtest]
fn test_polymorphic_destructor() {
    // Initial destructor count should be 0.
    expect_eq!(test_helpers::shared_ptr_test::get_derived_destructor_count(), 0);
    {
        let _shared_base = test_helpers::shared_ptr_test::create_virtual_base();
        expect_eq!(core::mem::size_of_val(&_shared_base), 16);
    }
    // After dropping shared_ptr<const Base> which points to Derived, count should be 1.
    expect_eq!(test_helpers::shared_ptr_test::get_derived_destructor_count(), 1);
}

#[gtest]
fn test_deref() {
    let shared = test_helpers::shared_ptr_test::create_shared_ptr();
    expect_eq!(*shared_ptr::try_as_ref(&shared).unwrap(), 1);
}

#[gtest]
fn test_as_ptr_and_is_null() {
    let shared = test_helpers::shared_ptr_test::create_shared_ptr();
    expect_false!(shared_ptr::is_null(&shared));
    expect_eq!(unsafe { *shared_ptr::as_ptr(&shared) }, 1);

    let null_shared = test_helpers::shared_ptr_test::create_shared_ptr_void_ptr();
    expect_true!(shared_ptr::is_null(&null_shared));
}

#[gtest]
fn test_release() {
    let mut shared = test_helpers::shared_ptr_test::create_shared_ptr();
    expect_false!(shared_ptr::is_null(&shared));
    let (ptr, cntrl) = shared_ptr::release(&mut shared);
    expect_true!(shared_ptr::is_null(&shared));
    expect_eq!(unsafe { *ptr }, 1);
    // Re-wrap and drop to clean up reference count.
    let _rewrapped = unsafe { shared_ptr::from_raw_parts(ptr, cntrl) };
}

#[gtest]
fn test_into_raw_parts() {
    let shared = test_helpers::shared_ptr_test::create_shared_ptr();
    let (ptr, cntrl) = shared_ptr::into_raw_parts(shared);
    expect_eq!(unsafe { *ptr }, 1);
    // Re-wrap and drop to clean up reference count.
    let _rewrapped = unsafe { shared_ptr::from_raw_parts(ptr, cntrl) };
}
