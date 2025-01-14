// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bumpalo::Bump;
use googletest::{expect_eq, expect_ne, gtest};
use interner::Interner;

#[gtest]
fn test_intern_does_not_change_value() {
    let bump = Bump::default();
    let interner = Interner::new(&bump);
    let values = [2, 4, 6, 0, 1, 2];
    for value in values {
        let interned = interner.intern(value);
        expect_eq!(**interned, value);
    }
}

#[gtest]
fn test_multiple_intern_calls_with_same_value_return_same_reference() {
    let bump = Bump::default();
    let interner = Interner::new(&bump);
    let entries = [1, 2, 3, 4, 5];
    let interned = entries.map(|v| interner.intern(v));
    let interned_again = entries.map(|v| interner.intern(v));
    for (first, second) in interned.iter().zip(interned_again.iter()) {
        expect_eq!(**first as *const i32, **second as *const i32);
        expect_eq!(first, second);
    }
}

#[gtest]
fn test_intern_with_different_values_are_not_equal() {
    let bump = Bump::default();
    let interner = Interner::new(&bump);
    let first = interner.intern(1);
    let second = interner.intern(2);
    expect_ne!(first, second);
}

#[gtest]
fn test_multiple_intern_calls_with_same_slice_value_return_same_reference() {
    let bump = Bump::default();
    let interner = Interner::<[i32]>::new(&bump);

    let data = [1, 2, 3, 1, 2];
    expect_eq!(interner.intern_slice(&data[0..2]), interner.intern_slice(&data[3..5]));
}

#[gtest]
fn test_intern_slice_with_same_data_but_different_length_returns_different_reference() {
    let bump = Bump::default();
    let interner = Interner::<[i32]>::new(&bump);

    let data = [1, 2, 3, 1, 2];
    expect_ne!(interner.intern_slice(&data[0..2]), interner.intern_slice(&data[0..3]));
}

#[gtest]
fn test_multiple_intern_calls_with_same_str_return_same_reference() {
    let bump = Bump::default();
    let interner = Interner::<str>::new(&bump);
    let matches: Vec<&str> = "aaaabcxxxabcyyyabcdef".matches("abc").collect();
    for match_ in matches {
        let interned = interner.intern_str(match_);
        expect_eq!(interner.intern_str("abc"), interned);
    }
}
