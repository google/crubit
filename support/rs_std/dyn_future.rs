// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! A Rust wrapper around a type-erased `Future` designed for use from C++.

#![deny(missing_docs)]

use cpp_waker::rs_std::CppWaker;
use std::future::Future;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::task::{RawWaker, RawWakerVTable, Waker};

/// A type-erased wrapper for Rust `Future` types that exposes a Crubit-compatible API.
#[derive(Default)]
pub struct DynFuture(Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>);

impl DynFuture {
    /// Returns whether `try_complete` has returned `true` or `discard` has been called.
    pub fn is_completed_or_discarded(&self) -> bool {
        self.0.is_none()
    }

    /// Discards the `DynFuture`.
    pub fn discard(&mut self) {
        self.0 = None;
    }

    /// Creates a new `DynFuture` from a Rust `Future`.
    pub fn from_future(future: impl Future<Output = ()> + Send + 'static) -> Self {
        Self(Some(Box::pin(future)))
    }

    /// Attempts to complete the future by polling it. Returns `true` if the future was completed.
    ///
    /// This function must not be called on a completed or discarded `DynFuture`.
    ///
    /// # Safety
    ///
    /// `waker_ptr` must be a valid pointer to a `CppWaker`.
    pub unsafe fn try_complete(&mut self, waker_ptr: *const CppWaker) -> bool {
        // Wrap in `ManuallyDrop` because we don't want to decrease the refcount when this function
        // completes.
        // Safety: the `wakeup_object` is managed by `wakeup_vtable` as required by the function.
        let waker =
            ManuallyDrop::new(unsafe { Waker::new(waker_ptr as UnitPtr, RS_STD_CPP_WAKER_VTABLE) });
        let future =
            self.0.as_mut().expect("Attempted to `poll` a completed or discarded `DynFuture`.");
        let mut cx = std::task::Context::from_waker(&waker);
        if future.as_mut().poll(&mut cx).is_ready() {
            self.0 = None;
            true
        } else {
            false
        }
    }
}

// A type-erased pointer to a `CppWaker`.
// This is necessary in order to match the signatures required by `RawWakerVTable`.
type UnitPtr = *const ();

// Wakeup functions for `RustFuture` defined in `from_rust.cc`.
unsafe extern "C" {
    fn rs_std_cpp_waker_clone(_: UnitPtr) -> UnitPtr;
    fn rs_std_cpp_waker_wake_and_destroy(_: UnitPtr);
    fn rs_std_cpp_waker_wake_by_ref(_: UnitPtr);
    fn rs_std_cpp_waker_drop(_: UnitPtr);
}

unsafe fn rs_std_cpp_waker_clone_wrapper(obj: UnitPtr) -> RawWaker {
    RawWaker::new(rs_std_cpp_waker_clone(obj), RS_STD_CPP_WAKER_VTABLE)
}

unsafe fn rs_std_cpp_waker_wake_and_destroy_wrapper(obj: UnitPtr) {
    rs_std_cpp_waker_wake_and_destroy(obj)
}

unsafe fn rs_std_cpp_waker_wake_by_ref_wrapper(obj: UnitPtr) {
    rs_std_cpp_waker_wake_by_ref(obj)
}

unsafe fn rs_std_cpp_waker_drop_wrapper(obj: UnitPtr) {
    rs_std_cpp_waker_drop(obj)
}

static RS_STD_CPP_WAKER_VTABLE: &RawWakerVTable = &RawWakerVTable::new(
    rs_std_cpp_waker_clone_wrapper,
    rs_std_cpp_waker_wake_and_destroy_wrapper,
    rs_std_cpp_waker_wake_by_ref_wrapper,
    rs_std_cpp_waker_drop_wrapper,
);
