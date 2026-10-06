// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! [`Nullable`], the explicit opt-in to nullness for C++ smart pointers.
//!
//! In C++, `std::unique_ptr` and `std::shared_ptr` can always be null. Following
//! <internal link>, whose goal is that pointers are non-null unless annotated
//! `absl_nullable`, Crubit maps a smart pointer that is known to be non-null to the bare Rust type
//! (e.g. [`unique_ptr<T>`](crate::std::unique_ptr)), and one that may be null to
//! [`Nullable<unique_ptr<T>>`](Nullable).
//!
//! `Nullable` deliberately has almost no API of its own. Adding `absl_nonnull` to a C++ API
//! changes its Rust type from `Nullable<Ptr>` to `Ptr`, so any method on `Nullable` would break
//! its callers when that happens. Instead, callers should convert to an [`Option`] as soon as
//! possible using [`OptionLike`], which both `Ptr` and `Nullable<Ptr>` implement:
//!
//! ```ignore
//! use cc_std::std::OptionLike;
//! // Compiles whether `make_foo` returns `Nullable<unique_ptr<Foo>>` or `unique_ptr<Foo>`.
//! let foo: Option<unique_ptr<Foo>> = make_foo().into_option();
//! ```
//!
//! See crubit.rs-nullable for the design.

use crate::std::{shared_ptr, unique_ptr, virtual_unique_ptr, Delete};
use core::fmt::{Debug, Formatter, Result};

/// A smart pointer type that can be null, and so can be wrapped in [`Nullable`].
///
/// Implemented by [`unique_ptr`], [`virtual_unique_ptr`], and [`shared_ptr`].
///
/// # Safety
///
/// [`is_null`](SupportsNullable::is_null) must return `true` if and only if the wrapped pointer is
/// null. [`null`](SupportsNullable::null) must return a value for which `is_null` returns `true`,
/// and which is safe to drop.
pub unsafe trait SupportsNullable: Sized {
    /// Returns whether or not `this` is null.
    ///
    /// This is used to answer the question that the `_Nullable` annotation from C++ requires the
    /// user to answer before dereferencing. It has no impact on whether or not `Self` should be
    /// dropped when wrapped in `Nullable<Self>`.
    fn is_null(this: &Nullable<Self>) -> bool;

    /// Returns a null value of type `Self`.
    fn null() -> Nullable<Self>;
}

/// A [`SupportsNullable`] pointer that may be null.
///
/// This is the Rust equivalent of an `absl_nullable` (or unannotated) C++ smart pointer. The bare
/// `Ptr` type is the equivalent of an `absl_nonnull` one.
///
/// `Nullable<Ptr>` is `#[repr(transparent)]`, so it has the same layout and ABI as `Ptr`.
///
/// # Proper usage
///
/// Avoid naming this type outside of generated bindings. Instead, convert it to an [`Option`] as
/// soon as possible using [`OptionLike`]: see the [module documentation](self) for why.
#[crubit_annotate::cpp_layout_equivalent(
    cpp_type = "{Ptr} crubit_nullable",
    include_path = "<crubit/support/annotations_internal.h>"
)]
#[repr(transparent)]
pub struct Nullable<Ptr: SupportsNullable> {
    ptr: Ptr,
}

// There are intentionally no public methods on `Nullable`, so that adding `absl_nonnull` in C++
// (which changes the Rust type from `Nullable<Ptr>` to `Ptr`) doesn't break Rust callers. Callers
// who want to query or access the pointer should use `OptionLike`.

impl<Ptr: SupportsNullable> Default for Nullable<Ptr> {
    fn default() -> Self {
        Ptr::null()
    }
}

impl<Ptr: SupportsNullable + Debug> Debug for Nullable<Ptr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if Ptr::is_null(self) {
            f.pad("null")
        } else {
            Debug::fmt(&self.ptr, f)
        }
    }
}

impl<Ptr: SupportsNullable> From<Ptr> for Nullable<Ptr> {
    fn from(ptr: Ptr) -> Self {
        Nullable { ptr }
    }
}

// There is intentionally no `From<&mut Ptr> for &mut Nullable<Ptr>`: it would allow writing a null
// pointer to a `Ptr` through the `&mut Nullable<Ptr>`.
impl<'a, Ptr: SupportsNullable> From<&'a Ptr> for &'a Nullable<Ptr> {
    fn from(ptr: &'a Ptr) -> Self {
        // SAFETY: `Nullable<Ptr>` is a `#[repr(transparent)]` wrapper around `Ptr`, and every `Ptr`
        // is a valid `Nullable<Ptr>`.
        unsafe { &*(ptr as *const Ptr as *const Nullable<Ptr>) }
    }
}

/// Conversion of a smart pointer, or a [`Nullable`] one, to an [`Option`].
///
/// Implemented by both `Ptr` and `Nullable<Ptr>`, so that code using it keeps compiling when a C++
/// API adds or removes `absl_nonnull`.
pub trait OptionLike {
    /// The non-null pointer type.
    type Target;

    /// Converts `self` into `Some(ptr)`, or `None` if it is null.
    fn into_option(self) -> Option<Self::Target>;

    /// Returns `Some(&ptr)`, or `None` if it is null.
    fn as_option(&self) -> Option<&Self::Target>;

    /// Returns `Some(&mut ptr)`, or `None` if it is null.
    fn as_option_mut(&mut self) -> Option<&mut Self::Target>;
}

impl<Ptr: SupportsNullable> OptionLike for Nullable<Ptr> {
    type Target = Ptr;

    fn into_option(self) -> Option<Ptr> {
        if Ptr::is_null(&self) {
            None
        } else {
            Some(self.ptr)
        }
    }

    fn as_option(&self) -> Option<&Ptr> {
        if Ptr::is_null(self) {
            None
        } else {
            Some(&self.ptr)
        }
    }

    fn as_option_mut(&mut self) -> Option<&mut Ptr> {
        if Ptr::is_null(self) {
            None
        } else {
            Some(&mut self.ptr)
        }
    }
}

/// Implements `OptionLike` for a bare smart pointer.
///
/// A bare smart pointer is conventionally non-null, but can still be null, e.g. when moved from
/// in C++, so this still checks. Every `Ptr` is a valid `Nullable<Ptr>`, so the check views it as
/// one.
macro_rules! impl_option_like_for_bare_pointer {
    ($ptr:ty, $($bounds:tt)*) => {
        impl<T: $($bounds)*> OptionLike for $ptr {
            type Target = Self;

            fn into_option(self) -> Option<Self> {
                if SupportsNullable::is_null(<&Nullable<Self>>::from(&self)) {
                    None
                } else {
                    Some(self)
                }
            }

            fn as_option(&self) -> Option<&Self> {
                if SupportsNullable::is_null(<&Nullable<Self>>::from(&*self)) {
                    None
                } else {
                    Some(self)
                }
            }

            fn as_option_mut(&mut self) -> Option<&mut Self> {
                if SupportsNullable::is_null(<&Nullable<Self>>::from(&*self)) {
                    None
                } else {
                    Some(self)
                }
            }
        }
    };
}

impl_option_like_for_bare_pointer!(unique_ptr<T>, Sized);
impl_option_like_for_bare_pointer!(virtual_unique_ptr<T>, Sized + Delete);
impl_option_like_for_bare_pointer!(shared_ptr<T>, Sized);

// SAFETY: `unique_ptr::is_null` returns whether the held pointer is null, and a null `unique_ptr`
// owns nothing, so dropping it does nothing.
unsafe impl<T: Sized> SupportsNullable for unique_ptr<T> {
    fn is_null(this: &Nullable<Self>) -> bool {
        unique_ptr::is_null(&this.ptr)
    }

    fn null() -> Nullable<Self> {
        // SAFETY: `from_raw` accepts a null pointer.
        Nullable { ptr: unsafe { unique_ptr::from_raw(core::ptr::null_mut()) } }
    }
}

// SAFETY: `virtual_unique_ptr::is_null` returns whether the held pointer is null, and a null
// `virtual_unique_ptr` owns nothing, so dropping it does nothing.
unsafe impl<T: Sized + Delete> SupportsNullable for virtual_unique_ptr<T> {
    fn is_null(this: &Nullable<Self>) -> bool {
        virtual_unique_ptr::is_null(&this.ptr)
    }

    fn null() -> Nullable<Self> {
        // SAFETY: `from_raw` accepts a null pointer.
        Nullable { ptr: unsafe { virtual_unique_ptr::from_raw(core::ptr::null_mut()) } }
    }
}

// SAFETY: `shared_ptr::is_null` returns whether the held pointer is null, and a `shared_ptr` with
// a null pointer and a null control block owns nothing, so dropping it does nothing.
unsafe impl<T: Sized> SupportsNullable for shared_ptr<T> {
    fn is_null(this: &Nullable<Self>) -> bool {
        shared_ptr::is_null(&this.ptr)
    }

    fn null() -> Nullable<Self> {
        // SAFETY: A null pointer with a null control block is the state of a default-constructed
        // `std::shared_ptr`, and holds no strong reference.
        Nullable {
            ptr: unsafe { shared_ptr::from_raw_parts(core::ptr::null(), core::ptr::null_mut()) },
        }
    }
}
