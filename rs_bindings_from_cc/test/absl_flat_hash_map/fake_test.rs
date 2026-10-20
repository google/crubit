// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This test exercises each of the codepaths from a call to a generated public method on the Rust
//! projection of an `absl::flat_hash_map` through FFI to a fake implementation of the C++ type.
//! All combinations of `Unpin`/`!Unpin` key and value types are covered for methods that accept or
//! return them.

use std::pin::Pin;

use ctor::{emplace, mov, CtorNew};
use fake::crubit::test::{
    IntsMap, MoveOnly, MoveOnlyMap, Nonunpin, NonunpinBothMap, NonunpinKeyMap, NonunpinValueMap,
};
use googletest::matchers::{derefs_to, eq, err, ok, pat, points_to, property, result_of};
use googletest::{expect_eq, expect_false, expect_that, expect_true, gtest};
use map::OccupiedError;

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
    let result: Result<(&i32, &mut u64), OccupiedError<(&i32, &mut u64)>> = map.try_insert(1, 2);
    expect_that!(result, ok((derefs_to(eq(&1)), derefs_to(eq(&2)))));
}

#[gtest]
fn ints_try_insert_err() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();

    let result: Result<(&i32, &mut u64), OccupiedError<(&i32, &mut u64)>> = map.try_insert(1, 3);
    expect_that!(
        result,
        err(pat!(OccupiedError { entry: (points_to(eq(&1)), derefs_to(eq(&2))) })),
    );
}

#[gtest]
fn try_insert_nonunpin_key_ok() {
    let map = emplace!(NonunpinKeyMap::ctor_new(()));
    let result: Result<(&Nonunpin, &mut u64), OccupiedError<(&Nonunpin, &mut u64)>> =
        map.try_insert(Nonunpin::ctor_new(1), 2);
    expect_that!(result, ok((points_to(property!(&Nonunpin.value(), 1)), derefs_to(eq(&2)))));
}

#[gtest]
fn try_insert_nonunpin_key_err() {
    let mut map = emplace!(NonunpinKeyMap::ctor_new(()));
    map.as_mut().try_insert(Nonunpin::ctor_new(1), 2).unwrap();

    let result: Result<(&Nonunpin, &mut u64), OccupiedError<(&Nonunpin, &mut u64)>> =
        map.try_insert(Nonunpin::ctor_new(1), 3);
    expect_that!(
        result,
        err(pat!(OccupiedError {
            entry: (points_to(property!(&Nonunpin.value(), 1)), derefs_to(eq(&2))),
        })),
    );
}

#[gtest]
fn try_insert_nonunpin_value_ok() {
    let map = emplace!(NonunpinValueMap::ctor_new(()));
    let result: Result<(&i32, Pin<&mut Nonunpin>), OccupiedError<(&i32, Pin<&mut Nonunpin>)>> =
        map.try_insert(1, Nonunpin::ctor_new(2));
    expect_that!(
        result,
        ok((points_to(eq(&1)), result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(2))))
    );
}

#[gtest]
fn try_insert_nonunpin_value_err() {
    let mut map = emplace!(NonunpinValueMap::ctor_new(()));
    map.as_mut().try_insert(1, Nonunpin::ctor_new(2)).unwrap();

    let result: Result<(&i32, Pin<&mut Nonunpin>), OccupiedError<(&i32, Pin<&mut Nonunpin>)>> =
        map.try_insert(1, Nonunpin::ctor_new(3));
    expect_that!(
        result,
        err(pat!(OccupiedError {
            entry: (
                points_to(eq(&1)),
                result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(2)),
            ),
        })),
    );
}

#[gtest]
fn try_insert_nonunpin_both_ok() {
    let map = emplace!(NonunpinBothMap::ctor_new(()));
    let result: Result<
        (&Nonunpin, Pin<&mut Nonunpin>),
        OccupiedError<(&Nonunpin, Pin<&mut Nonunpin>)>,
    > = map.try_insert(Nonunpin::ctor_new(1), Nonunpin::ctor_new(2));
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
    map.as_mut().try_insert(Nonunpin::ctor_new(1), Nonunpin::ctor_new(2)).unwrap();

    let result: Result<
        (&Nonunpin, Pin<&mut Nonunpin>),
        OccupiedError<(&Nonunpin, Pin<&mut Nonunpin>)>,
    > = map.try_insert(Nonunpin::ctor_new(1), Nonunpin::ctor_new(3));
    expect_that!(
        result,
        err(pat!(OccupiedError {
            entry: (
                points_to(property!(&Nonunpin.value(), 1)),
                result_of!(|x: &Pin<&mut Nonunpin>| { x.as_ref().value() }, eq(2)),
            ),
        })),
    );
}

#[gtest]
fn try_insert_moveonly_ok() {
    let map = emplace!(MoveOnlyMap::ctor_new(()));
    let result: Result<
        (&MoveOnly, Pin<&mut MoveOnly>),
        OccupiedError<(&MoveOnly, Pin<&mut MoveOnly>)>,
    > = map.try_insert(MoveOnly::ctor_new(1), MoveOnly::ctor_new(2));
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
    map.as_mut().try_insert(MoveOnly::ctor_new(1), MoveOnly::ctor_new(2)).unwrap();

    let result: Result<
        (&MoveOnly, Pin<&mut MoveOnly>),
        OccupiedError<(&MoveOnly, Pin<&mut MoveOnly>)>,
    > = map.try_insert(MoveOnly::ctor_new(1), MoveOnly::ctor_new(3));
    expect_that!(
        result,
        err(pat!(OccupiedError {
            entry: (
                points_to(property!(&MoveOnly.value(), 1)),
                result_of!(|x: &Pin<&mut MoveOnly>| { x.as_ref().value() }, eq(2)),
            ),
        })),
    );
}

#[gtest]
fn try_insert_moveonly_mov() {
    let mut map = emplace!(MoveOnlyMap::ctor_new(()));
    let mut key = emplace!(MoveOnly::ctor_new(1));
    let mut value = emplace!(MoveOnly::ctor_new(2));

    let result = map.as_mut().try_insert(mov!(key), mov!(value)).unwrap();
    expect_eq!(result.0.value(), 1);
    expect_eq!(result.1.value(), 2);

    expect_eq!(key.value(), 0);
    expect_eq!(value.value(), 0);
}

#[gtest]
fn ints_clear() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();
    expect_eq!(map.len(), 1);
    map.as_mut().clear();
    expect_eq!(map.len(), 0);
    expect_true!(map.is_empty());
}

#[gtest]
fn ints_reserve() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().reserve(10);
    expect_that!(map.capacity(), googletest::matchers::ge(10));
}

#[gtest]
#[should_panic(expected = "capacity overflow")]
fn ints_reserve_overflow() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();
    map.as_mut().reserve(usize::MAX);
}

#[gtest]
fn ints_contains_key() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();
    expect_true!(map.contains_key(&1));
    expect_false!(map.contains_key(&2));
}

#[gtest]
fn ints_get() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();
    expect_that!(map.get(&1), eq(Some(&2)));
    expect_that!(map.get(&2), eq(None));
}

#[gtest]
fn ints_get_key_value() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();
    expect_that!(map.get_key_value(&1), eq(Some((&1, &2))));
    expect_that!(map.get_key_value(&2), eq(None));
}

#[gtest]
fn ints_get_mut() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();
    {
        let val = IntsMap::get_mut(map.as_mut(), &1).unwrap();
        *val = 3;
    }
    expect_that!(map.get(&1), eq(Some(&3)));
    expect_true!(IntsMap::get_mut(map.as_mut(), &2).is_none());
}

#[gtest]
fn ints_insert() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    {
        let (k, v) = map.as_mut().insert(1, 2);
        expect_eq!(*k, 1);
        expect_eq!(*v, 2);
    }
    expect_that!(map.get(&1), eq(Some(&2)));
    {
        let (k, v) = map.as_mut().insert(1, 3);
        expect_eq!(*k, 1);
        expect_eq!(*v, 3);
    }
    expect_that!(map.get(&1), eq(Some(&3)));
}

#[gtest]
fn ints_remove() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    map.as_mut().try_insert(1, 2).unwrap();
    expect_true!(map.as_mut().remove(&1));
    expect_false!(map.as_mut().remove(&1));
    expect_that!(map.get(&1), eq(None));
}

#[gtest]
fn nonunpin_key_tests() {
    let mut map = emplace!(NonunpinKeyMap::ctor_new(()));
    map.as_mut().try_insert(Nonunpin::ctor_new(1), 2).unwrap();

    let key_pin = emplace!(Nonunpin::ctor_new(1));
    let key_ref = &*key_pin;

    expect_true!(map.contains_key(key_ref));
    expect_that!(map.get(key_ref), eq(Some(&2)));
    expect_that!(map.get_key_value(key_ref).map(|(k, v)| (k.value(), *v)), eq(Some((1, 2))));

    {
        let val_mut: &mut u64 = NonunpinKeyMap::get_mut(map.as_mut(), key_ref).unwrap();
        *val_mut = 3;
    }
    expect_that!(map.get(key_ref), eq(Some(&3)));

    {
        let (k, v): (&Nonunpin, &mut u64) = map.as_mut().insert(Nonunpin::ctor_new(1), 4);
        expect_eq!(k.value(), 1);
        expect_eq!(*v, 4);
    }
    expect_that!(map.get(key_ref), eq(Some(&4)));

    expect_true!(map.as_mut().remove(key_ref));
    expect_false!(map.contains_key(key_ref));
}

#[gtest]
fn nonunpin_value_tests() {
    let mut map = emplace!(NonunpinValueMap::ctor_new(()));
    map.as_mut().try_insert(1, Nonunpin::ctor_new(2)).unwrap();

    expect_true!(map.contains_key(&1));
    expect_that!(map.get(&1).map(|v| v.value()), eq(Some(2)));
    expect_that!(map.get_key_value(&1).map(|(k, v)| (*k, v.value())), eq(Some((1, 2))));

    {
        let mut val_mut: Pin<&mut Nonunpin> = NonunpinValueMap::get_mut(map.as_mut(), &1).unwrap();
        val_mut.as_mut().set_value(3);
    }
    expect_that!(map.get(&1).map(|v| v.value()), eq(Some(3)));

    {
        let (k, v): (&i32, Pin<&mut Nonunpin>) = map.as_mut().insert(1, Nonunpin::ctor_new(4));
        expect_eq!(*k, 1);
        expect_eq!(v.value(), 4);
    }
    expect_that!(map.get(&1).map(|v| v.value()), eq(Some(4)));

    expect_true!(map.as_mut().remove(&1));
    expect_false!(map.contains_key(&1));
}

#[gtest]
fn nonunpin_both_tests() {
    let mut map = emplace!(NonunpinBothMap::ctor_new(()));
    map.as_mut().try_insert(Nonunpin::ctor_new(1), Nonunpin::ctor_new(2)).unwrap();

    let key_pin = emplace!(Nonunpin::ctor_new(1));
    let key_ref = &*key_pin;

    expect_true!(map.contains_key(&*key_ref));
    expect_that!(map.get(&*key_ref).map(|v| v.value()), eq(Some(2)));
    expect_that!(
        map.get_key_value(&*key_ref).map(|(k, v)| (k.value(), v.value())),
        eq(Some((1, 2)))
    );

    {
        let mut val_mut = NonunpinBothMap::get_mut(map.as_mut(), &*key_ref).unwrap();
        val_mut.as_mut().set_value(3);
    }
    expect_that!(map.get(&*key_ref).map(|v| v.value()), eq(Some(3)));

    map.as_mut().insert(Nonunpin::ctor_new(1), Nonunpin::ctor_new(4));
    expect_that!(map.get(&*key_ref).map(|v| v.value()), eq(Some(4)));

    expect_true!(map.as_mut().remove(&*key_ref));
    expect_false!(map.contains_key(&*key_ref));
}

#[gtest]
fn moveonly_tests() {
    let mut map = emplace!(MoveOnlyMap::ctor_new(()));
    let mut k1 = emplace!(MoveOnly::ctor_new(1));
    let mut v1 = emplace!(MoveOnly::ctor_new(2));
    map.as_mut().try_insert(mov!(k1), mov!(v1)).unwrap();

    let k_lookup = emplace!(MoveOnly::ctor_new(1));

    expect_true!(map.contains_key(&*k_lookup));
    expect_that!(map.get(&*k_lookup).map(|v| v.value()), eq(Some(2)));
    expect_that!(
        map.get_key_value(&*k_lookup).map(|(k, v)| (k.value(), v.value())),
        eq(Some((1, 2)))
    );

    {
        let mut val_mut = MoveOnlyMap::get_mut(map.as_mut(), &*k_lookup).unwrap();
        val_mut.as_mut().set_value(3);
    }
    expect_that!(map.get(&*k_lookup).map(|v| v.value()), eq(Some(3)));

    let mut k2 = emplace!(MoveOnly::ctor_new(1));
    let mut v2 = emplace!(MoveOnly::ctor_new(4));
    map.as_mut().insert(mov!(k2), mov!(v2));
    expect_that!(map.get(&*k_lookup).map(|v| v.value()), eq(Some(4)));

    expect_true!(map.as_mut().remove(&*k_lookup));
    expect_false!(map.contains_key(&*k_lookup));
}

fn test_map_trait_generic<
    M: for<'a> map::Map<Key = i32, Value = u64, ValueMut<'a> = &'a mut u64>,
>(
    mut map: Pin<&mut M>,
) {
    map.as_mut().try_insert(1, 2).unwrap();
    assert_eq!(map.len(), 1);
    let val = map::Map::get_mut(map.as_mut(), &1).unwrap();
    *val = 3;
    assert_eq!(map.get(&1), Some(&3));
}

#[gtest]
fn ints_map_trait_generic() {
    let mut map = emplace!(IntsMap::ctor_new(()));
    test_map_trait_generic(map.as_mut());
}
