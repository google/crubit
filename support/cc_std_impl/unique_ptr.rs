// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::std::Allocator;
use core::pin::Pin;
use core::ptr::null_mut;

pub use operator::Delete;

/// A pointer type that uniquely owns a heap allocation of type `T`.
///
/// This type is analogous to a `Pin<Box<T, cc_std::Allocator>>` with an invalid moved-from state.
/// When working with an owned `unique_ptr` in Rust, prefer to convert it to a Rust type using
/// [`into_box`], [`into_pin_box`] or [`into_inner`].
///
/// ## Moved-from `unique_ptr`
///
/// Unlike `Box`, `unique_ptr` may hold a `nullptr` value when moved-from (via C++ move
/// semantics). This allows it to be ABI-compatible with C++'s `std::unique_ptr`. However,
/// `unique_ptr` is conventionally non-null, and some methods (e.g. `as_mut`, `into_box`,
/// `into_pin_box`, `into_inner`) will panic if the pointer is null.
///
/// ## `unique_ptr` vs `virtual_unique_ptr`
///
/// For `T` types with virtual destructors or which overload `operator delete`, Crubit will
/// instead generate bindings to `virtual_unique_ptr`. It is UB to use `unique_ptr` in that case.
///
/// ## Shallow const
///
/// `unique_ptr` has "shallow" `const` semantics: C++ allows accessing a non-`const` `T`
/// via a `const unique_ptr<T>&`, so the existence of a Rust `&unique_ptr<T>` does not guarantee
/// that the underlying `T` is not mutated. Therefore, to get access to `T`, you must have exclusive
/// access to the `unique_ptr`.
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
    // Note: the pointer is mutable, but `const` for covariance.
    ptr: *const T,
}

// SAFETY: unique_ptr exclusively owns `T` and adds no additional constraints on sending the
// pointer.
unsafe impl<T: Sized + Send> Send for unique_ptr<T> {}

// SAFETY: unique_ptr has "shallow" semantics, and you cannot do anything with a `&unique_ptr`
// except pass it to C++, where the unsafe operation of internal mutability requires C++ programmers
// to prove safety.
unsafe impl<T: Sized + Sync> Sync for unique_ptr<T> {}

impl<T: Sized> unique_ptr<T> {
    /// Allocates a new `unique_ptr` that owns the given object.
    pub fn new(value: T) -> Self {
        Box::new_in(value, Allocator).into()
    }

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
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self { ptr }
    }

    /// Returns `true` if `this` is a null pointer.
    pub fn is_null(this: &Self) -> bool {
        this.ptr.is_null()
    }

    /// Returns a raw pointer to the contents.
    ///
    /// Note that `unique_ptr` has "shallow const" semantics: see the [type documentation](Self#shallow-const)
    /// for details.
    pub fn as_ptr(this: &Self) -> *const T {
        this.ptr
    }

    /// Returns a mutable raw pointer to the contents.
    pub fn as_mut_ptr(this: &mut Self) -> *mut T {
        this.ptr as *mut T
    }

    /// Returns a copy of the held raw pointer to the owned object.
    ///
    /// Deprecated: prefer `as_ptr` or `as_mut_ptr` instead.
    pub fn get(&self) -> *mut T {
        self.ptr as *mut T
    }

    /// Releases the ownership of the object pointed to by `this`, replacing this `unique_ptr` with
    /// a null pointer.
    ///
    /// Where possible, prefer `into_raw` in order to avoid null `unique_ptr`s.
    pub fn release(this: &mut Self) -> *mut T {
        core::mem::replace(&mut this.ptr, null_mut()).cast_mut()
    }

    /// Consumes the `unique_ptr`, returning the owned pointer.
    pub fn into_raw(this: Self) -> *mut T {
        let mut this = core::mem::ManuallyDrop::new(this);
        Self::release(&mut this)
    }

    /// Consumes the `unique_ptr`, returning the owned object.
    ///
    /// # Panics
    ///
    /// Panics if `this` is null.
    #[track_caller]
    pub fn into_box(this: Self) -> Box<T, Allocator>
    where
        T: Unpin,
    {
        assert!(!unique_ptr::is_null(&this), "attempted to convert null `unique_ptr` into `Box`");
        // SAFETY: `value` is exclusively owned by this `unique_ptr`, and `Box::from_raw`
        // preserves the pointer's alignment and size. The pointee is not pinned.
        unsafe { Box::from_raw_in(unique_ptr::into_raw(this), Allocator) }
    }

    /// Consumes the `unique_ptr`, returning the owned, pinned object.
    ///
    /// If `T: Unpin`, prefer using [`unique_ptr::into_box`].
    ///
    /// # Panics
    ///
    /// Panics if `this` is null.
    #[track_caller]
    pub fn into_pin_box(this: Self) -> Pin<Box<T, Allocator>> {
        assert!(
            !unique_ptr::is_null(&this),
            "attempted to convert null `unique_ptr` into `Pin<Box>`"
        );
        // SAFETY: `value` is exclusively owned by this `unique_ptr`, and `Box::from_raw`
        // preserves the pointer's alignment and size.
        unsafe { Pin::new_unchecked(Box::from_raw_in(unique_ptr::into_raw(this), Allocator)) }
    }

    /// Consumes the `unique_ptr`, returning the owned object.
    ///
    /// # Panics
    ///
    /// Panics if `this` is null.
    #[track_caller]
    pub fn into_inner(this: Self) -> T
    where
        T: Unpin,
    {
        *unique_ptr::into_box(this)
    }

    /// Returns an shared reference to the owned object, if-non-null, or None otherwise.
    ///
    /// Note that it is not safe to obtain a `&T` from a `&unique_ptr`, because the pointed-to `T`
    /// may be mutated when a `&unique_ptr` is shared between C++ and Rust.
    #[deprecated = "`as_ref` on `unique_ptr` has no valid use cases-- prefer `as_pin` or `as_mut`"]
    pub fn as_ref(this: &mut Self) -> Option<&T> {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        unsafe { this.ptr.as_ref() }
    }

    /// Returns an exclusive reference to the owned object, if-non-null, or None otherwise.
    pub fn as_pin(this: &mut Self) -> Option<Pin<&mut T>> {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        // The pointee is pinned.
        unsafe { Some(Pin::new_unchecked((this.ptr as *mut T).as_mut()?)) }
    }
}

impl<T: Unpin> AsMut<T> for unique_ptr<T> {
    /// Note: this method will panic if `this` is null.
    fn as_mut(&mut self) -> &mut T {
        // SAFETY: `self.ptr` is either null or points to a valid, exclusively owned, `T`.
        unsafe { (self.ptr as *mut T).as_mut().unwrap() }
    }
}

// Creates a `unique_ptr` from a `T`.
impl<T> From<T> for unique_ptr<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

// --- Conversions from `Box` and `Pin<Box>` to `unique_ptr` ---

impl<T> From<Box<T, Allocator>> for unique_ptr<T> {
    fn from(value: Box<T, Allocator>) -> Self {
        Self { ptr: Box::into_raw_with_allocator(value).0 }
    }
}

impl<T> From<Pin<Box<T, Allocator>>> for unique_ptr<T> {
    fn from(value: Pin<Box<T, Allocator>>) -> Self {
        Self {
            // Safety: `unique_ptr` continues to treat the pointee as pinned.
            ptr: Box::into_raw_with_allocator(unsafe { Pin::into_inner_unchecked(value) }).0,
        }
    }
}

impl<T> Drop for unique_ptr<T> {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        };
        // Safety: `self.ptr` is nonnull and exclusively owned by this `unique_ptr`.
        drop(unsafe { Box::<T, Allocator>::from_raw_in(self.ptr as *mut T, Allocator) });
    }
}

/// A smart pointer that owns and manages a polymorphic object with base class `T`.
///
/// This type is ABI-compatible with C++'s `std::unique_ptr<T>`, where `T` is a base class with a
/// virtual destructor.
///
/// ## Moved-from `virtual_unique_ptr`
///
/// Unlike `Box`, `virtual_unique_ptr` may hold a `nullptr` value when moved-from (via C++ move
/// semantics). This allows it to be ABI-compatible with C++'s `std::unique_ptr`. However,
/// `virtual_unique_ptr` is conventionally non-null.
///
/// ## Shallow const
///
/// `virtual_unique_ptr` has "shallow" `const` semantics: C++ allows accessing a non-`const` `T`
/// via a `const unique_ptr<T>&`, so the existence of a Rust `&virtual_unique_ptr<T>` does not guarantee
/// that the underlying `T` is not mutated. Therefore, to get access to `T`, you must have exclusive
/// access to the `virtual_unique_ptr`.
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
    // Note: the pointer is mutable, but `const` for covariance.
    ptr: *const T,
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
    /// Deprecated: prefer `from_raw`.
    ///
    /// # Safety
    ///
    /// `ptr` must either be null, or allocated by C++ `new`.
    ///
    /// This pins `ptr`, and the same preconditions apply as for `Pin::new_unchecked`.
    pub unsafe fn new(ptr: *mut T) -> Self {
        // SAFETY: Forwarding to `from_raw` which has identical preconditions.
        unsafe { Self::from_raw(ptr) }
    }

    /// Takes ownership of the provided raw pointer to a polymorphic type.
    ///
    /// If `T` doesn't implement `Delete`, use [`unique_ptr`] instead.
    ///
    /// # Safety
    ///
    /// `ptr` must either be null, or allocated by C++ `new`.
    ///
    /// This pins `ptr`, and the same preconditions apply as for `Pin::new_unchecked`.
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self { ptr }
    }

    /// Returns a raw pointer to the contents.
    ///
    /// Note that `virtual_unique_ptr` has "shallow const" semantics: see the
    /// [type documentation](Self#shallow-const) for details.
    pub fn as_ptr(this: &Self) -> *const T {
        this.ptr
    }

    /// Returns a mutable raw pointer to the contents.
    pub fn as_mut_ptr(this: &mut Self) -> *mut T {
        this.ptr as *mut T
    }

    /// Returns a copy of the held raw pointer to the owned object.
    ///
    /// Deprecated: prefer `as_ptr` or `as_mut_ptr` instead.
    pub fn get(&self) -> *mut T {
        self.ptr as *mut T
    }

    /// Returns `true` if `this` is a null pointer.
    pub fn is_null(this: &Self) -> bool {
        this.ptr.is_null()
    }

    /// Releases the ownership of the object pointed to by `this`, replacing this `unique_ptr` with
    /// a null pointer.
    ///
    /// Where possible, prefer `into_raw` in order to avoid null `unique_ptr`s.
    pub fn release(this: &mut Self) -> *mut T {
        core::mem::replace(&mut this.ptr, null_mut()).cast_mut()
    }

    /// Consumes the `unique_ptr`, returning the owned pointer.
    pub fn into_raw(this: Self) -> *mut T {
        let mut this = core::mem::ManuallyDrop::new(this);
        Self::release(&mut this)
    }

    /// Returns an shared reference to the owned object, if-non-null, or None otherwise.
    ///
    /// Note that it is not safe to obtain a `&T` from a `&unique_ptr`, because the pointed-to `T`
    /// may be mutated when a `&unique_ptr` is shared between C++ and Rust.
    #[deprecated = "`as_ref` on `virtual_unique_ptr` has no valid use cases-- prefer `as_pin`"]
    pub fn as_ref(this: &mut Self) -> Option<&T> {
        // SAFETY: `this.ptr` is either null or points to a valid, exclusively owned, `T`.
        unsafe { this.ptr.as_ref() }
    }

    /// Returns an exclusive reference to the owned object, if-non-null, or None otherwise.
    pub fn as_pin(this: &mut Self) -> Option<Pin<&mut T>> {
        // SAFETY: `this.ptr` is either null or points to a valid, exclusively owned, `T`.
        // The pointee is pinned.
        unsafe { Some(Pin::new_unchecked((this.ptr as *mut T).as_mut()?)) }
    }
}

impl<T: Delete> From<T> for virtual_unique_ptr<T> {
    fn from(value: T) -> Self {
        unique_ptr::new(value).into()
    }
}

impl<T: Delete> From<unique_ptr<T>> for virtual_unique_ptr<T> {
    fn from(value: unique_ptr<T>) -> Self {
        Self { ptr: unique_ptr::into_raw(value) }
    }
}

impl<T: Delete> Drop for virtual_unique_ptr<T> {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        };
        unsafe {
            // SAFETY: valid and allocated with `new` per type invariants.
            T::delete(self.ptr as *mut T);
        }
    }
}
