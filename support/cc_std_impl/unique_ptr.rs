// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::crubit_cc_std_internal::std_allocator;
use core::ptr::null_mut;
pub use operator::Delete;

/// A smart pointer that owns and manages another object of type `T` via a
/// pointer, ABI-compatible with `std::unique_ptr` using the default deleter from
/// C++.
///
/// If the class has a virtual destructor and is not the most-derived class, or if it overloads
/// `operator delete`, it is UB to use `unique_ptr`. Instead, use [`unique_ptr_dyn`].
#[allow(non_snake_case)]
#[repr(C)]
pub struct unique_ptr<T: Sized> {
    // Invariants:
    // 1. `ptr` is either null, or allocated by C++ `new`.
    // 2. If `ptr` is not null, it is exclusively owned by this `unique_ptr`.
    ptr: *mut T,
}

// SAFETY: unique_ptr exclusively owns `T` and adds no additional constraints on sending the
// pointer.
unsafe impl<T: Sized + Send> Send for unique_ptr<T> {}

// This is _not_ Sync, because we want to be able to pass unique_ptr to C++ in safe code,
// but the following would have UB in a multithreaded context:
//
// ```c++
// void Foo(const std::unique_ptr<MyClass>& ptr) {
//   ptr->Mutate();
// }
// ```
//
// ```rust
// let ptr : unique_ptr<MyClass> = ...;
// cpp_lib::Foo(&ptr);
// ```
//
// In other words, because unique_ptr is internally mutable, we have the choice between safely
// sharing `&unique_ptr` in Rust, or safely calling functions that take a `&unique_ptr`.
//
// In either case, we can feel a little bit relieved that it's extraordinarily rare to pass
// a reference to a unique_ptr. If you want to borrow T, you can unsafely dereference the
// unique_ptr and obtain a `&T`.

impl<T: Sized> unique_ptr<T> {
    /// Takes ownership of the provided raw pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must either be null, or allocated by C++ `new`.
    ///
    /// The object must not overload `operator delete`. If the destructor is
    /// virtual, `T` must be type of the most derived class.
    pub unsafe fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }

    pub fn get(&self) -> *mut T {
        self.ptr
    }

    pub fn release(&mut self) -> *mut T {
        core::mem::replace(&mut self.ptr, null_mut())
    }

    /// Returns an exclusive reference to the owned object, if-non-null, or None otherwise.
    pub fn as_mut(&mut self) -> Option<&mut T>
    where
        T: Unpin,
    {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        unsafe { self.ptr.as_mut() }
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

/// A smart pointer that owns and manages another object of type `T` via a
/// pointer, ABI-compatible with `std::unique_ptr` using default deleter from
/// C++. This is analogous to `Pin<Box<T>>`.
#[allow(non_snake_case)]
#[repr(C)]
pub struct unique_ptr_dyn<T: Sized + Delete> {
    // Invariants:
    // 1. `ptr` is either null, or allocated by C++ `new`.
    // 2. If `ptr` is not null, it is exclusively owned by this `unique_ptr_dyn`.
    ptr: *mut T,
}

// SAFETY: unique_ptr_dyn exclusively owns `T` and adds no additional constraints on sending the
// pointer.
unsafe impl<T: Sized + Delete + Send> Send for unique_ptr_dyn<T> {}

// This is _not_ Sync for the same reason as unique_ptr.

impl<T: Sized + Delete> unique_ptr_dyn<T> {
    /// Takes ownership of the provided raw pointer to a polymorphic type.
    ///
    /// If `T` doesn't implement `Delete`, use [`unique_ptr`] instead.
    ///
    /// # Safety
    ///
    /// `ptr` must either be null, or allocated by C++ `new`.
    pub unsafe fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }

    pub fn get(&self) -> *mut T {
        self.ptr
    }

    pub fn release(&mut self) -> *mut T {
        core::mem::replace(&mut self.ptr, null_mut())
    }

    /// Returns an exclusive reference to the owned object, if-non-null, or None otherwise.
    pub fn as_mut(&mut self) -> Option<&mut T>
    where
        T: Unpin,
    {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: Delete> Drop for unique_ptr_dyn<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                // SAFETY: valid and allocated with `new` per type invariants.
                T::delete(self.ptr);
            }
        }
    }
}
