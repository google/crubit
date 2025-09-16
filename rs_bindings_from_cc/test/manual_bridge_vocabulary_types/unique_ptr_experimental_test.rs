// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use common::*;
use googletest::prelude::*;

/// Even if MakeUniquePtrOverloadedDelete gets bindings (due to experimental), it must not
/// be the Rust unique_ptr reimplementation, because of the overloaded operator delete.
#[gtest]
fn test_unique_ptr_overloaded_delete() {
    let p = unique_ptr_experimental_lib::MakeUniquePtrOverloadedDelete();
    assert_ne!(
        std::any::Any::type_id(&p),
        std::any::TypeId::of::<cc_std::std::unique_ptr<OverloadedDelete>>()
    );
}

/// Even if MakeUniquePtrOverloadedDestroyingDelete gets bindings (due to experimental), it must not
/// be the Rust unique_ptr reimplementation, because of the overloaded operator delete.
#[gtest]
fn test_unique_ptr_overloaded_destroying_delete() {
    let p = unique_ptr_experimental_lib::MakeUniquePtrOverloadedDestroyingDelete();
    assert_ne!(
        std::any::Any::type_id(&p),
        std::any::TypeId::of::<cc_std::std::unique_ptr<OverloadedDestroyingDelete>>()
    );
}

/// Even if MakeUniquePtrPolymorphicType gets bindings (due to experimental), it must not
/// be the Rust unique_ptr reimplementation, because it can call a derived class's overloaded
/// operator delete, but the Rust reimplementation will not.
#[gtest]
fn test_unique_ptr_polymorphic_type() {
    let p = unique_ptr_experimental_lib::MakeUniquePtrPolymorphicType();
    assert_ne!(
        std::any::Any::type_id(&p),
        std::any::TypeId::of::<cc_std::std::unique_ptr<PolymorphicType>>()
    );
}
