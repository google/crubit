// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::{optional, shared_ptr, trivial_optional, vector};
use ctor::{emplace, CtorNew};
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

/// A payload which is not Rust-movable makes the `optional` itself `!Unpin`, because the payload
/// is stored inline. Such an `optional` is produced by a `Ctor`, like any other non-Rust-movable
/// value.
#[gtest]
fn test_non_movable_payload() {
    static_assertions::assert_not_impl_any!(optional<NonMovable>: Unpin);

    // `emplace!` yields a `Pin<&mut optional<NonMovable>>`, so rebind as a plain reference before
    // calling `optional`'s own methods: `Pin` has an inherent `as_ref` of its own.
    let o = emplace!(MakeOptionalNonMovable(7));
    let o: &optional<NonMovable> = &o;
    expect_eq!(o.as_ref().map(|v| v.value), Some(7));
    // SAFETY: `optional_lib.h` does not enable lifetime elision, so its reference parameters bind
    // as raw pointers and its functions are `unsafe`. `o` is live for the duration of the call.
    expect_eq!(unsafe { UseOptionalNonMovable(o) }, 7);
}

/// The payload can also be constructed directly into the `optional`, without going through C++.
#[gtest]
fn test_non_movable_payload_in_place() {
    // `NonMovable::ctor_new(8)` is a `Ctor` for the payload, not a `NonMovable`: it runs only once
    // `emplace!` has an address to construct at, which is the payload slot inside the `optional`.
    let o = emplace!(optional::ctor_some(NonMovable::ctor_new(8)));
    let o: &optional<NonMovable> = &o;
    expect_eq!(o.as_ref().map(|v| v.value), Some(8));

    // `ctor_nullopt` requires nothing of `NonMovable`, since an empty `optional` has no payload.
    let none = emplace!(optional::<NonMovable>::ctor_nullopt());
    let none: &optional<NonMovable> = &none;
    expect_true!(none.is_none());
    // SAFETY: as above -- a raw pointer parameter, and `none` is live for the call.
    expect_eq!(unsafe { UseOptionalNonMovable(none) }, -1);
}

/// The motivating case: a struct field holding a `std::optional<T>` for a non-Rust-movable `T`.
#[gtest]
fn test_non_movable_optional_field() {
    // `s` is a `Pin<&mut StructWithNonMovableOptionalField>`; the field read goes through `Pin`'s
    // `Deref`, which is all that is needed to inspect the `optional` behind a shared reference.
    let s = emplace!(MakeStructWithNonMovableOptionalField(9));
    expect_eq!(s.optional_non_movable.as_ref().map(|v| v.value), Some(9));
}

/// `std::string` is layout-compatible but not Rust-movable, so `std::optional<std::string>` is
/// the same case, and is the field which motivated this feature (b/493262489).
#[gtest]
fn test_optional_string() {
    // `MakeOptionalString` returns a `Ctor`, because `optional<string>` is not Rust-movable; the
    // C++ return value is constructed directly into the storage `emplace!` provides.
    let o = emplace!(MakeOptionalString(true));
    let o: &optional<cc_std::std::string> = &o;
    let s = o.as_ref().expect("should be engaged");
    expect_eq!(s.as_slice(), b"hello");

    let none = emplace!(MakeOptionalString(false));
    let none: &optional<cc_std::std::string> = &none;
    expect_true!(none.is_none());
}

/// A payload constructed in place is destroyed in place, exactly once: while the `optional` is
/// engaged, the payload is a second owner of `owner`, and dropping the `optional` releases that
/// share. An empty `optional` never had a payload, so it owns nothing.
#[gtest]
fn test_non_movable_payload_is_destroyed_once() {
    // Without this, the test would still pass for a Rust-movable payload, but would no longer be
    // exercising in-place construction at all.
    static_assertions::assert_not_impl_any!(optional<SharedPayload>: Unpin);

    // The counter is a `shared_ptr<i32>`, which is itself Rust-movable -- unlike the
    // `SharedPayload` that will share ownership of it.
    let owner: shared_ptr<i32> = shared_ptr::new(0);
    expect_eq!(shared_ptr::use_count(&owner), 1);

    {
        // An empty `optional<SharedPayload>`, which never constructs a payload at all.
        let _empty = emplace!(optional::<SharedPayload>::ctor_nullopt());
        expect_eq!(shared_ptr::use_count(&owner), 1);
    }
    expect_eq!(shared_ptr::use_count(&owner), 1);

    {
        // The cloned `shared_ptr` is moved into `SharedPayload`'s constructor, which `emplace!`
        // runs directly inside the `optional`: the payload is never moved.
        let _engaged = emplace!(optional::ctor_some(SharedPayload::ctor_new(owner.clone())));
        expect_eq!(shared_ptr::use_count(&owner), 2);
    }
    expect_eq!(shared_ptr::use_count(&owner), 1);
}
