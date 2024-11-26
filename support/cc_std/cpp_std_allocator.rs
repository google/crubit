// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
// #![feature(allocator_api)]

use crate::crubit_cc_std_internal::std_allocator::{
    cpp_delete, cpp_delete_with_alignment, cpp_new, cpp_new_with_alignment,
    StdCppDefaultNewAlignment,
};
use core::alloc::AllocError;
use core::alloc::Allocator;
use core::alloc::Layout;
use core::ffi::c_void;
use core::ptr::NonNull;

pub struct StdAllocator {}

unsafe impl Allocator for StdAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // The algorithm is:
        // - If the alignment does not exceed the default alignment, allocate with
        //   `new[]`.
        // - Otherwise, allocate with `new with alignment`.
        //
        // This emulates the behavior of `std::allocator<T>::allocate` from C++.
        let raw_ptr = if layout.align() > StdCppDefaultNewAlignment::Value.into() {
            // overaligned allocation
            cpp_new_with_alignment(layout.size(), layout.align()) as *mut u8
        } else {
            cpp_new(layout.size()) as *mut u8
        };
        let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            if layout.align() > StdCppDefaultNewAlignment::Value.into() {
                // overaligned allocation
                cpp_delete_with_alignment(
                    ptr.as_ptr() as *mut c_void,
                    layout.size(),
                    layout.align(),
                )
            } else {
                cpp_delete(ptr.as_ptr() as *mut c_void, layout.size())
            }
        }
    }
}

impl Clone for StdAllocator {
    fn clone(&self) -> Self {
        Self {}
    }
}
