// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
// #![feature(allocator_api)]

use crate::crubit_cc_std_internal::std_allocator::{cpp_delete, cpp_new};
use core::alloc::AllocError;
use core::alloc::Allocator;
use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::ffi::c_void;
use core::ptr::NonNull;

pub struct StdAllocator {}

unsafe impl Allocator for StdAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // SAFETY: StdAllocator allows for zero-sized allocations.
        let raw_ptr = unsafe { self.alloc(layout) };
        let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // SAFETY: Allocator and Dealloc have identical preconditions.
        unsafe {
            self.dealloc(ptr.as_ptr(), layout);
        }
    }

    // NOTE: Also change the GlobalAlloc impl if you add grow/etc.
}

/// StdAllocator is a global allocator which also accepts zero-sized allocations.
///
/// This allows allocations (even of size 0) in Rust to be mixed with deallocations in C++.
/// (Though, since `Global` will not pass through 0-sized allocations, this is currently of
/// limited use.)
unsafe impl GlobalAlloc for StdAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        cpp_new(layout.size(), layout.align()) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            cpp_delete(ptr as *mut c_void, layout.size(), layout.align());
        }
    }
}

impl Clone for StdAllocator {
    fn clone(&self) -> Self {
        Self {}
    }
}
