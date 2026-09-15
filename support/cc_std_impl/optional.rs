// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::fmt::{Debug, Formatter, Result};
use core::mem::{ManuallyDrop, MaybeUninit};
use core::pin::Pin;

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

/// Rust layout-compatible implementation of C++ `std::optional<T>`, where `T` is `Copy`.
///
/// ## Relationship to [`trivial_optional`]
///
/// This type is [`trivial_optional<T>`](trivial_optional) plus a [`Drop`] impl which destroys the
/// engaged value and minus the `T: Copy` requirement. That makes it usable with any `T`, but Crubit
/// still needs [`trivial_optional<T>`] for fields of `Copy` structs to enable them to be `Copy`.
///
/// Every `trivial_optional<T>` converts into an `optional<T>` with [`From`]; the reverse
/// conversion requires `T: Copy`. Therefore, use `optional` when you can and [`trivial_optional`]
/// only when you must.
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
    pub const fn take(&mut self) -> Option<T> {
        self.inner.take()
    }
}

impl<T> Drop for optional<T> {
    fn drop(&mut self) {
        if self.inner.engaged {
            // SAFETY: payload is initialized because engaged is true.
            unsafe { self.inner.payload.assume_init_drop() };
        }
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
