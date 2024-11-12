// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(allocator_api)]
use core::alloc::AllocError;
use core::ffi::c_void;
use std::alloc::Allocator;
use std::alloc::Layout;
use std::ptr::NonNull;

pub struct StdAllocator {}

unsafe impl Allocator for StdAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // The algorithm is:
        // - If the alignment does not exceed the default alignment, allocate with
        //   `new[]`.
        // - Otherwise, allocate with `new with alignment`.
        //
        // This emulates the behavior of `std::allocator<T>::allocate` from C++.
        let raw_ptr = if layout.align()
            > std_allocator::crubit_internal::StdCppDefaultNewAlignment::Value.into()
        {
            // overaligned allocation
            std_allocator::crubit_internal::cpp_new_with_alignment(layout.size(), layout.align())
                as *mut u8
        } else {
            std_allocator::crubit_internal::cpp_new(layout.size()) as *mut u8
        };
        let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            if layout.align()
                > std_allocator::crubit_internal::StdCppDefaultNewAlignment::Value.into()
            {
                // overaligned allocation
                std_allocator::crubit_internal::cpp_delete_with_alignment(
                    ptr.as_ptr() as *mut c_void,
                    layout.size(),
                    layout.align(),
                )
            } else {
                std_allocator::crubit_internal::cpp_delete(
                    ptr.as_ptr() as *mut c_void,
                    layout.size(),
                )
            }
        }
    }
}
