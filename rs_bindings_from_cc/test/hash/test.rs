// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use absl_hashables::ns::HashedByFreeFunction;
use absl_hashables::{HashedByFriend, NotHashable, WrappedNotHashable};
use googletest::{expect_eq, expect_ne, gtest};
use static_assertions::{assert_impl_all, assert_not_impl_any};
use std::hash::{DefaultHasher, Hash, Hasher};

fn hash_of<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[gtest]
fn test_hidden_friend() {
    assert_impl_all!(HashedByFriend: Hash);
    expect_eq!(
        hash_of(&HashedByFriend { value: 1, ignored_by_hash: 2 }),
        hash_of(&HashedByFriend { value: 1, ignored_by_hash: 3 })
    );
    expect_ne!(
        hash_of(&HashedByFriend { value: 1, ignored_by_hash: 2 }),
        hash_of(&HashedByFriend { value: 4, ignored_by_hash: 2 })
    );
}

#[gtest]
fn test_free_function() {
    assert_impl_all!(HashedByFreeFunction: Hash);
    expect_eq!(
        hash_of(&HashedByFreeFunction { value: 1, ignored_by_hash: 2 }),
        hash_of(&HashedByFreeFunction { value: 1, ignored_by_hash: 3 })
    );
    expect_ne!(
        hash_of(&HashedByFreeFunction { value: 1, ignored_by_hash: 2 }),
        hash_of(&HashedByFreeFunction { value: 4, ignored_by_hash: 2 })
    );
}

#[gtest]
fn test_not_hashable() {
    assert_not_impl_any!(NotHashable: Hash);
    assert_not_impl_any!(WrappedNotHashable: Hash);
}
