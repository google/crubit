// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use std::sync::{Arc, Mutex};
use uses_anyinvocable::absl_functional_internal::{CallVoidVoid, ReturnIntVoid};

#[gtest]
fn test_call_void_void() {
    let called = Arc::new(Mutex::new(false));
    let called_clone = Arc::clone(&called);
    CallVoidVoid(Box::new(move || {
        assert_eq!(Arc::strong_count(&called_clone), 2);
        *called_clone.lock().unwrap() = true;
    }));
    assert_eq!(Arc::strong_count(&called), 1);
    expect_that!(*called.lock().unwrap(), eq(true));
}

#[gtest]
fn test_return_int_void() {
    let f = ReturnIntVoid();
    expect_that!(f(41), eq(42));
}
