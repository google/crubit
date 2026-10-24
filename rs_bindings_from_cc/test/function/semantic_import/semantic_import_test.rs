// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::emplace;
use ctor::CtorNew as _;
use googletest::gtest;
use item_exists::value_exists;

#[gtest]
fn test_int_accessor() {
    let mut s = semantic_import::S::from(42);
    assert_eq!(s.x(), 42);
    s.set_x(100);
    assert_eq!(s.x(), 100);
    assert_eq!(s.get_x(), 100);
}

#[gtest]
fn test_float_and_inherited_int_accessor() {
    let t = emplace!(semantic_import::T::ctor_new((42, 1.23f32)));
    assert_eq!(t.y(), 1.23f32);
    assert!(!value_exists!(semantic_import::t::x));
    assert!(!value_exists!(semantic_import::t::get_x));
}

#[gtest]
fn test_char_accessors() {
    let mut chars = semantic_import::Chars::default();
    assert_eq!(chars.c(), b'c');
    assert_eq!(chars.sc(), b's' as i8);
    assert_eq!(chars.uc(), b'u');
    chars.set_c(b'C'.into());
    chars.set_sc(b'S' as i8);
    chars.set_uc(b'U');
    assert_eq!(chars.c(), b'C');
    assert_eq!(chars.sc(), b'S' as i8);
    assert_eq!(chars.uc(), b'U');
}

#[gtest]
fn test_bool_accessors() {
    let mut bools = semantic_import::Bools::default();
    assert_eq!(bools.b(), true);
    bools.set_b(false);
    assert_eq!(bools.b(), false);
}

#[gtest]
fn test_pointer_accessors() {
    let mut pointers = semantic_import::Pointers::default();
    assert_eq!(pointers.p(), core::ptr::null());
    assert_eq!(pointers.mut_p(), core::ptr::null_mut());
    let mut x: i32 = 42;
    unsafe {
        pointers.set_p(&x);
        pointers.set_mut_p(&mut x);
        assert_eq!(*pointers.p(), 42);
        *pointers.mut_p() = 100;
        assert_eq!(*pointers.p(), 100);
        assert_eq!(x, 100);
    }
}

#[gtest]
fn test_non_trivial_pointer_accessors() {
    let mut nt = emplace!(semantic_import::NonTrivial::ctor_new(()));
    assert_eq!(nt.p(), core::ptr::null());
    assert_eq!(nt.as_mut().mut_p(), core::ptr::null_mut());
    let mut other = emplace!(semantic_import::NonTrivial::ctor_new(()));
    unsafe {
        let other_const_ptr: *const semantic_import::NonTrivial = &*other;
        let other_mut_ptr: *mut semantic_import::NonTrivial =
            core::pin::Pin::into_inner_unchecked(other.as_mut()) as *mut _;
        nt.as_mut().set_p(other_const_ptr);
        nt.as_mut().set_mut_p(other_mut_ptr);
        assert_eq!(nt.p(), other_const_ptr);
        assert_eq!(nt.as_mut().mut_p(), other_mut_ptr);
    }
}

/// `DerivedPointers::p()` is declared in the derived class but reads a field of `PointerBase`,
/// which is at a non-zero offset within `DerivedPointers`. Using the base-relative offset would
/// read `Padding::pad_` instead.
#[gtest]
fn test_inherited_pointer_accessors() {
    let mut derived = semantic_import::DerivedPointers::default();
    assert_eq!(derived.p(), core::ptr::null());
    let x: i32 = 42;
    unsafe {
        derived.set_p(&x);
        assert_eq!(derived.p(), &x as *const i32);
        assert_eq!(*derived.p(), 42);
    }
}

#[gtest]
fn test_base_pointer_accessors() {
    let mut base = semantic_import::PointerBase::default();
    assert_eq!(base.base_p(), core::ptr::null());
    let x: i32 = 7;
    unsafe {
        base.set_base_p(&x);
        assert_eq!(base.base_p(), &x as *const i32);
    }
}

#[gtest]
fn test_non_standard_layout_pointer_accessors() {
    let mut nsl = semantic_import::NonStandardLayoutPointers::default();
    assert_eq!(nsl.p(), core::ptr::null());
    assert_eq!(nsl.x(), 0);
    let x: i32 = 42;
    unsafe {
        nsl.set_p(&x);
        assert_eq!(nsl.p(), &x as *const i32);
        assert_eq!(*nsl.p(), 42);
    }
    nsl.set_x(99);
    assert_eq!(nsl.x(), 99);
}

#[gtest]
fn test_void_pointer_accessors() {
    let mut more = semantic_import::MorePointers::default();
    assert_eq!(more.v(), core::ptr::null_mut());
    let mut x: i32 = 42;
    let x_ptr = &raw mut x;
    unsafe {
        more.set_v(x_ptr as *mut core::ffi::c_void);
        assert_eq!(more.v(), x_ptr as *mut core::ffi::c_void);
        assert_eq!(*(more.v() as *mut i32), 42);
    }
}

#[gtest]
fn test_incomplete_pointee_accessors() {
    let mut more = semantic_import::MorePointers::default();
    assert_eq!(more.i(), core::ptr::null_mut());
    // `Incomplete` is never defined, so this pointer must never be dereferenced. It only has to
    // round-trip, which is what verifies that a pointer to an incomplete type is still thin.
    let mut x: i32 = 42;
    let i_ptr = (&raw mut x) as *mut semantic_import::Incomplete;
    unsafe {
        more.set_i(i_ptr);
        assert_eq!(more.i(), i_ptr);
    }
}

#[gtest]
fn test_record_and_free_getter() {
    let wrapper = semantic_import::StructWithRecordAndFreeGetter::from(77);
    assert_eq!(wrapper.s().x(), 77);
    assert_eq!(semantic_import::free_get_s(&wrapper).x(), 77);
}
