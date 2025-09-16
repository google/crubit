// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use common::*;
use googletest::prelude::*;

/// Even if MakeVectorBool gets bindings (due to experimental), it must not
/// be the Rust vector reimplementation, because of the vector<bool> specialization.
#[gtest]
fn test_vector_bool() {
    let v = vector_experimental_lib::MakeVectorBool();
    assert_ne!(std::any::Any::type_id(&v), std::any::TypeId::of::<cc_std::std::vector<bool>>());
}

/// Even if MakeVectorOverloadedDelete gets bindings (due to experimental), it must not
/// be the Rust vector reimplementation, because of the overloaded operator delete.
#[gtest]
fn test_vector_overloaded_delete() {
    let v = vector_experimental_lib::MakeVectorOverloadedDelete();
    assert_ne!(
        std::any::Any::type_id(&v),
        std::any::TypeId::of::<cc_std::std::vector<OverloadedDelete>>()
    );
}

/// Even if MakeVectorOverloadedDestroyingDelete gets bindings (due to experimental), it must not
/// be the Rust vector reimplementation, because of the overloaded operator delete.
#[gtest]
fn test_vector_overloaded_destroying_delete() {
    let v = vector_experimental_lib::MakeVectorOverloadedDestroyingDelete();
    assert_ne!(
        std::any::Any::type_id(&v),
        std::any::TypeId::of::<cc_std::std::vector<OverloadedDestroyingDelete>>()
    );
}

/// Even if MakeVectorPolymorphicType gets bindings (due to experimental), it must not
/// be the Rust vector reimplementation, because it can call a derived class's overloaded
/// operator delete, but the Rust reimplementation will not.
#[gtest]
fn test_vector_polymorphic() {
    let v = vector_experimental_lib::MakeVectorPolymorphicType();
    assert_ne!(
        std::any::Any::type_id(&v),
        std::any::TypeId::of::<cc_std::std::vector<PolymorphicType>>()
    );
}

/// The Rust std::vector reimplementation does support non-trivial types, but it's not very
/// useful with them!
#[gtest]
fn test_vector_non_trivial() {
    let v: cc_std::std::vector<NonTrivialType> = vector_experimental_lib::MakeVectorNonTrivial();
    assert_eq!(v.len(), 0);
}
