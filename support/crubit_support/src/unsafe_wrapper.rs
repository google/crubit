// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::fmt;

/// A value whose lifetime could not be verified across the C++/Rust boundary.
///
/// When Crubit binds a C++ API that mentions a lifetime-parameterized Rust type without knowing
/// its lifetime, it instantiates the type with `'static` and wraps it in `Unsafe<T>`. The value
/// may in reality borrow data that lives for less than `'static`, so reading or unwrapping it
/// requires `unsafe`: the caller asserts that the data it borrows is still live.
///
/// Apart from that, `Unsafe<T>` behaves like `T`:
///
/// - It is `#[repr(transparent)]` over `T`, so it has the same size, alignment and ABI as `T`.
/// - Dropping an `Unsafe<T>` drops the `T` it holds, which runs `T`'s destructor. A destructor may
///   use the data that `T` borrows. Code that may still hold an `Unsafe<T>` after that data is gone
///   must unwrap and drop the value while the data is live, or not drop it at all (for example,
///   with [`core::mem::forget`]).
/// - It is `Send`, `Sync` and `Unpin` exactly when `T` is, and `Copy` when `T` is `Copy`.
///
/// Its [`fmt::Debug`] implementation does not read the value.
#[repr(transparent)]
pub struct Unsafe<T> {
    value: T,
}

impl<T> Unsafe<T> {
    /// Wraps `value` in an `Unsafe<T>`.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Consumes `self` and returns the wrapped value.
    ///
    /// # Safety
    ///
    /// Every lifetime parameter of `T` (such as a `'static` that Crubit substituted for an unknown
    /// C++ lifetime) must remain valid for as long as the returned `T`, or anything derived from
    /// it, is used, and until it is dropped.
    #[inline]
    pub unsafe fn into_inner(self) -> T {
        self.value
    }

    /// Returns a shared reference to the wrapped value.
    ///
    /// # Safety
    ///
    /// Every lifetime parameter of `T` must remain valid for as long as the returned reference is
    /// used.
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        &self.value
    }

    /// Returns a mutable reference to the wrapped value.
    ///
    /// # Safety
    ///
    /// Every lifetime parameter of `T` must remain valid for as long as the returned reference is
    /// used.
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

// `Clone` is only implemented for `Copy` types: a bitwise copy does not run any code of `T`,
// whereas `T::clone` could read borrowed data whose lifetime is not known.
impl<T: Copy> Copy for Unsafe<T> {}

impl<T: Copy> Clone for Unsafe<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> fmt::Debug for Unsafe<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Unsafe").finish_non_exhaustive()
    }
}
