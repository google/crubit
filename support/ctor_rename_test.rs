// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! A sample of tests from ctor_proc_macros_test.rs, but using an alternate crate name.

// Callers are expected to enable `negative_impls`.
#![feature(negative_impls)]

use googletest::prelude::*;

#[gtest]
fn test_derive_default_unit_struct() {
    #[derive(renamed_ctor::CtorFrom_Default)]
    #[ctor(crate = renamed_ctor)]
    struct Struct;
    unsafe impl renamed_ctor::RecursivelyPinned for Struct {
        type CtorInitializedFields = Self;
    }
    impl !Unpin for Struct {}

    let _p = renamed_ctor::emplace!(<Struct as renamed_ctor::CtorNew<()>>::ctor_new(()));
}

#[gtest]
fn test_derive_move_and_assign_via_copy() {
    #[derive(Copy, Clone, renamed_ctor::MoveAndAssignViaCopy)]
    #[ctor(crate = renamed_ctor)]
    struct Struct {
        #[allow(unused)]
        x: i32,
        #[allow(unused)]
        y: f32,
    }

    fn implements_traits<T>()
    where
        T: for<'a> From<renamed_ctor::RvalueReference<'a, T>>
            + for<'a> renamed_ctor::CtorNew<renamed_ctor::RvalueReference<'a, T>>
            + for<'a> renamed_ctor::UnpinAssign<&'a T>
            + for<'a> renamed_ctor::UnpinAssign<renamed_ctor::RvalueReference<'a, T>>,
    {
    }

    implements_traits::<Struct>();
}

#[gtest]
fn test_recursively_pinned_unit_struct() {
    #[renamed_ctor::recursively_pinned(crate = renamed_ctor)]
    struct S;
    let _ = Box::pin(S).as_mut().project_pin();
    assert_eq!(std::mem::size_of::<renamed_ctor::project_pin_type!(S)>(), 0);
}
