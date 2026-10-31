// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bitwise_relocatable::*;
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

assert_impl_all!(OptInt: Unpin);
assert_not_impl_any!(OptUserMove: Unpin); // TODO: std table follow-up will make this work

assert_impl_all!(FieldOverrideUserMove: Unpin);

assert_impl_all!(UserCopy: Unpin);

assert_not_impl_any!(UnionWithStr: Unpin);

assert_not_impl_any!(DerivesUserMove: Unpin);
assert_not_impl_any!(IfEligibleDerivesUserMove: Unpin);

assert_not_impl_any!(Polymorphic: Unpin);

assert_impl_all!(FieldOverride: Unpin);

#[googletest::gtest]
fn test_compiles() {}
