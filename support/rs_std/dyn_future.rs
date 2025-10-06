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
    /// Returns whether `poll` has returned `true` or `discard` has been called.
    pub fn is_completed_or_discarded(&self) -> bool {
        self.0.is_none()
    }

    /// Discards the `DynFuture`, throwing away the underlying future.
    pub fn discard(&mut self) {
        self.0 = None;
    }

    /// Creates a new `DynFuture` from a Rust `Future`.
    pub fn from_future(future: impl Future<Output = ()> + Send + 'static) -> Self {
        // Safety: we know the DynFuture won't outlive 'static, because nothing does.
        unsafe { Self::from_future_unchecked(future) }
    }

    /// Creates a new `DynFuture` from a Rust `Future` that may bind references of arbitrary
    /// lifetimes.
    ///
    /// # Safety
    ///
    /// The caller is required to ensure that the `DynFuture` does not outlive the lifetime `'a`.
    //
    // TODO(b/449475779): get rid of this in favor of DynFuture being generic over lifetime.
    pub unsafe fn from_future_unchecked<'a>(future: impl Future<Output = ()> + Send + 'a) -> Self {
        // Smuggle the 'a future into a boxed 'static future.
        //
        // Safety: there is no difference in memory layout (lifetimes aren't used in generated
        // code), and it's on the user to ensure that the future embedded within the DynFuture
        // doesn't live too long.
        let raw = Box::into_raw(Box::new(future));
        let future = Box::from_raw(raw as *mut (dyn Future<Output = ()> + Send + 'static));

        Self(Some(Box::into_pin(future)))
    }

    /// Attempts to complete the future by polling it. Returns `true` if the future became ready.
    ///
    /// This function must not be called after `discard` or after `poll` returns true.
    ///
    /// # Safety
    ///
    /// `waker_ptr` must be a valid pointer to a `CppWaker`.
    pub unsafe fn poll(&mut self, waker_ptr: *const CppWaker) -> bool {
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
