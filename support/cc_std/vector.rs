// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(allocator_api)]
use core::ffi::c_void;
use std::alloc::AllocError;
use std::alloc::Allocator;
use std::alloc::Layout;
use std::ops::{Deref, DerefMut};
use std::ops::{Index, IndexMut};
use std::ptr::NonNull;

struct StdAllocator {}
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

/// A mutable, contiguous, dynamically-sized container of elements of type `T`,
/// ABI-compatible with `std::vector` from C++.
/// This layout was found empirically on Linux with modern g++ and libc++. If
/// for some version of libc++ it is different, we will need to update it with
/// conditional compilation.
pub struct Vector<T> {
    // TODO(b/356221873): Ensure Vector<T> is covariant.
    begin: *mut T,
    end: *mut T,
    capacity_end: *mut T,
}

// TODO(b/356221873): Add a test that checks that the layout of this struct is
// consistent with the layout of `std::vector` from C++.
// TODO(b/356221873): Implement Send and Sync.
// TODO(b/356221873): Implement conversion to and from std::Vec
// TODO(b/356221873): Implement FromIterator, FromIteratorMut.
// TODO(b/356221873): Implement function for resizing (resize, shrink_to_fit,
// reserve etc).
// TODO(b/356221873): Implement clear().
// TODO(b/356221873): implement insertion, removal of elements.
// TODO(b/356221873): implement append, extend.

impl<T> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector {
            begin: core::ptr::null_mut(),
            end: core::ptr::null_mut(),
            capacity_end: core::ptr::null_mut(),
        }
    }

    pub fn len(&self) -> usize {
        // TODO(b/356221873): delete the `if` once a stable Rust release allows
        // offset_from for "the same address"
        if self.begin.is_null() {
            0
        } else {
            unsafe { self.end.offset_from(self.begin).try_into().unwrap() }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        // TODO(b/356221873): delete the `if` once a stable Rust release allows
        // offset_from for "the same address"
        if self.begin.is_null() {
            0
        } else {
            unsafe { self.capacity_end.offset_from(self.begin).try_into().unwrap() }
        }
    }
}

impl<T: Unpin> Vector<T> {
    /// Mutates `self` as if it were a `Vec<T>`.
    fn mutate_self_as_vec<F, R>(&mut self, mutate_self: F) -> R
    where
        F: FnOnce(&mut Vec<T, StdAllocator>) -> R,
    {
        unsafe {
            let mut v = if self.begin.is_null() {
                Vec::new_in(StdAllocator {})
            } else {
                Vec::from_raw_parts_in(self.begin, self.len(), self.capacity(), StdAllocator {})
            };
            let result = mutate_self(&mut v);
            let len = v.len();
            let capacity = v.capacity();
            self.begin = v.as_mut_ptr();
            self.end = self.begin.add(len);
            self.capacity_end = self.begin.add(capacity);
            core::mem::forget(v);
            result
        }
    }

    pub fn push(&mut self, value: T) {
        self.mutate_self_as_vec(|v| v.push(value));
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if !self.begin.is_null() {
            unsafe {
                _ = Vec::from_raw_parts_in(
                    self.begin,
                    self.len(),
                    self.capacity(),
                    StdAllocator {},
                );
            }
        }
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T: Unpin> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.is_empty() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.begin, self.len()) }
        }
    }
}

impl<T: Unpin> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.is_empty() {
            &mut []
        } else {
            unsafe { std::slice::from_raw_parts_mut(self.begin, self.len()) }
        }
    }
}
