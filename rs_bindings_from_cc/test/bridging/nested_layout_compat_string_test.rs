// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests for nesting a layout-compatible `std::string` inside another type.
//!
//! A *bridged* type is encoded into a byte buffer, which requires every part of it to be
//! Rust-movable. A layout-compatible `std::string` is not, so Crubit must refuse to bind the
//! individual functions which would need that, rather than emitting generated code which does
//! not compile.
//!
//! A *layout-compatible* container has no such restriction, because it holds its payload
//! in place. This test contrasts the two.
//!
//! The functions which cannot be bound have no bindings to call, so merely building this test is
//! the assertion that they cost only themselves and not the whole target.

use ctor::emplace;
use googletest::matchers::{anything, err as is_err, ok as is_ok};
use googletest::{expect_eq, expect_that, expect_true, gtest};
use nested_layout_compat_string_lib::{
    MakeLayoutCompatString, MakeLayoutCompatStringOrError, MaybeMakeRustMovable, RustMovable,
};

#[gtest]
fn test_bare_layout_compat_string_is_bound() {
    let s = emplace!(MakeLayoutCompatString());
    expect_eq!(s.as_slice(), &b"hello"[..]);
}

#[gtest]
fn test_layout_compat_string_in_status_or_is_bound() {
    // `absl::StatusOr` is layout-compatible under `CRUBIT_NEW_STATUS`, so unlike `std::optional`
    // it can carry a layout-compatible `std::string`.
    let ok = emplace!(MakeLayoutCompatStringOrError(true));
    expect_that!(&*ok, is_ok(anything()));

    let err = emplace!(MakeLayoutCompatStringOrError(false));
    expect_that!(&*err, is_err(anything()));
}

#[gtest]
fn test_rust_movable_payload_in_optional_is_bound() {
    // A user-defined Rust-movable type can still be bridged inside a `std::optional`.
    let engaged: Option<RustMovable> = MaybeMakeRustMovable(true);
    let Some(value) = engaged else {
        panic!("expected an engaged optional");
    };
    expect_eq!(value.x, 7);
    expect_true!(value.y);

    expect_true!(MaybeMakeRustMovable(false).is_none());
}
