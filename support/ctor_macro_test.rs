// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests for ctor macros, focusing on their use from external crates.
//!
//! To assist testing hygiene, this module must not add any imports (e.g. for
//! Pin, etc.), and must shadow std and ctor with "incorrect" modules to
//! ensure that the macros do not depend on that aspect of the caller.
#![cfg(test)]
// Allow unused imports so that we can produce pathological aliases.
#![allow(unused_imports)]

use googletest::gtest;

// pathological shadowed names: shadow important modules that the macros use.
mod std {}
mod ctor {}

/// Expand emplace!(expr) to test for hygiene.
#[gtest]
fn test_emplace_expr_hygiene() {
    let _ = ::ctor::emplace!(4);
}

/// Expand ctor!{Struct{...}} to test for hygiene.
#[gtest]
fn test_ctor_struct_hygiene() {
    struct Struct {
        x: i32,
    }
    unsafe impl ::ctor::RecursivelyPinned for Struct {
        type CtorInitializedFields = Self;
    }
    let _ = ::ctor::ctor! {Struct { x: 0 }};
}

/// Expand ctor!{TupleStruct(...)} to test for hygiene.
#[gtest]
fn test_ctor_tuple_struct_hygiene() {
    struct TupleStruct(i32);
    unsafe impl ::ctor::RecursivelyPinned for TupleStruct {
        type CtorInitializedFields = Self;
    }
    let _ = ::ctor::ctor! {TupleStruct(0)};
}

struct PinnedField {
    value: i32,
    self_addr: usize,
    _pin: ::core::marker::PhantomPinned,
}

impl PinnedField {
    fn new(value: i32) -> impl ::ctor::Ctor<Output = Self, Error = ::core::convert::Infallible> {
        // SAFETY: unconditionally initializes `dest` in place.
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                dest.write(Self {
                    value,
                    self_addr: dest as usize,
                    _pin: ::core::marker::PhantomPinned,
                });
            })
        }
    }

    fn assert_pinned_with(&self, expected: i32) {
        assert_eq!(self.value, expected);
        assert_eq!(::core::ptr::from_ref(self) as usize, self.self_addr);
    }
}

#[gtest]
fn test_raw_ctor_unit_struct() {
    struct UnitStruct;
    mod nested {
        pub struct NestedUnit;
    }

    let _ = ::ctor::emplace!(unsafe { ::ctor::raw_ctor!(UnitStruct) });
    let _ = ::ctor::emplace!(unsafe { ::ctor::raw_ctor!(UnitStruct {}) });
    let _ = ::ctor::emplace!(unsafe { ::ctor::raw_ctor!(nested::NestedUnit) });
}

#[gtest]
fn test_raw_ctor_tuple_structs() {
    struct Tuple0();
    struct Tuple1(PinnedField);
    struct Tuple2(PinnedField, PinnedField);
    struct Tuple3(PinnedField, PinnedField, PinnedField);
    struct Tuple7(i32, i32, i32, i32, i32, i32, i32);
    mod nested {
        pub struct NestedTuple(pub super::PinnedField, pub i32);
    }

    let _ = ::ctor::emplace!(unsafe { ::ctor::raw_ctor!(Tuple0()) });

    let t1 = ::ctor::emplace!(unsafe { ::ctor::raw_ctor!(Tuple1(PinnedField::new(10))) });
    t1.0.assert_pinned_with(10);

    let t2 = ::ctor::emplace!(unsafe {
        ::ctor::raw_ctor!(Tuple2(PinnedField::new(20), PinnedField::new(21)))
    });
    t2.0.assert_pinned_with(20);
    t2.1.assert_pinned_with(21);

    #[rustfmt::skip]
    let t3 = ::ctor::emplace!(unsafe {
        ::ctor::raw_ctor!(Tuple3(
            PinnedField::new(30),
            PinnedField::new(31),
            PinnedField::new(32),
        ))
    });
    t3.0.assert_pinned_with(30);
    t3.1.assert_pinned_with(31);
    t3.2.assert_pinned_with(32);

    let t7 = ::ctor::emplace!(unsafe { ::ctor::raw_ctor!(Tuple7(1, 2, 3, 4, 5, 6, 7)) });
    assert_eq!((t7.0, t7.1, t7.2, t7.3, t7.4, t7.5, t7.6), (1, 2, 3, 4, 5, 6, 7));

    let tn = ::ctor::emplace!(unsafe {
        ::ctor::raw_ctor!(nested::NestedTuple(PinnedField::new(40), 41))
    });
    tn.0.assert_pinned_with(40);
    assert_eq!(tn.1, 41);
}

#[gtest]
fn test_raw_ctor_generic_tuple_struct() {
    struct InnerPinned<T> {
        value: T,
        _pin: ::core::marker::PhantomPinned,
    }

    impl<T> InnerPinned<T> {
        fn some<C>(
            value: C,
        ) -> impl ::ctor::Ctor<Output = Self, Error = ::core::convert::Infallible>
        where
            C: ::ctor::Ctor<Output = T, Error = ::core::convert::Infallible>,
        {
            // SAFETY: `InnerPinned` treats `value` as structurally pinned.
            unsafe {
                ::ctor::raw_ctor!(InnerPinned::<T> {
                    value: value,
                    _pin: ::core::marker::PhantomPinned,
                })
            }
        }
    }

    struct PinnedTuple<T>(InnerPinned<T>);

    impl<T> PinnedTuple<T> {
        fn some<C>(
            value: C,
        ) -> impl ::ctor::Ctor<Output = Self, Error = ::core::convert::Infallible>
        where
            C: ::ctor::Ctor<Output = T, Error = ::core::convert::Infallible>,
        {
            // SAFETY: `PinnedTuple` treats field `0` as structurally pinned.
            unsafe { ::ctor::raw_ctor!(PinnedTuple::<T>(InnerPinned::some(value))) }
        }
    }

    let scalar = ::ctor::emplace!(PinnedTuple::<i32>::some(42));
    assert_eq!(scalar.0.value, 42);

    let pinned = ::ctor::emplace!(PinnedTuple::<PinnedField>::some(PinnedField::new(99)));
    pinned.0.value.assert_pinned_with(99);
}

#[gtest]
fn test_raw_ctor_fields_attr() {
    struct UnitWithZst {
        _zst: [(); 0],
    }
    struct UnitFields;

    struct PartialEmpty([(); 0]);
    struct PartialEmptyFields();

    struct PartialTuple(PinnedField, [(); 0]);
    struct PartialTupleFields(#[allow(dead_code)] PinnedField);

    struct PartialGeneric<T>(T, PinnedField, [(); 0]);
    struct PartialGenericFields<T>(#[allow(dead_code)] T, #[allow(dead_code)] PinnedField);

    let _ = ::ctor::emplace!(unsafe {
        ::ctor::raw_ctor!(
            #[fields = UnitFields]
            UnitWithZst
        )
    });

    let _ = ::ctor::emplace!(unsafe {
        ::ctor::raw_ctor!(
            #[fields = PartialEmptyFields]
            PartialEmpty()
        )
    });

    let pt = ::ctor::emplace!(unsafe {
        ::ctor::raw_ctor!(
            #[fields = PartialTupleFields]
            PartialTuple(PinnedField::new(55))
        )
    });
    pt.0.assert_pinned_with(55);

    let pg = ::ctor::emplace!(unsafe {
        ::ctor::raw_ctor!(
            #[fields = PartialGenericFields<i32>]
            PartialGeneric::<i32>(7, PinnedField::new(77))
        )
    });
    assert_eq!(pg.0, 7);
    pg.1.assert_pinned_with(77);
}
