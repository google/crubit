// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::crubit_cc_std_internal::std_allocator;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::null_mut;

pub use operator::Delete;

/// A smart pointer that owns and manages another object of type `T` via a
/// pointer, ABI-compatible with `std::unique_ptr` using the default deleter from
/// C++. This is analogous to `Pin<Box<T>>`.
///
/// If the class has a virtual destructor and is not the most-derived class, or if it overloads
/// `operator delete`, it is UB to use `unique_ptr`. Instead, use [`virtual_unique_ptr`].
///
/// Note that while C++ `unique_ptr` is only shallow-const (i.e. a `const unique_ptr<T>&` allows
/// mutating the underlying `T`), it is conventionally treated as deep-const. In order for
/// `&unique_ptr<T>` to be usable at all from Rust, we treat it as deep-const. C++ code which
/// mutates a value of type `T` while Rust has obtained a `&T` via `&unique_ptr<T>` -> `&T`
/// dereference will result in undefined behavior.
///
/// # Safety
///
/// Because `unique_ptr` implicitly pins the underlying object, direct accesses to the underlying
/// object that would violate the pin guarantee are potentially UB.
///
/// Because Rust treats `unique_ptr` as deep-const, C++ code that mutates the underlying `T`
/// while a Rust `&T` or `&unique_ptr<T>` exists will violate Rust's aliasing rules and cause
/// undefined behavior.
#[crubit_annotate::cpp_layout_equivalent(
    cpp_type = "::std::unique_ptr<{T}>",
    include_path = "<memory>"
)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct unique_ptr<T: Sized> {
    // Invariants:
    // 1. `ptr` is either null, or allocated by C++ `new`.
    // 2. If `ptr` is not null, it is exclusively owned by this `unique_ptr`.
    // 3. the pointee is pinned.
    ptr: *mut T,
}

// SAFETY: unique_ptr exclusively owns `T` and adds no additional constraints on sending the
// pointer.
unsafe impl<T: Sized + Send> Send for unique_ptr<T> {}

// SAFETY: unique_ptr exclusively owns `T`, and `&unique_ptr<T>` allows obtaining a `&T`
// (treating unique_ptr as deep-const). Thus, sharing `&unique_ptr<T>` across threads is safe
// if and only if `T: Sync`.
unsafe impl<T: Sized + Sync> Sync for unique_ptr<T> {}

impl<T: Sized> unique_ptr<T> {
    /// Takes ownership of the provided raw pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must either be null, or allocated by C++ `new`.
    ///
    /// The object must not overload `operator delete`. If the destructor is
    /// virtual, `T` must be type of the most derived class.
    ///
    /// This pins `ptr`, and the same preconditions apply as for `Pin::new_unchecked`.
    pub unsafe fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }

    /// Takes ownership of the provided raw pointer.
    ///
    /// # Safety
    ///
    /// Same safety requirements as [`new`].
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self { ptr }
    }

    /// Returns `true` if `self` is a null pointer.
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Returns a raw pointer to the contents.
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    /// Returns a mutable raw pointer to the contents.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }

    /// Returns a copy of the held raw pointer to the owned object.
    ///
    /// The returned pointer is null for moved-from `unique_ptr` objects. It is valid until
    /// the underlying object is destroyed, such as by dropping the `unique_ptr`.
    pub fn get(&self) -> *mut T {
        self.ptr
    }

    /// Releases the ownership of the object pointed to by `self`, replacing this `unique_ptr` with
    /// a null pointer.
    ///
    /// Where possible, prefer `into_raw` in order to avoid null `unique_ptr`s.
    pub fn release(&mut self) -> *mut T {
        core::mem::replace(&mut self.ptr, null_mut())
    }

    /// Consumes the `unique_ptr`, returning the owned pointer.
    pub fn into_raw(mut self) -> *mut T {
        self.release()
    }

    /// Returns a shared reference to the owned object, if non-null, or None otherwise.
    pub fn as_ref(&self) -> Option<&T> {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        // C++ unique_ptr is treated as deep-const to allow safe Rust references.
        unsafe { self.ptr.as_ref() }
    }

    /// Returns an exclusive reference to the owned object, if non-null, or None otherwise.
    pub fn as_mut(&mut self) -> Option<&mut T>
    where
        T: Unpin,
    {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        unsafe { self.ptr.as_mut() }
    }

    /// Returns an exclusive reference to the owned object, if non-null, or None otherwise.
    pub fn as_pin(&mut self) -> Option<Pin<&mut T>> {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        // The pointee is pinned.
        unsafe { Some(Pin::new_unchecked(self.ptr.as_mut()?)) }
    }
}

impl<T: Sized> Deref for unique_ptr<T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        assert!(!self.ptr.is_null(), "dereferencing a null unique_ptr");
        // SAFETY: `self.ptr` is non-null, properly aligned, and points to a valid `T`.
        // C++ unique_ptr is treated as deep-const to allow safe Rust references.
        unsafe { &*self.ptr }
    }
}

impl<T: Sized + Unpin> DerefMut for unique_ptr<T> {
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self.ptr.is_null(), "dereferencing a null unique_ptr");
        // SAFETY: `self.ptr` is non-null, properly aligned, points to an exclusively-owned `T`,
        // and `T: Unpin` guarantees that obtaining a `&mut T` does not violate pinning invariants.
        unsafe { &mut *self.ptr }
    }
}

impl<T: Sized + Delete> From<unique_ptr<T>> for virtual_unique_ptr<T> {
    fn from(value: unique_ptr<T>) -> Self {
        Self { ptr: unique_ptr::into_raw(value) }
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

/// A smart pointer that owns and manages a polymorphic object with base class `T`.
///
/// This type is ABI-compatible with C++'s `std::unique_ptr<T>`, where `T` is a base class with a
/// virtual destructor.
///
/// Note that while C++ `unique_ptr` is only shallow-const (i.e. a `const unique_ptr<T>&` allows
/// mutating the underlying `T`), it is conventionally treated as deep-const. In order for
/// `&virtual_unique_ptr<T>` to be usable at all from Rust, we treat it as deep-const. C++ code
/// which mutates a value of type `T` while Rust has obtained a `&T` via `&virtual_unique_ptr<T>` ->
/// `&T` dereference will result in undefined behavior.
///
/// # Safety
///
/// Because `virtual_unique_ptr` implicitly pins the underlying object, direct accesses to the
/// underlying object that would violate the pin guarantee are potentially UB.
///
/// Because Rust treats `virtual_unique_ptr` as deep-const, C++ code that mutates the underlying `T`
/// while a Rust `&T` or `&virtual_unique_ptr<T>` exists will violate Rust's aliasing rules and cause
/// undefined behavior.
#[crubit_annotate::cpp_layout_equivalent(
    cpp_type = "::std::unique_ptr<{T}>",
    include_path = "<memory>"
)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct virtual_unique_ptr<T: Sized + Delete> {
    // Invariants:
    // 1. `ptr` is either null, or allocated by C++ `new`.
    // 2. If `ptr` is not null, it is exclusively owned by this `virtual_unique_ptr`.
    // 3. the pointee is pinned.
    ptr: *mut T,
}

// SAFETY: Same as for `unique_ptr`
unsafe impl<T: Sized + Delete + Send> Send for virtual_unique_ptr<T> {}

// SAFETY: Same as for `unique_ptr`.
unsafe impl<T: Sized + Delete + Sync> Sync for virtual_unique_ptr<T> {}

impl<T: Sized + Delete> virtual_unique_ptr<T> {
    /// Takes ownership of the provided raw pointer to a polymorphic type.
    ///
    /// If `T` doesn't implement `Delete`, use [`unique_ptr`] instead.
    ///
    /// # Safety
    ///
    /// `ptr` must either be null, or allocated by C++ `new`.
    ///
    /// This pins `ptr`, and the same preconditions apply as for `Pin::new_unchecked`.
    pub unsafe fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }

    /// Takes ownership of the provided raw pointer to a polymorphic type.
    ///
    /// # Safety
    ///
    /// Same safety requirements as [`new`].
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self { ptr }
    }

    /// Returns `true` if `self` is a null pointer.
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Returns a raw pointer to the contents.
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    /// Returns a mutable raw pointer to the contents.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }

    /// Returns a copy of the held raw pointer to the owned object.
    ///
    /// The returned pointer is null for null `virtual_unique_ptr` objects. It is valid until
    /// the underlying object is destroyed, such as by dropping the `virtual_unique_ptr`.
    pub fn get(&self) -> *mut T {
        self.ptr
    }

    /// Releases the ownership of the object pointed to by `self`, replacing this `virtual_unique_ptr` with
    /// a null pointer.
    ///
    /// Where possible, prefer `into_raw` in order to avoid null `virtual_unique_ptr`s.
    pub fn release(&mut self) -> *mut T {
        core::mem::replace(&mut self.ptr, null_mut())
    }

    /// Consumes the `virtual_unique_ptr`, returning the owned pointer.
    pub fn into_raw(mut self) -> *mut T {
        self.release()
    }

    /// Returns a shared reference to the owned object, if non-null, or None otherwise.
    pub fn as_ref(&self) -> Option<&T> {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        // C++ unique_ptr is treated as deep-const to allow safe Rust references.
        unsafe { self.ptr.as_ref() }
    }

    /// Returns an exclusive reference to the owned object, if non-null, or None otherwise.
    pub fn as_mut(&mut self) -> Option<&mut T>
    where
        T: Unpin,
    {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        unsafe { self.ptr.as_mut() }
    }

    /// Returns an exclusive reference to the owned object, if non-null, or None otherwise.
    pub fn as_pin(&mut self) -> Option<Pin<&mut T>> {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        // The pointee is pinned.
        unsafe { Some(Pin::new_unchecked(self.ptr.as_mut()?)) }
    }
}

impl<T: Sized + Delete> Deref for virtual_unique_ptr<T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        assert!(!self.ptr.is_null(), "dereferencing a null virtual_unique_ptr");
        // SAFETY: `self.ptr` is non-null, properly aligned, and points to a valid `T`.
        // C++ unique_ptr is treated as deep-const to allow safe Rust references.
        unsafe { &*self.ptr }
    }
}

impl<T: Sized + Delete + Unpin> DerefMut for virtual_unique_ptr<T> {
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self.ptr.is_null(), "dereferencing a null virtual_unique_ptr");
        // SAFETY: `self.ptr` is non-null, properly aligned, points to an exclusively-owned `T`,
        // and `T: Unpin` guarantees that obtaining a `&mut T` does not violate pinning invariants.
        unsafe { &mut *self.ptr }
    }
}

impl<T: Delete> Drop for virtual_unique_ptr<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                // SAFETY: valid and allocated with `new` per type invariants.
                T::delete(self.ptr);
            }
        }
    }
}
