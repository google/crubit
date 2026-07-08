// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![no_std]
#![deny(missing_docs, warnings)]

//! # `declval`
//!
//! A support library for unevaluated type-checking and type-inference contexts in Rust,
//! analogous to `std::declval<T>()` in C++.
//!
//! When generating bindings or `extern "C"` thunks for opaque return types such as
//! Return Position Impl Trait (`-> impl Trait`), it is often necessary to name or infer
//! concrete types inside closures or generic helper functions without ever executing them
//! at runtime:
//!
//! ```ignore
//! fn inner<F>(_: impl FnOnce() -> F, this: *mut ()) {
//!     unsafe { core::ptr::drop_in_place(this as *mut F); }
//! }
//! inner(|| my_async_fn(declval::<Arg1>(), declval::<Arg2>()), this)
//! ```
//!
//! Unlike `todo!()` (which returns the `!` never type and can fail trait-bound verification
//! when passed to generic functions expecting specific traits), `declval::<T>()` has the
//! exact return type `T`.

/// Type-checks to return a value of type `T`, but panics when called at runtime.
///
/// Because `declval` is immediately panics when called, `T` is not required to implement
/// [`Default`] as the function signature suggests.
///
/// # Panics
///
/// Always panics if evaluated at runtime.
#[track_caller]
pub fn declval<T>() -> T {
    panic!("`declval` must only be used in unevaluated contexts for type checking/inference");
}
