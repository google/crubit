// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use conversion_operators::{
    ConvertsIntoIntImplicitly, ConvertsIntoIntsRef, ConvertsIntoVariousTypes, ConvertsRvalueOnly,
    DstLocalMovable, DstLocalNonMovable,
};
use cref::{CMut, CRef};
use ctor::emplace;
use ctor::CtorNew as _;
use foreign_type::{ForeignImmovable, ForeignMovable};
use googletest::assert_that;
use googletest::gtest;
use googletest::matchers::eq;

#[gtest]
fn test_as_ref() {
    let src = ConvertsIntoIntsRef { value: 0, inner_val: 42 };
    let val_ref: CRef<'_, i32> = (&src).into();
    assert_that!(*unsafe { CRef::unchanging(val_ref) }, eq(42));
    let val_ref2 = CRef::from(&src);
    assert_that!(*unsafe { CRef::unchanging(val_ref2) }, eq(42));
}

#[gtest]
fn test_as_mut() {
    let mut src = ConvertsIntoIntsRef { value: 0, inner_val: 42 };
    let val_mut: CMut<'_, i32> = (&mut src).into();
    let val_mut_ref = unsafe { CMut::unpin_unique(val_mut) };
    assert_that!(*val_mut_ref, eq(42));
    let val_mut2 = CMut::from(&mut src);
    let val_mut2_ref = unsafe { CMut::unpin_unique(val_mut2) };
    assert_that!(*val_mut2_ref, eq(42));
}

#[gtest]
fn test_from_local_movable() {
    let src = ConvertsIntoVariousTypes { value: 42 };
    let dst = DstLocalMovable::from(&src);
    assert_that!(dst.val, eq(42));
}

#[gtest]
fn test_into_local_movable() {
    let src = ConvertsIntoVariousTypes { value: 42 };
    let dst: DstLocalMovable = (&src).into();
    assert_that!(dst.val, eq(42));
}

#[gtest]
fn test_into_primitive() {
    let src = ConvertsIntoVariousTypes { value: 42 };
    let val: i32 = (&src).into();
    assert_that!(val, eq(42));
    let val2 = i32::from(&src);
    assert_that!(val2, eq(42));
}

#[gtest]
fn test_ctor_new_local_non_movable() {
    let src = ConvertsIntoVariousTypes { value: 42 };
    let dst = emplace!(DstLocalNonMovable::ctor_new(&src));
    assert_that!(dst.val, eq(42));
}

#[gtest]
fn test_implicit_conversion() {
    let src = ConvertsIntoIntImplicitly { value: 42 };
    let val: i32 = (&src).into();
    assert_that!(val, eq(42));
}

#[gtest]
fn test_into_double_non_const() {
    let mut src = ConvertsIntoVariousTypes { value: 42 };
    let val: f64 = (&mut src).into();
    assert_that!(val, eq(42.0));
    let val2 = f64::from(&mut src);
    assert_that!(val2, eq(42.0));
}

#[gtest]
fn test_rvalue_ref_qualified() {
    let src = ConvertsRvalueOnly { value: 42 };
    let dst = DstLocalMovable::from(src);
    assert_that!(dst.val, eq(42));
}

#[gtest]
fn test_cross_namespace() {
    let src = conversion_operators::namespace_a::Src { value: 42 };
    let dst = conversion_operators::namespace_b::Dst::from(&src);
    assert_that!(dst.val, eq(42));
}

#[gtest]
fn test_cross_namespace_into() {
    let src = conversion_operators::namespace_a::Src { value: 42 };
    let dst: conversion_operators::namespace_b::Dst = (&src).into();
    assert_that!(dst.val, eq(42));
}

#[gtest]
fn test_foreign_movable() {
    let src = ConvertsIntoVariousTypes { value: 42 };
    let dst: ForeignMovable = (&src).into();
    assert_that!(dst.val, eq(42));
    let dst2 = ForeignMovable::from(&src);
    assert_that!(dst2.val, eq(42));
}

#[gtest]
fn test_foreign_immovable() {
    let src = ConvertsIntoVariousTypes { value: 42 };
    let dst = emplace!(ForeignImmovable::ctor_new(&src));
    assert_that!(dst.val, eq(42));
}
