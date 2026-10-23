// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::cmp::Ordering;
use googletest::gtest;
use spaceship::*;

#[gtest]
fn test_defaulted_strong_ordering() {
    let s1 = DefaultedStrongOrdering { i: 1 };
    let s2 = DefaultedStrongOrdering { i: 2 };
    let s1_copy = DefaultedStrongOrdering { i: 1 };

    assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
    assert_eq!(s2.partial_cmp(&s1), Some(Ordering::Greater));
    assert_eq!(s1.partial_cmp(&s1_copy), Some(Ordering::Equal));
    assert!(s1 < s2);
    assert!(s1 <= s1_copy);
    assert!(s2 > s1);
    assert!(s1 == s1_copy);
    assert!(s1 != s2);
}

#[gtest]
fn test_custom_weak_ordering() {
    let s1 = CustomWeakOrdering { i: 1 };
    let s2 = CustomWeakOrdering { i: 2 };
    let s11 = CustomWeakOrdering { i: 11 };

    assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
    assert_eq!(s2.partial_cmp(&s11), Some(Ordering::Greater));
    assert_eq!(s1.partial_cmp(&s11), Some(Ordering::Equal));
    assert!(s1 == s11);
    assert!(s2 > s11);
}

#[gtest]
fn test_defaulted_partial_ordering() {
    let one = DefaultedPartialOrdering { f: 1.0 };
    let two = DefaultedPartialOrdering { f: 2.0 };
    let nan = DefaultedPartialOrdering { f: f32::NAN };

    assert_eq!(one.partial_cmp(&two), Some(Ordering::Less));
    assert_eq!(two.partial_cmp(&one), Some(Ordering::Greater));
    assert_eq!(one.partial_cmp(&one), Some(Ordering::Equal));
    assert_eq!(one.partial_cmp(&nan), None);
    assert_eq!(nan.partial_cmp(&nan), None);
    assert!(!(one < nan));
    assert!(!(one >= nan));
    assert!(nan != nan);
}

#[gtest]
fn test_friend_strong_ordering() {
    let s1 = FriendStrongOrdering { i: 1 };
    let s2 = FriendStrongOrdering { i: 2 };

    assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
    assert_eq!(s2.partial_cmp(&s1), Some(Ordering::Greater));
    assert_eq!(s1.partial_cmp(&s1), Some(Ordering::Equal));
}

#[gtest]
fn test_spaceship_preferred_over_less_than() {
    let s1 = SpaceshipAndLessThan { i: 1 };
    let s2 = SpaceshipAndLessThan { i: 2 };

    // `operator<` always returns `false`, so this only passes if `operator<=>`
    // backs the `PartialOrd` impl.
    assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
    assert!(s1 < s2);
}

#[gtest]
fn test_spaceship_returning_not_an_ordering() {
    static_assertions::assert_impl_all!(SpaceshipReturningNotAnOrdering: PartialEq);
    static_assertions::assert_not_impl_any!(SpaceshipReturningNotAnOrdering: PartialOrd);
}
