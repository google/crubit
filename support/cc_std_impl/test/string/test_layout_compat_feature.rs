// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cpp_std::string;
use cref::CRef;
use ctor::{emplace, CtorNew};
use googletest::{expect_eq, gtest};
use test_helpers_with_layout_compat_string::cpp_std_string_test;

fn as_bytes<'a>(r: CRef<'a, string>) -> &'a [u8] {
    unsafe { &**CRef::as_ptr(r) }
}

#[gtest]
fn test_string_round_trip_ref_feature() {
    let s = emplace!(string::ctor_new("Hello Ref Feature"));
    let r = cpp_std_string_test::ReturnRef(&*s);
    expect_eq!(as_bytes(r), &b"Hello Ref Feature"[..]);
}

#[gtest]
fn test_string_mutate_ref_feature() {
    let mut s = emplace!(string::ctor_new("Hello Feature"));
    cpp_std_string_test::MutateRef(s.as_mut());
    expect_eq!(&*s, &b"Hello Feature mutated"[..]);
}

#[gtest]
fn test_string_round_trip_ptr_feature() {
    let s = emplace!(string::ctor_new("Hello Ptr Feature"));
    let s_ptr = s.as_ptr();
    let r = unsafe { cpp_std_string_test::ReturnPtr(s_ptr) };
    let r_ref = unsafe { &*r };
    expect_eq!(&*r_ref, &b"Hello Ptr Feature"[..]);
}

#[gtest]
fn test_string_mutate_ptr_feature() {
    let mut s = emplace!(string::ctor_new("Hello Feature"));
    unsafe {
        let s_mut_ptr = s.as_mut().as_mut_ptr();
        cpp_std_string_test::MutatePtr(s_mut_ptr);
    }
    expect_eq!(&*s, &b"Hello Feature mutated"[..]);
}

#[gtest]
fn test_string_vector_feature() {
    let s = emplace!(string::ctor_new("hello feature"));
    let v = emplace!(cpp_std_string_test::MakeVector(&*s));

    expect_eq!(v.len(), 2);
    let slice: &[string] = &*v;
    expect_eq!(&*slice[0], &b"hello feature"[..]);
    expect_eq!(&*slice[1], &b"hello feature_2"[..]);

    let first = cpp_std_string_test::FirstElementRef(&*v);
    expect_eq!(as_bytes(first), &b"hello feature"[..]);
}

#[gtest]
fn test_string_struct_feature() {
    let s = emplace!(string::ctor_new("hello struct feature"));
    let strct = emplace!(cpp_std_string_test::MakeStringStruct(&*s));

    let s_field = cpp_std_string_test::GetStringRefFromStruct(&*strct);
    expect_eq!(as_bytes(s_field), &b"hello struct feature"[..]);

    let r = emplace!(cpp_std_string_test::GetStringFromStruct(&*strct));
    expect_eq!(&*r, &b"hello struct feature"[..]);
}

#[gtest]
fn test_string_round_trip_by_value_feature() {
    let r = emplace!(cpp_std_string_test::RoundTrip(string::ctor_new("hello by value feature")));
    expect_eq!(&*r, &b"hello by value feature"[..]);
}
