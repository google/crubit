// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::crubit_cc_std_internal::std_allocator;
use core::ffi::c_void;

/// A smart pointer that shares ownership of another object of type `T` via a pointer,
/// ABI-compatible with `std::shared_ptr<const T>`.
#[allow(non_snake_case)]
#[repr(C)]
pub struct shared_ptr_const<T: Sized> {
    ptr: *const T,
    /// We must type erase the concrete type of the control block because TSan changes the
    /// C++ symbol name from `std::__u::__shared_weak_count` to
    /// `std::__tsan::_shared_weak_count`.
    // Safety: `cntrl` is a nullable pointer to a `std::__shared_weak_count` control block,
    // type-erased as `*mut c_void`.
    cntrl: *mut c_void,
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
    /// - `cntrl` must be a valid pointer to a `std::shared_ptr` control block
    ///   (`std::__shared_weak_count`), type-erased as `*mut c_void`, or null.
    /// - The reference count must already account for the returned `shared_ptr_const`; this
    ///   function does not increment it.
    pub unsafe fn from_raw_parts(ptr: *const T, cntrl: *mut c_void) -> Self {
        shared_ptr_const { ptr, cntrl }
    }

    /// Returns `true` if `this` is a null pointer.
    pub fn is_null(this: &Self) -> bool {
        this.ptr.is_null()
    }

    /// Returns a raw pointer to the contents.
    pub fn as_ptr(this: &Self) -> *const T {
        this.ptr
    }

    /// Releases the ownership of the object and control block pointed to by `this`, replacing this
    /// `shared_ptr_const` with a null pointer.
    ///
    /// Where possible, prefer `into_raw_parts` in order to avoid null `shared_ptr_const`s.
    pub fn release(this: &mut Self) -> (*const T, *mut c_void) {
        (
            core::mem::replace(&mut this.ptr, core::ptr::null()),
            core::mem::replace(&mut this.cntrl, core::ptr::null_mut()),
        )
    }

    /// Consumes the `shared_ptr_const`, returning the owned raw pointer and control block pointer.
    pub fn into_raw_parts(mut this: Self) -> (*const T, *mut c_void) {
        Self::release(&mut this)
    }

    /// Returns a reference to the underlying object, if it exists.
    pub fn try_as_ref(this: &Self) -> Option<&T> {
        // SAFETY: `this.ptr` is either null (no object) or a valid pointer to a `T` because `this`
        // has ownership.
        unsafe { this.ptr.as_ref() }
    }
}

impl<T: Sized> Clone for shared_ptr_const<T> {
    fn clone(&self) -> Self {
        // SAFETY: `self.cntrl` is either null or a valid pointer to a `std::__shared_weak_count`
        // (type-erased as `*mut c_void`). The callee is responsible for checking for null.
        unsafe {
            std_allocator::shared_ptr_ref(self.cntrl);
        }
        shared_ptr_const { ptr: self.ptr, cntrl: self.cntrl }
    }
}

impl<T: Sized> Drop for shared_ptr_const<T> {
    fn drop(&mut self) {
        // SAFETY: `self.cntrl` is either null or a valid pointer to a `std::__shared_weak_count`
        // (type-erased as `*mut c_void`). The callee is responsible for checking for null.
        unsafe {
            std_allocator::shared_ptr_unref(self.cntrl);
        }
    }
}
