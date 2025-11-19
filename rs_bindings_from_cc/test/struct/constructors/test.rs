// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use constructors::{
    NonTrivialStructWithConstructors, OtherSimpleStruct, StructWithDeletedConstructors,
    StructWithExplicitConversionConstructor, StructWithExplicitlyDefaultedConstructors,
    StructWithImplicitConversionConstructor, StructWithImplicitConversionFromReference,
    StructWithInlineConstructors, StructWithPrivateConstructors,
    StructWithUserProvidedConstructors,
};
use ctor::emplace;
use ctor::CtorNew as _;
use googletest::gtest;
use static_assertions::{assert_impl_all, assert_not_impl_any};

#[gtest]
#[allow(clippy::redundant_clone)]
fn test_user_provided_constructors() {
    assert_impl_all!(StructWithUserProvidedConstructors: Default);
    let s: StructWithUserProvidedConstructors = Default::default();
    assert_eq!(42, s.int_field);

    assert_impl_all!(StructWithUserProvidedConstructors: Clone);
    let s_clone = s.clone();
    assert_eq!(10042, s_clone.int_field);

    // Trivial-ABI structs should not implement the Copy trait, if they have a
    // user-defined copy constructor (aka a non-trivial copy constructor).
    assert_not_impl_any!(StructWithUserProvidedConstructors: Copy);
}

#[gtest]
fn test_explicit_conversion_constructor() {
    assert_impl_all!(StructWithExplicitConversionConstructor: From<i32>);
    let i: StructWithExplicitConversionConstructor = 125.into();
    assert_eq!(125, i.int_field);
}

#[gtest]
fn test_implicit_conversion_constructor() {
    assert_impl_all!(StructWithImplicitConversionConstructor: From<i32>);
    let i: StructWithImplicitConversionConstructor = 125.into();
    assert_eq!(125, i.int_field);
}

#[gtest]
fn test_implicit_conversion_from_reference() {
    let other = OtherSimpleStruct { int_field: 126 };
    let i: StructWithImplicitConversionFromReference = (&other).into();
    assert_eq!(126, i.int_field);
}

#[gtest]
#[allow(clippy::redundant_clone)]
fn test_inline_constructors() {
    assert_impl_all!(StructWithInlineConstructors: Default);
    let s: StructWithInlineConstructors = Default::default();
    assert_eq!(123, s.int_field);

    assert_impl_all!(StructWithInlineConstructors: Clone);
    let s_clone = s.clone();
    assert_eq!(20123, s_clone.int_field);

    // Trivial-ABI structs should not implement the Copy trait, if they have a
    // user-defined copy constructor (aka a non-trivial copy constructor).
    assert_not_impl_any!(StructWithInlineConstructors: Copy);

    assert_impl_all!(StructWithInlineConstructors: From<i32>);
    let i: StructWithInlineConstructors = 456.into();
    assert_eq!(456, i.int_field);
}

#[gtest]
fn test_deleted_constructors() {
    assert_not_impl_any!(StructWithDeletedConstructors: Clone, Copy, Default, From<i32>);
}

#[gtest]
fn test_private_constructors() {
    assert_not_impl_any!(StructWithPrivateConstructors: Clone, Copy, Default, From<i32>);
}

#[gtest]
#[allow(clippy::clone_on_copy)]
fn test_explicitly_defaulted_constructors() {
    assert_impl_all!(StructWithExplicitlyDefaultedConstructors: Default);
    let s: StructWithExplicitlyDefaultedConstructors = Default::default();
    assert_eq!(0, s.field_with_no_initializer); // Using `MaybeUninit<T>::zeroed()`.
    assert_eq!(123, s.field_with_explicit_initializer);

    // In some scenarios the bindings generator may be able to ask Rust to
    // `#[derive(Clone)]` (e.g. when the C++ constructor has been
    // implicitly or explicitly `=default`-ed + when Rust can mimic how C++
    // would copy/clone all the fields). Therefore, the test assertions
    // below may mostly be testing/exercising how Rust derives Clone.  This
    // should be okay.
    assert_impl_all!(StructWithExplicitlyDefaultedConstructors: Clone);
    let s_clone = s.clone();
    assert_eq!(0, s_clone.field_with_no_initializer);
    assert_eq!(123, s_clone.field_with_explicit_initializer);

    assert_impl_all!(StructWithExplicitlyDefaultedConstructors: Copy);
}

#[gtest]
fn test_nontrivial_struct() {
    // Non-trivial types cannot be copied.
    assert_not_impl_any!(NonTrivialStructWithConstructors: Copy);

    // Non-trivial types cannot be constructed by-value, despite having default
    // constructor, copy constructor, and constructor taking an int.
    assert_not_impl_any!(NonTrivialStructWithConstructors: Clone, Default, From<i32>);

    let s = emplace!(NonTrivialStructWithConstructors::ctor_new(123));
    assert_eq!(s.int_field, 123);

    let s_clone = emplace!(ctor::copy(&*s));
    assert_eq!(s_clone.int_field, 123);
}
