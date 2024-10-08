// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(allocator_api)]
use std::ops::{Deref, DerefMut};
use std::ops::{Index, IndexMut};
use std::slice;

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
            let mut v = create_vec_from_raw_parts(self.begin, self.len(), self.capacity());
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

/// Helper method for creating a `Vec<T>` from raw parts.
fn create_vec_from_raw_parts<T>(
    begin: *mut T,
    len: usize,
    capacity: usize,
) -> Vec<T, cpp_std_allocator::StdAllocator> {
    unsafe {
        if begin.is_null() {
            Vec::new_in(cpp_std_allocator::StdAllocator {})
        } else {
            Vec::from_raw_parts_in(begin, len, capacity, cpp_std_allocator::StdAllocator {})
        }
    }
}

mod iter {
    use crate::Vector;
    use std::fmt;
    use std::fmt::Debug;
    use std::iter::FusedIterator;
    /// An iterator that moves out of a vector.
    ///
    /// This type is currently a wrapper around `std::vec::IntoIter`,
    /// however users should not depend on it, and the wrapped iterator
    /// should not be made public. If the current implementation
    /// strategy stops working, the wrapped iterator will be replaced with
    /// a more complex implementation.
    pub struct VectorIntoIter<T>(std::vec::IntoIter<T, cpp_std_allocator::StdAllocator>);

    impl<T> VectorIntoIter<T> {
        pub fn new(v: std::vec::IntoIter<T, cpp_std_allocator::StdAllocator>) -> Self {
            VectorIntoIter(v)
        }

        /// Returns the remaining items of this iterator as a slice.
        #[inline]
        pub fn as_slice(&mut self) -> &[T] {
            self.0.as_slice()
        }

        /// Returns the remaining items of this iterator as a mutable slice.
        #[inline]
        pub fn as_mut_slice(&mut self) -> &mut [T] {
            self.0.as_mut_slice()
        }
    }

    impl<T> Iterator for VectorIntoIter<T> {
        type Item = T;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }

        // TODO(b/356638830): Uncomment when feature advance_by is stable
        // fn advance_by(&mut self, n: usize) -> Result<(), NonZeroUsize> {
        //     self.0.advance_by(n)
        // }

        #[inline]
        fn count(self) -> usize {
            self.0.len()
        }
    }

    impl<T> AsRef<[T]> for VectorIntoIter<T> {
        fn as_ref(&self) -> &[T] {
            self.0.as_ref()
        }
    }

    impl<T: fmt::Debug> Debug for VectorIntoIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<T: Unpin> Default for VectorIntoIter<T> {
        fn default() -> Self {
            Vector::<T>::default().into_iter()
        }
    }

    impl<T> DoubleEndedIterator for VectorIntoIter<T> {
        #[inline]
        fn next_back(&mut self) -> Option<T> {
            self.0.next_back()
        }

        // TODO(b/356638830): Uncomment when feature advance_back_by is stable
        // #[inline]
        // fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        //     self.0.advance_back_by()
        // }
    }

    impl<T> ExactSizeIterator for VectorIntoIter<T> {
        // TODO(b/356638830): Uncomment when feature is_empty is stable
        // #[inline]
        // fn is_empty(&self) -> bool {
        //     self.0.is_empty()
        // }
    }

    impl<T> FusedIterator for VectorIntoIter<T> {}
    // TODO(b/356638830): Uncomment when feature TrustedLen is stable
    // unsafe impl<T> TrustedLen for VectorIntoIter<T> {}
}

use iter::*;

impl<T: Unpin> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = VectorIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let v = create_vec_from_raw_parts(self.begin, self.len(), self.capacity());
            core::mem::forget(self);
            VectorIntoIter::new(v.into_iter())
        }
    }
}

impl<'a, T: Unpin> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Unpin> IntoIterator for &'a mut Vector<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: Unpin> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v = Self::new();
        v.mutate_self_as_vec(|u| {
            u.extend(iter);
        });
        v
    }
}
