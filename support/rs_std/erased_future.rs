// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! erased_future provides `ErasedFuture`, a type erased version of the `Future` trait that is
//! dyn-compatible.
//!
//! Note that this crate is intended to be used as a Crubit support library for cc_bindings_from_rs,
//! and not intended for direct use by Rust library authors.

use std::pin::Pin;
use std::ptr;
use std::task::{Context, Poll};

/// A type-erased version of [`Future`] that writes its output into a caller-provided raw pointer.
///
/// This trait is designed to be type erased, allowing futures of arbitrary output types to be
/// stored behind a single unified FFI interface. It has a blanket implementation for all `Future`s,
/// and uses a sealed trait pattern to ensure it cannot be implemented for anything else.
pub trait ErasedFuture: Send + sealed::Sealed {
    /// Polls the underlying future.
    ///
    /// If the future completes, its output is written directly into the memory location pointed to
    /// by `out`, and `Poll::Ready(())` is returned. If the future is pending, `out` remains
    /// untouched, and `Poll::Pending` is returned.
    ///
    /// # Safety
    ///
    /// If the underlying future's output type `T` contains data (`size_of::<T>() > 0`), `out` must
    /// be a valid, `align_of::<T>()`-aligned pointer to uninitialized memory suitable for writing a
    /// value of type `T`.  If `T` is zero-sized, `out` is ignored.
    ///
    /// Polling after completion: unlike `Future`, which is technically safe to poll after
    /// completion, it is considered unsafe to poll an ErasedFuture after completion.
    unsafe fn poll_erased(self: Pin<&mut Self>, cx: &mut Context<'_>, out: *mut ()) -> Poll<()>;
}

impl<T, F: Future<Output = T> + Send> ErasedFuture for F {
    unsafe fn poll_erased(self: Pin<&mut Self>, cx: &mut Context<'_>, out: *mut ()) -> Poll<()> {
        self.poll(cx).map(|val| {
            if size_of::<T>() > 0 {
                // SAFETY: `out` is valid for writes of `T` and properly aligned by the caller's
                // preconditions.
                unsafe { ptr::write(out as *mut T, val) };
            }
        })
    }
}

mod sealed {
    /// Sealed trait to ensure that only `Future`s can implement `ErasedFuture`.
    pub trait Sealed {}

    impl<T, F: Future<Output = T> + Send> Sealed for F {}
}
