// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use erased_future::*;
use googletest::{expect_eq, gtest};
use std::future;
use std::ptr;
use std::task::{Context, Poll, Waker};

#[gtest]
fn poll_erased_ready_with_value() {
    let mut fut = Box::pin(future::ready(42u32));
    let mut out = 0u32;
    let mut cx = Context::from_waker(Waker::noop());
    // SAFETY: `out` is a valid pointer to a `u32`.
    let poll = unsafe { fut.as_mut().poll_erased(&mut cx, &mut out as *mut u32 as *mut ()) };
    expect_eq!(poll, Poll::Ready(()));
    expect_eq!(out, 42);
}

#[gtest]
fn poll_erased_pending_with_value() {
    let mut fut = Box::pin(future::pending::<u32>());
    let mut out = 0u32;
    let mut cx = Context::from_waker(Waker::noop());
    // SAFETY: `out` is a valid pointer to a `u32`.
    let poll = unsafe { fut.as_mut().poll_erased(&mut cx, &mut out as *mut u32 as *mut ()) };
    expect_eq!(poll, Poll::Pending);
    expect_eq!(out, 0);
}

#[gtest]
fn poll_erased_ready_zero_sized() {
    let mut fut = Box::pin(future::ready(()));
    let mut cx = Context::from_waker(Waker::noop());
    // SAFETY: the output is zero-sized, so `out` is ignored.
    let poll = unsafe { fut.as_mut().poll_erased(&mut cx, ptr::null_mut()) };
    expect_eq!(poll, Poll::Ready(()));
}
