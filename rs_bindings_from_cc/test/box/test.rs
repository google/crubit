// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use boxes::test;
use googletest::expect_eq;
use googletest::gtest;

#[gtest]
fn test_box_roundtrip() {
    let b = Box::new(42);
    let out = test::ConsumeBox(b);
    expect_eq!(out, 42);

    let b2 = Box::new(10);
    let mut b3 = test::RoundtripBox(b2);
    expect_eq!(*b3, 10);

    test::MutateBox(&mut b3);
    expect_eq!(*b3, 100);

    let b4 = test::MakeBox(20);
    expect_eq!(test::ReadBox(&b4), 20);
    unsafe {
        expect_eq!(test::ReadBoxPtr(&b4 as *const _), 20);
    }

    let point = test::MakeBoxedPoint();
    expect_eq!(test::PointX(point), 1);
}

#[gtest]
fn test_pin_box() {
    let unpin = test::MakePinBoxUnpin();
    expect_eq!(*unpin, 100);

    let a = test::MakePinBoxNonTrivial(7);
    expect_eq!(test::ReadPinBoxNonTrivial(&a), 7);
    let b = test::SharePinBoxNonTrivial(&a);
    expect_eq!(test::NonTrivialUseCount(&a), 2);

    drop(b);
    expect_eq!(test::NonTrivialUseCount(&a), 1);

    test::ConsumePinBox(a);
}

#[gtest]
fn test_slice_box() {
    let b = test::MakeSliceBox(3);
    expect_eq!(test::SumSliceBox(&b), 126);

    let b2 = vec![1, 2, 3].into_boxed_slice();
    expect_eq!(test::ConsumeSliceBox(b2), 3);
}

#[gtest]
fn test_str_box() {
    let b = test::MakeStrBox();
    expect_eq!(test::StrBoxLen(&b), 5);

    let b2: Box<str> = "world".into();
    expect_eq!(test::ConsumeStrBox(b2), 5);
}

#[gtest]
fn test_holder() {
    static_assertions::assert_impl_all!(test::Holder: Unpin);
    static_assertions::assert_impl_all!(test::Outer: Unpin);
    static_assertions::assert_impl_all!(test::ArrayHolder: Unpin);

    let mut h = test::MakeHolder(5);
    expect_eq!(**h.b, 5);
    **h.b = 6;
    expect_eq!(test::HolderValue(&h), 6);
    test::SetHolderValue(&mut h, 7);
    let moved = h; // plain Rust move: Unpin
    expect_eq!(test::HolderValue(&moved), 7);
    let boxed: Box<test::Holder> = Box::new(moved);
    expect_eq!(test::HolderValue(&boxed), 7);

    let outer = test::MakeOuter(8);
    expect_eq!(outer.tag, 9);
    expect_eq!(test::HolderValue(&outer.h), 8);
}

#[gtest]
fn test_pin_holder_drop() {
    static_assertions::assert_impl_all!(test::PinHolder: Unpin);
    let a = test::MakePinBoxNonTrivial(42);
    let h = test::PinHolder { p: ::core::mem::ManuallyDrop::new(test::SharePinBoxNonTrivial(&a)) };
    expect_eq!(test::NonTrivialUseCount(&a), 2);
    drop(h);
    expect_eq!(test::NonTrivialUseCount(&a), 1);
}
