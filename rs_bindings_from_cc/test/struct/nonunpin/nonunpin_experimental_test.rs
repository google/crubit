// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(negative_impls)]

use ctor::{
    ctor, emplace, mov, Assign, ConstRvalueReference, Ctor, CtorNew, Emplace, Infallible,
    RvalueReference,
};
use googletest::prelude::*;
use nonunpin_experimental::Nonunpin;
use std::pin::Pin;

#[gtest]
fn test_move_construct() {
    let mut x = emplace!(Nonunpin::ctor_new(42));
    let y = emplace!(mov!(x.as_mut()));

    assert_eq!(x.value(), 0); // moved-from
    assert_eq!(y.value(), 42); // moved-to

    assert_eq!(x.addr(), &*x as *const _ as usize);
    assert_eq!(y.addr(), &*y as *const _ as usize);
}

#[gtest]
fn test_move_assign() {
    let mut x = emplace!(Nonunpin::ctor_new(42));
    let mut y = emplace!(Nonunpin::ctor_new(8));

    y.as_mut().assign(ctor::mov!(x.as_mut()));

    assert_eq!(x.value(), 0); // moved-from
    assert_eq!(y.value(), 42); // moved-to

    assert_eq!(x.addr(), &*x as *const _ as usize);
    assert_eq!(y.addr(), &*y as *const _ as usize);
}

#[gtest]
fn test_copy_construct() {
    let x = emplace!(Nonunpin::ctor_new(42));
    let y = emplace!(ctor::copy(&*x));

    assert_eq!(x.value(), 42);
    assert_eq!(y.value(), 42);

    assert_eq!(x.addr(), &*x as *const _ as usize);
    assert_eq!(y.addr(), &*y as *const _ as usize);
}

#[gtest]
fn test_copy_assign() {
    let x = emplace!(Nonunpin::ctor_new(42));
    let mut y = emplace!(Nonunpin::ctor_new(8));
    y.as_mut().assign(&*x);

    assert_eq!(x.value(), 42);
    assert_eq!(y.value(), 42);

    assert_eq!(x.addr(), &*x as *const _ as usize);
    assert_eq!(y.addr(), &*y as *const _ as usize);
}

/// Test that the struct can be returned and passed as all the reference
/// types, and passed by value.
#[gtest]
fn test_ref() {
    let mut x = emplace!(Nonunpin::ctor_new(42));
    {
        let x_ref: Pin<&mut Nonunpin> = x.as_mut().AsMutRef();
        assert_eq!(nonunpin_experimental::GetValueFromMutRef(x_ref), 42);
        assert_eq!(nonunpin_experimental::GetValueFromMutRef(x.as_mut()), 42);
    }
    {
        let x_ref: &Nonunpin = x.AsConstRef();
        assert_eq!(nonunpin_experimental::GetValueFromConstRef(x_ref), 42);
        assert_eq!(nonunpin_experimental::GetValueFromConstRef(&x), 42);
    }
    {
        let x_ref: RvalueReference<Nonunpin> = x.as_mut().AsRvalueRef();
        assert_eq!(nonunpin_experimental::GetValueFromRvalueRef(x_ref), 42);
        assert_eq!(nonunpin_experimental::GetValueFromRvalueRef(ctor::mov!(x.as_mut())), 42);
    }
    {
        let x_ref: ConstRvalueReference<Nonunpin> = x.AsConstRvalueRef();
        assert_eq!(nonunpin_experimental::GetValueFromConstRvalueRef(x_ref), 42);
        assert_eq!(nonunpin_experimental::GetValueFromConstRvalueRef(ctor::const_mov!(&*x)), 42);
        assert_eq!(
            nonunpin_experimental::GetValueFromConstRvalueRef(ctor::const_mov!(x.as_mut())),
            42
        );
    }
    {
        assert_eq!(nonunpin_experimental::GetValueFromValue(ctor::copy(&*x)), 42);
        assert_eq!(nonunpin_experimental::GetValueFromValue(ctor::mov!(x)), 42);
    }
}

/// An example showing a C++ non-trivially-relocatable class as a field in a
/// Rust struct. There are two ways to do this:
///
///   1. storing C++ class indirectly (e.g., in a Box), or,
///   2. storing by-value.
///
/// This test specicially demonstrates the second: storing a C++ class by
/// value, even in the worst case of it not being trivially-relocatable.
/// In that case, the struct containing it must *also* become
/// non-trivially-relocatable, and it becomes ~exactly as difficult to deal
/// with as the C++ class it contains.
#[gtest]
fn test_struct_field() {
    #[ctor::recursively_pinned]
    struct MyStruct {
        field_1: u32,
        field_2: Nonunpin,
    }

    impl MyStruct {
        fn new() -> impl Ctor<Output = Self, Error = Infallible> {
            ctor!(MyStruct { field_1: 4, field_2: Nonunpin::ctor_new(2) })
        }
    }

    let my_struct = emplace!(MyStruct::new());
    assert_eq!(my_struct.field_1, 4);
    assert_eq!(my_struct.field_2.value(), 2);
    // use projection (from recursively_pinned/pin_project) to mutate the struct:
    let mut my_struct = my_struct.project_pin();
    *my_struct.field_1 = 5;
    my_struct.field_2.as_mut().assign(mov!(emplace!(Nonunpin::ctor_new(3))));
    assert_eq!(*my_struct.field_1, 5);
    assert_eq!(my_struct.field_2.value(), 3);
}

/// The example from the ctor.rs docs; copy-pasted.
#[gtest]
fn test_swap() {
    fn swap(mut x: Pin<&mut Nonunpin>, mut y: Pin<&mut Nonunpin>) {
        let mut tmp = emplace!(mov!(x.as_mut()));
        x.assign(mov!(y.as_mut()));
        y.assign(mov!(tmp));
    }
    let mut c1 = Box::emplace(Nonunpin::ctor_new(1));
    let mut c2 = Box::emplace(Nonunpin::ctor_new(2));
    swap(c1.as_mut(), c2.as_mut());
    assert_eq!(c1.value(), 2);
    assert_eq!(c2.value(), 1);
}
