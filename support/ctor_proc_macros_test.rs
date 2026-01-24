// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Test crate for `ctor_proc_macros`.
//!
//! Because the `ctor` crate is not visible as `::ctor` within
//! itself, we use a separate crate for testing, which can depend on both.
//!
//! And because the proc macros have additional expectations on callers, this is
//! not added to macro_test.

#![cfg(test)]
// Callers are expected to enable `negative_impls`.
// more_qualified_paths is used to make project_pin_type!() simpler to use.
#![feature(negative_impls)]

use googletest::prelude::*;

// pathological shadowed names: shadow important modules that the macros use.
mod std {}
mod ctor {}
mod pin_project {}

#[gtest]
fn test_derive_default_unit_struct() {
    #[derive(::ctor::CtorFrom_Default)]
    struct Struct;
    unsafe impl ::ctor::RecursivelyPinned for Struct {
        type CtorInitializedFields = Self;
    }
    impl !Unpin for Struct {}

    let _p = ::ctor::emplace!(<Struct as ::ctor::CtorNew<()>>::ctor_new(()));
}

#[gtest]
fn test_derive_default_struct() {
    #[derive(::ctor::CtorFrom_Default)]
    struct Struct {
        x: i32,
        y: f32,
    }
    unsafe impl ::ctor::RecursivelyPinned for Struct {
        type CtorInitializedFields = Self;
    }
    impl !Unpin for Struct {}

    let p = ::ctor::emplace!(<Struct as ::ctor::CtorNew<()>>::ctor_new(()));
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0.0);
}

#[gtest]
fn test_derive_default_tuple_struct() {
    #[derive(::ctor::CtorFrom_Default)]
    struct Struct(i32, f32);
    unsafe impl ::ctor::RecursivelyPinned for Struct {
        type CtorInitializedFields = Self;
    }
    impl !Unpin for Struct {}

    let p = ::ctor::emplace!(<Struct as ::ctor::CtorNew<()>>::ctor_new(()));
    assert_eq!(p.0, 0);
    assert_eq!(p.1, 0.0);
}

#[gtest]
fn test_derive_move_and_assign_via_copy() {
    #[derive(Copy, Clone, ::ctor::MoveAndAssignViaCopy)]
    struct Struct {
        #[allow(unused)]
        x: i32,
        #[allow(unused)]
        y: f32,
    }

    fn implements_traits<T>()
    where
        T: for<'a> From<::ctor::RvalueReference<'a, T>>
            + for<'a> ::ctor::CtorNew<::ctor::RvalueReference<'a, T>>
            + for<'a> ::ctor::UnpinAssign<&'a T>
            + for<'a> ::ctor::UnpinAssign<::ctor::RvalueReference<'a, T>>,
    {
    }

    implements_traits::<Struct>();
}

#[gtest]
fn test_recursively_pinned_unit_struct() {
    #[::ctor::recursively_pinned]
    struct S;
    let _ = Box::pin(S).as_mut().project_pin();
    assert_eq!(::std::mem::size_of::<::ctor::project_pin_type!(S)>(), 0);
    let _ = Box::pin(S).as_ref().project_ref();
    assert_eq!(::std::mem::size_of::<::ctor::project_ref_type!(S)>(), 0);
}

#[gtest]
fn test_recursively_pinned_fieldless_struct() {
    #[::ctor::recursively_pinned]
    struct S {}
    let mut b = Box::pin(S {
        __must_use_ctor_to_initialize: [],  // for tests only!
    });
    let _ = b.as_mut().project_pin();
    assert_eq!(::std::mem::size_of::<::ctor::project_pin_type!(S)>(), 0);
    let _ = b.as_ref().project_ref();
    assert_eq!(::std::mem::size_of::<::ctor::project_ref_type!(S)>(), 0);
}

#[gtest]
fn test_recursively_pinned_fieldless_tuple_struct() {
    #[::ctor::recursively_pinned]
    struct S();
    let _ = Box::pin(S()).as_mut().project_pin();
    assert_eq!(::std::mem::size_of::<::ctor::project_pin_type!(S)>(), 0);
    let _ = Box::pin(S()).as_ref().project_ref();
    assert_eq!(::std::mem::size_of::<::ctor::project_ref_type!(S)>(), 0);
}

#[gtest]
fn test_recursively_pinned_fieldless_enum() {
    #[::ctor::recursively_pinned]
    enum E {
        A,
    }
    let <::ctor::project_pin_type!(E)>::A = Box::pin(E::A).as_mut().project_pin();
    assert_eq!(::std::mem::size_of::<::ctor::project_pin_type!(E)>(), 0);
    let <::ctor::project_ref_type!(E)>::A = Box::pin(E::A).as_ref().project_ref();
    assert_eq!(::std::mem::size_of::<::ctor::project_ref_type!(E)>(), 0);
}

#[gtest]
fn test_recursively_pinned_in_module() {
    mod submodule {
        #[::ctor::recursively_pinned]
        pub struct S;
    }
    let _: ::ctor::project_pin_type!(submodule::S) = Box::pin(submodule::S).as_mut().project_pin();
    let _: ::ctor::project_ref_type!(submodule::S) = Box::pin(submodule::S).as_ref().project_ref();
}

#[gtest]
fn test_recursively_pinned_struct() {
    #[::ctor::recursively_pinned]
    struct S {
        x: i32,
    }
    let mut b = Box::pin(S {
        x: 42,
        __must_use_ctor_to_initialize: [], // for tests only!
    });
    let _: ::std::pin::Pin<&mut i32> = b.as_mut().project_pin().x;

    let _: ::std::pin::Pin<&i32> = b.as_ref().project_ref().x;
}

#[gtest]
fn test_recursively_pinned_tuple_struct() {
    #[::ctor::recursively_pinned]
    struct S(i32);
    let _: ::std::pin::Pin<&mut i32> = Box::pin(S(42)).as_mut().project_pin().0;
    let _: ::std::pin::Pin<&i32> = Box::pin(S(42)).as_ref().project_ref().0;
}

// TODO(b/331688163): remove this workaround.
type Identity<T> = T;

#[gtest]
fn test_recursively_pinned_enum_struct() {
    #[::ctor::recursively_pinned]
    enum E {
        A { x: i32 },
    }
    let mut b = Box::pin(E::A { x: 42 });
    match b.as_mut().project_pin() {
        Identity::<::ctor::project_pin_type!(E)>::A { x } => {
            let _: ::std::pin::Pin<&mut i32> = x;
        }
    }
    match b.as_ref().project_ref() {
        Identity::<::ctor::project_ref_type!(E)>::A { x } => {
            let _: ::std::pin::Pin<&i32> = x;
        }
    }
}

#[gtest]
fn test_recursively_pinned_enum_tuple() {
    #[::ctor::recursively_pinned]
    enum E {
        A(i32),
    }
    let mut b = Box::pin(E::A(42));
    match b.as_mut().project_pin() {
        Identity::<::ctor::project_pin_type!(E)>::A(x) => {
            let _: ::std::pin::Pin<&mut i32> = x;
        }
    }
    match b.as_ref().project_ref() {
        Identity::<::ctor::project_ref_type!(E)>::A(x) => {
            let _: ::std::pin::Pin<&i32> = x;
        }
    }
}

#[gtest]
fn test_recursively_pinned_generic() {
    #[::ctor::recursively_pinned]
    struct S<'proj, 'proj_2: 'proj, 'proj_4, T>
    where
        'proj_4: 'proj_2,
    {
        x: T,
        /// 'proj* are not really used, but exist to try to throw a wrench in
        /// the works.
        _phantom: ::std::marker::PhantomData<&'proj &'proj_2 &'proj_4 T>,
    }
    let mut b = Box::pin(S::<i32> {
        x: 42,
        _phantom: ::std::marker::PhantomData,
        __must_use_ctor_to_initialize: [], // for tests only!
    });
    let _: ::std::pin::Pin<&mut i32> = b.as_mut().project_pin().x;
    let _: ::std::pin::Pin<&i32> = b.as_ref().project_ref().x;
}

#[gtest]
fn test_recursively_pinned_dst() {
    #[::ctor::recursively_pinned]
    struct Foo<T: ?Sized> {
        x: i32,
        y: T,
    }
    let x: ::std::pin::Pin<&mut Foo<[u8]>> =
        ::ctor::emplace!(::ctor::ctor!(Foo::<[u8; 1]> { x: 4, y: [2] }));
    assert_eq!(x.x, 4);
    assert_eq!(x.y, [2]);
}
#[gtest]
fn test_recursively_pinned_generic_maybe_unpin() {
    #[::ctor::recursively_pinned(?Unpin)]
    struct S<T, U> {
        x: T,
        y: U,
    }

    use ::std::marker::PhantomPinned; // TODO(b/477396909): make this workaround unnecessary.
    static_assertions::assert_impl_all!(S<i32, i32>: Unpin);
    static_assertions::assert_not_impl_any!(S<i32, ::std::marker::PhantomPinned>: Unpin);

    // And it can actually be constructed either way.
    let _ = ::ctor::emplace!(::ctor::ctor!(S::<i32, i32> { x: 42, y: 43 }));
    let _ = ::ctor::emplace!(::ctor::ctor!(S::<i32, PhantomPinned> {
        x: 42,
        y: ::ctor::PhantomPinnedCtor
    }));
}

#[gtest]
fn test_recursively_pinned_struct_derive_default() {
    #[::ctor::recursively_pinned]
    #[derive(::ctor::CtorFrom_Default)]
    struct Struct {
        x: i32,
        y: f32,
    }

    let p = ::ctor::emplace!(<Struct as ::ctor::CtorNew<()>>::ctor_new(()));
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0.0);
}

/// The same as the previous test, but with the attribute order swapped.
/// This only compiles with macro_attributes_in_derive_output.
#[gtest]
fn test_derive_default_recursively_pinned_struct() {
    #[derive(::ctor::CtorFrom_Default)]
    #[::ctor::recursively_pinned]
    struct Struct {
        x: i32,
        y: f32,
    }

    let p = ::ctor::emplace!(<Struct as ::ctor::CtorNew<()>>::ctor_new(()));
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0.0);
}

#[gtest]
fn test_recursively_pinned_actually_pinned() {
    #[::ctor::recursively_pinned]
    struct Struct {
        x: i32,
        y: f32,
        pin: ::std::marker::PhantomPinned,
    }

    let p =
        ::ctor::emplace!(::ctor::ctor!(Struct { x: 0, y: 0.0, pin: ::ctor::PhantomPinnedCtor }));
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0.0);
    // TODO(jeanpierreda): negative compilation test for e.g. `p.x = 1;`
}

// TODO(jeanpierreda): negative compilation tests for invalid parameters to
// #[recursively_pinned]:
// * unknown parameter
// * passing the same parameter twice
// * extra parameters after parameter parsing is complete

// TODO(jeanpierreda): negative compilation tests for Drop / PinnedDrop failures:
// * implemented Drop
// * forgot to implement PinnedDrop
// * implemented PinnedDrop, but forgot to pass in PinnedDrop to
//   `::ctor::recursively_pinned`.

#[gtest]
fn test_pinned_drop() {
    use ::std::cell::Cell;
    use ::std::rc::Rc;

    #[::ctor::recursively_pinned(PinnedDrop)]
    struct DropStruct(Rc<Cell<bool>>);
    impl ::ctor::PinnedDrop for DropStruct {
        unsafe fn pinned_drop(self: ::std::pin::Pin<&mut Self>) {
            (&*self.project_pin().0).set(true);
        }
    }

    let called_drop = Rc::new(Cell::new(false));
    let _ = DropStruct(called_drop.clone());
    assert!(called_drop.get(), "PinnedDrop::pinned_drop was not called");
}

#[gtest]
fn test_maybe_unpin() {
    #[::ctor::recursively_pinned(?Unpin)]
    struct S {
        x: i32,
    }

    static_assertions::assert_impl_all!(S: Unpin);

    // And it can actually be constructed.
    let _ = ::ctor::emplace!(::ctor::ctor!(S { x: 42 }));
}
