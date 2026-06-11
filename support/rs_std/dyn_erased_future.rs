// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! A Rust wrapper around a type-erased `Future`, designed to bridge asynchronous Rust
//! tasks with C++ coroutines via a unified FFI interface.

#![deny(missing_docs, unsafe_op_in_unsafe_fn)]

use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::ptr;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

mod sealed {
    /// Sealed trait to ensure that only `Future`s can implement `ErasedFuture`.
    pub trait Sealed {}
}
impl<T, F: Future<Output = T> + Send> sealed::Sealed for F {}

/// A type-erased version of [`Future`] that writes its output into a caller-provided raw pointer.
///
/// This trait is designed to be object-safe (`dyn ErasedFuture`), allowing futures of arbitrary
/// output types to be stored behind a single unified FFI interface. It uses a sealed trait pattern
/// to ensure it cannot be implemented outside this module.
pub trait ErasedFuture: Send + sealed::Sealed {
    /// Polls the underlying future.
    ///
    /// If the future completes, its output is written directly into the memory location pointed to by `out`,
    /// and `Poll::Ready(())` is returned. If the future is pending, `out` remains untouched, and `Poll::Pending`
    /// is returned.
    ///
    /// # Safety
    ///
    /// If the underlying future's output type `T` contains data (`size_of::<T>() > 0`), `out` must be a valid,
    /// `align_of::<T>()`-aligned pointer to uninitialized memory suitable for writing a value of type `T`.
    /// If `T` is zero-sized, `out` is ignored.
    unsafe fn erased_poll(self: Pin<&mut Self>, cx: &mut Context<'_>, out: *mut ()) -> Poll<()>;
}

impl<T, F: Future<Output = T> + Send> ErasedFuture for F {
    unsafe fn erased_poll(self: Pin<&mut Self>, cx: &mut Context<'_>, out: *mut ()) -> Poll<()> {
        self.poll(cx).map(|val| {
            if size_of::<T>() > 0 {
                // SAFETY: `out` is valid for writes of `T` and properly aligned by the caller's preconditions.
                unsafe { ptr::write(out as *mut T, val) };
            }
        })
    }
}

/// A type-erased, FFI-compatible container for a pinned Rust [`Future`].
///
/// Because `dyn ErasedFuture` is a trait object, this is represented as a 16-byte fat pointer
/// (data pointer + vtable pointer). `Option` leverages the null pointer optimization to represent
/// the empty/completed state without additional memory overhead.
pub type DynErasedFuture<'a> = Option<Pin<Box<dyn ErasedFuture + Send + 'a>>>;

const _: () = {
    assert!(size_of::<DynErasedFuture<'_>>() == 16);
    assert!(align_of::<DynErasedFuture<'_>>() == 8);
};

/// Initializes a new, empty `DynErasedFuture` in the provided storage.
///
/// # Safety
///
/// `storage` must point to a valid, properly aligned, uninitialized memory location
/// large enough to hold a `DynErasedFuture` (16 bytes).
#[unsafe(no_mangle)]
unsafe extern "C" fn rs_std_dyn_erased_future_init(storage: *mut DynErasedFuture<'_>) {
    // SAFETY: `storage` is valid, aligned, and uninitialized by the function's preconditions.
    unsafe { ptr::write(storage, None) };
}

/// Drops the `DynErasedFuture`, deallocating the underlying box if present.
///
/// # Safety
///
/// `storage` must point to a valid, properly aligned, and fully initialized `DynErasedFuture`.
#[unsafe(no_mangle)]
unsafe extern "C" fn rs_std_dyn_erased_future_drop(storage: *mut DynErasedFuture<'_>) {
    // SAFETY: `storage` is valid, aligned, and initialized by the function's preconditions.
    unsafe { ptr::drop_in_place(storage) };
}

/// Returns `true` if the future is empty (either completed or discarded).
///
/// # Safety
///
/// `storage` must point to a valid, properly aligned, and fully initialized `DynErasedFuture`.
#[unsafe(no_mangle)]
unsafe extern "C" fn rs_std_dyn_erased_future_is_completed_or_discarded(
    storage: *const DynErasedFuture<'_>,
) -> bool {
    // SAFETY: `storage` is valid, aligned, and initialized by the function's preconditions.
    unsafe { (*storage).is_none() }
}

/// Discards the in-flight future, deallocating it and leaving the storage in an empty state.
///
/// # Safety
///
/// `storage` must point to a valid, properly aligned, and fully initialized `DynErasedFuture`.
#[unsafe(no_mangle)]
unsafe extern "C" fn rs_std_dyn_erased_future_discard(storage: *mut DynErasedFuture<'_>) {
    // SAFETY: `storage` is valid, aligned, and initialized by the function's preconditions.
    unsafe {
        *storage = None;
    }
}

/// Polls the type-erased future, using the provided C++ waker and out pointer.
///
/// If the future completes, its output is written to `out`, the underlying box is deallocated,
/// `storage` is set to `None`, and `true` is returned. If the future is pending, `false` is returned.
///
/// # Safety
///
/// * `storage` must point to a valid, properly aligned, and fully initialized `DynErasedFuture`. It must
///   not be accessed concurrently for the duration of this call.
/// * `waker_ptr` must be a valid pointer to a C++ waker managed by `rs_std::Waker`.
/// * `out` must be a valid, properly aligned pointer to uninitialized memory suitable for writing a
///   value of the future's output type `T` (if `T` is not zero-sized).
#[unsafe(no_mangle)]
unsafe extern "C" fn rs_std_dyn_erased_future_poll(
    storage: *mut DynErasedFuture<'_>,
    waker_ptr: UnitPtr,
    out: *mut (),
) -> bool {
    // SAFETY: `storage` is valid and unaliased by the function's preconditions.
    let dyn_erased_future: &mut DynErasedFuture<'_> = unsafe { &mut *storage };
    let future = dyn_erased_future
        .as_mut()
        .expect("Attempted to `poll` a completed or discarded `DynErasedFuture`.")
        .as_mut();

    // Wrap the `Waker` in `ManuallyDrop` to avoid decrementing the C++ waker refcount upon exiting this function.
    // SAFETY: `waker_ptr` is a valid C++ waker pointer compatible with `RS_STD_WAKER_VTABLE`.
    let waker = ManuallyDrop::new(unsafe { Waker::new(waker_ptr, RS_STD_WAKER_VTABLE) });
    let mut cx = Context::from_waker(&waker);
    // SAFETY: `out` satisfies the safety contract of `erased_poll` by the function's preconditions.
    if unsafe { future.erased_poll(&mut cx, out) }.is_ready() {
        *dyn_erased_future = None;
        true
    } else {
        false
    }
}

/// A type-erased pointer representing the C++ waker instance.
///
/// This matches the raw pointer type expected by [`RawWaker`] and [`RawWakerVTable`].
type UnitPtr = *const ();

// External C++ waker virtual table functions defined in `waker.cc`.
unsafe extern "C" {
    fn rs_std_waker_clone(_: UnitPtr) -> UnitPtr;
    fn rs_std_waker_wake_and_destroy(_: UnitPtr);
    fn rs_std_waker_wake_by_ref(_: UnitPtr);
    fn rs_std_waker_drop(_: UnitPtr);
}

unsafe fn rs_std_waker_clone_wrapper(obj: UnitPtr) -> RawWaker {
    // SAFETY: Forwarding to the C++ FFI waker clone implementation.
    RawWaker::new(unsafe { rs_std_waker_clone(obj) }, RS_STD_WAKER_VTABLE)
}

unsafe fn rs_std_waker_wake_and_destroy_wrapper(obj: UnitPtr) {
    // SAFETY: Forwarding to the C++ FFI waker wake_and_destroy implementation.
    unsafe { rs_std_waker_wake_and_destroy(obj) }
}

unsafe fn rs_std_waker_wake_by_ref_wrapper(obj: UnitPtr) {
    // SAFETY: Forwarding to the C++ FFI waker wake_by_ref implementation.
    unsafe { rs_std_waker_wake_by_ref(obj) }
}

unsafe fn rs_std_waker_drop_wrapper(obj: UnitPtr) {
    // SAFETY: Forwarding to the C++ FFI waker drop implementation.
    unsafe { rs_std_waker_drop(obj) }
}

static RS_STD_WAKER_VTABLE: &RawWakerVTable = &RawWakerVTable::new(
    rs_std_waker_clone_wrapper,
    rs_std_waker_wake_and_destroy_wrapper,
    rs_std_waker_wake_by_ref_wrapper,
    rs_std_waker_drop_wrapper,
);
