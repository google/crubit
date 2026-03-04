// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// A trait for types that must be deleted, if heap-allocated, using C++ `delete`.
///
/// In particular, this is used for types with virtual destructors or overloaded
/// `operator delete`.
///
/// To safely use this trait, use `cc_std::std::virtual_unique_ptr`, which represents a
/// C++ unique_ptr for a `T` that requires `delete`.
///
/// # Safety
///
/// The `delete` method must be exactly equivalent to C++ `delete p`.
///
/// (Generally speaking, this trait should only be implemented by automatically generated
/// FFI code generation.)
#[diagnostic::on_unimplemented(
    note = "`Delete` is only implemented by C++ base classes with virtual destructors",
    note = "See https://crubit.rs/errors/delete for more information"
)]
pub unsafe trait Delete {
    /// Deletes the object pointed to by `p`, as if by C++ `delete p`.
    ///
    /// # Safety
    ///
    /// `p` must be a valid pointer to an object of type `Self` that was
    /// allocated with `new`.
    ///
    /// After this call, the pointer is dangling, and must not be used.
    unsafe fn delete(p: *mut Self);
}
