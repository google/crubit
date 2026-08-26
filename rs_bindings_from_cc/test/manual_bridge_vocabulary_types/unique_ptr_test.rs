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

/// unique_ptr<std::string> is supported because std::string is layout-compatible with string.
#[gtest]
fn test_unique_ptr_string() {
    let mut p: cc_std::std::unique_ptr<cc_std::std::string> = unique_ptr_lib::MakeUniquePtrString();
    let s: &cc_std::std::string = p.as_ref().unwrap();
    assert_eq!(&*s, &b"hello, world"[..]);
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

#[gtest]
fn test_unique_ptr_no_bindings() {
    assert!(!item_exists::value_exists!(unique_ptr_lib::MakeUniquePtrNoBindings))
}
