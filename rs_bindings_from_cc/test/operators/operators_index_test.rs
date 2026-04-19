// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::{emplace, CtorNew};
use googletest::prelude::*;
use operator::CcIndexMut;
use operators_index::crubit::test::*;
use std::os::raw::c_uint;
use std::pin::Pin;

/// When both the container and item are `Unpin` we do not need CcIndex or
/// CcIndexMut at all and can use regular Rust indexing syntax (`[]`).
#[gtest]
fn test_container_unpin_item_unpin_indexing() {
    let mut c = ContainerUnpinItemUnpin::default();
    let index: c_uint = 0;

    assert_eq!(c[index].value, 0);

    c[index].value = 10;

    assert_eq!(c[index].value, 10);
}

/// When the container is `!Unpin`, but the item is still `Unpin`, we can use
/// regular indexing syntax if we get a mutable reference to the item first.
#[gtest]
fn test_container_non_unpin_item_unpin_indexing() {
    let mut c = emplace!(ContainerNonUnpinItemUnpin::ctor_new(()));
    let index: c_uint = 0;

    assert_eq!(c[index].value, 0);

    c.as_mut()[index].value = 30;

    assert_eq!(c[index].value, 30);
}

/// When the item is `!Unpin` in the following two tests, we can use regular
/// syntax for reading values, but we must unsafely unwrap the pin and use
/// `cc_index_mut` to write data.
#[gtest]
fn test_container_unpin_item_non_unpin_indexing() {
    let mut c = emplace!(ContainerUnpinItemNonUnpin::ctor_new(()));
    let index: c_uint = 0;

    assert_eq!(c[index].value, 0);

    let item_mut = c.as_mut().cc_index_mut(index);

    // SAFETY: We are only mutating the `value` field and not moving the
    // `ItemNonUnpin` out of its pinned location, so the pinning invariant
    // is upheld.
    let item_mut_ref: &mut ItemNonUnpin = unsafe { Pin::get_unchecked_mut(item_mut) };

    item_mut_ref.value = 20;

    assert_eq!(c[index].value, 20);
}

#[gtest]
fn test_container_non_unpin_item_non_unpin_indexing() {
    let mut c = emplace!(ContainerNonUnpinItemNonUnpin::ctor_new(()));
    let index: c_uint = 0;

    assert_eq!(c[index].value, 0);

    let item_mut = c.as_mut().cc_index_mut(index);

    // SAFETY: We are only mutating the `value` field and not moving the
    // `ItemNonUnpin` out of its pinned location, so the pinning invariant
    // is upheld.
    let item_mut_ref: &mut ItemNonUnpin = unsafe { Pin::get_unchecked_mut(item_mut) };

    item_mut_ref.value = 40;

    assert_eq!(c[index].value, 40);
}
