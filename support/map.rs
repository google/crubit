// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Helper types and traits for the Rust projections of C++ map types, such as
//! `absl::flat_hash_map`.
//!
//! The generated bindings for an instantiation of a container's class template name these items
//! through the crate of the target that defines the class template, so that target's Rust bindings
//! must re-export them.

use core::pin::Pin;
use ctor::{emplace, Ctor, Infallible};

/// The error returned by [`Map::try_insert`] when the key already exists.
///
/// Contains references to the occupied entry's key and value. The key and value that
/// were not inserted have been destroyed.
#[derive(Debug)]
pub struct OccupiedError<E> {
    /// The entry in the map that was already occupied, as references to its key and value.
    pub entry: E,
}

/// Helper trait converting a raw mutable value pointer into `&'a mut V` (for `V: Unpin`)
/// or `Pin<&'a mut V>` (for `V: !Unpin`).
#[doc(hidden)]
pub unsafe trait ValueMutRef<'a, V: 'a> {
    unsafe fn from_raw(ptr: *mut V) -> Self;
}

unsafe impl<'a, V: 'a> ValueMutRef<'a, V> for &'a mut V {
    #[inline(always)]
    unsafe fn from_raw(ptr: *mut V) -> Self {
        unsafe { &mut *ptr }
    }
}

unsafe impl<'a, V: 'a> ValueMutRef<'a, V> for Pin<&'a mut V> {
    #[inline(always)]
    unsafe fn from_raw(ptr: *mut V) -> Self {
        unsafe { Pin::new_unchecked(&mut *ptr) }
    }
}

/// Unified interface implemented by C++ map containers (such as `absl::flat_hash_map`).
///
/// All public methods of this trait are also exposed as inherent methods on each generated
/// map type, so callers do not need to import `Map` to call them on a concrete map.
///
/// # Safety
///
/// This trait is only intended to be implemented by Crubit's generated bindings.
pub unsafe trait Map: Sized {
    /// The key type `K` of the map.
    type Key;

    /// The value type `V` of the map.
    type Value;

    /// A mutable reference to a value in the map: `&'a mut Self::Value` when
    /// `Self::Value: Unpin`, or `Pin<&'a mut Self::Value>` when `Self::Value: !Unpin`.
    type ValueMut<'a>: ValueMutRef<'a, Self::Value>
    where
        Self: 'a;

    /// Returns the number of elements currently within the map.
    #[inline(always)]
    #[must_use]
    fn len(&self) -> usize {
        unsafe { self.raw_len() }
    }

    /// Returns whether or not the map is empty.
    #[inline(always)]
    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of element slots available within the map.
    #[inline(always)]
    #[must_use]
    fn capacity(&self) -> usize {
        unsafe { self.raw_capacity() }
    }

    /// Clears the map, removing all elements.
    #[inline(always)]
    fn clear(self: Pin<&mut Self>) {
        unsafe { self.raw_clear() }
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in
    /// the map. The collection may reserve more space to speculatively avoid
    /// frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    #[inline(always)]
    fn reserve(self: Pin<&mut Self>, additional: usize) {
        let count = self.len().checked_add(additional).expect("capacity overflow");
        unsafe { self.raw_reserve(count) }
    }

    /// Returns `true` if the map contains a value for the specified key.
    #[inline(always)]
    #[must_use]
    fn contains_key(&self, key: &Self::Key) -> bool {
        self.get_key_value(key).is_some()
    }

    /// Returns a reference to the value corresponding to the key.
    #[inline(always)]
    #[must_use]
    fn get<'a>(&'a self, key: &Self::Key) -> Option<&'a Self::Value> {
        self.get_key_value(key).map(|(_, v)| v)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    #[inline(always)]
    #[must_use]
    fn get_key_value<'a>(&'a self, key: &Self::Key) -> Option<(&'a Self::Key, &'a Self::Value)> {
        let mut result_key: *const Self::Key = core::ptr::null();
        let mut result_value: *const Self::Value = core::ptr::null();
        if unsafe { self.raw_get_key_value(key, &raw mut result_key, &raw mut result_value) } {
            Some(unsafe { (&*result_key, &*result_value) })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    #[inline(always)]
    #[must_use]
    fn get_mut<'a>(self: Pin<&'a mut Self>, key: &Self::Key) -> Option<Self::ValueMut<'a>> {
        let ptr = unsafe { self.raw_get_mut(key) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Self::ValueMut::from_raw(ptr) })
        }
    }

    /// Inserts a key-value pair into the map, returning references to the
    /// key and value in the map.
    ///
    /// If the map did not have this key present, the key and value are inserted.
    /// If the map did have this key present, the value is updated in place and the
    /// existing key is retained.
    #[inline(always)]
    fn insert<'a>(
        self: Pin<&'a mut Self>,
        key: Ctor![Self::Key],
        value: Ctor![Self::Value],
    ) -> (&'a Self::Key, Self::ValueMut<'a>) {
        match insert_impl(self, key, value, true) {
            Ok(entry) => entry,
            Err(_) => unreachable!(),
        }
    }

    /// Inserts an element with the specified key and value by move-constructing them
    /// into the map (copy-constructing if there is no move constructor),
    /// provided that no element with the given key already exists, returning
    /// references to the newly inserted element.
    ///
    /// If an element with the given key already exists, returns an [`OccupiedError`]
    /// containing references to the occupied entry. The key and value are accepted
    /// as [`Ctor`]s, which means `Unpin` values can be passed directly,
    /// and `!Unpin` values as e.g. `KeyType::ctor_new(..)`, `ctor::copy(&x)`, or
    /// `mov!(x)`. They are constructed into temporaries which are then moved into the
    /// map by C++ `try_emplace`. On the occupied path, the temporaries are destroyed.
    ///
    /// Usage passing `Unpin` key and value types by value:
    ///
    /// ```ignore
    /// use ctor::{emplace, CtorNew};
    /// let mut map = emplace!(MyMap::ctor_new(()));
    /// map.as_mut().try_insert(1, 123).unwrap();
    /// ```
    ///
    /// Usage passing potentially-`!Unpin` key and value types:
    ///
    /// ```ignore
    /// use ctor::{emplace, CtorNew};
    /// let mut map = emplace!(MyMap::ctor_new(()));
    /// map.as_mut().try_insert(KeyType::ctor_new(1), ValueType::ctor_new(2)).unwrap();
    /// ```
    #[inline(always)]
    #[must_use]
    fn try_insert<'a>(
        self: Pin<&'a mut Self>,
        key: Ctor![Self::Key],
        value: Ctor![Self::Value],
    ) -> Result<
        (&'a Self::Key, Self::ValueMut<'a>),
        OccupiedError<(&'a Self::Key, Self::ValueMut<'a>)>,
    > {
        insert_impl(self, key, value, false)
    }

    /// Removes a key from the map, returning `true` if the key was previously in the map.
    #[inline(always)]
    fn remove(self: Pin<&mut Self>, key: &Self::Key) -> bool {
        unsafe { self.raw_remove(key) }
    }

    #[doc(hidden)]
    unsafe fn raw_len(&self) -> usize;

    #[doc(hidden)]
    unsafe fn raw_capacity(&self) -> usize;

    #[doc(hidden)]
    unsafe fn raw_clear(self: Pin<&mut Self>);

    #[doc(hidden)]
    unsafe fn raw_reserve(self: Pin<&mut Self>, count: usize);

    #[doc(hidden)]
    unsafe fn raw_get_key_value(
        &self,
        key: &Self::Key,
        result_key: *mut *const Self::Key,
        result_value: *mut *const Self::Value,
    ) -> bool;

    #[doc(hidden)]
    unsafe fn raw_get_mut(self: Pin<&mut Self>, key: &Self::Key) -> *mut Self::Value;

    #[doc(hidden)]
    unsafe fn raw_insert(
        self: Pin<&mut Self>,
        key: *mut Self::Key,
        value: *mut Self::Value,
        overwrite: bool,
        result_key: *mut *const Self::Key,
        result_value: *mut *mut Self::Value,
    ) -> bool;

    #[doc(hidden)]
    unsafe fn raw_remove(self: Pin<&mut Self>, key: &Self::Key) -> bool;
}

#[inline(always)]
fn insert_impl<'a, M: Map>(
    map: Pin<&'a mut M>,
    key: impl Ctor<Output = M::Key, Error = Infallible>,
    value: impl Ctor<Output = M::Value, Error = Infallible>,
    overwrite: bool,
) -> Result<(&'a M::Key, M::ValueMut<'a>), OccupiedError<(&'a M::Key, M::ValueMut<'a>)>> {
    // The slots hold the (possibly moved-from) temporaries and are
    // destroyed when the function returns.
    let mut key = emplace!(key);
    let mut value = emplace!(value);
    let mut result_key: *const M::Key = core::ptr::null();
    let mut result_value: *mut M::Value = core::ptr::null_mut();
    let inserted = unsafe {
        map.raw_insert(
            Pin::into_inner_unchecked(key.as_mut()) as *mut _,
            Pin::into_inner_unchecked(value.as_mut()) as *mut _,
            overwrite,
            &raw mut result_key,
            &raw mut result_value,
        )
    };
    let entry = unsafe { (&*result_key, M::ValueMut::from_raw(result_value)) };
    if inserted {
        Ok(entry)
    } else {
        Err(OccupiedError { entry })
    }
}
