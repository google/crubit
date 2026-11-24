// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

rustversion::select! {
    since(2020-01-01) => {
        const ITEM_FROM_SELECT: i32 = 42;
        fn item_fn_from_select() -> i32 {
            ITEM_FROM_SELECT
        }
    }
    _ => {
        compile_error!("should not be selected");
    }
}

rustversion::select! {
    before(2020-01-01) => {
        compile_error!("should not be selected");
    }
    _ => {
        const FALLBACK_ITEM: i32 = 7;
    }
}

rustversion::select! {
    since(2020-01-01) => {
        const NO_WILDCARD_ITEM: i32 = 11;
    }
}

use googletest::prelude::*;

#[gtest]
fn test_select_items() {
    expect_eq!(item_fn_from_select(), 42);
    expect_eq!(FALLBACK_ITEM, 7);
    expect_eq!(NO_WILDCARD_ITEM, 11);
}

#[gtest]
fn test_select_expressions() {
    let local = 100;
    let unbraced = rustversion::select! {
        before(2020-01-01) => nonexistent_ident(),
        since(2020-01-01) => local + 23,
        _ => nonexistent_ident(),
    };
    expect_eq!(unbraced, 123);

    let braced = rustversion::select! {
        before(2020-01-01) => { nonexistent_ident() }
        _ => {
            let offset = 56;
            local * 4 + offset
        }
    };
    expect_eq!(braced, 456);

    let nested = rustversion::select! {
        since(2020-01-01) => rustversion::select! {
            before(2020-01-01) => nonexistent_ident(),
            _ => local + 1,
        },
        _ => nonexistent_ident(),
    };
    expect_eq!(nested, 101);
}

#[gtest]
fn test_select_items_in_fn() {
    rustversion::select! {
        before(2020-01-01) => {
            use nonexistent_crate::Nonexistent as Value;
        }
        _ => {
            const VALUE: i32 = 99;
        }
    }
    expect_eq!(VALUE, 99);
}
