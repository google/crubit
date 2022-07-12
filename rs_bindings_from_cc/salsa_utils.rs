// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Copyable, equality-comparable types for Salsa.
//!
//! TODO(jeanpierreda): give this module a better name.

use std::ops::Deref;

/// A wrapper for a smart pointer, which implements `Eq` as pointer equality.
///
/// This was directly inspired by Chalk's `ArcEq`, which does the same.
/// However, unlike Chalk, `PtrEq` does not implement `Deref`: that would
/// normally imply that it has the same behavior as the underlying
/// pointee, and it obviously does not, as it implements `Eq` even if the
/// pointee doesn't.
///
/// Instead, to access the underlying value, use `.as_ref()`.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct PtrEq<T: Deref>(pub T);

impl<T: Deref> PartialEq<PtrEq<T>> for PtrEq<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.0, &*other.0)
    }
}

impl<T: Deref> Eq for PtrEq<T> {}

impl<T: Deref> PtrEq<T> {
    pub fn as_ref(&self) -> &T::Target {
        &*self.0
    }
}

