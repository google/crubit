// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests for constructing `cc_std::std::optional` in place via the `ctor` framework.
//!
//! The payloads here are all Rust types, so these tests exercise `optional`'s own construction
//! and destruction logic without involving C++.

// `#[recursively_pinned]` expands to a negative `Unpin` impl.
#![feature(negative_impls)]

use cc_std::std::optional;
use ctor::{ctor, emplace, recursively_pinned, Ctor, CtorNew};
use googletest::{expect_eq, expect_true, gtest};
use static_assertions::assert_not_impl_any;
use std::cell::Cell;
use std::pin::Pin;
use std::rc::Rc;

/// A payload which cannot be moved once constructed, and so can only be placed into an `optional`
/// by being constructed directly into the payload slot.
///
/// Dropping a `Pinned` drops its `counter`, so `Rc::strong_count` observes whether `optional`'s
/// destructor ran.
#[recursively_pinned]
struct Pinned {
    value: i32,
    counter: Rc<()>,
}

// `optional<T>` stores the payload inline, so it must inherit `!Unpin` from the payload. Without
// this, the tests below would silently degrade into testing the `Unpin` path.
assert_not_impl_any!(optional<Pinned>: Unpin);

#[gtest]
fn test_ctor_new_engaged() {
    let o: Pin<&mut optional<i32>> = emplace!(optional::ctor_new((42i32,)));
    // Rebind as a plain reference: `Pin` has an inherent `as_ref` which would otherwise shadow
    // `optional::as_ref`.
    let o: &optional<i32> = &o;
    expect_true!(o.is_some());
    expect_eq!(o.as_ref(), Some(&42));
}

#[gtest]
fn test_ctor_new_nullopt() {
    // `CtorNew<()>` comes from `ctor`'s blanket impl for `Default` types.
    let o: Pin<&mut optional<i32>> = emplace!(<optional<i32> as CtorNew<()>>::ctor_new(()));
    let o: &optional<i32> = &o;
    expect_true!(o.is_none());
}

#[gtest]
fn test_ctor_new_engaged_with_pinned_payload() {
    let counter: Rc<()> = Rc::new(());
    let o: Pin<&mut optional<Pinned>> =
        emplace!(optional::ctor_new((ctor!(Pinned { value: 42, counter: counter.clone() }),)));
    let o: &optional<Pinned> = &o;
    expect_true!(o.is_some());
    expect_eq!(o.as_ref().unwrap().value, 42);
}

#[gtest]
fn test_ctor_new_constructs_payload_in_place() {
    let counter: Rc<()> = Rc::new(());
    let construction_address: Rc<Cell<usize>> = Rc::new(Cell::new(0));

    let recorder: Rc<Cell<usize>> = construction_address.clone();
    let payload = ctor!(Pinned { value: 0, counter: counter.clone() }).ctor_then(
        move |payload: Pin<&mut Pinned>| {
            recorder.set(&*payload as *const Pinned as usize);
            Ok(())
        },
    );

    let o: Pin<&mut optional<Pinned>> = emplace!(optional::ctor_new((payload,)));
    let o: &optional<Pinned> = &o;
    // The payload is still at the address it was constructed at, i.e. `optional` constructed it
    // directly into the payload slot rather than constructing it elsewhere and moving it in.
    expect_eq!(o.as_ref().unwrap() as *const Pinned as usize, construction_address.get());
}

#[gtest]
fn test_drop_runs_payload_destructor() {
    let counter: Rc<()> = Rc::new(());
    {
        let o: Pin<&mut optional<Pinned>> =
            emplace!(optional::ctor_new((ctor!(Pinned { value: 0, counter: counter.clone() }),)));
        let o: &optional<Pinned> = &o;
        expect_true!(o.is_some());
        expect_eq!(Rc::strong_count(&counter), 2);
    }
    expect_eq!(Rc::strong_count(&counter), 1);
}

#[gtest]
fn test_drop_of_nullopt_does_not_run_payload_destructor() {
    // A disengaged `optional` must not drop its uninitialized payload. If it did, this would drop
    // an uninitialized `Rc` and be caught by the sanitizers.
    let o: Pin<&mut optional<Pinned>> = emplace!(<optional<Pinned> as CtorNew<()>>::ctor_new(()));
    let o: &optional<Pinned> = &o;
    expect_true!(o.is_none());
}

#[gtest]
fn test_take_after_ctor_new() {
    let mut o: Pin<&mut optional<i32>> = emplace!(optional::ctor_new((42i32,)));
    // `optional<i32>` is `Unpin`, so the payload can still be moved back out.
    let o: &mut optional<i32> = &mut o;
    expect_eq!(o.take(), Some(42));
    expect_true!(o.is_none());
}
