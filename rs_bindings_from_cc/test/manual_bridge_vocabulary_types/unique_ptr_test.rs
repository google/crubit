// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use common::*;
use googletest::prelude::*;
use unique_ptr_lib::*;

#[gtest]
fn test_trivial_type_wrapped_by_unique_ptr_as_function_arg_and_return_value() {
    let mut p: cc_std::std::unique_ptr<i32> = MakeUniquePtr(1);
    let r = unsafe { UseUniquePtrByRef(&mut p) };
    let v = UseUniquePtrByValue(p);
    assert_eq!(v, 1);
    assert_eq!(r, 1);
}

#[gtest]
fn test_nontrivial_type_wrapped_by_unique_ptr_as_function_arg_and_return_value() {
    let mut p: cc_std::std::unique_ptr<NonTrivialType> = MakeUniquePtrForNonTrivialType(1);
    let r = unsafe { UseUniquePtrByRefForNonTrivialType(&mut p) };
    let v = UseUniquePtrTypeByValueForNonTrivialType(p);
    assert_eq!(v, 1);
    assert_eq!(r, 1);
}

/// unique_ptr<std::string> is not supported - because std::string is a bridged type,
/// the corresponding Rust type is different, and a vector cannot be "reinterpreted" in place.
#[gtest]
fn test_unique_ptr_string() {
    // MakeUniquePtrString still gets bindings in :experimental, using ctor and templates
    // -- but it won't be the Rust unique_ptr reimplementation.
    // However, because of the bridging operation, we don't necessarily know how to spell
    // the underlying type, and can't safely generate bindings here.
    assert!(!item_exists::value_exists!(unique_ptr_lib::MakeUniquePtrString))
}

/// Even if MakeUniquePtrOverloadedDelete gets bindings (due to experimental), it must not
/// be the Rust unique_ptr reimplementation, because of the overloaded operator delete.
#[gtest]
fn test_unique_ptr_overloaded_delete() {
    let p = unique_ptr_lib::MakeUniquePtrOverloadedDelete();
    assert_ne!(
        std::any::Any::type_id(&p),
        std::any::TypeId::of::<cc_std::std::unique_ptr<OverloadedDelete>>()
    );
}

/// Even if MakeUniquePtrOverloadedDestroyingDelete gets bindings (due to experimental), it must not
/// be the Rust unique_ptr reimplementation, because of the overloaded operator delete.
#[gtest]
fn test_unique_ptr_overloaded_destroying_delete() {
    let p = unique_ptr_lib::MakeUniquePtrOverloadedDestroyingDelete();
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
    let p = unique_ptr_lib::MakeUniquePtrPolymorphicType();
    assert_ne!(
        std::any::Any::type_id(&p),
        std::any::TypeId::of::<cc_std::std::unique_ptr<PolymorphicType>>()
    );
}

#[gtest]
fn test_unique_ptr_incomplete() {
    assert!(!item_exists::value_exists!(unique_ptr_lib::MakeUniquePtrIncompleteType))
}

#[gtest]
fn test_unique_ptr_deleted_destructor() {
    assert!(!item_exists::value_exists!(unique_ptr_lib::MakeUniquePtrDeletedDestructor))
}

#[gtest]
fn test_unique_ptr_final_type() {
    let _: cc_std::std::unique_ptr<FinalType> = unique_ptr_lib::MakeUniquePtrFinalType();
}
