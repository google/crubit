// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use add_assign::*;
use googletest::prelude::*;

#[gtest]
fn test_add_assign_member_int() {
    let mut s = AddAssignMemberInt { i: 11 };
    s += 22;
    assert_eq!(33, s.i);
}

#[gtest]
fn test_add_assign_member_by_value() {
    let mut s1 = AddAssignMemberByValue { i: 11 };
    let s2 = AddAssignMemberByValue { i: 22 };
    s1 += s2;
    assert_eq!(33, s1.i);
}

#[gtest]
fn test_add_assign_member_by_ref() {
    let mut s1 = AddAssignMemberByRef { i: 11 };
    let mut s2 = AddAssignMemberByRef { i: 22 };
    s1 += &mut s2;
    assert_eq!(33, s1.i);
}

#[gtest]
fn test_add_assign_member_by_const_ref() {
    let mut s1 = AddAssignMemberByConstRef { i: 11 };
    let s2 = AddAssignMemberByConstRef { i: 22 };
    s1 += &s2;
    assert_eq!(33, s1.i);
}

#[gtest]
fn test_add_assign_free_by_value() {
    let mut s1 = AddAssignFreeByValue { i: 11 };
    let s2 = AddAssignFreeByValue { i: 22 };
    s1 += s2;
    assert_eq!(33, s1.i);
}

#[gtest]
fn test_add_assign_friend_by_value() {
    let mut s1 = AddAssignFriendByValue { i: 11 };
    let s2 = AddAssignFriendByValue { i: 22 };
    s1 += s2;
    assert_eq!(33, s1.i);
}

#[gtest]
fn test_add_assign_inline_friend_by_value() {
    let mut s1 = AddAssignInlineFriendByValue { i: 111 };
    let s2 = AddAssignInlineFriendByValue { i: 222 };
    s1 += s2;
    assert_eq!(333, s1.i);
}
