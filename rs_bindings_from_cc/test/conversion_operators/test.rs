// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use conversion_operators::{
    ConvertsIntoIntImplicitly, ConvertsIntoIntsRef, ConvertsIntoVariousTypes, DstLocalMovable,
    DstLocalNonMovable,
};
use cref::{CMut, CRef};
use ctor::emplace;
use ctor::CtorNew as _;
use googletest::gtest;

#[gtest]
fn test_as_ref() {
    let src = ConvertsIntoIntsRef { value: 0, inner_val: 42 };
    let val_ref: CRef<'_, i32> = (&src).into();
    assert_eq!(*unsafe { CRef::unchanging(val_ref) }, 42);
}

#[gtest]
fn test_as_mut() {
    let mut src = ConvertsIntoIntsRef { value: 0, inner_val: 42 };
    let val_mut: CMut<'_, i32> = (&mut src).into();
    let val_mut_ref = unsafe { CMut::unpin_unique(val_mut) };
    assert_eq!(*val_mut_ref, 42);
    *val_mut_ref = 100;
    assert_eq!(src.inner_val, 100);
}

#[gtest]
fn test_from_local_movable() {
    let src = ConvertsIntoVariousTypes { value: 42 };
    let dst = DstLocalMovable::from(&src);
    assert_eq!(dst.val, 42);
}

#[gtest]
fn test_into_primitive() {
    let src = ConvertsIntoVariousTypes { value: 42 };
    let val: i32 = (&src).into();
    assert_eq!(val, 42);
}

#[gtest]
fn test_ctor_new_local_non_movable() {
    let src = ConvertsIntoVariousTypes { value: 42 };
    let dst = emplace!(DstLocalNonMovable::ctor_new(&src));
    assert_eq!(dst.val, 42);
}

#[gtest]
fn test_implicit_conversion() {
    let src = ConvertsIntoIntImplicitly { value: 42 };
    let val: i32 = (&src).into();
    assert_eq!(val, 42);
}

#[gtest]
fn test_into_double_non_const() {
    let mut src = ConvertsIntoVariousTypes { value: 42 };
    let val: f64 = (&mut src).into();
    assert_eq!(val, 42.0);
}

#[gtest]
fn test_cross_namespace() {
    let src = conversion_operators::namespace_a::Src { value: 42 };
    let dst = conversion_operators::namespace_b::Dst::from(&src);
    assert_eq!(dst.val, 42);
}
