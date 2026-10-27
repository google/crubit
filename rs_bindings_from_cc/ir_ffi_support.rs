// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! `#[repr(C)]` FFI vocabulary types (`FfiString`, `FfiVec<T>`, `FfiRc<T>`, `FfiMap<K, V>`,
//! `ItemId`, `LifetimeId`, `BazelLabel`) shared in-memory between C++ (`ir_ffi_support.h`) and
//! Rust.

use ffi_types::{AllocFfiBytes, FreeFfiBytes};
use std::borrow::{Borrow, Cow};
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ptr;
use std::slice;

/// Trait providing a static empty default value for zero-allocation dereferencing of null
/// `FfiRc<T>` pointers.
///
/// The `'static` bound only requires `Self` to own its data (no non-`'static` lifetime
/// parameters), and `EMPTY_REF` points to a compile-time empty value that performs no heap
/// allocation.
pub trait FfiDefault: 'static {
    const EMPTY_REF: &'static Self;
}

impl FfiDefault for u8 {
    const EMPTY_REF: &'static Self = &0;
}

/// An owned heap-allocated UTF-8 string (`Box<str>` equivalent) with a `#[repr(C)]` layout
/// matching `crubit::ffi::FfiString` in C++.
///
/// When `len > 0`, `ptr` owns `len` bytes allocated via `AllocFfiBytes` and freed on `Drop` via
/// `FreeFfiBytes`. When `len == 0`, `ptr` holds a non-null dangling sentinel and performs no heap
/// allocation.
#[repr(C)]
pub struct FfiString {
    ptr: *const u8,
    len: usize,
}

impl FfiDefault for FfiString {
    const EMPTY_REF: &'static Self = &Self::EMPTY;
}

impl FfiString {
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
}

impl Default for FfiString {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Drop for FfiString {
    fn drop(&mut self) {
        if self.len > 0 {
            unsafe {
                FreeFfiBytes(self.ptr as *mut u8, self.len, 1);
            }
        }
    }
}

impl Clone for FfiString {
    fn clone(&self) -> Self {
        Self::from_str(self.as_str())
    }
}

impl Deref for FfiString {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for FfiString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for FfiString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for FfiString {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for FfiString {
    fn from(s: String) -> Self {
        Self::from_str(&s)
    }
}

impl From<&String> for FfiString {
    fn from(s: &String) -> Self {
        Self::from_str(s.as_str())
    }
}

impl From<Cow<'_, str>> for FfiString {
    fn from(s: Cow<'_, str>) -> Self {
        Self::from_str(&s)
    }
}

impl From<std::rc::Rc<str>> for FfiString {
    fn from(s: std::rc::Rc<str>) -> Self {
        Self::from_str(&s)
    }
}

impl From<&std::rc::Rc<str>> for FfiString {
    fn from(s: &std::rc::Rc<str>) -> Self {
        Self::from_str(s)
    }
}

impl From<&FfiString> for FfiString {
    fn from(s: &FfiString) -> Self {
        s.clone()
    }
}

impl From<&FfiString> for std::rc::Rc<str> {
    fn from(s: &FfiString) -> Self {
        std::rc::Rc::from(s.as_str())
    }
}

impl From<FfiString> for std::rc::Rc<str> {
    fn from(s: FfiString) -> Self {
        std::rc::Rc::from(s.as_str())
    }
}

impl From<&FfiString> for String {
    fn from(s: &FfiString) -> Self {
        String::from(s.as_str())
    }
}

impl From<FfiString> for String {
    fn from(s: FfiString) -> Self {
        String::from(s.as_str())
    }
}

impl fmt::Debug for FfiString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for FfiString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl PartialEq for FfiString {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for FfiString {}

impl PartialEq<str> for FfiString {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for FfiString {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<String> for FfiString {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<FfiString> for str {
    fn eq(&self, other: &FfiString) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<FfiString> for &str {
    fn eq(&self, other: &FfiString) -> bool {
        *self == other.as_str()
    }
}

impl PartialOrd for FfiString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FfiString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for FfiString {
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
            let ptr = AllocFfiBytes(size_of_val(slice), align_of::<T>()) as *mut T;
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

    pub fn clear(&mut self) {
        if self.len > 0 {
            unsafe {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr, self.len));
            }
            self.len = 0;
        }
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

    pub fn insert(&mut self, idx: usize, value: T) -> &mut T {
        assert!(idx <= self.len);
        assert!(size_of::<T>() > 0, "ZSTs are not supported in FfiVec");
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
            let slot = self.ptr.add(idx);
            if idx < self.len {
                ptr::copy(slot, slot.add(1), self.len - idx);
            }
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

impl<T> std::ops::DerefMut for FfiVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> AsRef<[T]> for FfiVec<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> Borrow<[T]> for FfiVec<T> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> From<Vec<T>> for FfiVec<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_vec(v)
    }
}

impl<T> From<std::rc::Rc<[T]>> for FfiVec<T>
where
    T: Clone,
{
    fn from(rc: std::rc::Rc<[T]>) -> Self {
        Self::from_slice(&rc)
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

impl<'a> FromIterator<&'a str> for FfiVec<FfiString> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().map(FfiString::from).collect())
    }
}

impl FromIterator<String> for FfiVec<FfiString> {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().map(FfiString::from).collect())
    }
}

impl FromIterator<std::rc::Rc<str>> for FfiVec<FfiString> {
    fn from_iter<I: IntoIterator<Item = std::rc::Rc<str>>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().map(FfiString::from).collect())
    }
}

impl<'a, T> IntoIterator for &'a FfiVec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut FfiVec<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}

impl<T> IntoIterator for FfiVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let mut v = Vec::with_capacity(self.len);
        unsafe {
            for i in 0..self.len {
                v.push(self.ptr.add(i).read());
            }
            if self.cap > 0 {
                FreeFfiBytes(
                    self.ptr as *mut u8,
                    self.cap.checked_mul(size_of::<T>()).unwrap(),
                    align_of::<T>(),
                );
            }
        }
        std::mem::forget(self);
        v.into_iter()
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

impl<T: PartialEq> PartialEq<[T]> for FfiVec<T> {
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T: PartialEq> PartialEq<&[T]> for FfiVec<T> {
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice() == *other
    }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for FfiVec<T> {
    fn eq(&self, other: &[T; N]) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq> Eq for FfiVec<T> {}

impl<T: Hash> Hash for FfiVec<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

#[repr(C)]
struct FfiRcInner<T> {
    strong_count: Cell<usize>,
    value: T,
}

/// Single-threaded reference-counted nullable heap pointer (`!Send + !Sync`) with a `#[repr(C)]`
/// layout matching `crubit::ffi::FfiRc<T>` in C++.
#[repr(C)]
pub struct FfiRc<T> {
    ptr: *mut FfiRcInner<T>,
}

impl<T> FfiRc<T> {
    pub const EMPTY: Self = Self { ptr: ptr::null_mut() };

    pub fn new(value: T) -> Self {
        unsafe {
            let raw = AllocFfiBytes(size_of::<FfiRcInner<T>>(), align_of::<FfiRcInner<T>>())
                as *mut FfiRcInner<T>;
            raw.write(FfiRcInner { strong_count: Cell::new(1), value });
            Self { ptr: raw }
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn as_opt_ref(&self) -> Option<&T> {
        unsafe { self.ptr.as_ref().map(|inner| &inner.value) }
    }

    pub fn get_or_default<'a>(&'a self, default: &'a T) -> &'a T {
        self.as_opt_ref().unwrap_or(default)
    }

    pub fn as_mut(&mut self) -> Option<&mut T>
    where
        T: Clone,
    {
        if self.ptr.is_null() {
            return None;
        }
        unsafe {
            if (*self.ptr).strong_count.get() > 1 {
                let cloned = (*self.ptr).value.clone();
                *self = Self::new(cloned);
            }
            Some(&mut (*self.ptr).value)
        }
    }

    pub fn make_mut(&mut self) -> &mut T
    where
        T: Clone + Default,
    {
        if self.ptr.is_null() {
            *self = Self::new(T::default());
        } else {
            unsafe {
                if (*self.ptr).strong_count.get() > 1 {
                    let cloned = (*self.ptr).value.clone();
                    *self = Self::new(cloned);
                }
            }
        }
        unsafe { &mut (*self.ptr).value }
    }
}

impl<T: FfiDefault> FfiRc<T> {
    #[allow(clippy::should_implement_trait)]
    pub fn as_ref(&self) -> &T {
        self.as_opt_ref().unwrap_or(T::EMPTY_REF)
    }
}

impl<T> Default for FfiRc<T> {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<T> Drop for FfiRc<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let prev = (*self.ptr).strong_count.get();
                if prev == 1 {
                    ptr::drop_in_place(&mut (*self.ptr).value);
                    FreeFfiBytes(
                        self.ptr as *mut u8,
                        size_of::<FfiRcInner<T>>(),
                        align_of::<FfiRcInner<T>>(),
                    );
                } else {
                    (*self.ptr).strong_count.set(prev - 1);
                }
            }
        }
    }
}

impl<T> Clone for FfiRc<T> {
    fn clone(&self) -> Self {
        if !self.ptr.is_null() {
            unsafe {
                let prev = (*self.ptr).strong_count.get();
                assert!(prev < isize::MAX as usize, "FfiRc strong_count overflow");
                (*self.ptr).strong_count.set(prev + 1);
            }
        }
        Self { ptr: self.ptr }
    }
}

impl<T: FfiDefault> Deref for FfiRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.as_opt_ref().unwrap_or(T::EMPTY_REF)
    }
}

impl<T: FfiDefault> AsRef<T> for FfiRc<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: FfiDefault> Borrow<T> for FfiRc<T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<T> From<T> for FfiRc<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: FfiDefault + fmt::Debug> fmt::Debug for FfiRc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: FfiDefault + PartialEq> PartialEq for FfiRc<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.ptr == other.ptr {
            return true;
        }
        **self == **other
    }
}

impl<T: FfiDefault + Eq> Eq for FfiRc<T> {}

impl<T: FfiDefault + Hash> Hash for FfiRc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

/// Key-value pair entry matching `crubit::ffi::FfiMapEntry<K, V>` in C++.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FfiMapEntry<K, V> {
    pub key: K,
    pub value: V,
}

/// An owned sorted map backed by `FfiVec<FfiMapEntry<K, V>>` matching `crubit::ffi::FfiMap<K, V>`
/// in C++. Entries are kept sorted by key so iteration order, `PartialEq`, and `Hash` are
/// deterministic.
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

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.entries
            .as_slice()
            .binary_search_by(|e| e.key.borrow().cmp(key))
            .ok()
            .map(|idx| &self.entries.as_slice()[idx].value)
    }

    pub fn insert(&mut self, key: K, value: V) -> &mut V
    where
        K: Ord,
    {
        match self.entries.as_slice().binary_search_by(|e| e.key.cmp(&key)) {
            Ok(idx) => {
                let entry = &mut self.entries.as_mut_slice()[idx];
                entry.value = value;
                &mut entry.value
            }
            Err(insert_idx) => {
                &mut self.entries.insert(insert_idx, FfiMapEntry { key, value }).value
            }
        }
    }
}

impl<K, V> Default for FfiMap<K, V> {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<K1: Into<K>, V1: Into<V>, K: Ord, V> FromIterator<(K1, V1)> for FfiMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K1, V1)>>(iter: I) -> Self {
        let mut map = Self::EMPTY;
        for (k, v) in iter {
            map.insert(k.into(), v.into());
        }
        map
    }
}

impl<K1: Into<K>, V1: Into<V>, K: Ord, V, const N: usize> From<[(K1, V1); N]> for FfiMap<K, V> {
    fn from(arr: [(K1, V1); N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for FfiMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

/// Identifier of an IR item (`#[repr(transparent)]` over `u64`, matching `uint64_t` in C++).
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub struct ItemId(pub u64);

impl FfiDefault for ItemId {
    const EMPTY_REF: &'static Self = &Self::EMPTY;
}

impl ItemId {
    pub const EMPTY: Self = Self(0);

    pub const fn new_for_testing(value: u64) -> Self {
        Self(value)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for ItemId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl From<usize> for ItemId {
    fn from(id: usize) -> Self {
        Self(id as u64)
    }
}

impl From<ItemId> for u64 {
    fn from(id: ItemId) -> Self {
        id.0
    }
}

impl quote::ToTokens for ItemId {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use std::str::FromStr;
        proc_macro2::Literal::from_str(&format!("{:#x}", self.0)).unwrap().to_tokens(tokens)
    }
}

impl fmt::Debug for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ItemId({:#x})", self.0)
    }
}

/// Numeric identifier of a lifetime annotation (`#[repr(transparent)]` over `i32`, matching
/// `int32_t` in C++).
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub struct LifetimeId(pub i32);

impl FfiDefault for LifetimeId {
    const EMPTY_REF: &'static Self = &Self::EMPTY;
}

impl LifetimeId {
    pub const EMPTY: Self = Self(0);
}

impl From<i32> for LifetimeId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<LifetimeId> for i32 {
    fn from(id: LifetimeId) -> Self {
        id.0
    }
}

impl quote::ToTokens for LifetimeId {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        proc_macro2::Literal::i32_unsuffixed(self.0).to_tokens(tokens)
    }
}

/// Prefix of the synthetic label `rs_bindings_from_cc` assigns to declarations whose header is not
/// owned by any Bazel target visible to Crubit.
const UNKNOWN_TARGET_PREFIX: &str = "//_unknown_target:";

/// An owned Bazel label, e.g. `//foo:bar`, backed by `FfiString` (`#[repr(transparent)]`, matching
/// `FfiString` in C++).
#[repr(transparent)]
#[derive(Debug, Eq, Clone, Default)]
pub struct BazelLabel(pub FfiString);

impl FfiDefault for BazelLabel {
    const EMPTY_REF: &'static Self = &Self::EMPTY;
}

impl BazelLabel {
    pub const EMPTY: Self = Self(FfiString::EMPTY);

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn as_unknown_target_header(&self) -> Option<&str> {
        self.as_str().strip_prefix(UNKNOWN_TARGET_PREFIX)
    }

    pub fn target_name(&self) -> &str {
        let s = self.as_str();
        if let Some((_package, target_name)) = s.split_once(':') {
            return target_name;
        }
        if let Some((_, last_package_component)) = s.rsplit_once('/') {
            return last_package_component;
        }
        s
    }

    pub fn is_cc_std(&self) -> bool {
        self.target_name() == "cc_std"
    }

    pub fn package_name(&self) -> &str {
        let s = self.as_str();
        s.rsplit_once(':').unwrap_or((s, "")).0
    }

    fn last_package_component(&self) -> &str {
        self.package_name().rsplit_once('/').unwrap_or(("", "")).1
    }

    pub fn target_name_escaped(&self) -> String {
        let mut target_name = self.target_name().to_owned();
        if target_name == "core" {
            target_name = "core_".to_owned() + self.last_package_component();
        } else if target_name.starts_with(char::is_numeric) {
            target_name.insert(0, 'n');
        }
        target_name.replace(|c: char| !c.is_ascii_alphanumeric(), "_")
    }

    pub fn convert_to_cc_identifier(&self) -> String {
        use std::fmt::Write;
        let s = self.as_str();
        let mut result = "_".to_string();
        result.reserve_exact(s.len().checked_mul(2).unwrap_or(s.len()));

        for b in s.bytes() {
            if (b as char).is_ascii_alphanumeric() {
                result.push(b as char);
            } else {
                write!(result, "_{b:02x}").unwrap();
            }
        }
        result.shrink_to_fit();

        #[cfg(debug_assertions)]
        for c in result.chars() {
            debug_assert!(
                c.is_ascii_alphanumeric() || c == '_',
                "invalid result identifier: {result:?}"
            );
        }

        result
    }

    fn components(&self) -> (&str, &str) {
        (self.target_name(), self.package_name())
    }
}

impl PartialEq for BazelLabel {
    fn eq(&self, other: &Self) -> bool {
        self.components() == other.components()
    }
}

impl PartialOrd for BazelLabel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BazelLabel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.components().cmp(&other.components())
    }
}

impl Hash for BazelLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.components().hash(state);
    }
}

impl From<FfiString> for BazelLabel {
    fn from(s: FfiString) -> Self {
        Self(s)
    }
}

impl From<&FfiString> for BazelLabel {
    fn from(s: &FfiString) -> Self {
        Self(s.clone())
    }
}

impl From<&str> for BazelLabel {
    fn from(s: &str) -> Self {
        Self(FfiString::from_str(s))
    }
}

impl From<String> for BazelLabel {
    fn from(s: String) -> Self {
        Self(FfiString::from_str(&s))
    }
}

impl From<&String> for BazelLabel {
    fn from(s: &String) -> Self {
        Self(FfiString::from_str(s.as_str()))
    }
}

impl From<Cow<'_, str>> for BazelLabel {
    fn from(s: Cow<'_, str>) -> Self {
        Self(FfiString::from_str(&s))
    }
}

impl From<std::rc::Rc<str>> for BazelLabel {
    fn from(s: std::rc::Rc<str>) -> Self {
        Self(FfiString::from_str(&s))
    }
}

impl From<&BazelLabel> for BazelLabel {
    fn from(label: &BazelLabel) -> Self {
        label.clone()
    }
}

impl From<&bazel_label::BazelLabel> for BazelLabel {
    fn from(label: &bazel_label::BazelLabel) -> Self {
        Self(FfiString::from_str(label.as_str()))
    }
}

impl From<bazel_label::BazelLabel> for BazelLabel {
    fn from(label: bazel_label::BazelLabel) -> Self {
        Self(FfiString::from_str(label.as_str()))
    }
}

impl From<&BazelLabel> for bazel_label::BazelLabel {
    fn from(label: &BazelLabel) -> Self {
        bazel_label::BazelLabel::from(label.as_str())
    }
}

impl From<BazelLabel> for bazel_label::BazelLabel {
    fn from(label: BazelLabel) -> Self {
        bazel_label::BazelLabel::from(label.as_str())
    }
}

impl From<BazelLabel> for FfiString {
    fn from(label: BazelLabel) -> Self {
        label.0
    }
}

impl From<&BazelLabel> for FfiString {
    fn from(label: &BazelLabel) -> Self {
        label.0.clone()
    }
}

impl fmt::Display for BazelLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.as_unknown_target_header() {
            write!(f, "{}", s)
        } else {
            write!(f, "{}", self.as_str())
        }
    }
}

/// Marker used by the `proto!` macro to initialize nested submessages (`__ { ... }`) and
/// container fields in-place via the field setter method.
pub struct ProtoDefault;

impl<T> From<ProtoDefault> for FfiVec<T> {
    fn from(_: ProtoDefault) -> Self {
        Self::EMPTY
    }
}

impl<K, V> From<ProtoDefault> for FfiMap<K, V> {
    fn from(_: ProtoDefault) -> Self {
        Self::EMPTY
    }
}

const _: () = {
    assert!(size_of::<Cell<usize>>() == size_of::<usize>());
    assert!(align_of::<Cell<usize>>() == align_of::<usize>());
    assert!(size_of::<FfiString>() == 2 * size_of::<usize>());
    assert!(align_of::<FfiString>() == align_of::<usize>());
    assert!(size_of::<FfiVec<u8>>() == 3 * size_of::<usize>());
    assert!(align_of::<FfiVec<u8>>() == align_of::<usize>());
    assert!(size_of::<FfiRc<u8>>() == size_of::<usize>());
    assert!(align_of::<FfiRc<u8>>() == align_of::<usize>());
    assert!(size_of::<FfiMap<FfiString, FfiString>>() == 3 * size_of::<usize>());
    assert!(align_of::<FfiMap<FfiString, FfiString>>() == align_of::<usize>());
    assert!(size_of::<ItemId>() == size_of::<u64>());
    assert!(align_of::<ItemId>() == align_of::<u64>());
    assert!(size_of::<LifetimeId>() == size_of::<i32>());
    assert!(align_of::<LifetimeId>() == align_of::<i32>());
    assert!(size_of::<BazelLabel>() == size_of::<FfiString>());
    assert!(align_of::<BazelLabel>() == align_of::<FfiString>());
};

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
    use std::collections::hash_map::DefaultHasher;

    fn hash_of<T: Hash>(val: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        hasher.finish()
    }

    #[gtest]
    fn test_ffi_rc_clone_and_drop() {
        let root = FfiRc::new(FfiString::from_str("shared_payload"));
        let mut clones = Vec::with_capacity(256);
        for _ in 0..256 {
            clones.push(root.clone());
        }
        for item in &clones {
            expect_eq!(item.as_str(), "shared_payload");
        }
        drop(clones);
        expect_eq!(root.as_str(), "shared_payload");
    }

    #[gtest]
    fn test_ffi_rc_cow_and_null_default_equality() {
        let empty_null: FfiRc<FfiString> = FfiRc::EMPTY;
        let explicit_empty = FfiRc::new(FfiString::EMPTY);
        expect_eq!(empty_null, explicit_empty);
        expect_eq!(hash_of(&empty_null), hash_of(&explicit_empty));

        let shared = FfiRc::new(FfiString::from_str("original"));
        let mut copy = shared.clone();
        *copy.make_mut() = FfiString::from_str("mutated");
        expect_eq!(shared.as_str(), "original");
        expect_eq!(copy.as_str(), "mutated");
    }

    #[gtest]
    fn test_ffi_map_sorted_order_and_equality() {
        let mut map_a: FfiMap<FfiString, FfiString> = FfiMap::new();
        map_a.insert("zeta".into(), "3".into());
        map_a.insert("alpha".into(), "1".into());
        map_a.insert("mu".into(), "2".into());

        let mut map_b: FfiMap<FfiString, FfiString> = FfiMap::new();
        map_b.insert("alpha".into(), "0".into());
        map_b.insert("mu".into(), "2".into());
        map_b.insert("zeta".into(), "3".into());
        map_b.insert("alpha".into(), "1".into());

        let keys: Vec<&str> = map_a.iter().map(|(k, _)| k.as_str()).collect();
        expect_eq!(keys, vec!["alpha", "mu", "zeta"]);
        expect_eq!(map_a.get("alpha").map(FfiString::as_str), Some("1"));
        expect_eq!(map_a.get("mu").map(FfiString::as_str), Some("2"));
        expect_eq!(map_a.get("zeta").map(FfiString::as_str), Some("3"));
        expect_eq!(map_a.get("missing"), None);

        expect_eq!(map_a, map_b);
        expect_eq!(hash_of(&map_a), hash_of(&map_b));
    }
}
