// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bitwise_relocatable::{
    AnnotatedNeverBR, AnnotatedNeverBRTrivialAbi, AssertedBR, ContainerOfAnnotatedNeverBR,
    DeletedMoveAssign, DerivesUserMove, FieldOverrideUserMove, FuncVoid, GetAssertedBR,
    GetStdStringOnLibcxxLength, GetStructWithVecAndUniquePtrLen, HolderK2,
    IfEligibleDerivesUserMove, IfEligibleTrivial, IfEligibleUserMove, MakeAssertedBR,
    MakeStdStringOnLibcxx, MakeStructWithVecAndUniquePtr, MapIntInt, NestedIfEligible, OptAsserted,
    OptInt, OptUserMove, Polymorphic, RowCursorShape, SharedPtrUserMove, StdStringOnLibcxx,
    Trivial, UnionWithStr, UniquePtrInt, UniquePtrUserMove, UserCopy, UserProvidedMoveDtor,
    VariantIntStr, VecInt, VecStr, A, B, J, K, K2, TA,
};
use googletest::expect_that;
use googletest::gtest;
use googletest::matchers::eq;
use static_assertions::{assert_impl_all, assert_not_impl_any};

assert_impl_all!(Trivial: Unpin);
assert_not_impl_any!(UserProvidedMoveDtor: Unpin);
assert_impl_all!(AssertedBR: Unpin);
assert_impl_all!(IfEligibleTrivial: Unpin);
assert_not_impl_any!(IfEligibleUserMove: Unpin);
assert_impl_all!(NestedIfEligible: Unpin);

assert_not_impl_any!(J: Unpin);
assert_not_impl_any!(K: Unpin);
assert_impl_all!(K2: Unpin);
assert_impl_all!(HolderK2: Unpin);
assert_not_impl_any!(DeletedMoveAssign: Unpin);

assert_not_impl_any!(A: Unpin);
assert_not_impl_any!(B: Unpin);
assert_impl_all!(TA: Unpin);

assert_impl_all!(FieldOverrideUserMove: Unpin);

assert_impl_all!(UserCopy: Unpin);

assert_not_impl_any!(UnionWithStr: Unpin);

assert_not_impl_any!(DerivesUserMove: Unpin);
assert_not_impl_any!(IfEligibleDerivesUserMove: Unpin);

assert_not_impl_any!(Polymorphic: Unpin);

assert_not_impl_any!(AnnotatedNeverBR: Unpin);
assert_not_impl_any!(AnnotatedNeverBRTrivialAbi: Unpin);
assert_not_impl_any!(ContainerOfAnnotatedNeverBR: Unpin);

assert_impl_all!(StdStringOnLibcxx: Unpin);

// Std table tests
assert_impl_all!(UniquePtrInt: Unpin);
assert_impl_all!(UniquePtrUserMove: Unpin);
assert_impl_all!(SharedPtrUserMove: Unpin);
assert_impl_all!(OptInt: Unpin);
assert_not_impl_any!(OptUserMove: Unpin);
assert_impl_all!(OptAsserted: Unpin);
assert_impl_all!(VecStr: Unpin);
assert_impl_all!(VecInt: Unpin);
assert_not_impl_any!(VariantIntStr: Unpin);
assert_not_impl_any!(FuncVoid: Unpin);
assert_not_impl_any!(MapIntInt: Unpin);
assert_impl_all!(RowCursorShape: Unpin);

#[gtest]
fn test_move_asserted_br() {
    let a = MakeAssertedBR(42);
    let b = a;
    expect_that!(GetAssertedBR(&b), eq(42));
}

#[gtest]
fn test_move_field_override() {
    let a = MakeStdStringOnLibcxx();
    let b = a;
    expect_that!(GetStdStringOnLibcxxLength(&b), eq(5));
}

#[gtest]
fn test_move_vec_unique_ptr() {
    let a = MakeStructWithVecAndUniquePtr();
    let b = a;
    expect_that!(GetStructWithVecAndUniquePtrLen(&b), eq(43)); // 5 + 42
}
