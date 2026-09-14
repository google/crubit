// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::{optional, trivial_optional, vector};
use googletest::prelude::*;
use optional_lib::*;

/// Exercises the `const` API of `optional<T>`. Everything here is evaluated at compile time.
///
/// Note that the `optional` must be forgotten rather than dropped: `optional<T>` has a `Drop`
/// impl, and destructors cannot run at compile time.
const fn drain(mut o: optional<i32>) -> (bool, Option<i32>, bool) {
    let was_some = o.is_some();
    let taken = o.take();
    let now_none = o.is_none();
    core::mem::forget(o);
    (was_some, taken, now_none)
}

const DRAINED_NEW: (bool, Option<i32>, bool) = drain(optional::new(5));
const DRAINED_NONE: (bool, Option<i32>, bool) = drain(optional::<i32>::nullopt());

#[gtest]
fn test_const_api() {
    expect_eq!(DRAINED_NEW, (true, Some(5), true));
    expect_eq!(DRAINED_NONE, (false, None, true));
}

/// Exercises the `const` API of `trivial_optional<T>`, which has no destructor and so may simply
/// go out of scope.
const fn peek(mut o: trivial_optional<i32>) -> (bool, Option<i32>, bool) {
    let was_some = o.is_some();
    let taken = o.take();
    let now_none = o.is_none();
    (was_some, taken, now_none)
}

const PEEKED_NEW: (bool, Option<i32>, bool) = peek(trivial_optional::new(5));
const PEEKED_NONE: (bool, Option<i32>, bool) = peek(trivial_optional::<i32>::nullopt());

#[gtest]
fn test_trivial_const_api() {
    expect_eq!(PEEKED_NEW, (true, Some(5), true));
    expect_eq!(PEEKED_NONE, (false, None, true));
}

/// `std::optional<T>` is layout-compatible even when passed by value. `int` is `Copy`, so the
/// binding is `trivial_optional`.
#[gtest]
fn test_optional_by_value() {
    let o: trivial_optional<i32> = MakeOptional(1);
    expect_eq!(o.as_ref(), Some(&1));
    expect_eq!(UseOptionalByValue(o), 1);
    expect_eq!(UseOptionalByValue(trivial_optional::<i32>::nullopt()), -1);
}

/// `trivial_optional<T>` has the same layout as C++'s `std::optional<T>`.
#[gtest]
fn test_layout() {
    expect_eq!(size_of::<trivial_optional<i32>>(), 8);
    expect_eq!(align_of::<trivial_optional<i32>>(), 4);
    expect_false!(std::mem::needs_drop::<trivial_optional<i32>>());
}

/// `std::optional<T>` in a struct field is layout-compatible as well. Because `trivial_optional`
/// has no destructor, the struct stays `Copy`, just as it is trivially copyable in C++.
#[gtest]
fn test_optional_field() {
    static_assertions::assert_impl_all!(StructWithOptionalField: Copy);

    let s = MakeStructWithOptionalField(2);
    expect_eq!(s.optional_int.as_ref(), Some(&2));

    let copied = s;
    expect_eq!(copied.optional_int.as_ref(), Some(&2));
    // `s` is still usable, because it was copied rather than moved.
    expect_eq!(s.optional_int.as_ref(), Some(&2));
}

/// Structs which only transitively contain an `optional` stay `Copy` too.
#[gtest]
fn test_nested_optional_field() {
    static_assertions::assert_impl_all!(StructWithNestedOptionalField: Copy);

    let s = MakeStructWithNestedOptionalField(5);
    expect_eq!(s.inner.optional_int.as_ref(), Some(&5));
}

/// A non-`Copy` element type selects the `Drop`-carrying `optional<T>` instead.
#[gtest]
fn test_non_trivial_optional() {
    let o: optional<vector<i32>> = MakeOptionalVector(6);
    expect_eq!(o.as_ref().map(|v| v.len()), Some(1));
    expect_eq!(o.as_ref().map(|v| v[0]), Some(6));
    expect_true!(std::mem::needs_drop::<optional<vector<i32>>>());
}

#[gtest]
fn test_optional_by_ref() {
    let o = trivial_optional::<i32>::new(7);
    expect_eq!(unsafe { UseOptionalByRef(&o) }, 7);

    let none = trivial_optional::<i32>::nullopt();
    expect_eq!(unsafe { UseOptionalByRef(&none) }, -1);
}

/// As a template argument of another layout-compatible generic.
#[gtest]
fn test_vector_of_optional() {
    let v: vector<trivial_optional<i32>> = MakeVectorOfOptional(3);
    expect_eq!(v.len(), 2);
    expect_eq!(v[0].as_ref(), Some(&3));
    expect_eq!(v[1].as_ref(), None);
}

/// `trivial_optional<T>` is itself layout-compatible and `Copy`, so it nests.
#[gtest]
fn test_nested_optional() {
    let o: trivial_optional<trivial_optional<i32>> = MakeNestedOptional(4);
    expect_eq!(o.as_ref().and_then(|inner| inner.as_ref()), Some(&4));
}

/// The two types convert into each other whenever `T: Copy`.
#[gtest]
fn test_conversions_between_optional_types() {
    let t = trivial_optional::<i32>::new(9);
    let o: optional<i32> = t.into();
    expect_eq!(o.as_ref(), Some(&9));

    let back: trivial_optional<i32> = o.into();
    expect_eq!(back.as_ref(), Some(&9));

    let none: optional<i32> = trivial_optional::<i32>::nullopt().into();
    expect_true!(none.is_none());

    let none: trivial_optional<i32> = optional::<i32>::nullopt().into();
    expect_true!(none.is_none());
}
