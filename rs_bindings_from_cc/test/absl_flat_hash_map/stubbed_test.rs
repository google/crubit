// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This test exercises each of the codepaths from a call to a generated public method on the Rust
//! projection of an `absl::flat_hash_map` through FFI to a stub implementation of the C++ type.
//! All combinations of `Unpin`/`!Unpin` key and value types are covered for methods that accept or
//! return them.

use std::pin::Pin;

use absl_container::{OccupiedError, OccupiedMovError};
use ctor::{emplace, mov, CtorNew};
use googletest::prelude::*;
use stubbed::crubit::test::{
    CapacityT, EmptyT, IntsMap, IntsStubs, Nonunpin, NonunpinBothMap, NonunpinBothStubs,
    NonunpinKeyMap, NonunpinKeyStubs, NonunpinValueMap, NonunpinValueStubs, SizeT, TryEmplaceT,
};

#[gtest]
fn ints_len_stub() {
    let map =
        emplace!(IntsMap::ctor_new(mov!(emplace!(IntsStubs::ctor_new((SizeT::default(), 123))))));
    expect_that!(map.len(), eq(123));
}

#[gtest]
fn ints_capacity_stub() {
    let map = emplace!(IntsMap::ctor_new(mov!(emplace!(IntsStubs::ctor_new((
        CapacityT::default(),
        234,
    ))))));
    expect_that!(map.capacity(), eq(234));
}

#[gtest]
fn ints_is_empty_stub() {
    let map = emplace!(IntsMap::ctor_new(mov!(emplace!(IntsStubs::ctor_new((
        EmptyT::default(),
        false
    ))))));
    expect_that!(map.is_empty(), eq(false));

    let map =
        emplace!(IntsMap::ctor_new(mov!(emplace!(IntsStubs::ctor_new((EmptyT::default(), true))))));
    expect_that!(map.is_empty(), eq(true));
}

#[gtest]
fn ints_try_insert_ok() {
    let map = emplace!(IntsMap::ctor_new(mov!(emplace!(IntsStubs::ctor_new((
        TryEmplaceT::default(),
        mov!(emplace!(1)),
        mov!(emplace!(2)),
        true,
    ))))));
    let result: Result<(&i32, &mut u64), OccupiedError<i32, u64>> = map.try_insert(3, 4);
    expect_that!(result, ok((derefs_to(eq(&1)), derefs_to(eq(&2)))));
}

#[gtest]
fn ints_try_insert_err() {
    let map = emplace!(IntsMap::ctor_new(mov!(emplace!(IntsStubs::ctor_new((
        TryEmplaceT::default(),
        mov!(emplace!(1)),
        mov!(emplace!(2)),
        false,
    ))))));
    let result: Result<(&i32, &mut u64), OccupiedError<i32, u64>> = map.try_insert(3, 4);
    expect_that!(
        result,
        err(pat!(OccupiedError {
            element: (points_to(eq(&1)), derefs_to(eq(&2))),
            key: &3,
            value: &4,
        })),
    );
}

#[gtest]
fn try_insert_mov_nonunpin_key_ok() {
    let map = emplace!(NonunpinKeyMap::ctor_new(mov!(emplace!(NonunpinKeyStubs::ctor_new((
        TryEmplaceT::default(),
        mov!(emplace!(Nonunpin::ctor_new(1))),
        mov!(emplace!(2)),
        true,
    ))))));
    let result: Result<(&Nonunpin, &mut u64), OccupiedMovError<(&Nonunpin, &mut u64)>> =
        map.try_insert_mov(mov!(emplace!(Nonunpin::ctor_new(3))), mov!(emplace!(4)));
    expect_that!(result, ok((points_to(property!(&Nonunpin.value(), 1)), derefs_to(eq(&2)))));
}

#[gtest]
fn try_insert_mov_nonunpin_key_err() {
    let map = emplace!(NonunpinKeyMap::ctor_new(mov!(emplace!(NonunpinKeyStubs::ctor_new((
        TryEmplaceT::default(),
        mov!(emplace!(Nonunpin::ctor_new(1))),
        mov!(emplace!(2)),
        false,
    ))))));
    let result: Result<(&Nonunpin, &mut u64), OccupiedMovError<(&Nonunpin, &mut u64)>> =
        map.try_insert_mov(mov!(emplace!(Nonunpin::ctor_new(3))), mov!(emplace!(4)));
    expect_that!(
        result,
        err(pat!(OccupiedMovError {
            element: (points_to(property!(&Nonunpin.value(), 1)), derefs_to(eq(&2))),
        })),
    );
}

#[gtest]
fn try_insert_mov_nonunpin_value_ok() {
    let map = emplace!(NonunpinValueMap::ctor_new(mov!(emplace!(NonunpinValueStubs::ctor_new((
        TryEmplaceT::default(),
        mov!(emplace!(1)),
        mov!(emplace!(Nonunpin::ctor_new(2))),
        true,
    ))))));
    let result: Result<(&i32, Pin<&mut Nonunpin>), OccupiedMovError<(&i32, Pin<&mut Nonunpin>)>> =
        map.try_insert_mov(mov!(emplace!(3)), mov!(emplace!(Nonunpin::ctor_new(4))));
    expect_that!(
        result,
        ok((points_to(eq(&1)), result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(2))))
    );
}

#[gtest]
fn try_insert_mov_nonunpin_value_err() {
    let map = emplace!(NonunpinValueMap::ctor_new(mov!(emplace!(NonunpinValueStubs::ctor_new((
        TryEmplaceT::default(),
        mov!(emplace!(1)),
        mov!(emplace!(Nonunpin::ctor_new(2))),
        false,
    ))))));
    let result: Result<(&i32, Pin<&mut Nonunpin>), OccupiedMovError<(&i32, Pin<&mut Nonunpin>)>> =
        map.try_insert_mov(mov!(emplace!(3)), mov!(emplace!(Nonunpin::ctor_new(4))));
    expect_that!(
        result,
        err(pat!(OccupiedMovError {
            element: (
                points_to(eq(&1)),
                result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(2)),
            ),
        })),
    );
}

#[gtest]
fn try_insert_mov_nonunpin_both_ok() {
    let map = emplace!(NonunpinBothMap::ctor_new(mov!(emplace!(NonunpinBothStubs::ctor_new((
        TryEmplaceT::default(),
        mov!(emplace!(Nonunpin::ctor_new(1))),
        mov!(emplace!(Nonunpin::ctor_new(2))),
        true,
    ))))));
    let result: Result<
        (&Nonunpin, Pin<&mut Nonunpin>),
        OccupiedMovError<(&Nonunpin, Pin<&mut Nonunpin>)>,
    > = map.try_insert_mov(
        mov!(emplace!(Nonunpin::ctor_new(3))),
        mov!(emplace!(Nonunpin::ctor_new(4))),
    );
    expect_that!(
        result,
        ok((
            points_to(property!(&Nonunpin.value(), 1)),
            result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(2)),
        )),
    );
}

#[gtest]
fn try_insert_mov_nonunpin_both_err() {
    let map = emplace!(NonunpinBothMap::ctor_new(mov!(emplace!(NonunpinBothStubs::ctor_new((
        TryEmplaceT::default(),
        mov!(emplace!(Nonunpin::ctor_new(1))),
        mov!(emplace!(Nonunpin::ctor_new(2))),
        false,
    ))))));
    let result: Result<
        (&Nonunpin, Pin<&mut Nonunpin>),
        OccupiedMovError<(&Nonunpin, Pin<&mut Nonunpin>)>,
    > = map.try_insert_mov(
        mov!(emplace!(Nonunpin::ctor_new(3))),
        mov!(emplace!(Nonunpin::ctor_new(4))),
    );
    expect_that!(
        result,
        err(pat!(OccupiedMovError {
            element: (
                points_to(property!(&Nonunpin.value(), 1)),
                result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(2)),
            ),
        })),
    );
}
