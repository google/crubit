// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! `#[repr(C)]` FFI vocabulary types (`FfiStr`, `FfiVec<T>`, `FfiBox<T>`, `FfiMap<K, V>`) and
//! view adapters (`AsView`, `RepeatedView`, `MapView`) shared in-memory between C++
//! (`ir_ffi_support.h`) and Rust.

use ffi_types::{AllocFfiBytes, FreeFfiBytes};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ptr;
use std::slice;

/// An owned UTF-8 string with a `#[repr(C)]` layout matching `crubit::ffi::FfiStr` in C++.
#[repr(C)]
pub struct FfiStr {
    ptr: *const u8,
    len: usize,
}

unsafe impl Send for FfiStr {}
unsafe impl Sync for FfiStr {}

impl FfiStr {
    pub const EMPTY: Self = Self { ptr: ptr::NonNull::<u8>::dangling().as_ptr(), len: 0 };

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        if s.is_empty() {
            return Self::EMPTY;
        }
        unsafe {
            let buf = AllocFfiBytes(s.len(), 1);
            ptr::copy_nonoverlapping(s.as_ptr(), buf, s.len());
            Self { ptr: buf, len: s.len() }
        }
    }

    pub fn as_str(&self) -> &str {
        if self.len == 0 {
            return "";
        }
        unsafe { std::str::from_utf8_unchecked(slice::from_raw_parts(self.ptr, self.len)) }
    }

    pub fn as_view(&self) -> &str {
        self.as_str()
    }
}

impl Default for FfiStr {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Drop for FfiStr {
    fn drop(&mut self) {
        if self.len > 0 {
            unsafe {
                FreeFfiBytes(self.ptr as *mut u8, self.len, 1);
            }
        }
    }
}

impl Clone for FfiStr {
    fn clone(&self) -> Self {
        Self::from_str(self.as_str())
    }
}

impl Deref for FfiStr {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for FfiStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for FfiStr {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for FfiStr {
    fn from(s: String) -> Self {
        Self::from_str(&s)
    }
}

impl fmt::Debug for FfiStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for FfiStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl PartialEq for FfiStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for FfiStr {}

impl PartialEq<str> for FfiStr {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for FfiStr {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialOrd for FfiStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FfiStr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for FfiStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

/// An owned contiguous vector with a `#[repr(C)]` layout matching `crubit::ffi::FfiVec<T>` in C++.
#[repr(C)]
pub struct FfiVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

unsafe impl<T: Send> Send for FfiVec<T> {}
unsafe impl<T: Sync> Sync for FfiVec<T> {}

impl<T> FfiVec<T> {
    pub const EMPTY: Self = Self { ptr: ptr::null_mut(), len: 0, cap: 0 };

    pub const fn new() -> Self {
        Self::EMPTY
    }

    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        if slice.is_empty() {
            return Self::EMPTY;
        }
        let cap = slice.len();
        unsafe {
            let ptr =
                AllocFfiBytes(cap.checked_mul(size_of::<T>()).unwrap(), align_of::<T>()) as *mut T;
            for (i, item) in slice.iter().enumerate() {
                ptr.add(i).write(item.clone());
            }
            Self { ptr, len: cap, cap }
        }
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        if v.is_empty() {
            return Self::EMPTY;
        }
        let cap = v.len();
        unsafe {
            let ptr =
                AllocFfiBytes(cap.checked_mul(size_of::<T>()).unwrap(), align_of::<T>()) as *mut T;
            for (i, item) in v.into_iter().enumerate() {
                ptr.add(i).write(item);
            }
            Self { ptr, len: cap, cap }
        }
    }

    pub fn as_slice(&self) -> &[T] {
        if self.len == 0 {
            return &[];
        }
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.len == 0 {
            return &mut [];
        }
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    pub fn push(&mut self, value: T) -> &mut T {
        unsafe {
            if self.len == self.cap {
                let new_cap = if self.cap == 0 { 4 } else { self.cap.checked_mul(2).unwrap() };
                let new_ptr =
                    AllocFfiBytes(new_cap.checked_mul(size_of::<T>()).unwrap(), align_of::<T>())
                        as *mut T;
                if self.len > 0 {
                    ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                }
                if self.cap > 0 {
                    FreeFfiBytes(
                        self.ptr as *mut u8,
                        self.cap.checked_mul(size_of::<T>()).unwrap(),
                        align_of::<T>(),
                    );
                }
                self.ptr = new_ptr;
                self.cap = new_cap;
            }
            let slot = self.ptr.add(self.len);
            slot.write(value);
            self.len += 1;
            &mut *slot
        }
    }
}

impl<T> Default for FfiVec<T> {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<T> Drop for FfiVec<T> {
    fn drop(&mut self) {
        unsafe {
            if self.len > 0 {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr, self.len));
            }
            if self.cap > 0 {
                FreeFfiBytes(
                    self.ptr as *mut u8,
                    self.cap.checked_mul(size_of::<T>()).unwrap(),
                    align_of::<T>(),
                );
            }
        }
    }
}

impl<T: Clone> Clone for FfiVec<T> {
    fn clone(&self) -> Self {
        Self::from_slice(self.as_slice())
    }
}

impl<T> Deref for FfiVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> From<Vec<T>> for FfiVec<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_vec(v)
    }
}

impl<T, const N: usize> From<[T; N]> for FfiVec<T> {
    fn from(arr: [T; N]) -> Self {
        Self::from_vec(Vec::from(arr))
    }
}

impl<T: Clone> From<&[T]> for FfiVec<T> {
    fn from(slice: &[T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T: Clone, const N: usize> From<&[T; N]> for FfiVec<T> {
    fn from(arr: &[T; N]) -> Self {
        Self::from_slice(arr.as_slice())
    }
}

impl<T> FromIterator<T> for FfiVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().collect())
    }
}

impl<'a, T> IntoIterator for &'a FfiVec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<T: fmt::Debug> fmt::Debug for FfiVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), f)
    }
}

impl<T: PartialEq> PartialEq for FfiVec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq> Eq for FfiVec<T> {}

impl<T: Hash> Hash for FfiVec<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

/// Single-owner nullable heap pointer with a `#[repr(C)]` layout matching `crubit::ffi::FfiBox<T>`
/// in C++.
#[repr(C)]
pub struct FfiBox<T> {
    ptr: *mut T,
}

unsafe impl<T: Send> Send for FfiBox<T> {}
unsafe impl<T: Sync> Sync for FfiBox<T> {}

impl<T> FfiBox<T> {
    pub const EMPTY: Self = Self { ptr: ptr::null_mut() };

    pub fn new(value: T) -> Self {
        unsafe {
            let raw = AllocFfiBytes(size_of::<T>(), align_of::<T>()) as *mut T;
            raw.write(value);
            Self { ptr: raw }
        }
    }
    pub fn as_ref(&self) -> Option<&T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_or_default<'a>(&'a self, default: &'a T) -> &'a T {
        self.as_ref().unwrap_or(default)
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> Default for FfiBox<T> {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<T> Drop for FfiBox<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ptr::drop_in_place(self.ptr);
                FreeFfiBytes(self.ptr as *mut u8, size_of::<T>(), align_of::<T>());
            }
        }
    }
}

impl<T: Clone> Clone for FfiBox<T> {
    fn clone(&self) -> Self {
        match self.as_ref() {
            Some(v) => Self::new(v.clone()),
            None => Self::EMPTY,
        }
    }
}

impl<T> From<T> for FfiBox<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: fmt::Debug> fmt::Debug for FfiBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_ref(), f)
    }
}

impl<T: PartialEq> PartialEq for FfiBox<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T: Eq> Eq for FfiBox<T> {}

impl<T: Hash> Hash for FfiBox<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

/// Key-value pair entry matching `crubit::ffi::FfiMapEntry<K, V>` in C++.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FfiMapEntry<K, V> {
    pub key: K,
    pub value: V,
}

/// An owned map backed by `FfiVec<FfiMapEntry<K, V>>` matching `crubit::ffi::FfiMap<K, V>` in C++.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FfiMap<K, V> {
    entries: FfiVec<FfiMapEntry<K, V>>,
}

impl<K, V> FfiMap<K, V> {
    pub const EMPTY: Self = Self { entries: FfiVec::EMPTY };

    pub const fn new() -> Self {
        Self::EMPTY
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &[FfiMapEntry<K, V>] {
        self.entries.as_slice()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.entries.as_slice().iter().map(|e| (&e.key, &e.value))
    }

    pub fn push(&mut self, key: K, value: V) -> &mut V
    where
        K: PartialEq,
    {
        if let Some(idx) = self.entries.as_slice().iter().position(|e| e.key == key) {
            let entry = &mut self.entries.as_mut_slice()[idx];
            entry.value = value;
            return &mut entry.value;
        }
        &mut self.entries.push(FfiMapEntry { key, value }).value
    }
}

impl<K, V> Default for FfiMap<K, V> {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<K1: Into<K>, V1: Into<V>, K: PartialEq, V> FromIterator<(K1, V1)> for FfiMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K1, V1)>>(iter: I) -> Self {
        let mut map = Self::EMPTY;
        for (k, v) in iter {
            map.push(k.into(), v.into());
        }
        map
    }
}

impl<K1: Into<K>, V1: Into<V>, K: PartialEq, V, const N: usize> From<[(K1, V1); N]>
    for FfiMap<K, V>
{
    fn from(arr: [(K1, V1); N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for FfiMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

/// Trait for viewing an owned FFI value as a lightweight `Copy` view.
pub trait AsView<'a> {
    type View: Copy;
    fn as_view(&'a self) -> Self::View;
}

impl<'a> AsView<'a> for i32 {
    type View = i32;
    fn as_view(&'a self) -> i32 {
        *self
    }
}

impl<'a> AsView<'a> for FfiStr {
    type View = &'a str;
    fn as_view(&'a self) -> &'a str {
        self.as_str()
    }
}

impl<'a, T: 'a> AsView<'a> for FfiVec<T> {
    type View = RepeatedView<'a, T>;
    fn as_view(&'a self) -> RepeatedView<'a, T> {
        RepeatedView(self.as_slice())
    }
}

/// Borrowed slice view over an `FfiVec<T>`.
#[derive(Debug)]
pub struct RepeatedView<'a, T>(pub &'a [T]);

impl<'a, T> Copy for RepeatedView<'a, T> {}
impl<'a, T> Clone for RepeatedView<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: AsView<'a>> RepeatedView<'a, T> {
    pub fn iter(self) -> impl DoubleEndedIterator<Item = T::View> + ExactSizeIterator + 'a {
        self.0.iter().map(|x| x.as_view())
    }
    pub fn len(self) -> usize {
        self.0.len()
    }
    pub fn is_empty(self) -> bool {
        self.0.is_empty()
    }
    pub fn get(self, idx: usize) -> Option<T::View> {
        self.0.get(idx).map(|x| x.as_view())
    }
    pub fn first(self) -> Option<T::View> {
        self.0.first().map(|x| x.as_view())
    }
}

/// Borrowed view over an `FfiMap<K, V>`.
#[derive(Copy, Clone, Debug)]
pub struct MapView<'a, K, V>(pub &'a FfiMap<K, V>);

impl<'a, K: AsView<'a>, V: AsView<'a>> MapView<'a, K, V> {
    pub fn iter(self) -> impl Iterator<Item = (K::View, V::View)> + 'a {
        self.0.entries().iter().map(|e| (e.key.as_view(), e.value.as_view()))
    }
    pub fn len(self) -> usize {
        self.0.len()
    }
    pub fn is_empty(self) -> bool {
        self.0.is_empty()
    }
}

/// Marker used by the `proto!` macro to initialize nested submessages (`__ { ... }`) in-place
/// via the field setter method on both inline message fields and `FfiBox<T>` fields.
pub struct ProtoDefault;

const _: () = {
    assert!(size_of::<FfiStr>() == 2 * size_of::<usize>());
    assert!(align_of::<FfiStr>() == align_of::<usize>());
    assert!(size_of::<FfiVec<u8>>() == 3 * size_of::<usize>());
    assert!(align_of::<FfiVec<u8>>() == align_of::<usize>());
    assert!(size_of::<FfiBox<u8>>() == size_of::<usize>());
    assert!(align_of::<FfiBox<u8>>() == align_of::<usize>());
    assert!(size_of::<FfiMap<FfiStr, FfiStr>>() == 3 * size_of::<usize>());
    assert!(align_of::<FfiMap<FfiStr, FfiStr>>() == align_of::<usize>());
};
