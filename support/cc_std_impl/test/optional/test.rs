// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests for constructing and destroying `cc_std::std::optional` payloads which cannot be moved
//! or produced by value, such as `cc_std::std::string`.

use core::cell::Cell;
use core::marker::{PhantomPinned, Unpin};
use cpp_std::{optional, string};
use ctor::{emplace, Ctor, CtorNew, FnCtor, Infallible};
use googletest::{expect_eq, expect_true, gtest};
use static_assertions::assert_not_impl_any;

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
