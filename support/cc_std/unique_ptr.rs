// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::crubit_cc_std_internal::std_allocator;
use core::ptr::null_mut;

/// A smart pointer that owns and manages another object of type `T` via a
/// pointer, ABI-compatible with `std::unique_ptr` using default deleter from
/// C++.
#[allow(non_snake_case)]
#[repr(C)]
pub struct unique_ptr<T> {
    // Invariants:
    // 1. `ptr` is either null, or allocated by C++ `new`.
    // 2. If `ptr` is not null, it is owned by this `unique_ptr`.
    ptr: *mut T,
}

impl<T> unique_ptr<T> {
    /// Takes ownership of the provided raw pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must either be null, or allocated by C++ `new`. Otherwise, the
    /// behavior is undefined.
    pub unsafe fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }

    pub fn get(&self) -> *mut T {
        self.ptr
    }

    pub fn release(&mut self) -> *mut T {
        core::mem::replace(&mut self.ptr, null_mut())
    }
}

impl<T> Drop for unique_ptr<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                // SAFETY: a non-null `self.ptr` is a pointer to a `T` allocated with C++ `new`,
                // which should be satisfied by the constructor.
                core::ptr::drop_in_place(self.ptr);
                std_allocator::cpp_delete(
                    self.ptr as _,
                    core::mem::size_of::<T>(),
                    core::mem::align_of::<T>(),
                );
            }
        }
    }
}
