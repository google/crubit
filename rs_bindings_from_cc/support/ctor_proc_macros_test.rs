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
// more_qualified_paths is used to make projected!() simpler to use.
#![feature(negative_impls, more_qualified_paths)]

// pathological shadowed names: shadow important modules that the macros use.
mod std {}
mod ctor {}
mod pin_project {}

#[test]
fn test_derive_default_struct() {
    #[derive(::ctor::CtorFrom_Default)]
    struct Struct {
        x: i32,
        y: f32,
    }
    unsafe impl ::ctor::RecursivelyPinned for Struct {}
    impl !Unpin for Struct {}

    ::ctor::emplace! {let p = <Struct as ::ctor::CtorNew<()>>::ctor_new(()); }
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0.0);
}

#[test]
fn test_derive_default_tuple_struct() {
    #[derive(::ctor::CtorFrom_Default)]
    struct Struct(i32, f32);
    unsafe impl ::ctor::RecursivelyPinned for Struct {}
    impl !Unpin for Struct {}

    ::ctor::emplace! {let p = <Struct as ::ctor::CtorNew<()>>::ctor_new(()); }
    assert_eq!(p.0, 0);
    assert_eq!(p.1, 0.0);
}

#[test]
fn test_recursively_pinned_struct() {
    #[::ctor::recursively_pinned]
    struct S {
        x: i32,
    }
    let _: ::std::pin::Pin<&mut i32> = Box::pin(S { x: 42 }).as_mut().project().x;
}

#[test]
fn test_recursively_pinned_tuple_struct() {
    #[::ctor::recursively_pinned]
    struct S(i32);
    let _: ::std::pin::Pin<&mut i32> = Box::pin(S(42)).as_mut().project().0;
}

#[test]
fn test_recursively_pinned_enum_struct() {
    #[::ctor::recursively_pinned]
    enum E {
        A { x: i32 },
    }
    match Box::pin(E::A { x: 42 }).as_mut().project() {
        <::ctor::projected!(E)>::A { x } => {
            let _: ::std::pin::Pin<&mut i32> = x;
        }
    }
}

#[test]
fn test_recursively_pinned_enum_tuple() {
    #[::ctor::recursively_pinned]
    enum E {
        A(i32),
    }
    match Box::pin(E::A(42)).as_mut().project() {
        <::ctor::projected!(E)>::A(x) => {
            let _: ::std::pin::Pin<&mut i32> = x;
        }
    }
}

#[test]
fn test_recursively_pinned_struct_derive_default() {
    #[::ctor::recursively_pinned]
    #[derive(::ctor::CtorFrom_Default)]
    struct Struct {
        x: i32,
        y: f32,
    }

    ::ctor::emplace! {let p = <Struct as ::ctor::CtorNew<()>>::ctor_new(()); }
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0.0);
}

/// The same as the previous test, but with the attribute order swapped.
/// This only compiles with macro_attributes_in_derive_output.
#[test]
fn test_derive_default_recursively_pinned_struct() {
    #[derive(::ctor::CtorFrom_Default)]
    #[::ctor::recursively_pinned]
    struct Struct {
        x: i32,
        y: f32,
    }

    ::ctor::emplace! {let p = <Struct as ::ctor::CtorNew<()>>::ctor_new(()); }
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0.0);
}

#[test]
fn test_recursively_pinned_actually_pinned() {
    #[::ctor::recursively_pinned]
    struct Struct {
        x: i32,
        y: f32,
        pin: ::std::marker::PhantomPinned,
    }

    ::ctor::emplace! {
        let p = ::ctor::ctor!(Struct {
            x: 0,
            y: 0.0,
            pin: ::ctor::PhantomPinnedCtor,
        });
    }
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0.0);
    // TODO(jeanpierreda): negative compilation test for e.g. `p.x = 1;`
}
