// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests for ctor macros, focusing on their use from external crates.
//!
//! To assist testing hygiene, this module must not add any imports (e.g. for
//! Pin, etc.), and must shadow std and ctor with "incorrect" modules to
//! ensure that the macros do not depend on that aspect of the caller.
#![cfg(test)]
// Allow unused imports so that we can produce pathological aliases.
#![allow(unused_imports)]

use googletest::prelude::*;

// pathological shadowed names: shadow important modules that the macros use.
mod std {}
mod ctor {}

/// Expand emplace!{let ...} to test for hygiene.
#[gtest]
fn test_emplace_stmt_hygiene() {
    ::ctor::emplace! {
        let _x1 = 0;
        let mut _x2 = 0;
        let _x3 : ::std::pin::Pin<&mut u32> = 0;
        let mut _x4 : ::std::pin::Pin<&mut u32> = 0;
    }
}

/// Expand emplace!(expr) to test for hygiene.
#[gtest]
fn test_emplace_expr_hygiene() {
    let _ = ::ctor::emplace!(4);
}

/// Expand ctor!{Struct{...}} to test for hygiene.
#[gtest]
fn test_ctor_struct_hygiene() {
    struct Struct {
        x: i32,
    }
    unsafe impl ::ctor::RecursivelyPinned for Struct {
        type CtorInitializedFields = Self;
    }
    let _ = ::ctor::ctor! {Struct { x: 0 }};
}

/// Expand ctor!{TupleStruct(...)} to test for hygiene.
#[gtest]
fn test_ctor_tuple_struct_hygiene() {
    struct TupleStruct(i32);
    unsafe impl ::ctor::RecursivelyPinned for TupleStruct {
        type CtorInitializedFields = Self;
    }
    let _ = ::ctor::ctor! {TupleStruct(0)};
}
