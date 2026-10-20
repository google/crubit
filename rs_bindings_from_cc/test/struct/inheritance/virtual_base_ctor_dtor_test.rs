// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::{emplace, CtorNew as _};
use googletest::{expect_eq, gtest};
use virtual_base_ctor_dtor::virtual_base_ctor_dtor::{Derived, VirtualBase};

#[gtest]
fn test_constructor_and_destructor_of_class_with_virtual_base() {
    let destructor_call_count_before = Derived::GetDestructorCallCount();
    let virtual_base_destructor_call_count_before = VirtualBase::GetDestructorCallCount();
    {
        let derived = emplace!(Derived::ctor_new(42));
        expect_eq!(derived.value(), 42);
        expect_eq!(derived.base_field_from_cpp(), 123);
    }
    expect_eq!(Derived::GetDestructorCallCount(), destructor_call_count_before + 1);
    // The complete destructor also destroys the virtual base.
    expect_eq!(
        VirtualBase::GetDestructorCallCount(),
        virtual_base_destructor_call_count_before + 1
    );
}
