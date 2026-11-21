// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
use str_ref_cc_lib::crubit;

#[gtest]
fn test_hold_str() {
    let s = String::from("hello");
    let h: leaf_rs_lib::LeafRsStr<'_> = crubit::HoldStr(&s);
    assert_eq!(crubit::FirstByte(h), b'h');
}

#[gtest]
fn test_hold_str_ambiguous() {
    let s = String::from("hello");
    let x = 0u8;
    // SAFETY: `s` outlives the result.
    let h = unsafe { crubit::HoldStrAmbiguous(&s, &x).into_inner() };
    assert_eq!(crubit::FirstByte(h), b'h');
}

#[gtest]
fn test_echo() {
    let s = String::from("hello");
    let e: &str = crubit::Echo(&s);
    assert_eq!(e, "hello");
}

#[gtest]
fn test_call_with_str() {
    // The closure must accept a `&str` of any lifetime (here: the C++ caller's temporary).
    let first = crubit::CallWithStr(Box::new(|s: &str| s.as_bytes()[0]));
    assert_eq!(first, b'h');
}

#[gtest]
fn test_pick() {
    let a = String::from("a");
    let b = String::from("b");
    // SAFETY: `a` outlives the result.
    let p: &str = unsafe { crubit::Pick(&a, &b).into_inner() };
    assert_eq!(p, "a");
}
