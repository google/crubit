// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate alloc;

use crate::crubit_cc_std_internal::std_allocator::{
    self, shared_weak_count, DynControlBlock, FunctionToCall,
};
use alloc::boxed::Box;
use core::ffi::c_void;
use core::mem::{ManuallyDrop, MaybeUninit};

/// A smart pointer that shares ownership of another object of type `T` via a pointer,
/// ABI-compatible with `std::shared_ptr<T>`.
///
/// # Thread Safety
///
/// Like `Arc<T>`, `shared_ptr<T>` implements [`Send`] and [`Sync`] only if `T` implements both
/// [`Send`] and [`Sync`], and only offers access to the underlying `T` via shared references.
///
/// # Mutation
///
/// `shared_ptr<T>` only hands out shared references (`&T`), which are only valid to mutate when the
/// mutated data is wrapped in [`UnsafeCell`](std::cell::UnsafeCell). This has two consequences for
/// users of `shared_ptr<T>` in Rust:
///
/// 1. Thread-safe C++ types `T` that manage their own internal synchronization should be marked
///    with `CRUBIT_THREAD_SAFE` so their methods can be called from Rust within `shared_ptr<T>`.
///    Internally, this annotation wraps all fields in `UnsafeCell`, making them not only safe to
///    mutate, but also safe to hold a shared reference to (`&T`) while being mutated by someone
///    else.
///
/// 2. Non-thread-safe C++ types `T` can still be mutated safely from C++ as long as synchronization
///    is handled externally, but due to Rust's aliasing rules, it is undefined behavior for a Rust
///    user to hold a `&T` during the mutation (modulo anything within `UnsafeCell`, of course).
///    If the Rust user expects externally synchronized mutation from C++, they must also
///    participate in the synchronization to ensure that they never hold an active `&T` during the
///    C++ mutation window, otherwise they are subject to undefined behavior.
///
/// See crubit.rs/cpp/cookbook#thread_safety for more details.
#[crubit_annotate::cpp_layout_equivalent(
    cpp_type = "::std::shared_ptr<{T}>",
    include_path = "<memory>"
)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct shared_ptr<T: Sized> {
    // Safety: `ptr` and `cntrl` come from a valid `std::shared_ptr<T>`
    ptr: *const T,
    // Safety: `cntrl` is a nullable pointer to a valid  `std::__shared_weak_count`
    cntrl: *mut shared_weak_count,
}

// SAFETY: shared_ptr is Send and Sync if T is Send and Sync. Anything less is too restrictive
// since a &shared_ptr<T> can be cloned by another thread, effectively moving the `T` to the
// other thread.
unsafe impl<T: Sized + Send + Sync> Send for shared_ptr<T> {}
unsafe impl<T: Sized + Send + Sync> Sync for shared_ptr<T> {}

impl<T: Sized> shared_ptr<T> {
    /// Creates a new `shared_ptr` managing `value`.
    ///
    /// This function performs a single heap allocation (similar to `std::make_shared`),
    /// co-allocating the control block and the `value`.
    pub fn new(value: T) -> Self {
        let mut inner = Box::leak(Box::new(SharedInner {
            cntrl: MaybeUninit::uninit(),
            value: ManuallyDrop::new(value),
        }));

        // SAFETY: `this.cntrl.as_mut_ptr()` is a valid location to write a `DynControlBlock` to,
        // and `SharedInner::<T>::deleter` satisfies the C++ deleter callback preconditions.
        let cntrl = unsafe {
            DynControlBlock::Emplace(inner.cntrl.as_mut_ptr(), Some(SharedInner::<T>::deleter))
        };

        shared_ptr { ptr: &*inner.value, cntrl }
    }

    /// Creates a `shared_ptr` from a raw pointer and a control block pointer.
    ///
    /// # Safety
    ///
    /// ## Preconditions
    /// - `ptr` must be a valid, aligned pointer to an initialized `T`, or null.
    /// - `cntrl` must be a valid, nullable pointer to a `std::__shared_weak_count` control block
    ///   managing `ptr` (or null if `ptr` is null).
    /// - `(ptr, cntrl)` must constitute a valid `std::shared_ptr<T>` state.
    /// - The strong reference count of `cntrl` must already account for the returned `shared_ptr`;
    ///   this function transfers ownership of an existing strong reference without incrementing it.
    ///
    /// ## Postconditions
    /// - Returns a `shared_ptr<T>` owning the transferred strong reference, which will decrement
    ///   the reference count when dropped.
    pub unsafe fn from_raw_parts(ptr: *const T, cntrl: *mut shared_weak_count) -> Self {
        shared_ptr { ptr, cntrl }
    }

    /// Returns `true` if `this` is a null pointer.
    pub fn is_null(this: &Self) -> bool {
        this.ptr.is_null()
    }

    /// Returns a raw pointer to the contents.
    pub fn as_ptr(this: &Self) -> *const T {
        this.ptr
    }

    /// Releases the ownership of the object and control block pointed to by `this` without
    /// decrementing the reference count, replacing this `shared_ptr` with a null pointer.
    ///
    /// Where possible, prefer `into_raw_parts` in order to avoid null `shared_ptr`s.
    pub fn release(this: &mut Self) -> (*const T, *mut shared_weak_count) {
        (
            core::mem::replace(&mut this.ptr, core::ptr::null()),
            core::mem::replace(&mut this.cntrl, core::ptr::null_mut()),
        )
    }

    /// Consumes the `shared_ptr` without decrementing the reference count, returning the
    /// owned raw pointer and control block pointer.
    pub fn into_raw_parts(mut this: Self) -> (*const T, *mut shared_weak_count) {
        Self::release(&mut this)
    }

    /// Returns a reference to the underlying object, if it exists.
    ///
    /// # Safety
    ///
    /// If the inner object not thread safe and is mutated by C++ in a nonlocally-synchronized way,
    /// it's unsafe to hold references to `&T` across the mutations (unless the mutated data is
    /// wrapped in `UnsafeCell`).
    pub fn try_as_ref(this: &Self) -> Option<&T> {
        // SAFETY: By `shared_ptr` invariants, `this.ptr` is either null or points to an
        // initialized `T` whose lifetime is managed by `this`.
        unsafe { this.ptr.as_ref() }
    }

    /// Returns the number of `shared_ptr` instances managing the current object.
    #[must_use]
    pub fn use_count(this: &Self) -> usize {
        // SAFETY: `this.cntrl` is a nullable pointer to a valid `std::__shared_weak_count`.
        unsafe { std_allocator::shared_ptr_use_count(this.cntrl) }
    }
}

impl<T: Sized> Clone for shared_ptr<T> {
    fn clone(&self) -> Self {
        // SAFETY: `self.cntrl` is a nullable pointer to a valid `std::__shared_weak_count`.
        // Incrementing the strong reference count via `shared_ptr_ref` keeps the object alive
        // for the new clone.
        unsafe {
            std_allocator::shared_ptr_ref(self.cntrl);
        }
        shared_ptr { ptr: self.ptr, cntrl: self.cntrl }
    }
}

impl<T: Sized> Drop for shared_ptr<T> {
    fn drop(&mut self) {
        // SAFETY: `self.cntrl` is a nullable pointer to a valid `std::__shared_weak_count`.
        // Decrementing the strong reference count via `shared_ptr_unref` releases this instance's
        // ownership and triggers destruction if the strong count reaches zero.
        unsafe {
            std_allocator::shared_ptr_unref(self.cntrl);
        }
    }
}

/// A control block for `shared_ptr` that embeds a `DynControlBlock` and a `T` value.
///
/// # Layout
///
/// `SharedInner<T>` is `#[repr(C)]` with `MaybeUninit<DynControlBlock>` as its
/// first field. This means that pointers to this type can be freely upcasted to
/// `*mut DynControlBlock`, and (when safe to do so) downcasted back to
/// `*mut SharedInner<T>`.
#[repr(C)]
struct SharedInner<T> {
    /// The C++ control block.
    ///
    /// It is safety critical that this remains the first field, and that this type is repr(C).
    cntrl: MaybeUninit<DynControlBlock>,

    /// The value.
    ///
    /// The value is dropped in place when the strong refcount hits 0, which may happen before this
    /// object is deallocated (due to lingering weak pointers).
    value: ManuallyDrop<T>,
}

impl<T> SharedInner<T> {
    /// Deletes the `value` or destructs and deallocates the entire `SharedInner<T>`.
    ///
    /// This function is only intended to be used by `DynControlBlock`s embedded in
    /// `SharedInner<T>`.
    ///
    /// # Safety
    ///
    /// ## Preconditions
    /// - `cntrl` must point to a live `DynControlBlock` embedded as the first field of an
    ///   active `SharedInner<T>` allocated via `SharedInner::new`.
    /// - `function_to_call` must correspond to the exact lifecycle transition:
    ///   - `FunctionToCall::kDestroyValue`: called exactly once when the strong reference count
    ///     reaches 0, while `value` is still initialized and live.
    ///   - `FunctionToCall::kDeleteControlBlock`: called exactly once when all strong and weak
    ///     reference counts reach 0, after `kDestroyValue` has already run.
    ///
    /// ## Postconditions
    /// - If `kDestroyValue`: `value` is dropped in place; the control block and
    ///   containing allocation remain live.
    /// - If `kDeleteControlBlock`: `cntrl` is dropped in place and the entire
    ///   `SharedInner<T>` allocation is freed via `Box::from_raw`.
    unsafe extern "C" fn deleter(function_to_call: FunctionToCall, cntrl: *mut DynControlBlock) {
        // `SharedInner<T>` is `#[repr(C)]` with `MaybeUninit<DynControlBlock>` as
        // its first field at byte offset 0. Since `cntrl` was allocated as part of a
        // `SharedInner<T>`, upcasting `cntrl as *mut Self` safely recovers the
        // pointer to the containing `SharedInner<T>`.
        let this = cntrl as *mut Self;

        match function_to_call {
            FunctionToCall::kDestroyValue => {
                // SAFETY: Preconditions guarantee this is called exactly once when the strong
                // reference count is zero, and that `value` contains a live, undropped `T`.
                unsafe { core::ptr::drop_in_place(&raw mut (*this).value as *mut T) };
            }
            FunctionToCall::kDeleteControlBlock => {
                // SAFETY: Preconditions guarantee this is called exactly once when the weak
                // reference count hits zero, and that `this` points to a valid
                // `SharedInner<T>` that was previously leaked from a `Box`.
                let mut this = unsafe { Box::from_raw(this) };

                // SAFETY: Preconditions guarantee `cntrl` was initialized with placement-new in
                // `DynControlBlock::Emplace` and is valid to drop in place.
                unsafe { this.cntrl.assume_init_drop() };

                // `this` is dropped here, deallocating the `SharedInner<T>`.
            }
        }
    }
}
