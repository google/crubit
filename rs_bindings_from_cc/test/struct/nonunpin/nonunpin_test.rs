// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(negative_impls)]

use ctor::{ctor, emplace, CtorNew, ReconstructUnchecked};
use googletest::prelude::*;
use nonunpin::{Nonmovable, Nonunpin, NonunpinStruct, ReturnsNonmovable};
use std::pin::Pin;

/// When a value is constructed in-place, it is initialized, has the correct
/// address.
#[gtest]
fn test_onearg_ctor() {
    let mut x = emplace!(Nonunpin::ctor_new(42));
    assert_eq!(x.value(), 42);
    assert_eq!(x.addr(), &*x as *const _ as usize);
}

#[gtest]
fn test_default_ctor() {
    let mut x = emplace!(Nonunpin::ctor_new(()));
    assert_eq!(x.value(), 0);
    assert_eq!(x.addr(), &*x as *const _ as usize);
}

#[gtest]
fn test_methods() {
    let mut x = emplace!(Nonunpin::ctor_new(42));
    x.as_mut().set_value(24);
    assert_eq!(x.value(), 24);
}

#[gtest]
fn test_aggregate() {
    let mut x = emplace!(ctor!(NonunpinStruct { value: 42 }));
    assert_eq!(x.value, 42);
    {
        // Read/write via a pin-projection.
        let mut x = x.as_mut().project_pin();
        assert_eq!(*x.value, 42);
        *x.value = 0;
        assert_eq!(*x.value, 0);
    }
    assert_eq!(x.value, 0);
}

#[gtest]
fn test_return_by_value() {
    let x = emplace!(Nonunpin::ctor_new(42));
    let y = emplace!(x.AsValue());

    assert_eq!(x.value(), 42);
    assert_eq!(y.value(), 42);

    assert_eq!(x.addr(), &*x as *const _ as usize);
    assert_eq!(y.addr(), &*y as *const _ as usize);
}

#[gtest]
fn test_nonmovable_ctor() {
    let x = emplace!(Nonmovable::ctor_new(()));
    assert_eq!(x.addr, &*x as *const _ as usize);
}

/// Thanks to C++17 prvalue semantics, we can in fact return a non-movable
/// type by value.
#[gtest]
fn test_nonmovable_return_value() {
    let x = emplace!(ReturnsNonmovable());
    assert_eq!(x.addr, &*x as *const _ as usize);
}

/// An example showing a C++ non-trivially-relocatable class as a field in a
/// Rust union. This mirrors the struct case, storing by value.
///
/// It is also quite ugly, but, fortunately, these unions are not common.
#[gtest]
fn test_union_field() {
    union MyUnion {
        int: u32,
        cxx_class: ::std::mem::ManuallyDrop<Nonunpin>,
    }
    unsafe impl ctor::RecursivelyPinned for MyUnion {
        type CtorInitializedFields = Self;
    }

    // No safe helpers here. :)
    unsafe {
        let mut my_union = emplace!(ctor!(MyUnion {
            cxx_class: ctor::ManuallyDropCtor::new(Nonunpin::ctor_new(4))
        }));
        assert_eq!(my_union.cxx_class.value(), 4);
        std::mem::ManuallyDrop::drop(&mut Pin::into_inner_unchecked(my_union.as_mut()).cxx_class);
        my_union.as_mut().reconstruct_unchecked(ctor!(MyUnion { int: 2 }));
        assert_eq!(my_union.int, 2);
    }
}
