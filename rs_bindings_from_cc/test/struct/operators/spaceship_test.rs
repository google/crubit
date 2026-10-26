// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::cmp::Ordering;
use googletest::prelude::*;
use spaceship::{
    CustomWeakOrdering, DefaultedPartialOrdering, DefaultedStrongOrdering, FriendStrongOrdering,
    SpaceshipAndLessThan, SpaceshipReturningNotAnOrdering,
};

#[gtest]
fn test_defaulted_strong_ordering() {
    let s1 = DefaultedStrongOrdering { i: 1 };
    let s2 = DefaultedStrongOrdering { i: 2 };
    let s1_copy = DefaultedStrongOrdering { i: 1 };

    expect_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
    expect_eq!(s2.partial_cmp(&s1), Some(Ordering::Greater));
    expect_eq!(s1.partial_cmp(&s1_copy), Some(Ordering::Equal));
    expect_true!(s1 < s2);
    expect_true!(s1 <= s1_copy);
    expect_true!(s2 > s1);
    expect_true!(s1 == s1_copy);
    expect_true!(s1 != s2);
}

#[gtest]
fn test_custom_weak_ordering() {
    let s1 = CustomWeakOrdering { i: 1 };
    let s2 = CustomWeakOrdering { i: 2 };
    let s11 = CustomWeakOrdering { i: 11 };

    expect_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
    expect_eq!(s2.partial_cmp(&s11), Some(Ordering::Greater));
    expect_eq!(s1.partial_cmp(&s11), Some(Ordering::Equal));
    expect_true!(s1 == s11);
    expect_true!(s2 > s11);
}

#[gtest]
fn test_defaulted_partial_ordering() {
    let one = DefaultedPartialOrdering { f: 1.0 };
    let two = DefaultedPartialOrdering { f: 2.0 };
    let nan = DefaultedPartialOrdering { f: f32::NAN };

    expect_eq!(one.partial_cmp(&two), Some(Ordering::Less));
    expect_eq!(two.partial_cmp(&one), Some(Ordering::Greater));
    expect_eq!(one.partial_cmp(&one), Some(Ordering::Equal));
    expect_eq!(one.partial_cmp(&nan), None);
    expect_eq!(nan.partial_cmp(&nan), None);
    expect_false!(one < nan);
    expect_false!(one >= nan);
    expect_true!(nan != nan);
}

#[gtest]
fn test_friend_strong_ordering() {
    let s1 = FriendStrongOrdering { i: 1 };
    let s2 = FriendStrongOrdering { i: 2 };

    expect_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
    expect_eq!(s2.partial_cmp(&s1), Some(Ordering::Greater));
    expect_eq!(s1.partial_cmp(&s1), Some(Ordering::Equal));
}

#[gtest]
fn test_spaceship_preferred_over_less_than() {
    let s1 = SpaceshipAndLessThan { i: 1 };
    let s2 = SpaceshipAndLessThan { i: 2 };

    // `operator<` always returns `false`, so this only passes if `operator<=>`
    // backs the `PartialOrd` impl.
    expect_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
    expect_true!(s1 < s2);
}

#[gtest]
fn test_spaceship_returning_not_an_ordering() {
    static_assertions::assert_impl_all!(SpaceshipReturningNotAnOrdering: PartialEq);
    static_assertions::assert_not_impl_any!(SpaceshipReturningNotAnOrdering: PartialOrd);
}
