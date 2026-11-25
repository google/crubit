// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, gtest};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Mutex;
use std_function_ref::{
    CallStdFunctionRefFromTwoThreads, CallStdFunctionRefIntInt, CallStdFunctionRefMultipleTimes,
    CallStdFunctionRefNoexcept, CallStdFunctionRefVoidVoid,
};

#[gtest]
fn test_void_void() {
    let called = AtomicBool::new(false);
    CallStdFunctionRefVoidVoid(&|| {
        called.store(true, Ordering::Relaxed);
    });
    expect_eq!(called.load(Ordering::Relaxed), true);
}

#[gtest]
fn test_int_int_const() {
    let x = Mutex::new(0);
    let result = CallStdFunctionRefIntInt(
        &|i| {
            let mut guard = x.lock().unwrap();
            *guard += i;
            *guard
        },
        5,
    );
    expect_eq!(result, 5);
}

#[gtest]
fn test_noexcept() {
    let called = AtomicBool::new(false);
    CallStdFunctionRefNoexcept(&|| {
        called.store(true, Ordering::Relaxed);
    });
    expect_eq!(called.load(Ordering::Relaxed), true);
}

#[gtest]
fn test_multiple_times() {
    let result = CallStdFunctionRefMultipleTimes(&|i| i * 2);
    expect_eq!(result, 4);
}

#[gtest]
fn test_borrowed_stack_local() {
    let sum = AtomicI32::new(0);
    {
        let f = |i| {
            let prev = sum.fetch_add(i, Ordering::Relaxed);
            prev + i
        };
        CallStdFunctionRefMultipleTimes(&f);
    }
    expect_eq!(sum.load(Ordering::Relaxed), 2);
}

#[gtest]
fn test_from_two_threads() {
    let count = AtomicI32::new(0);
    let log = Mutex::new(Vec::new());
    CallStdFunctionRefFromTwoThreads(&|| {
        let prev = count.fetch_add(1, Ordering::SeqCst);
        log.lock().unwrap().push(prev);
    });
    expect_eq!(count.load(Ordering::SeqCst), 2);
    expect_eq!(log.lock().unwrap().len(), 2);
}

#[gtest]
fn test_fn_item() {
    fn double(x: i32) -> i32 {
        x * 2
    }
    let result = CallStdFunctionRefMultipleTimes(&double);
    expect_eq!(result, 4);
}

#[gtest]
fn test_pre_existing_dyn_fn() {
    let f: &(dyn Fn(i32) -> i32 + Sync) = &|x| x * 3;
    let result = CallStdFunctionRefMultipleTimes(f);
    expect_eq!(result, 9); // 1 * 3 = 3, 3 * 3 = 9
}
