// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Copyable, equality-comparable types for Salsa.
//!
//! TODO(jeanpierreda): give this module a better name.

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::ops::Deref;
use std::rc::Rc;

/// A wrapper for a smart pointer, which implements `Eq` as pointer equality.
///
/// This was directly inspired by Chalk's `ArcEq`, which does the same.
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct RcEq<T>(pub Rc<T>);

impl<T> Clone for RcEq<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> PartialEq<RcEq<T>> for RcEq<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.0, &*other.0)
    }
}

impl<T> Eq for RcEq<T> {}

impl<T> Deref for RcEq<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<T> From<T> for RcEq<T> {
    fn from(x: T) -> Self {
        RcEq::new(x)
    }
}

impl<T> RcEq<T> {
    pub fn new(x: T) -> Self {
        RcEq(Rc::new(x))
    }
}

impl<T> ToTokens for RcEq<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.as_ref().to_tokens(tokens)
    }
}
