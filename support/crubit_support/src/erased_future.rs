// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! erased_future provides `ErasedFuture`, a type erased version of the `Future` trait that is
//! dyn-compatible.
//!
//! Note that this crate is intended to be used as a Crubit support library for cc_bindings_from_rs,
//! and not intended for direct use by Rust library authors.

use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::ptr;
use core::task::{Context, Poll};

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
    /// In either case, ownership of the output is transferred to the caller: `poll_erased` never
    /// drops it. This includes zero-sized output types, for which nothing is written to `out`.
    ///
    /// # Safety
    ///
    /// If the underlying future's output type `T` contains data (`size_of::<T>() > 0`), `out` must
    /// be a valid, `align_of::<T>()`-aligned pointer to uninitialized memory suitable for writing a
    /// value of type `T`.  If `T` is zero-sized, `out` is ignored (and may be null).
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
            } else {
                // `out` may be null for zero-sized `T`, so we can't write through it. Forget `val`
                // instead of dropping it, because ownership of the output is transferred to the
                // caller just like in the non-zero-sized case.
                mem::forget(val);
            }
        })
    }
}

mod sealed {
    use core::future::Future;

    /// Sealed trait to ensure that only `Future`s can implement `ErasedFuture`.
    pub trait Sealed {}

    impl<T, F: Future<Output = T> + Send> Sealed for F {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;
    use core::future;
    use core::sync::atomic::{AtomicUsize, Ordering};
    use core::task::Waker;
    use googletest::{expect_eq, gtest};

    static ZST_DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

    /// A zero-sized type that counts how many times it is dropped.
    struct ZstDropCounter;

    impl Drop for ZstDropCounter {
        fn drop(&mut self) {
            ZST_DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[gtest]
    fn poll_erased_ready_zero_sized_with_drop_transfers_ownership() {
        const { assert!(size_of::<ZstDropCounter>() == 0) };
        let mut fut = Box::pin(future::ready(ZstDropCounter));
        let mut cx = Context::from_waker(Waker::noop());
        // SAFETY: the output is zero-sized, so `out` is ignored.
        let poll = unsafe { fut.as_mut().poll_erased(&mut cx, ptr::null_mut()) };
        expect_eq!(poll, Poll::Ready(()));
        // Ownership of the output was transferred to the caller, so `poll_erased` must not drop it.
        expect_eq!(ZST_DROP_COUNT.load(Ordering::SeqCst), 0);
        drop(fut);
        expect_eq!(ZST_DROP_COUNT.load(Ordering::SeqCst), 0);

        // SAFETY: `poll_erased` transferred ownership of the zero-sized output to us, and reading a
        // zero-sized value through a dangling, well-aligned pointer is valid.
        let output = unsafe { ptr::read(ptr::NonNull::<ZstDropCounter>::dangling().as_ptr()) };
        drop(output);
        expect_eq!(ZST_DROP_COUNT.load(Ordering::SeqCst), 1);
    }
}
