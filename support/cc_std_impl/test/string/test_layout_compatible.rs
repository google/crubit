// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::new_string;
use ctor::{emplace, CtorNew};
use googletest::{expect_eq, gtest};

#[gtest]
fn test_ffi_round_trip_handle_non_utf8() {
    let input = b"Hello\xffworld";
    let s = emplace!(new_string::ctor_new(&input[..]));
    let s2 = emplace!(test_helpers_layout_compatible::cpp_std_string_test::RoundTrip(
        new_string::ctor_new(&*s)
    ));
    expect_eq!(&*s, &*s2);
}

#[gtest]
fn test_new_string_round_trip_ref() {
    let s = emplace!(new_string::ctor_new("Hello Ref"));
    let r = emplace!(test_helpers_layout_compatible::cpp_std_string_test::RoundTripRef(&*s));
    expect_eq!(&*r, &b"Hello Ref"[..]);
}

#[gtest]
fn test_new_string_mutate_ref() {
    let mut s = emplace!(new_string::ctor_new("Hello"));
    test_helpers_layout_compatible::cpp_std_string_test::MutateRef(s.as_mut());
    expect_eq!(&*s, &b"Hello mutated"[..]);
}

#[gtest]
fn test_new_string_round_trip_ptr() {
    let s = emplace!(new_string::ctor_new("Hello Ptr"));
    let s_ptr = s.as_ptr();
    let r = emplace!(unsafe {
        test_helpers_layout_compatible::cpp_std_string_test::RoundTripPtr(s_ptr)
    });
    expect_eq!(&*r, &b"Hello Ptr"[..]);
}

#[gtest]
fn test_new_string_mutate_ptr() {
    let mut s = emplace!(new_string::ctor_new("Hello"));
    unsafe {
        let s_mut_ptr = s.as_mut().as_mut_ptr();
        test_helpers_layout_compatible::cpp_std_string_test::MutatePtr(s_mut_ptr);
    }
    expect_eq!(&*s, &b"Hello mutated"[..]);
}

#[gtest]
fn test_new_string_vector() {
    let s = emplace!(new_string::ctor_new("hello"));
    let v = emplace!(test_helpers_layout_compatible::cpp_std_string_test::MakeVector(&*s));

    expect_eq!(v.len(), 2);
    let slice: &[new_string] = &*v;
    expect_eq!(&*slice[0], &b"hello"[..]);
    expect_eq!(&*slice[1], &b"hello_2"[..]);

    let first = emplace!(test_helpers_layout_compatible::cpp_std_string_test::FirstElement(&*v));
    expect_eq!(&*first, &b"hello"[..]);
}

#[gtest]
fn test_new_string_struct() {
    let s = emplace!(new_string::ctor_new("hello struct"));
    let strct =
        emplace!(test_helpers_layout_compatible::cpp_std_string_test::MakeStringStruct(&*s));

    let s_field: &new_string = &strct.s;
    expect_eq!(&*s_field, &b"hello struct"[..]);

    let r =
        emplace!(test_helpers_layout_compatible::cpp_std_string_test::GetStringFromStruct(&*strct));
    expect_eq!(&*r, &b"hello struct"[..]);
}

#[gtest]
fn test_new_string_needs_drop() {
    expect_eq!(std::mem::needs_drop::<new_string>(), true);
}
