// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use common::*;
use googletest::prelude::*;
use vector_lib::*;

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
    assert!(!item_exists::value_exists!(vector_lib::MakeVectorString))
}

#[gtest]
fn test_vector_bool() {
    assert!(!item_exists::value_exists!(vector_lib::MakeVectorBool))
}

#[gtest]
fn test_vector_overloaded_delete() {
    assert!(!item_exists::value_exists!(vector_lib::MakeVectorOverloadedDelete))
}

#[gtest]
fn test_vector_overloaded_destroying_delete() {
    assert!(!item_exists::value_exists!(vector_lib::MakeVectorOverloadedDestroyingDelete))
}

#[gtest]
fn test_vector_polymorphic() {
    assert!(!item_exists::value_exists!(vector_lib::MakeVectorPolymorphicType))
}

#[gtest]
fn test_vector_final_type() {
    let _: cc_std::std::vector<FinalType> = vector_lib::MakeVectorFinalType();
}

#[gtest]
fn test_vector_deleted_destructor() {
    assert!(!item_exists::value_exists!(vector_lib::MakeVectorDeletedDestructor))
}

#[gtest]
fn test_vector_no_bindings() {
    assert!(!item_exists::value_exists!(vector_lib::MakeUniquePtrNoBindings))
}
