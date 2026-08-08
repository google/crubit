// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::crubit_cc_std_internal::std_allocator::{self, shared_weak_count};

/// A smart pointer that shares ownership of another object of type `T` via a pointer,
/// ABI-compatible with `std::shared_ptr<T>`.
///
/// # Thread Safety
///
/// Like `Arc<T>`, `shared_ptr<T>` implements [`Send`] and [`Sync`] only if `T` implements both
/// [`Send`] and [`Sync`], and only offers access to the underlying `T` via shared references.
/// If `T` is a C++ type with thread-safe methods, ensure that the type is annotated with
/// `CRUBIT_THREAD_SAFE` on its C++ definition so its methods are callable from within a shared_ptr.
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
    /// Creates a `shared_ptr` from a raw pointer and a control block pointer.
    ///
    /// # Safety
    ///
    /// - `ptr` must be a valid pointer to a `T` or null.
    /// - `ptr` and `cntrl` must have come from `into_raw_parts` or `release` of a
    ///   `shared_ptr`, i.e. they constitute a valid `std::shared_ptr<T>`.
    /// - The reference count must already account for the returned `shared_ptr`; this
    ///   function does not increment it.
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
    pub fn try_as_ref(this: &Self) -> Option<&T> {
        // SAFETY: `this.ptr` is either null (no object) or a valid pointer to a `T` because `this`
        // has ownership.
        unsafe { this.ptr.as_ref() }
    }
}

impl<T: Sized> Clone for shared_ptr<T> {
    fn clone(&self) -> Self {
        // SAFETY: `self.cntrl` is a nullable pointer to a valid  `std::__shared_weak_count`
        unsafe {
            std_allocator::shared_ptr_ref(self.cntrl);
        }
        shared_ptr { ptr: self.ptr, cntrl: self.cntrl }
    }
}

impl<T: Sized> Drop for shared_ptr<T> {
    fn drop(&mut self) {
        // SAFETY: `self.cntrl` is a nullable pointer to a valid  `std::__shared_weak_count`
        unsafe {
            std_allocator::shared_ptr_unref(self.cntrl);
        }
    }
}
