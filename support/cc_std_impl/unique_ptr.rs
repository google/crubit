// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::crubit_cc_std_internal::std_allocator;
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
/// Note that `unique_ptr` has "shallow" semantics: having a `&unique_ptr<T>` means that the
/// `unique_ptr` will not be mutated, but does not guarantee that the underlying `T` will not be
/// mutated. Therefore, to get access to `T`, you must have exclusive access to the `unique_ptr`.
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

// SAFETY: unique_ptr has "shallow" semantics, and you cannot do anything with a `&unique_ptr`
// except pass it to C++, where the unsafe operation of internal mutability requires C++ programmers
// to prove safety.
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

    pub fn get(&self) -> *mut T {
        self.ptr
    }

    pub fn release(&mut self) -> *mut T {
        core::mem::replace(&mut self.ptr, null_mut())
    }

    /// Returns an shared reference to the owned object, if-non-null, or None otherwise.
    ///
    /// Note that it is not safe to obtain a `&T` from a `&unique_ptr`, because the pointed-to `T`
    /// may be mutated when a `&unique_ptr` is shared between C++ and Rust.
    pub fn as_ref(&mut self) -> Option<&T> {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        unsafe { self.ptr.as_ref() }
    }

    /// Returns an exclusive reference to the owned object, if-non-null, or None otherwise.
    pub fn as_mut(&mut self) -> Option<&mut T>
    where
        T: Unpin,
    {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        unsafe { self.ptr.as_mut() }
    }

    /// Returns an exclusive reference to the owned object, if-non-null, or None otherwise.
    pub fn as_pin(&mut self) -> Option<Pin<&mut T>> {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        // The pointee is pinned.
        unsafe { Some(Pin::new_unchecked(self.ptr.as_mut()?)) }
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

/// Legacy alias for [`virtual_unique_ptr`].
pub type unique_ptr_dyn<T> = virtual_unique_ptr<T>;

/// A smart pointer that owns and manages a polymorphic object with base class `T`.
///
/// This type is ABI-compatible with C++'s `std::unique_ptr<T>`, where `T` is a base class with a
/// virtual destructor.
///
/// Note that `virtual_unique_ptr` has "shallow" semantics: having a `&virtual_unique_ptr<T>` means that
/// the `virtual_unique_ptr` will not be mutated, but does not guarantee that the underlying `T` will
/// not be mutated. Therefore, to get access to `T`, you must have exclusive access to the
/// `virtual_unique_ptr`.
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

    pub fn get(&self) -> *mut T {
        self.ptr
    }

    pub fn release(&mut self) -> *mut T {
        core::mem::replace(&mut self.ptr, null_mut())
    }

    /// Returns an shared reference to the owned object, if-non-null, or None otherwise.
    ///
    /// Note that it is not safe to obtain a `&T` from a `&unique_ptr`, because the pointed-to `T`
    /// may be mutated when a `&unique_ptr` is shared between C++ and Rust.
    pub fn as_ref(&mut self) -> Option<&T> {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        unsafe { self.ptr.as_ref() }
    }

    /// Returns an exclusive reference to the owned object, if-non-null, or None otherwise.
    pub fn as_mut(&mut self) -> Option<&mut T>
    where
        T: Unpin,
    {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        unsafe { self.ptr.as_mut() }
    }

    /// Returns an exclusive reference to the owned object, if-non-null, or None otherwise.
    pub fn as_pin(&mut self) -> Option<Pin<&mut T>> {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        // The pointee is pinned.
        unsafe { Some(Pin::new_unchecked(self.ptr.as_mut()?)) }
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
