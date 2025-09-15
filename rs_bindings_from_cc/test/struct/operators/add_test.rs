// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use add::*;
use googletest::prelude::*;

#[gtest]
fn test_add_const_member_int() {
    let s = AddableConstMemberInt { i: 11 };
    assert_eq!(33, &s + 22);
}

#[gtest]
fn test_add_const_member_by_ref() {
    let s1 = AddableConstMemberByRef { i: 11 };
    let s2 = AddableConstMemberByRef { i: 22 };
    assert_eq!(33, (&s1 + &s2).i);
}

#[gtest]
fn test_add_non_const_member_by_ref() {
    let mut s1 = AddableNonConstMemberByRef { i: 11 };
    let s2 = AddableNonConstMemberByRef { i: 22 };
    assert_eq!(33, (&mut s1 + &s2).i);
}

#[gtest]
fn test_add_const_member_by_value() {
    let s1 = AddableConstMemberByValue { i: 11 };
    let s2 = AddableConstMemberByValue { i: 22 };
    assert_eq!(33, (&s1 + s2).i);
}

#[gtest]
fn test_add_non_const_member_by_value() {
    let mut s1 = AddableNonConstMemberByValue { i: 11 };
    let s2 = AddableNonConstMemberByValue { i: 22 };
    assert_eq!(33, (&mut s1 + s2).i);
}

#[gtest]
fn test_add_returns_void() {
    let mut s1 = AddableReturnsVoid { i: 11 };
    let s2 = AddableReturnsVoid { i: 22 };
    &mut s1 + &s2;
    assert_eq!(s1.i, 33);
}

#[gtest]
fn test_add_nontrivial_by_value() {
    ctor::emplace! {
        let s1 = ctor::ctor!(AddableNontrivialByValue {i: 11});
        let s2 = ctor::ctor!(AddableNontrivialByValue {i: 22});
    }
    ctor::emplace! {
        let sum = &*s1 + ctor::mov!(s2);
    }
    assert_eq!(sum.i, 33);
}

#[gtest]
fn test_add_free_by_const_ref() {
    let s1 = UnpinStructByConstRef { i: 11 };
    let s2 = UnpinStructByConstRef { i: 22 };
    assert_eq!(33, (&s1 + &s2).i);
}

#[gtest]
fn test_add_free_by_mut_ref() {
    let mut s1 = UnpinStructByMutRef { i: 11 };
    let mut s2 = UnpinStructByMutRef { i: 22 };
    assert_eq!(33, (&mut s1 + &mut s2).i);
}

#[gtest]
fn test_add_free_by_value() {
    let s1 = UnpinStructByValue { i: 11 };
    let s2 = UnpinStructByValue { i: 22 };
    assert_eq!(33, (s1 + s2).i);
}

#[gtest]
fn test_add_overloaded() {
    let s = AddableOverloaded { int16_char: b'A', int32_char: b'B' };
    assert_eq!(b'A', s + 0i16);
    assert_eq!(b'B', s + 0i32);
}

#[gtest]
fn test_add_friend_by_const_ref() {
    let s1 = AddableFriendByConstRef { i: 11 };
    let s2 = AddableFriendByConstRef { i: 22 };
    assert_eq!(33, (&s1 + &s2).i);
}

#[gtest]
fn test_add_friend_by_ref() {
    let mut s1 = AddableFriendByRef { i: 11 };
    let mut s2 = AddableFriendByRef { i: 22 };
    assert_eq!(33, (&mut s1 + &mut s2).i);
}

#[gtest]
fn test_add_friend_by_value() {
    let s1 = AddableFriendByValue { i: 11 };
    let s2 = AddableFriendByValue { i: 22 };
    assert_eq!(33, (s1 + s2).i);
}

#[gtest]
fn test_add_returns_nontrivial() {
    ctor::emplace! {
        let s1 = ctor::ctor!(AddableReturnsNontrivial {i: 11});
        let s2 = ctor::ctor!(AddableReturnsNontrivial {i: 22});
    }
    ctor::emplace! {
        let sum = &*s1 + &*s2;
    }
    assert_eq!(sum.i, 33);
}
