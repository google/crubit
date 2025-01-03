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

#[gtest]
fn test_vector_wrapped_by_value_as_function_arg_and_return_value() {
    let mut v: cc_std::std::Vector<i32> = MakeVector(1);
    let r = unsafe { UseVectorByRef(&mut v) };
    let v = UseVectorByValue(v);
    assert_eq!(v, 1);
    assert_eq!(r, 1);
}
