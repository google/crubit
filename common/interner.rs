// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bumpalo::Bump;
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hash;

/// An interned value which performs by-pointer comparison.
///
/// Note that all values for a given type must originate from the same interner
/// or they will compare not-equal.
pub struct Interned<'arena, T: ?Sized>(&'arena T);

impl<T: ?Sized> Copy for Interned<'_, T> {}
impl<T: ?Sized> Clone for Interned<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'arena, T: ?Sized + 'arena> Interned<'arena, T> {
    /// Access the inner reference.
    ///
    /// Note that this can also be accessed using `*interned`.
    #[inline(always)]
    pub fn inner(&self) -> &'arena T {
        self.0
    }

    #[inline(always)]
    fn ptr(&self) -> *const () {
        self.0 as *const T as *const ()
    }
}

impl<T: ?Sized + std::fmt::Debug> std::fmt::Debug for Interned<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'arena, T: ?Sized + 'arena> std::ops::Deref for Interned<'arena, T> {
    type Target = &'arena T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> PartialEq for Interned<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr() == other.ptr()
    }
}

impl<T: ?Sized> Eq for Interned<'_, T> {}

impl<T: ?Sized> std::cmp::PartialOrd for Interned<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ?Sized> std::cmp::Ord for Interned<'_, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ptr().cmp(&other.ptr())
    }
}

impl<T: ?Sized> std::hash::Hash for Interned<'_, T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        std::ptr::hash(self.ptr(), state)
    }
}

/// An interner for values of type `T`.
pub struct Interner<'arena, T: ?Sized> {
    alloc: &'arena Bump,
    map: RefCell<HashSet<&'arena T>>,
}

impl<'arena, T: ?Sized> Interner<'arena, T> {
    /// Create a new interner which will store interned values in `alloc`.
    pub fn new(alloc: &'arena Bump) -> Self {
        Self { alloc, map: Default::default() }
    }
}

impl<'arena, T: Hash + Eq + Copy> Interner<'arena, T> {
    /// Intern `value` or access the interned copy if one exists.
    ///
    /// Note: the `Copy` bound on this method is not strictly required, but
    /// the interner will not run `Drop` glue for its contents, so users must
    /// ensure that they do not store types which need to be dropped.
    pub fn intern(&self, value: T) -> Interned<'arena, T> {
        let mut map = self.map.borrow_mut();
        if let Some(interned) = map.get(&value) {
            return Interned(interned);
        }
        let v = self.alloc.alloc(value);
        map.insert(v);
        Interned(v)
    }
}

impl<'arena, Elem: Hash + Eq + Copy> Interner<'arena, [Elem]> {
    /// Intern `value` or access the interned copy if one exists.
    pub fn intern_slice(&self, value: &[Elem]) -> Interned<'arena, [Elem]> {
        let mut map = self.map.borrow_mut();
        if let Some(interned) = map.get(value) {
            return Interned(interned);
        }
        let v = self.alloc.alloc_slice_copy(value);
        map.insert(v);
        Interned(v)
    }
}

impl<'arena> Interner<'arena, str> {
    /// Intern `value` or access the interned copy if one exists.
    pub fn intern_str(&self, value: &str) -> Interned<'arena, str> {
        let mut map = self.map.borrow_mut();
        if let Some(interned) = map.get(value) {
            return Interned(interned);
        }
        let v = self.alloc.alloc_str(value);
        map.insert(v);
        Interned(v)
    }
}
