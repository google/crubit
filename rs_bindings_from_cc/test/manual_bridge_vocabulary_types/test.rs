// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use helper_lib::*;

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
    assert!(!item_exists::value_exists!(helper_lib::MakeUniquePtrString))
}

/// Even if MakeUniquePtrOverloadedDelete gets bindings (due to experimental), it must not
/// be the Rust unique_ptr reimplementation, because of the overloaded operator delete.
#[gtest]
fn test_unique_ptr_overloaded_delete() {
    let p = helper_lib::MakeUniquePtrOverloadedDelete();
    assert_ne!(
        std::any::Any::type_id(&p),
        std::any::TypeId::of::<cc_std::std::unique_ptr<OverloadedDelete>>()
    );
}

/// Even if MakeUniquePtrOverloadedDestroyingDelete gets bindings (due to experimental), it must not
/// be the Rust unique_ptr reimplementation, because of the overloaded operator delete.
#[gtest]
fn test_unique_ptr_overloaded_destroying_delete() {
    let p = helper_lib::MakeUniquePtrOverloadedDestroyingDelete();
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
    let p = helper_lib::MakeUniquePtrPolymorphicType();
    assert_ne!(
        std::any::Any::type_id(&p),
        std::any::TypeId::of::<cc_std::std::unique_ptr<PolymorphicType>>()
    );
}

#[gtest]
fn test_unique_ptr_incomplete() {
    assert!(!item_exists::value_exists!(helper_lib::MakeUniquePtrIncompleteType))
}

#[gtest]
fn test_unique_ptr_deleted_destructor() {
    assert!(!item_exists::value_exists!(helper_lib::MakeUniquePtrDeletedDestructor))
}

#[gtest]
fn test_unique_ptr_final_type() {
    let _: cc_std::std::unique_ptr<FinalType> = helper_lib::MakeUniquePtrFinalType();
}

#[gtest]
fn test_vector_wrapped_by_value_as_function_arg_and_return_value() {
    let mut v: cc_std::std::vector<i32> = MakeVector(1);
    let r = unsafe { UseVectorByRef(&mut v) };
    let v = UseVectorByValue(v);
    assert_eq!(v, 1);
    assert_eq!(r, 1);
}

/// std::vector<std::string> is not supported - because std::vector is a bridged type,
/// the corresponding Rust type is different, and a vector cannot be "reinterpreted" in place.
#[gtest]
fn test_vector_string() {
    // MakeVectorString could still get bindings in :wrapper, using ctor and templates
    // -- but it won't be the Rust vector reimplementation.
    // However, because of the bridging operation, we don't necessarily know how to spell
    // the underlying type, and can't safely generate bindings here.
    assert!(!item_exists::value_exists!(helper_lib::MakeVectorString))
}
/// Even if MakeVectorBool gets bindings (due to experimental), it must not
/// be the Rust vector reimplementation, because of the vector<bool> specialization.
#[gtest]
fn test_vector_bool() {
    let v = helper_lib::MakeVectorBool();
    assert_ne!(std::any::Any::type_id(&v), std::any::TypeId::of::<cc_std::std::vector<bool>>());
}

/// Even if MakeVectorOverloadedDelete gets bindings (due to experimental), it must not
/// be the Rust vector reimplementation, because of the overloaded operator delete.
#[gtest]
fn test_vector_overloaded_delete() {
    let v = helper_lib::MakeVectorOverloadedDelete();
    assert_ne!(
        std::any::Any::type_id(&v),
        std::any::TypeId::of::<cc_std::std::vector<OverloadedDelete>>()
    );
}

/// Even if MakeVectorOverloadedDestroyingDelete gets bindings (due to experimental), it must not
/// be the Rust vector reimplementation, because of the overloaded operator delete.
#[gtest]
fn test_vector_overloaded_destroying_delete() {
    let v = helper_lib::MakeVectorOverloadedDestroyingDelete();
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
    let v = helper_lib::MakeVectorPolymorphicType();
    assert_ne!(
        std::any::Any::type_id(&v),
        std::any::TypeId::of::<cc_std::std::vector<PolymorphicType>>()
    );
}

#[gtest]
fn test_vector_final_type() {
    let _: cc_std::std::vector<FinalType> = helper_lib::MakeVectorFinalType();
}

#[gtest]
fn test_vector_deleted_destructor() {
    assert!(!item_exists::value_exists!(helper_lib::MakeVectorDeletedDestructor))
}

/// The Rust std::vector reimplementation does support non-trivial types, but it's not very
/// useful with them!
#[gtest]
fn test_vector_non_trivial() {
    let v: cc_std::std::vector<NonTrivialType> = helper_lib::MakeVectorNonTrivial();
    assert_eq!(v.len(), 0);
}
