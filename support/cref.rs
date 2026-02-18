// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![no_std]
#![deny(missing_docs, warnings)]

//! This crate defines Rust types which correspond to C++ references:
//!
//! * `CRef<T>` <-> `const T&`
//! * `CMut<T>` <-> `T&`

use core::marker::PhantomData;
use core::pin::Pin;
use core::ptr::NonNull;

/// A reference similar to `const T&` in C++.
///
/// During its lifetime, this reference is guaranteed to be non-null,
/// non-dangling, and point to a value of type `T`.
///
/// Unlike Rust's shared references, mutations to the pointed-to value may occur
/// while this reference is still in use.
///
/// Additionally, this reference behaves similarly to `Pin<&T>`: the target
/// value must be pinned in memory unless `T: Unpin`.
#[repr(transparent)]
pub struct CRef<'a, T: ?Sized> {
    // Invariant: `ptr` is non-null, non-dangling, and points to a valid value of type `T`.
    // If `T` is not `Unpin`, the pointed-to value must be pinned in memory.
    ptr: NonNull<T>,
    marker: PhantomData<&'a T>,
}

impl<'a, T: ?Sized> CRef<'a, T> {
    /// Creates a `CRef` from a Rust reference.
    ///
    /// This method is only usable if `T: Unpin`. For other types, use `Box::pin` or `std::pin::pin`
    /// to create a pinned reference, then pass it to `CRef::from_pin`.
    pub const fn from_ref(r: &'a T) -> Self
    where
        T: Unpin,
    {
        // Safety: `r` is a valid Rust reference, so it is non-null, non-dangling, and points to a
        // valid value of type `T`.
        unsafe { Self::from_ptr(r) }
    }

    /// Creates a `CRef` from a pinned Rust reference.
    pub const fn from_pin(r: Pin<&'a T>) -> Self {
        // Safety: `r` is a valid pinned Rust reference, so it is non-null, non-dangling, and points
        // to a valid value of type `T`.
        unsafe { Self::from_ptr(Pin::get_ref(r)) }
    }

    /// Creates a `CRef` from a raw pointer.
    ///
    /// # Safety
    ///
    ///`ptr` must be non-null, non-dangling, and point to a value of type `T` for the lifetime of
    /// the resulting `CRef`.
    pub const unsafe fn from_ptr(ptr: *const T) -> Self {
        // Safety: `ptr` is a valid pointer to a `T`, so it is non-null, non-dangling, and points to
        // a valid value of type `T`.
        CRef { ptr: unsafe { NonNull::new_unchecked(ptr as *mut _) }, marker: PhantomData }
    }

    /// Returns a pinned reference to the target value. The target value *must* be unchanged
    /// for the lifetime of the returned reference.
    ///
    /// # Safety
    ///
    /// The target value of this reference must not be mutated during the lifetime of the returned
    /// reference.
    pub const unsafe fn unchanging(this: Self) -> Pin<&'a T> {
        // Safety: `CRef` guarantees that if `T` is not `Unpin`, then the target is pinned.
        unsafe { Pin::new_unchecked(&*this.ptr.as_ptr()) }
    }

    /// Converts this `CRef` to a `CMut` as if by C++'s `const_cast` conversion.
    ///
    /// # Safety
    ///
    /// The target value must be valid to mutate on the current thread. This is *not* valid to use
    /// with a `CRef` that was created via a Rust `&T` or `Pin<&T>`, e.g. via `CRef::from_ref` or
    /// `CRef::from_pin`.
    pub const unsafe fn const_cast(this: Self) -> CMut<'a, T> {
        CMut { ptr: this.ptr, marker: PhantomData }
    }

    /// Returns a raw pointer to the target value.
    pub const fn as_ptr(this: Self) -> *const T {
        this.ptr.as_ptr()
    }
}

/// A reference similar to `T&` in C++.
///
/// During its lifetime, this reference is guaranteed to be non-null,
/// non-dangling, and point to a value of type `T` which is valid to mutate on
/// the current thread.
///
/// Unlike Rust's mutable references, `CMut` is not guaranteed to be unique,
/// and other reads and writes to the pointed-to value may occur while this
/// reference is still in use. APIs which use `CMut` to mutate a value should
/// avoid mutations which may invalidate other existing references, such
/// as via iterator invalidation, setting an optional to "none" while a
/// reference to the innards persists, or any other invalidation of a
/// "child group" (see https://verdagon.dev/blog/group-borrowing#child-groups).
///
/// Additionally, this reference behaves similarly to `Pin<&mut T>`: the target
/// value must be pinned in memory unless `T: Unpin`.
#[repr(transparent)]
pub struct CMut<'a, T: ?Sized> {
    // Invariant: `ptr` is non-null, non-dangling, and points to a valid value of type `T`.
    // If `T` is not `Unpin`, the pointed-to value must be pinned in memory.
    ptr: NonNull<T>,
    marker: PhantomData<&'a mut T>,
}

impl<'a, T: ?Sized> CMut<'a, T> {
    /// Creates a `CMut` from a Rust reference.
    ///
    /// This method is only usable if `T: Unpin`. For other types, use `Box::pin` or `std::pin::pin`
    /// to create a pinned reference, then pass it to `CMut::from_pin`.
    pub const fn from_ref(r: &'a mut T) -> Self
    where
        T: Unpin,
    {
        // Safety: `r` is a valid Rust reference, so it is non-null, non-dangling, and points to a
        // valid value of type `T`.
        unsafe { Self::from_ptr(r as *mut T) }
    }

    /// Creates a `CMut` from a pinned Rust reference.
    pub const fn from_pin(r: Pin<&'a mut T>) -> Self {
        // Safety: `r` is a valid pinned Rust reference, so it is non-null, non-dangling, and points
        // to a valid value of type `T`.
        unsafe { Self::from_ptr(Pin::into_inner_unchecked(r) as *mut _) }
    }

    /// Creates a `CMut` from a raw pointer.
    ///
    /// # Safety
    ///
    /// For the remainder of the resuting `CMut`'s lifetime, `ptr` must be non-null, non-dangling,
    /// and point to a value of type `T` which is valid to mutate on the current thread.
    pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
        // Safety: `ptr` is a valid pointer to a `T`, so it is non-null, non-dangling, and points to
        // a valid value of type `T`.
        CMut { ptr: unsafe { NonNull::new_unchecked(ptr) }, marker: PhantomData }
    }

    /// Converts this `CMut` to a `CRef`.
    ///
    /// This conversion is "free" and does not impose any additional safety requirements.
    pub const fn into_const(this: Self) -> CRef<'a, T> {
        // Safety: `this` is a valid pointer to a `T`, so it is non-null, non-dangling, and points to
        // a valid value of type `T`.
        unsafe { CRef::from_ptr(this.ptr.as_ptr()) }
    }

    /// Returns a pinned reference to the target value. The target value *must* be unchanged
    /// for the lifetime of the returned reference.
    ///
    /// # Safety
    ///
    /// The target value of this reference must not be mutated during the lifetime of the returned
    /// reference.
    pub const unsafe fn unchanging(this: Self) -> Pin<&'a T> {
        // Safety: `CMut` guarantees that if `T` is not `Unpin`, then the target is pinned.
        unsafe { Pin::new_unchecked(&*this.ptr.as_ptr()) }
    }

    /// Returns a pinned mutable reference to the target value.
    ///
    /// # Safety
    ///
    /// For the lifetime of the returned reference, the target value of the reference must not be
    /// mutated or read via any other pointers or references.
    pub const unsafe fn unique(this: Self) -> Pin<&'a mut T> {
        // Safety: `CMut` guarantees that if `T` is not `Unpin`, then the target is pinned.
        unsafe { Pin::new_unchecked(&mut *this.ptr.as_ptr()) }
    }

    /// Returns a regular (non-pinned) mutable reference to the target value.
    /// This method is only available if `T: Unpin`.
    ///
    /// This method is equivalent to `Pin::into_inner(CMut::unique(c_mut))`.
    ///
    /// # Safety
    ///
    /// For the lifetime of the returned reference, the target value of the reference must not be
    /// mutated or read via any other pointers or references.
    pub const unsafe fn unpin_unique(this: Self) -> &'a mut T
    where
        T: Unpin,
    {
        // Safety: this function has the same safety requirements as `CMut::unique`.
        Pin::into_inner(unsafe { CMut::unique(this) })
    }

    /// Returns a raw pointer to the target value.
    pub const fn as_ptr(this: Self) -> *const T {
        this.ptr.as_ptr()
    }

    /// Returns a raw mutable pointer to the target value.
    pub const fn as_mut_ptr(this: Self) -> *mut T {
        this.ptr.as_ptr()
    }
}

// Both `CRef` and `CMut` are unconditionally `Clone` and `Copy`, as both
// assume that shared mutable access may occur.
impl<'a, T: ?Sized> Copy for CRef<'a, T> {}
impl<'a, T: ?Sized> Clone for CRef<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T: ?Sized> Copy for CMut<'a, T> {}
impl<'a, T: ?Sized> Clone for CMut<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

/// A trait corresponding to the common C++ notion of "thread-safe" types.
///
/// A type is `ThreadSafe` if its `const` and non-`const` methods and APIS are safe to access
/// concurrently via multiple threads.
///
/// `CRef<T>` and `CMut<T>` are only `Send` or `Sync` when `T: ThreadSafe`.
/// When accessing only `const` methods across multiple threads concurrently,
/// prefer to share a `&T` or `Pin<&T>` between threads, and then downgrade to
/// a `CRef<T>` using `CRef::from_ref` or `CRef::from_pin`.
///
/// Other C++ threading concepts align with existing Rust traits:
/// * Thread-compatible types: `const` methods of these types may be accessed concurrently.
///   Thread-compatible types are `Sync` and `Send`, but not `ThreadSafe`.
/// * Thread-unsafe types: neither `const` nor non-`const` methods may be accessed concurrently.
///   Thread-unsafe types are not `Sync`. In some cases, they may additionally have affinity to a
///   particular thread, additionally making them not `Send`.
///
/// # Safety
///
/// This trait is only safe to implement if all functions which consume a `CRef<Self>` or
/// `CMut<Self>` can be invoked concurrently from any thread.
unsafe trait ThreadSafe {}
unsafe impl<'a, T: ThreadSafe + ?Sized> Send for CRef<'a, T> {}
unsafe impl<'a, T: ThreadSafe + ?Sized> Sync for CRef<'a, T> {}
unsafe impl<'a, T: ThreadSafe + ?Sized> Send for CMut<'a, T> {}
unsafe impl<'a, T: ThreadSafe + ?Sized> Sync for CMut<'a, T> {}

mod private {
    // https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
    pub trait Sealed {}
}

/// A trait for types that can act like a `CRef<T>`.
///
/// # Safety
///
/// Types which implement this trait must be freely transmutable to a `CRef<Target>`.
pub unsafe trait CRefLike<'a>: private::Sealed {
    /// The type of the value being referenced.
    type Target: ?Sized;

    /// Converts `self` to a `CRef<'a, Self::Target>` as if by `transmute`.
    fn into_cref(self) -> CRef<'a, Self::Target>
    where
        Self: Sized,
    {
        // Safety: implementing this trait guarantees that transmuting `Self` to
        // `CRef<'a, Self::Target>` is valid.
        unsafe { *(&self as *const Self as *const CRef<'a, Self::Target>) }
    }
}

/// A trait for types that can act like a `CMut<T>`.
///
/// # Safety
///
/// Types which implement this trait must be freely transmutable to a `CMut<Target>`.
pub unsafe trait CMutLike<'a>: CRefLike<'a> {
    /// Converts `self` to a `CRef<'a, Self::Target>` as if by `transmute`.
    fn into_cmut(mut self) -> CMut<'a, Self::Target>
    where
        Self: Sized,
    {
        // Safety: implementing this trait guarantees that transmuting `Self` to
        // `CMut<'a, Self::Target>` is valid.
        unsafe { *(&mut self as *mut Self as *mut CMut<'a, Self::Target>) }
    }
}

/// A macro designed for use in autogenerated Rust bindings to C++ APIs.
///
/// When used in argument position, this will accept any type which acts like
/// a const reference to the target type.
///
/// Note that use of this macro in argument position will result in a function which is generic
/// over the argument type. As with other conversion-style generics, prefer to perform the
/// converison and then immediately delegate to a non-generic helper function in order to avoid
/// monomorphization-induced code bloat.
#[macro_export]
macro_rules! CRefTo {
    ($lt:lifetime, $target_type:ty) => {
        impl $crate::CRefLike<$lt, Target=$target_type>
    }
}

/// A macro designed for use in autogenerated Rust bindings to C++ APIs.
///
/// When used in argument position, this will accept any type which acts like
/// a const reference to the target type.
///
/// Note that use of this macro in argument position will result in a function which is generic
/// over the argument type. As with other conversion-style generics, prefer to perform the
/// converison and then immediately delegate to a non-generic helper function in order to avoid
/// monomorphization-induced code bloat.
#[macro_export]
macro_rules! CMutTo {
    ($lt:lifetime, $target_type:ty) => {
        impl $crate::CMutLike<$lt, Target=$target_type>
    }
}

impl<'a, T: ?Sized> private::Sealed for CRef<'a, T> {}
impl<'a, T: ?Sized> private::Sealed for CMut<'a, T> {}
impl<T: ?Sized> private::Sealed for &T {}
impl<T: ?Sized> private::Sealed for &mut T {}
impl<T: ?Sized> private::Sealed for Pin<&T> {}
impl<T: ?Sized> private::Sealed for Pin<&mut T> {}

unsafe impl<'a, T: ?Sized> CRefLike<'a> for CRef<'a, T> {
    type Target = T;
}
unsafe impl<'a, T: ?Sized> CRefLike<'a> for CMut<'a, T> {
    type Target = T;
}
unsafe impl<'a, T: ?Sized> CMutLike<'a> for CMut<'a, T> {}
unsafe impl<'a, T: ?Sized> CRefLike<'a> for &'a T
where
    T: Unpin,
{
    type Target = T;
}
unsafe impl<'a, T: ?Sized> CRefLike<'a> for &'a mut T
where
    T: Unpin,
{
    type Target = T;
}
unsafe impl<'a, T: ?Sized> CMutLike<'a> for &'a mut T where T: Unpin {}
unsafe impl<'a, T: ?Sized> CRefLike<'a> for Pin<&'a T> {
    type Target = T;
}
unsafe impl<'a, T: ?Sized> CRefLike<'a> for Pin<&'a mut T> {
    type Target = T;
}
unsafe impl<'a, T: ?Sized> CMutLike<'a> for Pin<&'a mut T> {}
