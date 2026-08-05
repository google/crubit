// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::crubit_cc_std_internal::std_allocator;
use forward_declare::{symbol, Incomplete};

type __shared_weak_count = Incomplete<symbol!(":: std :: __u :: __shared_weak_count"), ()>;

/// A smart pointer that shares ownership of another object of type `T` via a pointer,
/// ABI-compatible with `std::shared_ptr<const T>`.
#[allow(non_snake_case)]
#[repr(C)]
pub struct shared_ptr_const<T: Sized> {
    ptr: *const T,
    // Safety: `cntrl` is a nullable pointer to a `std::__shared_weak_count`.
    cntrl: *mut __shared_weak_count,
}

// SAFETY: shared_ptr_const is Send and Sync if T is Send and Sync. Anything less is too restrictive
// since a &shared_ptr_const<T> can be cloned by another thread, effectively moving the `T` to the
// other thread.
unsafe impl<T: Sized + Send + Sync> Send for shared_ptr_const<T> {}
unsafe impl<T: Sized + Send + Sync> Sync for shared_ptr_const<T> {}

impl<T: Sized> shared_ptr_const<T> {
    /// Creates a `shared_ptr_const` from a raw pointer and a control block pointer.
    ///
    /// # Safety
    ///
    /// - `ptr` must be a valid pointer to a `T` or null.
    /// - `cntrl` must be a valid pointer to a `std::shared_ptr` control block or null.
    /// - The reference count must already account for the returned `shared_ptr_const`; this
    ///   function does not increment it.
    pub unsafe fn from_raw_parts(ptr: *const T, cntrl: *mut __shared_weak_count) -> Self {
        shared_ptr_const { ptr, cntrl }
    }

    /// Returns the underlying pointer.
    pub fn get(&self) -> *const T {
        self.ptr
    }

    /// Returns a reference to the underlying object, if it exists.
    pub fn as_ref(&self) -> Option<&T> {
        // SAFETY: `self.ptr` is either null (no object) or a valid pointer to a `T` because `self`
        // has ownership.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: Sized> Clone for shared_ptr_const<T> {
    fn clone(&self) -> Self {
        // SAFETY: `self.cntrl` is either null or a valid pointer to a `std::__shared_weak_count`.
        // The callee is responsible for checking for null.
        unsafe {
            std_allocator::shared_ptr_ref(self.cntrl);
        }
        shared_ptr_const { ptr: self.ptr, cntrl: self.cntrl }
    }
}

impl<T: Sized> Drop for shared_ptr_const<T> {
    fn drop(&mut self) {
        // SAFETY: `self.cntrl` is either null or a valid pointer to a `std::__shared_weak_count`.
        // The callee is responsible for checking for null.
        unsafe {
            std_allocator::shared_ptr_unref(self.cntrl);
        }
    }
}
