// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Helper library for Crubit codegen providing [`decltype_cast`].

/// Casts a pointer to any type that can be returned by a given function.
///
/// This is useful for recovering anonymous types that cannot be named directly.
///
/// # Example
///
/// ```rust
/// // async functions have an unnamable return type.
/// async fn async_foo() { /* ... */ }
///
/// extern "C" fn drop_ret_ty_of_async_foo(ptr: *mut ()) {
///     let ptr = decltype_cast::decltype_cast(|| async_foo(), ptr);
///     unsafe {
///         std::ptr::drop_in_place(ptr);
///     }
/// }
/// ```
pub fn decltype_cast<T>(_: fn() -> T, pointer: *mut ()) -> *mut T {
    pointer as *mut T
}
