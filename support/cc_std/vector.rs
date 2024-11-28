// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(allocator_api)]
#![feature(cfg_sanitize)]
use std::cmp::Ordering;
use std::collections::TryReserveError;
#[cfg(sanitize = "address")]
use std::ffi::c_void;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem::ManuallyDrop;
use std::ops::RangeBounds;
use std::ops::{Deref, DerefMut};
use std::ops::{Index, IndexMut};
use std::slice;
use std::slice::SliceIndex;

use cc_std::crubit_cc_std_internal::std_allocator as cpp_std_allocator;

extern "C" {
    // https://github.com/llvm/llvm-project/blob/9d0616ce52fc2a75c8e4808adec41d5189f4240c/compiler-rt/lib/sanitizer_common/sanitizer_interface_internal.h#L70
    #[cfg(sanitize = "address")]
    fn __sanitizer_annotate_contiguous_container(
        beg: *const c_void,
        end: *const c_void,
        old_mid: *const c_void,
        new_mid: *const c_void,
    );
}

/// A mutable, contiguous, dynamically-sized container of elements
/// of type `T`, ABI-compatible with `std::vector` from C++.
// TODO(b/356221873): Ensure Vector<T> is covariant.
/// 2 layouts are supported.
/// 1. This layout was found empirically on Linux with modern g++ and libc++. If
/// for some version of libc++ it is different, we will need to update it with
/// conditional compilation.
#[cfg(not(len_capacity_encoding))]
#[repr(C)]
pub struct Vector<T> {
    begin: *mut T,
    _end: *mut T,
    _capacity_end: *mut T,
}

/// 2. This layout is experimental.
#[cfg(len_capacity_encoding)]
#[repr(C)]
pub struct Vector<T> {
    begin: *mut T,
    len: usize,
    capacity: usize,
}

// TODO(b/356221873): Implement Send and Sync.

impl<T> Vector<T> {
    pub fn new() -> Vector<T> {
        #[cfg(len_capacity_encoding)]
        {
            Vector { begin: core::ptr::null_mut(), len: 0, capacity: 0 }
        }
        #[cfg(not(len_capacity_encoding))]
        {
            Vector {
                begin: core::ptr::null_mut(),
                _end: core::ptr::null_mut(),
                _capacity_end: core::ptr::null_mut(),
            }
        }
    }

    pub fn len(&self) -> usize {
        #[cfg(len_capacity_encoding)]
        {
            self.len
        }
        #[cfg(not(len_capacity_encoding))]
        {
            // TODO(b/356221873): delete the `if` once a stable Rust release allows
            //offset_from for "the same address"
            if self.begin.is_null() {
                0
            } else {
                unsafe { self._end.offset_from(self.begin).try_into().unwrap() }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        #[cfg(len_capacity_encoding)]
        {
            self.capacity
        }
        #[cfg(not(len_capacity_encoding))]
        {
            // TODO(b/356221873): delete the `if` once a stable Rust release allows
            // offset_from for "the same address"
            if self.begin.is_null() {
                0
            } else {
                unsafe { self._capacity_end.offset_from(self.begin).try_into().unwrap() }
            }
        }
    }

    fn end(&self) -> *mut T {
        #[cfg(len_capacity_encoding)]
        {
            unsafe { self.begin.add(self.len) }
        }
        #[cfg(not(len_capacity_encoding))]
        {
            self._end
        }
    }

    fn capacity_end(&self) -> *mut T {
        #[cfg(len_capacity_encoding)]
        {
            unsafe { self.begin.add(self.capacity) }
        }
        #[cfg(not(len_capacity_encoding))]
        {
            self._capacity_end
        }
    }

    /// Sets the begin, len and capacity of the vector.
    ///
    /// This function overrides the pointer `self.begin` w/o deleting the
    /// pointed memory. That is the responsibility of the caller to ensure that
    /// no leaks occur.
    ///
    /// # Safety
    ///
    /// - `begin` must be a null and (len == capacity == 0) or  `begin` must be
    ///   a valid pointer and the memory pointed by `begin` must be allocated
    ///   with `StdAllocator`.
    /// - `len` must be less than or equal to `capacity`.
    /// - The first `len` values must be properly initialized values of type
    ///   `T`.
    /// - The size of `T `times the `capacity` (i.e. the allocated size in
    ///   bytes) needs to be the same size as the pointer was allocated with.
    /// - `T` needs to have the same alignment as what `begin` was allocated
    ///   with.
    /// - nothing else believes to have ownership over the memory of `begin`,
    ///   and that no outstanding references to this memory are still present by
    ///   the time `self` is next used.
    ///
    /// These requirements are always upheld by any `begin` that has been
    /// allocated via Vec<T, StdAllocator> and the corresponding Vec is
    /// forgotten after the call of this function. It follows by the properties
    /// of the `Vec`.
    unsafe fn set_begin_len_capacity(&mut self, begin: *mut T, len: usize, capacity: usize) {
        #[cfg(len_capacity_encoding)]
        {
            self.begin = begin;
            self.len = len;
            self.capacity = capacity;
        }
        #[cfg(not(len_capacity_encoding))]
        {
            self.begin = begin;
            self._end = begin.add(len);
            self._capacity_end = begin.add(capacity);
        }
    }

    /// Prepares the vector to write into the tail.
    /// See docstring of [`Vector::set_len`] for more details.
    pub fn prepare_to_write_into_tail(&mut self) {
        self.asan_unpoison_tail();
    }

    /// Sets the length of the vector.
    ///
    ///  # Safety
    ///
    /// - `new_len` must be less than or equal to [`capacity()`].
    /// - The elements at `old_len..new_len` must be initialized.
    ///
    /// See [`std::vec::Vec::set_len`] for more details.
    ///
    /// The difference with `std::vec::Vec::set_len` is that the tail of the
    /// Vector is poisoned with ASan, so when writing to the tail for
    /// avoiding ASan errors [`Vector::prepare_to_write_into_tail`] must be
    /// called first:
    /// ```
    /// v.prepare_to_write_into_tail()
    /// // write to tail (i.e. between v.len() and v.capacity())
    /// v.set_len(len)
    /// ```
    pub unsafe fn set_len(&mut self, len: usize) {
        #[cfg(len_capacity_encoding)]
        {
            self.len = len;
        }
        #[cfg(not(len_capacity_encoding))]
        {
            self._end = self.begin.add(len);
        }
        self.asan_poison_tail();
    }

    #[inline]
    #[cfg(not(sanitize = "address"))]
    fn asan_poison_tail(&self) {}

    #[inline]
    #[cfg(not(sanitize = "address"))]
    fn asan_unpoison_tail(&self) {}

    #[inline]
    #[cfg(sanitize = "address")]
    fn asan_poison_tail(&self) {
        // C++ std::vector supports an ASan container annotation feature
        // (https://github.com/google/sanitizers/wiki/AddressSanitizerContainerOverflow)
        // that allows ASan to detect reads and writes in the uninitialized tail of the
        // std::vector's storage (between the end iterator and capacity).
        //
        // Rust std::vec::Vec intentionally does not support this ASan annotation
        // feature, because it allows users to initialize the elements in the
        // tail of the storage and then call set_len to tell the Vec about new
        // elements.
        //
        // ASan uses the term "poisoned" for data that cannot be accessed. When marking
        // data as inaccessible, we poison it, to make them accessible we
        // unpoison the data. So, the tail of a Rust Vec's storage is always
        // unpoisoned, even when ASan is enabled.
        //
        // Thus, when we use Rust Vec to implement operations on C++ std::vector's
        // storage, we hit an incompatibility: the tail of the C++ std::vector
        // is poisoned, but Rust Vec does not unpoison before writing into it.
        //
        // Therefore, when we create a Rust Vec using the storage of a C++ std::vector
        // we need to establish the ASan poision/unpoison invariants that the Rust Vec
        // expects. Specifically, we unpoison the storage before a Rust Vec
        // writes into the uninitialized tail. Furthermore, once we are done
        // using a Rust Vec to manipulate the storage, we poison the tail agail.
        //
        // As an optimization we don't poison/unpoison the tail when we create a Rust
        // Vec purely to perform reads, or to mutate existing elements in-place.
        unsafe {
            // The following call is the same as __annotate_new in C++ std::vector
            // https://github.com/llvm/llvm-project/blob/9d0616ce52fc2a75c8e4808adec41d5189f4240c/libcxx/include/vector#L920
            __sanitizer_annotate_contiguous_container(
                self.begin as *const c_void,
                self.capacity_end() as *const c_void,
                self.capacity_end() as *const c_void,
                self.end() as *const c_void,
            );
        }
    }

    #[inline]
    #[cfg(sanitize = "address")]
    fn asan_unpoison_tail(&self) {
        unsafe {
            // The following call is the same as __annotate_delete in C++
            // std::vector https://github.com/llvm/llvm-project/blob/9d0616ce52fc2a75c8e4808adec41d5189f4240c/libcxx/include/vector#L927
            __sanitizer_annotate_contiguous_container(
                self.begin as *const c_void,
                self.capacity_end() as *const c_void,
                self.end() as *const c_void,
                self.capacity_end() as *const c_void,
            );
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
            self.asan_unpoison_tail();
            let mut v = ManuallyDrop::new(create_vec_from_raw_parts(
                self.begin,
                self.len(),
                self.capacity(),
            ));
            let result = mutate_self(v.deref_mut());
            self.set_begin_len_capacity(v.as_mut_ptr(), v.len(), v.capacity());
            self.asan_poison_tail();
            result
        }
    }

    // Methods for changing the capacity of the vector.
    pub fn reserve(&mut self, capacity: usize) {
        self.mutate_self_as_vec(|v| v.reserve(capacity));
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.mutate_self_as_vec(|v| v.reserve_exact(additional));
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.mutate_self_as_vec(|v| v.try_reserve(additional))
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.mutate_self_as_vec(|v| v.try_reserve_exact(additional))
    }

    pub fn shrink_to_fit(&mut self) {
        self.mutate_self_as_vec(|v| v.shrink_to_fit());
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.mutate_self_as_vec(|v| v.shrink_to(min_capacity));
    }

    pub fn with_capacity(capacity: usize) -> Vector<T> {
        let mut result = Vector::new();
        result.reserve(capacity);
        result
    }

    // Methods for adding elements to the vector.
    pub fn push(&mut self, value: T) {
        self.mutate_self_as_vec(|v| v.push(value));
    }

    pub fn insert(&mut self, index: usize, element: T) {
        self.mutate_self_as_vec(|v| v.insert(index, element));
    }

    pub fn append(&mut self, other: &mut Self) {
        self.mutate_self_as_vec(|v| other.mutate_self_as_vec(|other_v| v.append(other_v)))
    }

    // Methods for deleting elements from the vector.
    pub fn swap_remove(&mut self, index: usize) -> T {
        self.mutate_self_as_vec(|v| v.swap_remove(index))
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.mutate_self_as_vec(|v| v.remove(index))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.mutate_self_as_vec(|v| v.pop())
    }

    pub fn clear(&mut self) {
        self.mutate_self_as_vec(|v| v.clear());
    }

    pub fn truncate(&mut self, len: usize) {
        self.mutate_self_as_vec(|v| v.truncate(len));
    }

    // Different Vector transformations.
    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.mutate_self_as_vec(|v| v.dedup_by(same_bucket));
    }

    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.mutate_self_as_vec(|v| v.dedup_by_key(key));
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.mutate_self_as_vec(|v| v.retain(f));
    }

    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.mutate_self_as_vec(|v| v.retain_mut(f));
    }

    pub fn split_off(&mut self, at: usize) -> Self {
        Vector::from(self.mutate_self_as_vec(|v| v.split_off(at)))
    }

    // Methods returning different vector representations.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut result = Vec::<T>::with_capacity(self.len());
        unsafe {
            std::ptr::copy_nonoverlapping(self.as_ptr(), result.as_mut_ptr(), self.len());
            result.set_len(self.len());

            // The elements were moved out. Now mark `self` empty, without calling drop on
            // elements.
            self.asan_unpoison_tail();
            self.set_len(0);
        }
        result
    }

    pub fn as_ptr(&self) -> *const T {
        self.begin
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.begin
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.begin, self.len()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.begin, self.len()) }
    }
}

impl<T: Unpin + Clone> Vector<T> {
    pub fn extend_from_within<R>(&mut self, src: R)
    where
        R: RangeBounds<usize>,
    {
        self.mutate_self_as_vec(|v| v.extend_from_within(src));
    }

    pub fn resize(&mut self, new_len: usize, value: T) {
        self.mutate_self_as_vec(|v| v.resize(new_len, value));
    }

    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> T,
    {
        self.mutate_self_as_vec(|v| v.resize_with(new_len, f));
    }
}

impl<T: Unpin + PartialEq> Vector<T> {
    pub fn dedup(&mut self) {
        self.mutate_self_as_vec(|v| v.dedup());
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
                self.asan_unpoison_tail();
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

impl<T: Unpin, I: SliceIndex<[T]>> Index<I> for Vector<T> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        self.as_slice().index(index)
    }
}

impl<T: Unpin, I: SliceIndex<[T]>> IndexMut<I> for Vector<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.as_mut_slice().index_mut(index)
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

impl<T: Clone> Vector<T> {
    /// Clone elements from `self` to `Vec<T>`.
    pub fn to_vec(&self) -> Vec<T> {
        let mut v = Vec::<T>::with_capacity(self.len());
        for el in self.iter() {
            v.push(el.clone());
        }
        v
    }
}

impl<T: Unpin + Clone> Clone for Vector<T> {
    fn clone(&self) -> Self {
        unsafe {
            let vec = ManuallyDrop::new(create_vec_from_raw_parts(
                self.begin,
                self.len(),
                self.capacity(),
            ));
            Vector::from((*vec).clone())
        }
    }
}

impl<T: Unpin> Extend<T> for Vector<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.mutate_self_as_vec(|v| v.extend(iter));
    }
}

impl<T: Hash + Unpin> Hash for Vector<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

/// Helper method for creating a `Vec<T>` from raw parts.
///
/// # Safety
///
/// - `begin` must be a null or  `begin` must be a valid pointer and the memory
///   pointed by `begin` must be allocated with `StdAllocator`.
/// - `len` must be less than or equal to `capacity`.
/// - The first `len` values must be properly initialized values of type `T`.
/// - The size of `T `times the `capacity` (i.e. the allocated size in bytes)
///   needs to be the same size as the pointer was allocated with.
/// - `T` needs to have the same alignment as what `begin` was allocated with.
///
/// These requirements are always upheld by any `begin` that has been
/// allocated via Vec<T, StdAllocator>.
unsafe fn create_vec_from_raw_parts<T>(
    begin: *mut T,
    len: usize,
    capacity: usize,
) -> Vec<T, cpp_std_allocator::StdAllocator> {
    if begin.is_null() {
        Vec::new_in(cpp_std_allocator::StdAllocator {})
    } else {
        Vec::from_raw_parts_in(begin, len, capacity, cpp_std_allocator::StdAllocator {})
    }
}

mod iter {
    use crate::cpp_std_allocator;
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
            self.asan_unpoison_tail();
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

impl<T: Unpin> From<Vector<T>> for std::vec::Vec<T> {
    fn from(v: Vector<T>) -> std::vec::Vec<T> {
        v.into_vec()
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

impl<T: Unpin> From<Vec<T, cpp_std_allocator::StdAllocator>> for Vector<T> {
    /// Creates a `Vector<T>` from a `Vec<T, StdAllocator>`.
    ///
    /// Ownership of elements from `v` are taken by the returned `Vector<T>`.
    fn from(mut v: Vec<T, cpp_std_allocator::StdAllocator>) -> Self {
        let mut result = Vector::new();
        // Safety:
        //
        // It is safe since the memory is allocated with `Vec<T, StdAllocator>`.
        unsafe {
            result.set_begin_len_capacity(v.as_mut_ptr(), v.len(), v.capacity());
            std::mem::forget(v);
        }
        result.asan_poison_tail();
        result
    }
}
