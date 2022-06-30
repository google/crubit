// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Copyable, equality-comparable types for Salsa.
//!
//! TODO(jeanpierreda): give this module a better name.

#![feature(backtrace)]

use std::ops::Deref;
use std::sync::Arc;

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

/// A clonable, equality-comparable error which is interconvertible with
/// `anyhow::Error`.
///
/// Two errors are equal if they are identical (i.e. they both have a common
/// cloned-from ancestor.)
///
/// Salsa queries should return `Result<Rc<T>, SalsaError>`, and not
/// `Rc<Result<T, anyhow::Error>>`. Because `anyhow::Error` cannot be cloned,
/// `Rc<Result<T, anyhow::Error>>` is very nearly useless, as one cannot create
/// a new `Rc<Result<U, anyhow::Error>>` containing the same error.
/// Error propagation with cached errors requires that the underlying error type
/// be copyable.
///
/// (Implementation note: SalsaError itself uses `Arc`, not `Rc`, because
/// `anyhow::Error` requires `Send`+`Sync`.)
#[derive(Clone, Debug)]
pub struct SalsaError(Arc<dyn std::error::Error + Send + Sync + 'static>);

impl PartialEq for SalsaError {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.0, &*other.0)
    }
}

impl Eq for SalsaError {}

impl std::fmt::Display for SalsaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(&*self.0, f)
    }
}

impl std::error::Error for SalsaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
    fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        self.0.backtrace()
    }
}

impl From<anyhow::Error> for SalsaError {
    fn from(e: anyhow::Error) -> Self {
        let e: Box<dyn std::error::Error + Send + Sync + 'static> = e.into();
        SalsaError(e.into())
    }
}

pub type SalsaResult<T> = Result<T, SalsaError>;
