// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::CtorNew;
use googletest::{expect_eq, gtest};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Mutex;
use uses_function_ref::{
    CallAlias, CallFromTwoThreads, CallIntIntMultiple, CallVoidVoid, CallWithNonTrivial,
    CallbackRunner, CreateNonTrivial, PinnedCallbackRunner,
};

#[gtest]
fn test_call_void_void() {
    let called = AtomicBool::new(false);
    CallVoidVoid(&|| {
        called.store(true, Ordering::Relaxed);
    });
    expect_eq!(called.load(Ordering::Relaxed), true);
}

#[gtest]
fn test_call_int_int_multiple() {
    let acc = Mutex::new(0);
    let sum = CallIntIntMultiple(
        &|i| {
            let mut guard = acc.lock().unwrap();
            *guard += i;
            *guard
        },
        5,
    );
    // i=0 -> 0, i=1 -> 1, i=2 -> 3, i=3 -> 6, i=4 -> 10; sum = 20.
    expect_eq!(sum, 20);
}

#[gtest]
fn test_borrowed_stack_local() {
    let offset = 10;
    let total = AtomicI32::new(0);
    {
        let f = |i| {
            total.fetch_add(i + offset, Ordering::Relaxed);
            i + offset
        };
        let sum = CallIntIntMultiple(&f, 3);
        expect_eq!(sum, 10 + 11 + 12);
    }
    expect_eq!(total.load(Ordering::Relaxed), 33);
}

#[gtest]
fn test_call_from_two_threads() {
    let count = AtomicI32::new(0);
    let log = Mutex::new(Vec::new());
    CallFromTwoThreads(&|| {
        let prev = count.fetch_add(1, Ordering::SeqCst);
        log.lock().unwrap().push(prev);
    });
    expect_eq!(count.load(Ordering::SeqCst), 2);
    expect_eq!(log.lock().unwrap().len(), 2);
}

#[gtest]
fn test_call_with_non_trivial() {
    let val = CreateNonTrivial(42);
    let res = CallWithNonTrivial(&|nt| nt.value + 1, val);
    expect_eq!(res, 43);
}

#[gtest]
fn test_call_alias() {
    expect_eq!(CallAlias(&|i| i + 1, 41), 42);
}

#[gtest]
fn test_constructors_with_function_ref() {
    let unpin_calls = AtomicI32::new(0);
    let f = || {
        unpin_calls.fetch_add(1, Ordering::Relaxed);
    };
    let runner = CallbackRunner::from(&f as &_);
    expect_eq!(runner.calls, 1);
    expect_eq!(unpin_calls.load(Ordering::Relaxed), 1);

    let pinned_calls = AtomicI32::new(0);
    let g = || {
        pinned_calls.fetch_add(1, Ordering::Relaxed);
    };
    let pinned = ctor::emplace!(PinnedCallbackRunner::ctor_new(&g as &_));
    expect_eq!(pinned.calls, 1);
    expect_eq!(pinned_calls.load(Ordering::Relaxed), 1);
}
