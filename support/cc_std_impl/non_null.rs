// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Abstractions for fallibly dereferenceable pointers and their guaranteed non-null wrappers.
//!
//! In C++, pointer and smart pointer types such as `std::unique_ptr` and `std::shared_ptr` can be
//! in a null state (for example, after being moved from, default-constructed, or reset). In Rust,
//! dereferencing via [`Deref`] or [`DerefMut`] expects a valid, non-null target.
//!
//! This module provides:
//! - [`StableNullness`]: A trait for pointer types that may be null, reporting whether they
//!   currently are, and guaranteeing that the answer does not change on its own.
//! - [`TryDeref`], [`TryDerefMut`], and [`TryDerefPin`]: Traits for accessing the target of a
//!   [`StableNullness`], each pairing an unchecked accessor with a derived, checked `try_*`
//!   accessor yielding an [`Option`].
//! - [`NonNull<Ptr>`]: A `#[repr(transparent)]` wrapper over a [`StableNullness`] type `Ptr` that
//!   guarantees the underlying pointer is non-null, and so can use the unchecked accessors to
//!   implement [`Deref`] and [`DerefMut`].
//!
//! # A shared precondition
//!
//! The unchecked accessors are not checked accessors with the check taken out; they have no
//! null-handling behavior to skip in the first place. Rather, all three share a single
//! precondition, that [`is_null`](StableNullness::is_null) returns `false` for the pointer, which
//! the caller is responsible for establishing.
//!
//! [`NonNull`] establishes that precondition once, when it is constructed, and thereafter relies
//! on [`StableNullness`]'s guarantee to know that it still holds at every subsequent access.
//! Callers holding a pointer directly can instead establish it per-access by using the `try_*`
//! accessors, which is all those accessors do.

use core::ops::{Deref, DerefMut};
use core::pin::Pin;

/// Asserts in debug builds that `ptr` is non-null, and does nothing in release builds.
///
/// [`NonNull`] checks for non-nullness once, at construction, and thereafter relies on
/// [`StableNullness`] to keep that check valid. This guards that reliance without paying for it in
/// release builds.
#[inline]
#[track_caller]
fn debug_assert_non_null(ptr: &impl StableNullness) {
    debug_assert!(
        !ptr.is_null(),
        "NonNull is null: either C++ moved out of it, or its StableNullness impl is unsound"
    );
}

/// A pointer type whose nullness can be checked once and relied on afterwards.
///
/// # Safety
///
/// Once [`is_null`](StableNullness::is_null) returns `false` for a given value, every subsequent
/// call must also return `false`, until the value is either mutated through a `&mut Self`
/// reference or moved from (including by a C++ move). In particular, implementers must not use
/// interior mutability to transition from non-null to null.
///
/// [`NonNull`] checks for non-nullness only once, at construction, so it must be able to rely on
/// that check remaining valid in order to elide subsequent checks.
pub unsafe trait StableNullness {
    /// Returns whether the pointer is currently null.
    fn is_null(&self) -> bool;
}

/// Shared access to the target of a [`StableNullness`].
///
/// [`deref_unchecked`](TryDeref::deref_unchecked) is preconditioned on the pointer being non-null;
/// [`try_deref`](TryDeref::try_deref) establishes that precondition by checking.
pub trait TryDeref: StableNullness {
    /// The pointee or referenced target type.
    type Target: ?Sized;

    /// Dereferences the pointer.
    ///
    /// # Safety
    ///
    /// [`is_null`](StableNullness::is_null) must return `false` for `self`.
    unsafe fn deref_unchecked(&self) -> &Self::Target;

    /// Attempts to dereference the pointer.
    ///
    /// Users looking to obtain a non-null reference to the pointer wrapper itself may also be
    /// interested in [`NonNull::from_ref`].
    fn try_deref(&self) -> Option<&Self::Target> {
        if self.is_null() {
            return None;
        }
        // SAFETY: `is_null` just returned `false`.
        Some(unsafe { self.deref_unchecked() })
    }
}

/// Mutable access to the target of a [`StableNullness`].
///
/// [`deref_mut_unchecked`](TryDerefMut::deref_mut_unchecked) is preconditioned on the pointer
/// being non-null; [`try_deref_mut`](TryDerefMut::try_deref_mut) establishes that precondition by
/// checking.
pub trait TryDerefMut: TryDeref {
    /// Mutably dereferences the pointer.
    ///
    /// # Safety
    ///
    /// [`is_null`](StableNullness::is_null) must return `false` for `self`.
    unsafe fn deref_mut_unchecked(&mut self) -> &mut Self::Target;

    /// Attempts to mutably dereference the pointer.
    ///
    /// Users looking to obtain a non-null mutable reference to the pointer wrapper itself may also
    /// be interested in [`NonNull::from_mut`].
    fn try_deref_mut(&mut self) -> Option<&mut Self::Target> {
        if self.is_null() {
            return None;
        }
        // SAFETY: `is_null` just returned `false`.
        Some(unsafe { self.deref_mut_unchecked() })
    }
}

/// Pinned mutable access to the target of a [`StableNullness`].
///
/// [`deref_pin_unchecked`](TryDerefPin::deref_pin_unchecked) is preconditioned on the pointer
/// being non-null; [`try_deref_pin`](TryDerefPin::try_deref_pin) establishes that precondition by
/// checking.
///
/// # Relationship with [`TryDerefMut`]
///
/// Any type implementing `TryDerefPin` automatically receives a blanket implementation of
/// [`TryDerefMut`] whenever `Self::Target: Unpin`. Therefore, implementers of pointer types with
/// stable target memory locations should implement `TryDerefPin` rather than [`TryDerefMut`].
///
/// For targets that are `!Unpin` (common for C++ types with non-trivial move semantics or
/// self-referential layouts), `TryDerefPin` allows obtaining a [`Pin<&mut Self::Target>`] via
/// [`NonNull::deref_pin`] while statically preventing unpinned `&mut Self::Target` references from
/// moving the target out of its pinned location.
pub trait TryDerefPin: TryDeref {
    /// Mutably dereferences the pointer into a pinned reference.
    ///
    /// # Safety
    ///
    /// [`is_null`](StableNullness::is_null) must return `false` for `self`.
    unsafe fn deref_pin_unchecked(&mut self) -> Pin<&mut Self::Target>;

    /// Attempts to mutably dereference the pointer into a pinned reference.
    fn try_deref_pin(&mut self) -> Option<Pin<&mut Self::Target>> {
        if self.is_null() {
            return None;
        }
        // SAFETY: `is_null` just returned `false`.
        Some(unsafe { self.deref_pin_unchecked() })
    }
}

impl<Ptr: TryDerefPin> TryDerefMut for Ptr
where
    Ptr::Target: Unpin,
{
    unsafe fn deref_mut_unchecked(&mut self) -> &mut Self::Target {
        // SAFETY: The caller guarantees that `self` is non-null.
        Pin::into_inner(unsafe { self.deref_pin_unchecked() })
    }
}

/// A zero-cost wrapper around a [`StableNullness`] type that guarantees it is not null.
///
/// `NonNull<Ptr>` is `#[repr(transparent)]`, guaranteeing the exact same memory layout and ABI as
/// the wrapped type `Ptr`.
///
/// By enforcing non-nullness at construction time, `NonNull<Ptr>` safely implements [`Deref`] (and
/// [`DerefMut`] when `Ptr: TryDerefMut`), providing direct access to the underlying
/// [`Target`](TryDeref::Target) without requiring callers to check for null on every access.
///
/// # Examples
///
/// ```ignore
/// let ptr: unique_ptr<i32> = ...;
/// if let Some(non_null) = NonNull::new(ptr) {
///     // Safely dereference without manual null checks:
///     assert_eq!(*non_null, 42);
/// }
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NonNull<Ptr: StableNullness> {
    // Invariant: this field was checked to be non-null at construction time, and `StableNullness`'s
    // contract keeps that check valid. No accessor hands out a `&mut Ptr`, so the only way to
    // reach a null `NonNull` is for C++ to move out of a shared reference to it, which is a bug
    // in that C++ code. `debug_assert_non_null` guards against both possibilities.
    ptr: Ptr,
}

impl<Ptr: StableNullness> NonNull<Ptr> {
    /// Constructs a `NonNull<Ptr>` if `ptr` is non-null, or returns `None` if `ptr` is null.
    #[inline]
    pub fn new(ptr: Ptr) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(NonNull { ptr })
        }
    }

    /// Converts a reference to a pointer `&Ptr` into `&NonNull<Ptr>` if non-null, or
    /// returns `None` if null.
    #[inline]
    pub fn from_ref(ptr: &Ptr) -> Option<&Self> {
        if ptr.is_null() {
            None
        } else {
            // SAFETY: NonNull is a transparent wrapper around Ptr.
            Some(unsafe { &*(ptr as *const _ as *const NonNull<Ptr>) })
        }
    }

    /// Converts a mutable reference `&mut Ptr` into `&mut NonNull<Ptr>` if non-null, or returns
    /// `None` if null.
    #[inline]
    pub fn from_mut(ptr: &mut Ptr) -> Option<&mut Self> {
        if ptr.is_null() {
            None
        } else {
            // SAFETY: NonNull is a transparent wrapper around Ptr.
            Some(unsafe { &mut *(ptr as *mut _ as *mut NonNull<Ptr>) })
        }
    }

    /// Returns the underlying `Ptr`.
    ///
    /// This is an associated function rather than a method taking `self` to avoid name collisions
    /// with methods on the underlying [`Target`](TryDeref::Target).
    #[inline]
    pub fn into_inner(this: Self) -> Ptr {
        this.ptr
    }

    /// Returns a reference to the underlying `Ptr`.
    ///
    /// This is an associated function rather than a method taking `&self` to avoid name collisions
    /// with methods on the underlying [`Target`](TryDeref::Target).
    #[inline]
    pub fn as_inner(this: &Self) -> &Ptr {
        &this.ptr
    }

    /// Returns a pinned mutable reference to the underlying target.
    #[inline]
    #[track_caller]
    pub fn deref_pin(this: &mut Self) -> Pin<&mut Ptr::Target>
    where
        Ptr: TryDerefPin,
    {
        debug_assert_non_null(&this.ptr);
        // SAFETY: `this.ptr` was non-null when this `NonNull` was constructed, and `StableNullness`
        // promises that it stays non-null unless `this` is moved from, which callers must not do
        // before reading through it.
        unsafe { this.ptr.deref_pin_unchecked() }
    }
}

impl<Ptr: TryDeref> Deref for NonNull<Ptr> {
    type Target = Ptr::Target;

    #[inline]
    #[track_caller]
    fn deref(&self) -> &Self::Target {
        debug_assert_non_null(&self.ptr);
        // SAFETY: `self.ptr` was non-null when this `NonNull` was constructed, and `StableNullness`
        // promises that it stays non-null unless `self` is moved from, which callers must not do
        // before reading through it.
        unsafe { self.ptr.deref_unchecked() }
    }
}

impl<Ptr: TryDerefMut> DerefMut for NonNull<Ptr> {
    #[inline]
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        debug_assert_non_null(&self.ptr);
        // SAFETY: `self.ptr` was non-null when this `NonNull` was constructed, and `StableNullness`
        // promises that it stays non-null unless `self` is moved from, which callers must not do
        // before reading through it.
        unsafe { self.ptr.deref_mut_unchecked() }
    }
}
