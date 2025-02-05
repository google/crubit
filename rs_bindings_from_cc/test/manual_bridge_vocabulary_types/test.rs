// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use helper_lib::*;
use std::any::Any as _;

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
    // MakeUniquePtrString still gets bindings in :experimental, using ctor
    // -- but it won't be the Rust vector reimplementation.
    let rv = helper_lib::MakeUniquePtrString();
    let rust_unique_ptr_string_type =
        std::any::TypeId::of::<cc_std::std::unique_ptr<cc_std::std::string>>();
    let rv_type = rv.type_id();
    assert_ne!(
        rust_unique_ptr_string_type, rv_type,
        "Crubit must not return a unique_ptr<std::string>, because string is bridged"
    )
}

#[gtest]
fn test_vector_wrapped_by_value_as_function_arg_and_return_value() {
    let mut v: cc_std::std::Vector<i32> = MakeVector(1);
    let r = unsafe { UseVectorByRef(&mut v) };
    let v = UseVectorByValue(v);
    assert_eq!(v, 1);
    assert_eq!(r, 1);
}

/// std::vector<std::string> is not supported - because std::vector is a bridged type,
/// the corresponding Rust type is different, and a vector cannot be "reinterpreted" in place.
#[gtest]
fn test_vector_string() {
    // MakeVectorString still gets bindings in :experimental, using ctor
    // -- but it won't be the Rust vector reimplementation.
    let rv = helper_lib::MakeVectorString();
    let rust_vector_string_type =
        std::any::TypeId::of::<cc_std::std::Vector<cc_std::std::string>>();
    let rv_type = rv.type_id();
    assert_ne!(
        rust_vector_string_type, rv_type,
        "Crubit must not return a Vector<std::string>, because string is bridged"
    )
}
