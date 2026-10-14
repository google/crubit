// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::expect_that;
use googletest::gtest;
use googletest::matchers::eq;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[gtest]
fn test_apply_multiplier() {
    let result = cpp_api::ApplyMultiplier(Box::new(|x: i32| x * 2), 21);
    expect_that!(result, eq(42));
}

#[gtest]
fn test_invoke_once() {
    let called = Arc::new(AtomicBool::new(false));
    let called_clone = Arc::clone(&called);
    cpp_api::InvokeOnce(Box::new(move || {
        called_clone.store(true, Ordering::SeqCst);
    }));
    expect_that!(called.load(Ordering::SeqCst), eq(true));
}

#[gtest]
fn test_make_adder() {
    let adder: Box<dyn Fn(i32) -> i32 + Send + Sync> = cpp_api::MakeAdder();
    expect_that!(adder(41), eq(42));
}

#[gtest]
fn test_apply_alias() {
    let result = cpp_api::ApplyAlias(Box::new(|x: i32| x + 10), 32);
    expect_that!(result, eq(42));
}
