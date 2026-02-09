// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use unique_ptr_dyn_test_helpers::unique_ptr_dyn_test::*;

#[gtest]
fn test_virtual_destructor() {
    unsafe {
        BaseWithVirtualDestructor::set_instances(1); // One for the upcoming allocation
        {
            let _up = create_base();
            assert_eq!(BaseWithVirtualDestructor::instances(), 1);
        }
        assert_eq!(BaseWithVirtualDestructor::instances(), 0);
    }
}

#[gtest]
fn test_overloaded_delete() {
    unsafe {
        WithOverloadedDelete::set_delete_called(false);
        {
            let _up = create_with_overloaded_delete();
        }
        assert!(WithOverloadedDelete::delete_called());
    }
}
