// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Passing a Rust-native type which is not C++-movable to a C++ constructor
//! by value.
//!
//! Constructor thunks call the constructor through `construct_at`, which
//! forwards its arguments by reference, so passing `crubit::UnsafeTakeValue(x)`
//! would need a move constructor. They pass a
//! `crubit::UnsafeTakeValueOnConversion` instead, which relocates the value
//! directly into the constructor's parameter.
//!
//! Each test gives the value a clone of an `Arc<u8>`, and checks that the value
//! was not leaked: only the test's own reference remains.

use ctor::CtorNew;
use ctor_takes_nonmovable::crubit_test::{CtorTakesByValue, NonUnpinCtorTakesByValue};
use googletest::prelude::*;
use nonmovable::NonMovable;
use static_assertions::assert_not_impl_any;
use std::sync::Arc;

// `NonUnpinCtorTakesByValue` must be `!Unpin`, so that its constructor is a
// deferred `Ctor`.
assert_not_impl_any!(NonUnpinCtorTakesByValue: Unpin);

#[gtest]
fn ctor_takes_by_value() {
    let byte = Arc::new(42);

    let _ = CtorTakesByValue::from(NonMovable::new(byte.clone()));

    expect_eq!(Arc::strong_count(&byte), 1);
}

/// When the `Ctor` runs, the value is relocated into the constructor's
/// parameter, and dropped when C++ destroys it.
#[gtest]
fn non_unpin_ctor_takes_by_value() {
    let byte = Arc::new(42);

    let _ = ctor::emplace!(<NonUnpinCtorTakesByValue as CtorNew<NonMovable>>::ctor_new(
        NonMovable::new(byte.clone())
    ));

    expect_eq!(Arc::strong_count(&byte), 1);
}

/// The `Ctor` holds the value until it runs. If it never runs, dropping it
/// must still drop the value.
#[gtest]
fn non_unpin_ctor_dropped_without_running() {
    let byte = Arc::new(42);

    let ctor =
        <NonUnpinCtorTakesByValue as CtorNew<NonMovable>>::ctor_new(NonMovable::new(byte.clone()));
    expect_eq!(Arc::strong_count(&byte), 2);
    drop(ctor);

    expect_eq!(Arc::strong_count(&byte), 1);
}
