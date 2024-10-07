// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(allocator_api)]
use std::ops::{Deref, DerefMut};
use std::ops::{Index, IndexMut};

/// A mutable, contiguous, dynamically-sized container of elements of type `T`,
/// ABI-compatible with `std::vector` from C++.
/// This layout was found empirically on Linux with modern g++ and libc++. If
/// for some version of libc++ it is different, we will need to update it with
/// conditional compilation.
#[repr(C)]
pub struct Vector<T> {
    // TODO(b/356221873): Ensure Vector<T> is covariant.
    begin: *mut T,
    end: *mut T,
    capacity_end: *mut T,
}

// TODO(b/356221873): Implement Send and Sync.
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
        F: FnOnce(&mut Vec<T, cpp_std_allocator::StdAllocator>) -> R,
    {
        unsafe {
            let mut v = if self.begin.is_null() {
                Vec::new_in(cpp_std_allocator::StdAllocator {})
            } else {
                Vec::from_raw_parts_in(
                    self.begin,
                    self.len(),
                    self.capacity(),
                    cpp_std_allocator::StdAllocator {},
                )
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

    pub fn reserve(&mut self, capacity: usize) {
        self.mutate_self_as_vec(|v| v.reserve(capacity));
    }

    pub fn with_capacity(capacity: usize) -> Vector<T> {
        let mut result = Vector::new();
        result.reserve(capacity);
        result
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
                    cpp_std_allocator::StdAllocator {},
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

impl<T: Unpin> From<Vec<T>> for Vector<T> {
    fn from(v: Vec<T>) -> Self {
        // Elements from `v` are moved. It would be more efficient to steal a buffer
        // from `v`. But `v` might have different allocator than Vector.
        // TODO(b/356221873): Figure out conditions when it is possible to steal buffer
        // from `v`.
        let mut result = Vector::<T>::with_capacity(v.len());
        result.mutate_self_as_vec(|u| {
            u.extend(v);
        });
        result
    }
}
