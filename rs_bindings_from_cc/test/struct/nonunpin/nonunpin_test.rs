// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(negative_impls)]

use ctor::{ctor, emplace, mov, ConstRvalueReference, Ctor, Emplace, Infallible, RvalueReference};
use ctor::{Assign as _, CtorNew as _, ReconstructUnchecked as _};
use googletest::prelude::*;
use nonunpin::{Nonmovable, Nonunpin, NonunpinStruct, ReturnsNonmovable};
use std::pin::Pin;

/// When a value is constructed in-place, it is initialized, has the correct
/// address.
#[gtest]
fn test_onearg_ctor() {
    ctor::emplace! {
        let mut x = Nonunpin::ctor_new(42);
    }
    assert_eq!(x.value(), 42);
    assert_eq!(x.addr(), &*x as *const _ as usize);
}

#[gtest]
fn test_default_ctor() {
    ctor::emplace! {
        let mut x = Nonunpin::ctor_new(());
    }
    assert_eq!(x.value(), 0);
    assert_eq!(x.addr(), &*x as *const _ as usize);
}

#[gtest]
fn test_move_construct() {
    ctor::emplace! {
        let mut x = Nonunpin::ctor_new(42);
        let mut y = ctor::mov!(x.as_mut());
    }

    assert_eq!(x.value(), 0); // moved-from
    assert_eq!(y.value(), 42); // moved-to

    assert_eq!(x.addr(), &*x as *const _ as usize);
    assert_eq!(y.addr(), &*y as *const _ as usize);
}

#[gtest]
fn test_move_assign() {
    ctor::emplace! {
        let mut x = Nonunpin::ctor_new(42);
        let mut y = Nonunpin::ctor_new(8);
    }

    y.as_mut().assign(ctor::mov!(x.as_mut()));

    assert_eq!(x.value(), 0); // moved-from
    assert_eq!(y.value(), 42); // moved-to

    assert_eq!(x.addr(), &*x as *const _ as usize);
    assert_eq!(y.addr(), &*y as *const _ as usize);
}

#[gtest]
fn test_copy_construct() {
    ctor::emplace! {
        let x = Nonunpin::ctor_new(42);
        let y = ctor::copy(&*x);
    }

    assert_eq!(x.value(), 42);
    assert_eq!(y.value(), 42);

    assert_eq!(x.addr(), &*x as *const _ as usize);
    assert_eq!(y.addr(), &*y as *const _ as usize);
}

#[gtest]
fn test_copy_assign() {
    ctor::emplace! {
        let x = Nonunpin::ctor_new(42);
        let mut y = Nonunpin::ctor_new(8);
    }
    y.as_mut().assign(&*x);

    assert_eq!(x.value(), 42);
    assert_eq!(y.value(), 42);

    assert_eq!(x.addr(), &*x as *const _ as usize);
    assert_eq!(y.addr(), &*y as *const _ as usize);
}

#[gtest]
fn test_methods() {
    ctor::emplace! {
        let mut x = Nonunpin::ctor_new(42);
    }
    x.as_mut().set_value(24);
    assert_eq!(x.value(), 24);
}

/// Test that the struct can be returned and passed as all the reference
/// types, and passed by value.
#[gtest]
fn test_ref() {
    ctor::emplace! {
        let mut x = Nonunpin::ctor_new(42);
    }
    {
        let x_ref: Pin<&mut Nonunpin> = x.as_mut().AsMutRef();
        assert_eq!(nonunpin::GetValueFromMutRef(x_ref), 42);
        assert_eq!(nonunpin::GetValueFromMutRef(x.as_mut()), 42);
    }
    {
        let x_ref: &Nonunpin = x.AsConstRef();
        assert_eq!(nonunpin::GetValueFromConstRef(x_ref), 42);
        assert_eq!(nonunpin::GetValueFromConstRef(&x), 42);
    }
    {
        let x_ref: RvalueReference<Nonunpin> = x.as_mut().AsRvalueRef();
        assert_eq!(nonunpin::GetValueFromRvalueRef(x_ref), 42);
        assert_eq!(nonunpin::GetValueFromRvalueRef(ctor::mov!(x.as_mut())), 42);
    }
    {
        let x_ref: ConstRvalueReference<Nonunpin> = x.AsConstRvalueRef();
        assert_eq!(nonunpin::GetValueFromConstRvalueRef(x_ref), 42);
        assert_eq!(nonunpin::GetValueFromConstRvalueRef(ctor::const_mov!(&*x)), 42);
        assert_eq!(nonunpin::GetValueFromConstRvalueRef(ctor::const_mov!(x.as_mut())), 42);
    }
    {
        assert_eq!(nonunpin::GetValueFromValue(ctor::copy(&*x)), 42);
        assert_eq!(nonunpin::GetValueFromValue(ctor::mov!(x)), 42);
    }
}

#[gtest]
fn test_aggregate() {
    ctor::emplace! {
        let mut x = ctor!(NonunpinStruct {value: 42});
    }
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
    ctor::emplace! {
        let x = Nonunpin::ctor_new(42);
        let y = x.AsValue();
    }

    assert_eq!(x.value(), 42);
    assert_eq!(y.value(), 42);

    assert_eq!(x.addr(), &*x as *const _ as usize);
    assert_eq!(y.addr(), &*y as *const _ as usize);
}

#[gtest]
fn test_nonmovable_ctor() {
    ctor::emplace! {
        let x = Nonmovable::ctor_new(());
    }
    assert_eq!(x.addr, &*x as *const _ as usize);
}

/// Thanks to C++17 prvalue semantics, we can in fact return a non-movable
/// type by value.
#[gtest]
fn test_nonmovable_return_value() {
    ctor::emplace! {
        let x = ReturnsNonmovable();
    }
    assert_eq!(x.addr, &*x as *const _ as usize);
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

    emplace! { let mut my_struct = MyStruct::new(); }
    assert_eq!(my_struct.field_1, 4);
    assert_eq!(my_struct.field_2.value(), 2);
    // use projection (from recursively_pinned/pin_project) to mutate the struct:
    let mut my_struct = my_struct.project_pin();
    *my_struct.field_1 = 5;
    my_struct.field_2.as_mut().assign(mov!(emplace!(Nonunpin::ctor_new(3))));
    assert_eq!(*my_struct.field_1, 5);
    assert_eq!(my_struct.field_2.value(), 3);
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
        emplace! {
            let mut my_union = ctor!(MyUnion {
                cxx_class: ctor::ManuallyDropCtor::new(Nonunpin::ctor_new(4))
            });
        }
        assert_eq!(my_union.cxx_class.value(), 4);
        std::mem::ManuallyDrop::drop(&mut Pin::into_inner_unchecked(my_union.as_mut()).cxx_class);
        my_union.as_mut().reconstruct_unchecked(ctor!(MyUnion { int: 2 }));
        assert_eq!(my_union.int, 2);
    }
}

/// The example from the ctor.rs docs; copy-pasted.
#[gtest]
fn test_swap() {
    fn swap(mut x: Pin<&mut Nonunpin>, mut y: Pin<&mut Nonunpin>) {
        emplace! { let mut tmp = mov!(x.as_mut()); }
        x.assign(mov!(y.as_mut()));
        y.assign(mov!(tmp));
    }
    let mut c1 = Box::emplace(Nonunpin::ctor_new(1));
    let mut c2 = Box::emplace(Nonunpin::ctor_new(2));
    swap(c1.as_mut(), c2.as_mut());
    assert_eq!(c1.value(), 2);
    assert_eq!(c2.value(), 1);
}
