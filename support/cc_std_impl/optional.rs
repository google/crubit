// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::fmt::{Debug, Formatter, Result};
use core::marker::PhantomData;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::pin::Pin;
use ctor::{Ctor, Infallible, SelfCtor};

/// Rust layout-compatible implementation of C++ `std::optional<T>`, where `T` is `Copy`.
///
/// See [`optional`] for more details.
// This is the layout that libc++ uses, which is guaranteed to be stable:
// https://github.com/llvm/llvm-project/blob/main/libc/src/__support/CPP/optional.h#L34-L39
#[crubit_annotate::cpp_layout_equivalent(
    cpp_type = "::std::optional<{T}>",
    include_path = "<optional>"
)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct trivial_optional<T> {
    // Safety invariant: payload is initialized if and only if engaged is true.
    payload: MaybeUninit<T>,
    engaged: bool,
}

impl<T: Copy> Copy for trivial_optional<T> {}
impl<T: Copy> Clone for trivial_optional<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> trivial_optional<T> {
    /// Returns a `trivial_optional` which does not contain a value (C++ `std::nullopt`).
    pub const fn nullopt() -> Self
    where
        T: Copy,
    {
        trivial_optional { payload: MaybeUninit::uninit(), engaged: false }
    }

    /// Returns a `trivial_optional` containing `value`.
    pub const fn new(value: T) -> Self
    where
        T: Copy,
    {
        trivial_optional { payload: MaybeUninit::new(value), engaged: true }
    }

    /// Returns true if the `trivial_optional` contains a value.
    pub const fn is_some(&self) -> bool {
        self.engaged
    }

    /// Returns true if the `trivial_optional` does not contain a value.
    pub const fn is_none(&self) -> bool {
        !self.engaged
    }

    /// Returns a reference to the contained value, if any.
    pub const fn as_ref(&self) -> Option<&T> {
        if self.engaged {
            // SAFETY: payload is initialized because engaged is true.
            Some(unsafe { self.payload.assume_init_ref() })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the contained value, if any.
    pub const fn as_mut(&mut self) -> Option<&mut T> {
        if self.engaged {
            // SAFETY: payload is initialized because engaged is true.
            Some(unsafe { self.payload.assume_init_mut() })
        } else {
            None
        }
    }

    /// Returns a pinned mutable reference to the contained value, if any.
    ///
    /// Use this when the `trivial_optional` is pinned; otherwise use
    /// [`as_mut`](trivial_optional::as_mut).
    pub const fn as_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>> {
        // SAFETY: we never move out of `this`, and the payload is pinned whenever `self` is.
        let this = unsafe { Pin::into_inner_unchecked(self) };
        match this.as_mut() {
            // SAFETY: the payload is pinned because `self` was.
            Some(value) => Some(unsafe { Pin::new_unchecked(value) }),
            None => None,
        }
    }

    /// Removes the contained value, if any, leaving the `trivial_optional` empty.
    pub const fn take(&mut self) -> Option<T> {
        if self.engaged {
            // Clear the flag first, so that the payload is not read (or dropped) twice if
            // the caller leaks or panics.
            self.engaged = false;
            // SAFETY: payload was initialized because engaged was true, and `engaged` is now
            // false, so it will not be dropped again.
            Some(unsafe { self.payload.assume_init_read() })
        } else {
            None
        }
    }
}

impl<T: Copy> Default for trivial_optional<T> {
    fn default() -> Self {
        trivial_optional::nullopt()
    }
}

impl<T: PartialEq> PartialEq for trivial_optional<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T: Eq> Eq for trivial_optional<T> {}

impl<T: Copy> From<Option<T>> for trivial_optional<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => trivial_optional::new(value),
            None => trivial_optional::nullopt(),
        }
    }
}

impl<T> From<trivial_optional<T>> for Option<T> {
    fn from(mut value: trivial_optional<T>) -> Self {
        value.take()
    }
}

impl<T: Debug> Debug for trivial_optional<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self.as_ref(), f)
    }
}

/// Rust layout-compatible implementation of C++ `std::optional<T>`.
///
/// ## Missing `Copy` implementation and [`trivial_optional`]
///
/// [`optional<T>`] is not `Copy`, even when `T: Copy`. If you need a `Copy` version of e.g.
/// C++ `std::optional<int>`, use [`trivial_optional<i32>`].
///
/// This restriction exists because it is not possible to make Rust's manual `Drop` impl for this
/// type conditional on whether or not the inner `T` has drop glue, so even `Copy` types would
/// incur a destructor call.
///
/// Prefer to use `optional<T>` over `trivial_optional<T>` in generic code. Any
/// `trivial_optional<T>` can be converted into `optional<T>` via [`From`], so `optional` is more
/// flexible.
#[crubit_annotate::cpp_layout_equivalent(
    cpp_type = "::std::optional<{T}>",
    include_path = "<optional>"
)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct optional<T> {
    // The representation and all of the payload logic. Note that holding a `trivial_optional<T>`
    // does not imply `T: Copy`: it is simply the destructor-free core, and this type is what
    // supplies the destructor.
    inner: trivial_optional<T>,
}

impl<T> optional<T> {
    /// Returns an `optional` which does not contain a value (C++ `std::nullopt`).
    pub const fn nullopt() -> Self {
        optional { inner: trivial_optional { payload: MaybeUninit::uninit(), engaged: false } }
    }

    /// Returns an `optional` containing `value`.
    pub const fn new(value: T) -> Self {
        optional { inner: trivial_optional { payload: MaybeUninit::new(value), engaged: true } }
    }

    /// Returns true if the `optional` contains a value.
    pub const fn is_some(&self) -> bool {
        self.inner.is_some()
    }

    /// Returns true if the `optional` does not contain a value.
    pub const fn is_none(&self) -> bool {
        self.inner.is_none()
    }

    /// Returns a reference to the contained value, if any.
    pub const fn as_ref(&self) -> Option<&T> {
        self.inner.as_ref()
    }

    /// Returns a mutable reference to the contained value, if any.
    pub const fn as_mut(&mut self) -> Option<&mut T> {
        self.inner.as_mut()
    }

    /// Returns a pinned mutable reference to the contained value, if any.
    ///
    /// Use this when the `optional` is pinned; otherwise use [`as_mut`](optional::as_mut).
    pub const fn as_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>> {
        // SAFETY: `optional` is `#[repr(transparent)]` over `inner`, and we never move out of it.
        unsafe { Pin::new_unchecked(&mut Pin::into_inner_unchecked(self).inner) }.as_pin_mut()
    }

    /// Removes the contained value, if any, leaving the `optional` empty.
    ///
    /// This moves the value out, and so is unavailable when `self` is pinned. For a `!Unpin`
    /// `T`, use [`reset`](optional::reset) to destroy the value where it lives instead.
    pub const fn take(&mut self) -> Option<T> {
        self.inner.take()
    }

    /// Destroys the contained value, if any, leaving the `optional` empty.
    ///
    /// This is the equivalent of C++ `std::optional::reset()`, and is the pinned counterpart to
    /// [`take`](optional::take): it works even when `T` is `!Unpin`, because the value is
    /// destroyed in place rather than moved out.
    pub fn reset(self: Pin<&mut Self>) {
        // SAFETY: we never move the payload; we only clear `engaged` and drop `payload` in place.
        let this = unsafe { Pin::into_inner_unchecked(self) };
        if this.inner.engaged {
            // Clear the flag first, so that the payload is not dropped twice if the payload's
            // destructor panics.
            this.inner.engaged = false;
            // SAFETY: payload was initialized because engaged was true, and `engaged` is now
            // false, so it will not be dropped again. The payload is dropped where it lives, so
            // a pinned payload is never moved.
            unsafe { this.inner.payload.assume_init_drop() };
        }
    }

    /// Destroys the contained value, if any, and constructs a new one in place from `value`.
    ///
    /// This is the equivalent of C++ `std::optional::emplace()`. It is the only way to store a
    /// value in a pinned `optional`, and therefore the only way to store a `!Unpin` `T` such as
    /// [`string`](crate::std::string), which cannot be produced by value.
    ///
    /// If `value` fails to construct, `self` is left empty.
    pub fn emplace<C: Ctor<Output = T>>(
        mut self: Pin<&mut Self>,
        value: C,
    ) -> core::result::Result<(), C::Error> {
        self.as_mut().reset();
        // SAFETY: nothing below moves the payload; it is constructed in place.
        let this = unsafe { Pin::into_inner_unchecked(self) };
        // SAFETY: `engaged` is false, so the payload is uninitialized and valid for writes.
        // `MaybeUninit<T>` is `repr(transparent)` over `T`, so the cast is valid. `Ctor::ctor`
        // pins the payload, which `optional` upholds: it never moves an initialized payload.
        unsafe { value.ctor((&raw mut this.inner.payload).cast::<T>())? };
        this.inner.engaged = true;
        Ok(())
    }
}

impl<T> Drop for optional<T> {
    fn drop(&mut self) {
        // SAFETY: `Drop::drop` is called when `*self` is being destroyed and will never be moved
        // again, so pinning `self` to drop the payload in place is sound.
        unsafe { Pin::new_unchecked(self) }.reset();
    }
}

impl<T> Default for optional<T> {
    fn default() -> Self {
        optional::nullopt()
    }
}

impl<T: Clone> Clone for optional<T> {
    fn clone(&self) -> Self {
        match self.as_ref() {
            // `T::clone` returns a fresh value, so writing it into the payload is fine even
            // for `!Unpin` `T`: nothing was pinned yet.
            Some(value) => optional::new(value.clone()),
            None => optional::nullopt(),
        }
    }
}

impl<T: PartialEq> PartialEq for optional<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T: Eq> Eq for optional<T> {}

impl<T> From<Option<T>> for optional<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => optional::new(value),
            None => optional::nullopt(),
        }
    }
}

impl<T> From<optional<T>> for Option<T> {
    fn from(mut value: optional<T>) -> Self {
        value.take()
    }
}

impl<T: Debug> Debug for optional<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self.as_ref(), f)
    }
}

impl<T> From<trivial_optional<T>> for optional<T> {
    fn from(inner: trivial_optional<T>) -> Self {
        optional { inner }
    }
}

impl<T: Copy> From<optional<T>> for trivial_optional<T> {
    fn from(value: optional<T>) -> Self {
        ManuallyDrop::new(value).inner
    }
}

// =================================
// In-place construction via `Ctor`
// =================================
//
// A `T` which is `!Unpin` — most importantly [`string`](crate::std::string) — cannot be produced
// by value in Rust, so [`optional::new`] and [`From<Option<T>>`] cannot construct one. Instead,
// the payload is constructed directly into the `optional`'s storage by a [`Ctor`], and the
// resulting `optional` is itself only ever reachable through a `Pin`.

/// A [`Ctor`] which constructs an empty [`optional`] (C++ `std::nullopt`).
///
/// Returned by [`optional::ctor_nullopt`].
#[must_use = "Ctors do nothing unless emplaced"]
pub struct NulloptCtor<T>(PhantomData<fn() -> T>);

// SAFETY: `ctor` unconditionally initializes `dest` to a disengaged `optional`. A disengaged
// `optional` has no payload, so writing `engaged` alone fully initializes it.
unsafe impl<T> Ctor for NulloptCtor<T> {
    type Output = optional<T>;
    type Error = Infallible;

    unsafe fn ctor(self, dest: *mut optional<T>) -> core::result::Result<(), Infallible> {
        // SAFETY: the caller guarantees that `dest` is valid for writes. This establishes the
        // safety invariant of `trivial_optional`: `engaged` is false, so `payload` may be
        // uninitialized.
        unsafe { (&raw mut (*dest).inner.engaged).write(false) };
        Ok(())
    }
}

/// `!SelfCtor` to override the blanket `Ctor` impl for owned values.
impl<T> !SelfCtor for NulloptCtor<T> {}

/// A [`Ctor`] which constructs an [`optional`] containing the value produced by `C`.
///
/// Returned by [`optional::ctor_some`].
#[must_use = "Ctors do nothing unless emplaced"]
pub struct SomeCtor<C>(C);

// SAFETY: `ctor` initializes `engaged` unconditionally, and only sets it to true once `payload`
// has been initialized, which maintains the safety invariant of `trivial_optional`.
unsafe impl<C: Ctor> Ctor for SomeCtor<C>
where
    C::Output: Sized,
{
    type Output = optional<C::Output>;
    type Error = C::Error;

    unsafe fn ctor(self, dest: *mut Self::Output) -> core::result::Result<(), Self::Error> {
        // SAFETY: the caller guarantees that `dest` is valid for writes. `engaged` is written
        // first so that `dest` holds a valid (empty) `optional` even if the payload's
        // constructor panics or fails, which keeps `Drop` from reading an uninitialized payload.
        unsafe { (&raw mut (*dest).inner.engaged).write(false) };
        // SAFETY: the payload is uninitialized and valid for writes, and `MaybeUninit<T>` is
        // `repr(transparent)` over `T`. `Ctor::ctor` pins the payload, which `optional` upholds:
        // it never moves an initialized payload.
        unsafe { self.0.ctor((&raw mut (*dest).inner.payload).cast::<C::Output>())? };
        // SAFETY: `dest` is valid for writes, and the payload is now initialized.
        unsafe { (&raw mut (*dest).inner.engaged).write(true) };
        Ok(())
    }
}

/// `!SelfCtor` to override the blanket `Ctor` impl for owned values.
impl<C> !SelfCtor for SomeCtor<C> {}

impl<T> optional<T> {
    /// Returns a [`Ctor`] which constructs an empty `optional` in place (C++ `std::nullopt`).
    ///
    /// This is the counterpart to [`nullopt`](optional::nullopt) for a `T` which cannot be
    /// produced by value.
    ///
    /// ```
    /// # use cc_std::std::{optional, string};
    /// # use ctor::{emplace, CtorNew};
    /// let empty = emplace!(optional::<string>::ctor_nullopt());
    /// assert!(empty.is_none());
    /// ```
    pub fn ctor_nullopt() -> NulloptCtor<T> {
        NulloptCtor(PhantomData)
    }

    /// Returns a [`Ctor`] which constructs an `optional` containing the value produced by
    /// `value`, constructed directly into the `optional`'s storage.
    ///
    /// This is the counterpart to [`new`](optional::new) for a `T` which cannot be produced by
    /// value.
    ///
    /// Note that this is an inherent function rather than a [`CtorNew`](ctor::CtorNew) impl,
    /// because `ctor` provides a blanket `impl<T: Default> CtorNew<()> for T` which would
    /// overlap with it.
    ///
    /// ```
    /// # use cc_std::std::{optional, string};
    /// # use ctor::{emplace, CtorNew};
    /// let hello = emplace!(optional::ctor_some(string::ctor_new("hello")));
    /// assert_eq!(hello.as_ref().unwrap().as_slice(), b"hello");
    /// ```
    pub fn ctor_some<C: Ctor<Output = T>>(value: C) -> SomeCtor<C> {
        SomeCtor(value)
    }
}
