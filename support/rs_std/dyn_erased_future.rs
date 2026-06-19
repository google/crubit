// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! The Rust implementation of the C++ `DynErasedFuture` class. This library is a Crubit support
//! library, meaning it provides functionality that generated code from cc_bindings_from_rs can
//! depend on at runtime. As such, it is not intended for direct use by authors of Rust code.
//!
//! As a Crubit support library, this library cannot depend on Crubit itself. Therefore, it relies
//! on exporting handwritten extern "C" functions to be used directly by `dyn_erased_future.h`.

#![deny(missing_docs, unsafe_op_in_unsafe_fn)]

use erased_future::ErasedFuture;
use std::future;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::ptr;
use std::task::{Context, RawWaker, RawWakerVTable, Waker};

/// A type-erased, FFI-compatible container for a pinned Rust [`Future`].
///
/// This type isn't intended to by used by Rust users directly. Instead, it's used in generated code
/// from cc_bindings_from_rs to poll and drop futures from C++.
#[repr(transparent)]
pub struct DynErasedFuture<'a>(Pin<Box<dyn ErasedFuture + Send + 'a>>);

// The correctness of DynErasedFuture relies on having the same memory layout as the C++
// definition. So we verify that on 64-bit platforms, the size is 16 bytes and the alignment is
// 8 bytes, and on 32-bit platforms, the size is 8 bytes and the alignment is 4 bytes.
#[cfg(target_pointer_width = "64")]
const _: () = {
    assert!(size_of::<DynErasedFuture<'_>>() == 16);
    assert!(align_of::<DynErasedFuture<'_>>() == 8);
};

#[cfg(target_pointer_width = "32")]
const _: () = {
    assert!(size_of::<DynErasedFuture<'_>>() == 8);
    assert!(align_of::<DynErasedFuture<'_>>() == 4);
};

#[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
compile_error!("Only 64-bit and 32-bit platforms are supported.");

impl<'a> DynErasedFuture<'a> {
    /// Creates a new `DynErasedFuture` from the given future.
    ///
    /// This function is only intended to be used in generated Rust code from cc_bindings_from_rs.
    pub fn new<T>(future: impl Future<Output = T> + Send + 'a) -> Self {
        DynErasedFuture(Box::pin(future))
    }
}

/// Initializes a new, empty `DynErasedFuture` in the provided storage.
///
/// This function is used by `dyn_erased_future.h` to initialize the `DynErasedFuture` in place.
///
/// # Safety
///
/// `dyn_erased_future` must point to a valid, properly aligned, uninitialized memory location large
/// enough to hold a `DynErasedFuture` (16 bytes).
#[unsafe(no_mangle)]
unsafe extern "C" fn rs_std_dyn_erased_future_init(dyn_erased_future: *mut DynErasedFuture<'_>) {
    // SAFETY: `dyn_erased_future` is valid, aligned, and uninitialized by the function's preconditions.
    unsafe { ptr::write(dyn_erased_future, DynErasedFuture(Box::pin(future::pending::<()>()))) };
}

// The following functions are used in `dyn_erased_future.h`, and exposed by the linker.
// Therefore, they do not need to be public here.

/// Drops the `DynErasedFuture`, deallocating the underlying box.
///
/// This function is used by `dyn_erased_future.h` to drop the `DynErasedFuture` in place.
///
/// # Safety
///
/// `dyn_erased_future` must be a valid mutable reference that won't be used again.
#[unsafe(no_mangle)]
unsafe extern "C" fn rs_std_dyn_erased_future_drop(dyn_erased_future: *mut DynErasedFuture<'_>) {
    // SAFETY: `dyn_erased_future` is valid, aligned, and initialized by the function's preconditions.
    unsafe { ptr::drop_in_place(dyn_erased_future) };
}

/// Polls the type-erased future, using the provided C++ waker and out pointer.
///
/// If the future completes, its output is written to `out` and `true` is returned. If the future is
/// pending, `false` is returned.
///
/// This function is used by `dyn_erased_future.h` to poll the `DynErasedFuture`.
///
/// # Safety
///
/// * `dyn_erased_future` must be a valid mutable reference.
/// * `waker_ptr` must be a valid pointer to a C++ waker managed by `rs_std::Waker`.
/// * `out` must be a valid, properly aligned pointer to uninitialized memory suitable for writing a
///   value of the future's output type `T` (if `T` is not zero-sized).
/// * The `DynErasedFuture` must not be polled again after this call returns `true`.
#[unsafe(no_mangle)]
unsafe extern "C" fn rs_std_dyn_erased_future_poll(
    dyn_erased_future: &mut DynErasedFuture<'_>,
    waker_ptr: UnitPtr,
    out: *mut (),
) -> bool {
    let future = dyn_erased_future.0.as_mut();

    // Wrap the `Waker` in `ManuallyDrop` to avoid decrementing the C++ waker refcount upon exiting
    // this function.
    // SAFETY: `waker_ptr` is a valid C++ waker pointer compatible with `RS_STD_WAKER_VTABLE`.
    let waker = ManuallyDrop::new(unsafe { Waker::new(waker_ptr, RS_STD_WAKER_VTABLE) });
    let mut cx = Context::from_waker(&waker);
    // SAFETY: `out` satisfies the safety contract of `poll_erased` by the function's preconditions.
    unsafe { future.poll_erased(&mut cx, out) }.is_ready()
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
