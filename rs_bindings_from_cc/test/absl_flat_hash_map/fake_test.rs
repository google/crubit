// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This test exercises each of the codepaths from a call to a generated public method on the Rust
//! projection of an `absl::flat_hash_map` through FFI to a fake implementation of the C++ type.
//! All combinations of `Unpin`/`!Unpin` key and value types are covered for methods that accept or
//! return them.

use std::pin::Pin;

use container::OccupiedError;
use ctor::{emplace, mov, CtorNew, RvalueReference};
use fake::crubit::test::{
    IntsMap, MoveOnly, MoveOnlyMap, Nonunpin, NonunpinBothMap, NonunpinKeyMap, NonunpinValueMap,
};
use googletest::prelude::*;

#[gtest]
fn ints_len_zero() {
    let map = emplace!(IntsMap::ctor_new(()));
    expect_that!(map.len(), eq(0));
}

#[gtest]
fn ints_len_three() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();
    map.as_mut().try_insert(3, 4).unwrap();
    map.as_mut().try_insert(5, 6).unwrap();
    expect_eq!(map.len(), 3);
}

#[gtest]
fn ints_is_empty_true() {
    let map = emplace!(IntsMap::ctor_new(()));
    expect_true!(map.is_empty());
}

#[gtest]
fn ints_is_empty_false() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();
    expect_false!(map.is_empty());
}

#[gtest]
fn ints_capacity_zero() {
    // NOTE: This fake doesn't have a real capacity and implements `capacity()` by returning double
    // the `len()`.
    let map = emplace!(IntsMap::ctor_new(()));
    expect_eq!(map.capacity(), 0);
}

#[gtest]
fn ints_capacity_four() {
    // NOTE: This fake doesn't have a real capacity and implements `capacity()` by returning double
    // the `len()`.
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();
    map.as_mut().try_insert(3, 4).unwrap();
    expect_eq!(map.capacity(), 4);
}

#[gtest]
fn ints_try_insert_ok() {
    let map = emplace!(IntsMap::ctor_new(()));
    let result: Result<(&i32, &mut u64), OccupiedError<(&i32, &mut u64), i32, u64>> =
        map.try_insert(1, 2);
    expect_that!(result, ok((derefs_to(eq(&1)), derefs_to(eq(&2)))));
}

#[gtest]
fn ints_try_insert_err() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();

    let result: Result<(&i32, &mut u64), OccupiedError<(&i32, &mut u64), i32, u64>> =
        map.try_insert(1, 3);
    expect_that!(
        result,
        err(pat!(OccupiedError {
            element: (points_to(eq(&1)), derefs_to(eq(&2))),
            key: &1,
            value: &3,
        })),
    );
}

#[gtest]
fn try_insert_nonunpin_key_ok() {
    let map = emplace!(NonunpinKeyMap::ctor_new(()));
    let mut key = emplace!(Nonunpin::ctor_new(1));
    let result: Result<
        (&Nonunpin, &mut u64),
        OccupiedError<(&Nonunpin, &mut u64), RvalueReference<Nonunpin>, u64>,
    > = map.try_insert(mov!(key), 2);
    expect_that!(result, ok((points_to(property!(&Nonunpin.value(), 1)), derefs_to(eq(&2)))));
}

#[gtest]
fn try_insert_nonunpin_key_err() {
    let mut map = emplace!(NonunpinKeyMap::ctor_new(()));
    map.as_mut().try_insert(mov!(emplace!(Nonunpin::ctor_new(1))), 2).unwrap();

    let mut key = emplace!(Nonunpin::ctor_new(1));
    let result: Result<
        (&Nonunpin, &mut u64),
        OccupiedError<(&Nonunpin, &mut u64), RvalueReference<Nonunpin>, u64>,
    > = map.try_insert(mov!(key), 3);
    expect_that!(
        result,
        err(pat!(OccupiedError {
            element: (points_to(property!(&Nonunpin.value(), 1)), derefs_to(eq(&2))),
            key: field!(
                RvalueReference.0,
                ref result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(1))
            ),
            value: &3,
        })),
    );
}

#[gtest]
fn try_insert_nonunpin_value_ok() {
    let map = emplace!(NonunpinValueMap::ctor_new(()));
    let mut value = emplace!(Nonunpin::ctor_new(2));
    let result: Result<
        (&i32, Pin<&mut Nonunpin>),
        OccupiedError<(&i32, Pin<&mut Nonunpin>), i32, RvalueReference<Nonunpin>>,
    > = map.try_insert(1, mov!(value));
    expect_that!(
        result,
        ok((points_to(eq(&1)), result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(2))))
    );
}

#[gtest]
fn try_insert_nonunpin_value_err() {
    let mut map = emplace!(NonunpinValueMap::ctor_new(()));
    map.as_mut().try_insert(1, mov!(emplace!(Nonunpin::ctor_new(2)))).unwrap();

    let mut value = emplace!(Nonunpin::ctor_new(3));
    let result: Result<
        (&i32, Pin<&mut Nonunpin>),
        OccupiedError<(&i32, Pin<&mut Nonunpin>), i32, RvalueReference<Nonunpin>>,
    > = map.try_insert(1, mov!(value));
    expect_that!(
        result,
        err(pat!(OccupiedError {
            element: (
                points_to(eq(&1)),
                result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(2)),
            ),
            key: &1,
            value: field!(
                RvalueReference.0,
                ref result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(3))
            ),
        })),
    );
}

#[gtest]
fn try_insert_nonunpin_both_ok() {
    let map = emplace!(NonunpinBothMap::ctor_new(()));
    let mut key = emplace!(Nonunpin::ctor_new(1));
    let mut value = emplace!(Nonunpin::ctor_new(2));
    let result: Result<
        (&Nonunpin, Pin<&mut Nonunpin>),
        OccupiedError<
            (&Nonunpin, Pin<&mut Nonunpin>),
            RvalueReference<Nonunpin>,
            RvalueReference<Nonunpin>,
        >,
    > = map.try_insert(mov!(key), mov!(value));
    expect_that!(
        result,
        ok((
            points_to(property!(&Nonunpin.value(), 1)),
            result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(2)),
        )),
    );
}

#[gtest]
fn try_insert_nonunpin_both_err() {
    let mut map = emplace!(NonunpinBothMap::ctor_new(()));
    map.as_mut()
        .try_insert(mov!(emplace!(Nonunpin::ctor_new(1))), mov!(emplace!(Nonunpin::ctor_new(2))))
        .unwrap();

    let mut key = emplace!(Nonunpin::ctor_new(1));
    let mut value = emplace!(Nonunpin::ctor_new(3));
    let result: Result<
        (&Nonunpin, Pin<&mut Nonunpin>),
        OccupiedError<
            (&Nonunpin, Pin<&mut Nonunpin>),
            RvalueReference<Nonunpin>,
            RvalueReference<Nonunpin>,
        >,
    > = map.try_insert(mov!(key), mov!(value));
    expect_that!(
        result,
        err(pat!(OccupiedError {
            element: (
                points_to(property!(&Nonunpin.value(), 1)),
                result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(2)),
            ),
            key: field!(
                RvalueReference.0,
                ref result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(1))
            ),
            value: field!(
                RvalueReference.0,
                ref result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(3))
            ),
        })),
    );
}

#[gtest]
fn try_insert_moveonly_ok() {
    let map = emplace!(MoveOnlyMap::ctor_new(()));
    let mut key = emplace!(MoveOnly::ctor_new(1));
    let mut value = emplace!(MoveOnly::ctor_new(2));
    let result: Result<
        (&MoveOnly, Pin<&mut MoveOnly>),
        OccupiedError<
            (&MoveOnly, Pin<&mut MoveOnly>),
            RvalueReference<MoveOnly>,
            RvalueReference<MoveOnly>,
        >,
    > = map.try_insert(mov!(key), mov!(value));
    expect_that!(
        result,
        ok((
            points_to(property!(&MoveOnly.value(), 1)),
            result_of!(|x: &Pin<&mut MoveOnly>| { x.as_ref().value() }, eq(2)),
        )),
    );
}

#[gtest]
fn try_insert_moveonly_err() {
    let mut map = emplace!(MoveOnlyMap::ctor_new(()));
    map.as_mut()
        .try_insert(mov!(emplace!(MoveOnly::ctor_new(1))), mov!(emplace!(MoveOnly::ctor_new(2))))
        .unwrap();

    let mut key = emplace!(MoveOnly::ctor_new(1));
    let mut value = emplace!(MoveOnly::ctor_new(3));
    let result: Result<
        (&MoveOnly, Pin<&mut MoveOnly>),
        OccupiedError<
            (&MoveOnly, Pin<&mut MoveOnly>),
            RvalueReference<MoveOnly>,
            RvalueReference<MoveOnly>,
        >,
    > = map.try_insert(mov!(key), mov!(value));
    expect_that!(
        result,
        err(pat!(OccupiedError {
            element: (
                points_to(property!(&MoveOnly.value(), 1)),
                result_of!(|x: &Pin<&mut MoveOnly>| { x.as_ref().value() }, eq(2)),
            ),
            key: field!(
                RvalueReference.0,
                ref result_of!(|x: &Pin<&mut MoveOnly>| { x.as_ref().value() }, eq(1))
            ),
            value: field!(
                RvalueReference.0,
                ref result_of!(|x: &Pin<&mut MoveOnly>| { x.as_ref().value() }, eq(3))
            ),
        })),
    );
}
