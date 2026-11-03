// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests for `cc_std::std::optional` and `cc_std::std::trivial_optional`.

use core::cell::Cell;
use core::marker::{PhantomPinned, Unpin};
use cpp_std::{optional, string, trivial_optional};
use ctor::{emplace, Ctor, CtorNew, FnCtor, Infallible};
use googletest::{expect_eq, expect_true, gtest};
use static_assertions::assert_not_impl_any;
use std::sync::Arc;

// An `optional` holding a `!Unpin` payload must itself be `!Unpin`, otherwise safe code could
// move it and invalidate the payload's self-references.
assert_not_impl_any!(optional<string>: Unpin);

/// A `!Unpin` payload which counts its own destruction.
struct Counted<'a> {
    counter: &'a Cell<u32>,
    _pinned: PhantomPinned,
}

impl Drop for Counted<'_> {
    fn drop(&mut self) {
        self.counter.set(self.counter.get() + 1);
    }
}

/// Returns a `Ctor` which constructs a [`Counted`] reporting to `counter`.
fn counted(counter: &Cell<u32>) -> impl Ctor<Output = Counted<'_>, Error = Infallible> + '_ {
    // SAFETY: `f` initializes its argument to a valid `Counted`.
    unsafe {
        FnCtor::new(move |dest: *mut Counted<'_>| {
            dest.write(Counted { counter, _pinned: PhantomPinned });
        })
    }
}

#[gtest]
fn test_ctor_nullopt_is_empty() {
    let opt = emplace!(optional::<string>::ctor_nullopt());
    expect_true!(opt.is_none());
    expect_true!((*opt).as_ref().is_none());
}

#[gtest]
fn test_ctor_some_holds_string() {
    let opt = emplace!(optional::ctor_some(string::ctor_new("hello")));
    expect_true!(opt.is_some());
    expect_eq!((*opt).as_ref().unwrap().as_slice(), &b"hello"[..]);
}

#[gtest]
fn test_ctor_from_option_some_holds_string() {
    let opt = emplace!(optional::ctor_from_option(Some(string::ctor_new("hello"))));
    expect_eq!((*opt).as_ref().unwrap().as_slice(), &b"hello"[..]);
}

#[gtest]
fn test_ctor_from_option_none_is_empty() {
    let value: Option<&str> = None;
    let opt = emplace!(optional::ctor_from_option(value.map(string::ctor_new)));
    expect_true!(opt.is_none());
}

#[gtest]
fn test_ctor_from_option_none_destroys_nothing() {
    let counter = Cell::new(0);
    {
        let opt = emplace!(optional::ctor_from_option(None.map(|()| counted(&counter))));
        expect_true!(opt.is_none());
    }
    expect_eq!(counter.get(), 0);
}

#[gtest]
fn test_ctor_new_from_some_holds_string() {
    let value: Option<&str> = Some("hello");
    let opt = emplace!(optional::<string>::ctor_new(value));
    expect_eq!((*opt).as_ref().unwrap().as_slice(), &b"hello"[..]);
}

#[gtest]
fn test_ctor_new_from_none_is_empty() {
    let value: Option<&str> = None;
    let opt = emplace!(optional::<string>::ctor_new(value));
    expect_true!(opt.is_none());
}

#[gtest]
fn test_ctor_new_from_option_of_string_ref_copies() {
    let source = emplace!(string::ctor_new("hello"));
    let opt = emplace!(optional::<string>::ctor_new(Some(&*source)));
    expect_eq!((*opt).as_ref().unwrap().as_slice(), &b"hello"[..]);
}

#[gtest]
fn test_emplace_into_empty_optional() {
    let mut opt = emplace!(optional::<string>::ctor_nullopt());
    expect_true!(opt.as_mut().emplace(string::ctor_new("first")).is_ok());
    expect_eq!((*opt).as_ref().unwrap().as_slice(), &b"first"[..]);
}

#[gtest]
fn test_emplace_replaces_existing_value() {
    let mut opt = emplace!(optional::ctor_some(string::ctor_new("first")));
    expect_true!(opt.as_mut().emplace(string::ctor_new("second")).is_ok());
    expect_eq!((*opt).as_ref().unwrap().as_slice(), &b"second"[..]);
}

#[gtest]
fn test_as_pin_mut_projects_to_pinned_payload() {
    let mut opt = emplace!(optional::ctor_some(string::ctor_new("hello")));
    let payload = opt.as_mut().as_pin_mut().unwrap();
    expect_eq!(payload.as_slice(), &b"hello"[..]);
}

#[gtest]
fn test_reset_destroys_payload() {
    let counter = Cell::new(0);
    let mut opt = emplace!(optional::ctor_some(counted(&counter)));
    expect_true!(opt.is_some());
    expect_eq!(counter.get(), 0);

    opt.as_mut().reset();
    expect_true!(opt.is_none());
    expect_eq!(counter.get(), 1);

    // Resetting an already-empty `optional` must not destroy the payload a second time.
    opt.as_mut().reset();
    expect_eq!(counter.get(), 1);
}

#[gtest]
fn test_emplace_destroys_previous_payload() {
    let counter = Cell::new(0);
    let mut opt = emplace!(optional::ctor_some(counted(&counter)));
    expect_true!(opt.as_mut().emplace(counted(&counter)).is_ok());

    // The first payload was destroyed; the second is still alive.
    expect_eq!(counter.get(), 1);

    opt.as_mut().reset();
    expect_eq!(counter.get(), 2);
}

#[gtest]
fn test_drop_destroys_payload() {
    let counter = Cell::new(0);
    {
        let opt = emplace!(optional::ctor_some(counted(&counter)));
        expect_true!(opt.is_some());
        expect_eq!(counter.get(), 0);
    }
    expect_eq!(counter.get(), 1);
}

#[gtest]
fn test_drop_of_empty_optional_destroys_nothing() {
    let counter = Cell::new(0);
    {
        let mut opt = emplace!(optional::ctor_some(counted(&counter)));
        opt.as_mut().reset();
        expect_eq!(counter.get(), 1);
    }
    expect_eq!(counter.get(), 1);
}

#[gtest]
fn test_trivial_optional_into_option() {
    const SOME: Option<i32> = trivial_optional::new(42).into_option();
    const NONE: Option<i32> = trivial_optional::<i32>::nullopt().into_option();
    expect_eq!(SOME, Some(42));
    expect_eq!(NONE, None);

    let opt = trivial_optional::new(7);
    let matched = match opt.into_option() {
        Some(v) => v,
        None => -1,
    };
    expect_eq!(matched, 7);
    // `trivial_optional<i32>` is `Copy`, so `opt` remains usable after `into_option()`.
    expect_eq!(opt.as_ref(), Some(&7));
}

#[gtest]
fn test_optional_into_option_moves_payload_once() {
    const SOME: Option<i32> = optional::new(42).into_option();
    const NONE: Option<i32> = optional::<i32>::nullopt().into_option();
    expect_eq!(SOME, Some(42));
    expect_eq!(NONE, None);

    let owner = Arc::new(99);
    expect_eq!(Arc::strong_count(&owner), 1);

    {
        let opt = optional::new(Arc::clone(&owner));
        expect_eq!(Arc::strong_count(&owner), 2);

        let converted = opt.into_option();
        expect_eq!(Arc::strong_count(&owner), 2);
        expect_eq!(converted.as_deref(), Some(&99));
    }
    expect_eq!(Arc::strong_count(&owner), 1);

    expect_eq!(optional::<Arc<i32>>::nullopt().into_option(), None);
}
