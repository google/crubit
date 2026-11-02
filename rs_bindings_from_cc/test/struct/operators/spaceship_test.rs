// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::cmp::Ordering;
use googletest::prelude::*;
use spaceship::{
    CompareWithOtherRecord, CustomWeakOrdering, DefaultedPartialOrdering, DefaultedStrongOrdering,
    FriendStrongOrdering, OtherRecord, SpaceshipAndLessThan, SpaceshipReturningNotAnOrdering,
    SpaceshipWithoutEq,
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

#[gtest]
fn test_eq_and_ord_only_for_strong_ordering() {
    static_assertions::assert_impl_all!(DefaultedStrongOrdering: Eq, Ord);
    static_assertions::assert_impl_all!(FriendStrongOrdering: Eq, Ord);
    static_assertions::assert_not_impl_any!(CustomWeakOrdering: Eq, Ord);
    static_assertions::assert_not_impl_any!(DefaultedPartialOrdering: Eq, Ord);
}

#[gtest]
fn test_strong_ordering_cmp() {
    let s1 = DefaultedStrongOrdering { i: 1 };
    let s2 = DefaultedStrongOrdering { i: 2 };

    expect_eq!(s1.cmp(&s2), Ordering::Less);
    expect_eq!(s2.cmp(&s1), Ordering::Greater);
    expect_eq!(s1.cmp(&s1), Ordering::Equal);
    expect_eq!(core::cmp::max(s1, s2).i, 2);
}

#[gtest]
fn test_strong_ordering_in_collections() {
    let mut v: Vec<DefaultedStrongOrdering> =
        [3, 1, 2].into_iter().map(|i| DefaultedStrongOrdering { i }).collect();
    v.sort();
    expect_eq!(v.iter().map(|s| s.i).collect::<Vec<_>>(), [1, 2, 3]);

    let set: std::collections::BTreeSet<FriendStrongOrdering> =
        [3, 1, 2, 1].into_iter().map(|i| FriendStrongOrdering { i }).collect();
    expect_eq!(set.iter().map(|s| s.i).collect::<Vec<_>>(), [1, 2, 3]);
}

#[gtest]
fn test_spaceship_with_rhs_of_different_type() {
    let one = CompareWithOtherRecord { i: 1 };

    expect_eq!(one.partial_cmp(&OtherRecord { i: 2 }), Some(Ordering::Less));
    expect_eq!(one.partial_cmp(&OtherRecord { i: 1 }), Some(Ordering::Equal));
    expect_eq!(one.partial_cmp(&OtherRecord { i: 0 }), Some(Ordering::Greater));
    expect_true!(one < OtherRecord { i: 2 });
    expect_true!(one >= OtherRecord { i: 1 });
    expect_true!(one == OtherRecord { i: 1 });
    expect_true!(one != OtherRecord { i: 2 });

    // `Eq` and `Ord` only describe comparisons with `Self`.
    static_assertions::assert_not_impl_any!(CompareWithOtherRecord: Eq, Ord);
}

#[gtest]
fn test_spaceship_with_rhs_of_different_type_reversed() {
    // The reversed operators are declared separately, off by one relative to the members.
    let two = OtherRecord { i: 2 };

    expect_eq!(two.partial_cmp(&CompareWithOtherRecord { i: 1 }), Some(Ordering::Equal));
    expect_eq!(two.partial_cmp(&CompareWithOtherRecord { i: 2 }), Some(Ordering::Less));
    expect_true!(two == CompareWithOtherRecord { i: 1 });
    expect_true!(two > CompareWithOtherRecord { i: 0 });
}

#[gtest]
fn test_spaceship_with_rhs_of_different_type_without_eq() {
    // `PartialOrd<OtherRecord>` requires `PartialEq<OtherRecord>`, which isn't declared.
    static_assertions::assert_not_impl_any!(SpaceshipWithoutEq: PartialOrd<OtherRecord>);
    static_assertions::assert_not_impl_any!(SpaceshipWithoutEq: PartialEq<OtherRecord>);
}
