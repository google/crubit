// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Passing a Rust-native type which is not C++-movable to C++ by value.
//!
//! `NonMovable` has no C++ move constructor, so it can only be handed to C++
//! by relocation. Crubit used to initialize by-value parameters with
//! `std::move`, so these calls could not be bound at all. Now the generated
//! thunks use `crubit::UnsafeTakeValue`, which relocates the value through its
//! `UnsafeRelocateTag` constructor.
//!
//! Each test gives the value a clone of an `Arc<u8>`, and checks that the value
//! was dropped once C++ is done with it: only the test's own reference remains.

use googletest::prelude::*;
use nonmovable::NonMovable;
use std::sync::Arc;
use takes_nonmovable::crubit_test::{Receiver, TakesByValue, TakesTwoByValue};

#[gtest]
fn takes_by_value() {
    let byte = Arc::new(42);

    expect_eq!(TakesByValue(NonMovable::new(byte.clone())), 42);

    expect_eq!(Arc::strong_count(&byte), 1);
}

#[gtest]
fn takes_multiple_by_value() {
    let a = Arc::new(40);
    let b = Arc::new(2);

    expect_eq!(TakesTwoByValue(NonMovable::new(a.clone()), NonMovable::new(b.clone())), 42);

    expect_eq!(Arc::strong_count(&a), 1);
    expect_eq!(Arc::strong_count(&b), 1);
}

#[gtest]
fn member_function_takes_by_value() {
    let byte = Arc::new(42);

    expect_eq!(Receiver::default().TakesByValue(NonMovable::new(byte.clone())), 42);

    expect_eq!(Arc::strong_count(&byte), 1);
}
