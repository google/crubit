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
        type ProjectedPin<'a>
            = Self
        where
            Self: 'a;
        type ProjectedRef<'a>
            = Self
        where
            Self: 'a;
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
        type ProjectedPin<'a>
            = Self
        where
            Self: 'a;
        type ProjectedRef<'a>
            = Self
        where
            Self: 'a;
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
        type ProjectedPin<'a>
            = Self
        where
            Self: 'a;
        type ProjectedRef<'a>
            = Self
        where
            Self: 'a;
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
fn test_derive_move_and_assign_via_copy_with_generics() {
    use ::std::fmt::Debug;

    // Include several bounds with both immediate and `where` clause syntax in order to ensure
    // they're handled correctly.
    #[derive(Copy, Clone, ::ctor::MoveAndAssignViaCopy)]
    struct Struct<'a, 'b, T: Copy>
    where
        'b: 'a,
        T: Debug,
    {
        #[allow(unused)]
        slice: &'a &'b [T],
        #[allow(unused)]
        value: T,
    }

    fn implements_traits<T>(_: T)
    where
        T: for<'a> From<::ctor::RvalueReference<'a, T>>
            + for<'a> ::ctor::CtorNew<::ctor::RvalueReference<'a, T>>
            + for<'a> ::ctor::UnpinAssign<&'a T>
            + for<'a> ::ctor::UnpinAssign<::ctor::RvalueReference<'a, T>>,
    {
    }

    fn check_implements_traits<'a, 'b: 'a, T: Copy + Debug>(x: Struct<'a, 'b, T>) {
        implements_traits(x);
    }

    check_implements_traits(Struct { slice: &[1, 2, 3].as_slice(), value: 42 });
}

#[gtest]
fn test_recursively_pinned_unit_struct() {
    #[::ctor::recursively_pinned]
    struct S;
    let _ = ::ctor::emplace!(::ctor::ctor!(S)).as_mut().project_pin();
    assert_eq!(::std::mem::size_of::<<S as ::ctor::RecursivelyPinned>::ProjectedPin<'static>>(), 0);
    let _ = ::ctor::emplace!(::ctor::ctor!(S)).as_ref().project_ref();
    assert_eq!(::std::mem::size_of::<<S as ::ctor::RecursivelyPinned>::ProjectedRef<'static>>(), 0);
}

#[gtest]
fn test_recursively_pinned_fieldless_struct() {
    #[::ctor::recursively_pinned]
    struct S {}
    let mut b = ::ctor::emplace!(::ctor::ctor!(S {}));
    let _ = b.as_mut().project_pin();
    assert_eq!(::std::mem::size_of::<<S as ::ctor::RecursivelyPinned>::ProjectedPin<'static>>(), 0);
    let _ = b.as_ref().project_ref();
    assert_eq!(::std::mem::size_of::<<S as ::ctor::RecursivelyPinned>::ProjectedRef<'static>>(), 0);
}

#[gtest]
fn test_recursively_pinned_fieldless_tuple_struct() {
    #[::ctor::recursively_pinned]
    struct S();
    let _ = ::ctor::emplace!(::ctor::ctor!(S())).as_mut().project_pin();
    assert_eq!(::std::mem::size_of::<<S as ::ctor::RecursivelyPinned>::ProjectedPin<'static>>(), 0);
    let _ = ::ctor::emplace!(::ctor::ctor!(S())).as_ref().project_ref();
    assert_eq!(::std::mem::size_of::<<S as ::ctor::RecursivelyPinned>::ProjectedRef<'static>>(), 0);
}

#[gtest]
fn test_recursively_pinned_in_module() {
    mod submodule {
        #[::ctor::recursively_pinned]
        pub struct S;

        // Re-export `project_pin` and `project_ref` methods. They are marked as non-public because
        // `S` has no public fields.
        pub fn project_pin(
            s: ::core::pin::Pin<&mut S>,
        ) -> <S as ::ctor::RecursivelyPinned>::ProjectedPin<'_> {
            s.project_pin()
        }

        pub fn project_ref(
            s: ::core::pin::Pin<&S>,
        ) -> <S as ::ctor::RecursivelyPinned>::ProjectedRef<'_> {
            s.project_ref()
        }
    }
    let mut b = ::ctor::emplace!(::ctor::ctor!(submodule::S));
    let _: <submodule::S as ::ctor::RecursivelyPinned>::ProjectedPin<'_> =
        submodule::project_pin(b.as_mut());
    let _: <submodule::S as ::ctor::RecursivelyPinned>::ProjectedRef<'_> =
        submodule::project_ref(b.as_ref());
}

#[gtest]
fn test_recursively_pinned_struct() {
    #[::ctor::recursively_pinned]
    struct S {
        x: i32,
    }
    let mut b = ::ctor::emplace!(::ctor::ctor!(S { x: 42 }));
    let _: ::std::pin::Pin<&mut i32> = b.as_mut().project_pin().x;

    let _: ::std::pin::Pin<&i32> = b.as_ref().project_ref().x;
}

#[gtest]
fn test_recursively_pinned_tuple_struct() {
    #[::ctor::recursively_pinned]
    struct S(i32);
    let _: ::std::pin::Pin<&mut i32> =
        ::ctor::emplace!(::ctor::ctor!(S(42))).as_mut().project_pin().0;
    let _: ::std::pin::Pin<&i32> = ::ctor::emplace!(::ctor::ctor!(S(42))).as_ref().project_ref().0;
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
    let mut b =
        ::ctor::emplace!(::ctor::ctor!(S::<i32> { x: 42, _phantom: ::std::marker::PhantomData }));
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
        y: ::std::marker::PhantomPinned,
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
        ::ctor::emplace!(::ctor::ctor!(Struct { x: 0, y: 0.0, pin: ::std::marker::PhantomPinned }));
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
    {
        let _ = ::ctor::emplace!(::ctor::ctor!(DropStruct(called_drop.clone())));
    }
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

#[gtest]
fn test_recursively_pinned_visibility() {
    mod submodule {
        #[::ctor::recursively_pinned]
        pub struct PublicStructWithPublicFields {
            pub x: i32,
            y: i32, // This will be private in projection
        }

        impl PublicStructWithPublicFields {
            pub fn new(
                x: i32,
                y: i32,
            ) -> impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> {
                ::ctor::ctor!(PublicStructWithPublicFields { x: x, y: y })
            }
        }
    }

    let mut b = ::ctor::emplace!(submodule::PublicStructWithPublicFields::new(42, 43));

    // We can access project_pin and project_ref because the struct has at least one public field.
    let mut p: <submodule::PublicStructWithPublicFields as ::ctor::RecursivelyPinned>::ProjectedPin<'_> = b.as_mut().project_pin();

    // We can access the public field through the projection.
    p.x.set(44);

    assert_eq!(*b.as_ref().project_ref().x, 44);
}

#[gtest]
fn test_recursively_pinned_visibility_crate() {
    mod submodule {
        #[::ctor::recursively_pinned]
        pub struct PublicStructWithCrateFields {
            pub(crate) x: i32,
        }

        impl PublicStructWithCrateFields {
            pub fn new(x: i32) -> impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> {
                ::ctor::ctor!(PublicStructWithCrateFields { x: x })
            }
        }
    }

    let mut b = ::ctor::emplace!(submodule::PublicStructWithCrateFields::new(42));

    // project_pin should be pub(crate) because fields are pub(crate).
    // Since we are in the same crate, we should be able to access it!
    let mut p = b.as_mut().project_pin();
    p.x.set(44);
    assert_eq!(*b.as_ref().project_ref().x, 44);
}

#[gtest]
fn test_recursively_pinned_union() {
    #[::ctor::recursively_pinned]
    union U {
        x: i32,
        #[allow(unused)]
        y: f32,
    }
    let b = ::ctor::emplace!(::ctor::ctor!(U { x: 42 }));
    assert_eq!(::std::mem::size_of::<<U as ::ctor::RecursivelyPinned>::ProjectedPin<'static>>(), 0);
    assert_eq!(::std::mem::size_of::<<U as ::ctor::RecursivelyPinned>::ProjectedRef<'static>>(), 0);
    unsafe {
        assert_eq!(b.as_ref().get_ref().x, 42);
    }
}
