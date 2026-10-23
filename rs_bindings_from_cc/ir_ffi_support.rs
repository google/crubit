// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! `#[repr(C)]` FFI vocabulary types (`FfiStr`, `FfiVec<T>`, `FfiRc<T>`, `FfiMap<K, V>`,
//! `ItemId`, `LifetimeId`, `BazelLabel`) shared in-memory between C++ (`ir_ffi_support.h`) and
//! Rust.

use ffi_types::{AllocFfiBytes, FreeFfiBytes};
use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ptr;
use std::slice;

/// Trait providing a static empty reference for default-fallback dereferencing of nullable FFI
/// pointers (`FfiRc<T>`).
pub trait FfiDefault: 'static {
    const EMPTY_REF: &'static Self;
}

impl FfiDefault for u8 {
    const EMPTY_REF: &'static Self = &0;
}

/// An owned UTF-8 string with a `#[repr(C)]` layout matching `crubit::ffi::FfiStr` in C++.
#[repr(C)]
pub struct FfiStr {
    ptr: *const u8,
    len: usize,
}

unsafe impl Send for FfiStr {}
unsafe impl Sync for FfiStr {}

impl FfiDefault for FfiStr {
    const EMPTY_REF: &'static Self = &Self::EMPTY;
}

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

impl Borrow<str> for FfiStr {
    fn borrow(&self) -> &str {
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

impl From<&String> for FfiStr {
    fn from(s: &String) -> Self {
        Self::from_str(s.as_str())
    }
}

impl From<Cow<'_, str>> for FfiStr {
    fn from(s: Cow<'_, str>) -> Self {
        Self::from_str(&s)
    }
}

impl From<std::rc::Rc<str>> for FfiStr {
    fn from(s: std::rc::Rc<str>) -> Self {
        Self::from_str(&s)
    }
}

impl From<&std::rc::Rc<str>> for FfiStr {
    fn from(s: &std::rc::Rc<str>) -> Self {
        Self::from_str(s)
    }
}

impl From<&FfiStr> for FfiStr {
    fn from(s: &FfiStr) -> Self {
        s.clone()
    }
}

impl From<&FfiStr> for std::rc::Rc<str> {
    fn from(s: &FfiStr) -> Self {
        std::rc::Rc::from(s.as_str())
    }
}

impl From<FfiStr> for std::rc::Rc<str> {
    fn from(s: FfiStr) -> Self {
        std::rc::Rc::from(s.as_str())
    }
}

impl From<&FfiStr> for String {
    fn from(s: &FfiStr) -> Self {
        String::from(s.as_str())
    }
}

impl From<FfiStr> for String {
    fn from(s: FfiStr) -> Self {
        String::from(s.as_str())
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

impl PartialEq<String> for FfiStr {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<FfiStr> for str {
    fn eq(&self, other: &FfiStr) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<FfiStr> for &str {
    fn eq(&self, other: &FfiStr) -> bool {
        *self == other.as_str()
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

impl<'a> FromIterator<&'a str> for FfiVec<FfiStr> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().map(FfiStr::from).collect())
    }
}

impl FromIterator<String> for FfiVec<FfiStr> {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().map(FfiStr::from).collect())
    }
}

impl FromIterator<std::rc::Rc<str>> for FfiVec<FfiStr> {
    fn from_iter<I: IntoIterator<Item = std::rc::Rc<str>>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().map(FfiStr::from).collect())
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
    strong_count: usize,
    value: T,
}

/// Single-threaded reference-counted nullable heap pointer with a `#[repr(C)]` layout matching
/// `crubit::ffi::FfiRc<T>` in C++.
#[repr(C)]
pub struct FfiRc<T> {
    ptr: *mut FfiRcInner<T>,
}

pub type FfiBox<T> = FfiRc<T>;

unsafe impl<T: Send> Send for FfiRc<T> {}
unsafe impl<T: Sync> Sync for FfiRc<T> {}

impl<T> FfiRc<T> {
    pub const EMPTY: Self = Self { ptr: ptr::null_mut() };

    pub fn new(value: T) -> Self {
        unsafe {
            let raw = AllocFfiBytes(size_of::<FfiRcInner<T>>(), align_of::<FfiRcInner<T>>())
                as *mut FfiRcInner<T>;
            raw.write(FfiRcInner { strong_count: 1, value });
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
            if (*self.ptr).strong_count > 1 {
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
                if (*self.ptr).strong_count > 1 {
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
                (*self.ptr).strong_count -= 1;
                if (*self.ptr).strong_count == 0 {
                    ptr::drop_in_place(&mut (*self.ptr).value);
                    FreeFfiBytes(
                        self.ptr as *mut u8,
                        size_of::<FfiRcInner<T>>(),
                        align_of::<FfiRcInner<T>>(),
                    );
                }
            }
        }
    }
}

impl<T> Clone for FfiRc<T> {
    fn clone(&self) -> Self {
        if !self.ptr.is_null() {
            unsafe {
                (*self.ptr).strong_count += 1;
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

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        self.entries.as_slice().iter().find(|e| e.key.borrow() == key).map(|e| &e.value)
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

/// A Bazel label, e.g. `//foo:bar`, backed by `FfiStr` (`#[repr(transparent)]`, matching `FfiStr`
/// in C++).
#[repr(transparent)]
#[derive(Debug, Eq, Clone, Default)]
pub struct BazelLabel(pub FfiStr);

impl FfiDefault for BazelLabel {
    const EMPTY_REF: &'static Self = &Self::EMPTY;
}

impl BazelLabel {
    pub const EMPTY: Self = Self(FfiStr::EMPTY);

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

impl Borrow<str> for BazelLabel {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<FfiStr> for BazelLabel {
    fn from(s: FfiStr) -> Self {
        Self(s)
    }
}

impl From<&FfiStr> for BazelLabel {
    fn from(s: &FfiStr) -> Self {
        Self(s.clone())
    }
}

impl From<&str> for BazelLabel {
    fn from(s: &str) -> Self {
        Self(FfiStr::from_str(s))
    }
}

impl From<String> for BazelLabel {
    fn from(s: String) -> Self {
        Self(FfiStr::from_str(&s))
    }
}

impl From<&String> for BazelLabel {
    fn from(s: &String) -> Self {
        Self(FfiStr::from_str(s.as_str()))
    }
}

impl From<Cow<'_, str>> for BazelLabel {
    fn from(s: Cow<'_, str>) -> Self {
        Self(FfiStr::from_str(&s))
    }
}

impl From<std::rc::Rc<str>> for BazelLabel {
    fn from(s: std::rc::Rc<str>) -> Self {
        Self(FfiStr::from_str(&s))
    }
}

impl From<&BazelLabel> for BazelLabel {
    fn from(label: &BazelLabel) -> Self {
        label.clone()
    }
}

impl From<&bazel_label::BazelLabel> for BazelLabel {
    fn from(label: &bazel_label::BazelLabel) -> Self {
        Self(FfiStr::from_str(label.as_str()))
    }
}

impl From<bazel_label::BazelLabel> for BazelLabel {
    fn from(label: bazel_label::BazelLabel) -> Self {
        Self(FfiStr::from_str(label.as_str()))
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

impl From<BazelLabel> for FfiStr {
    fn from(label: BazelLabel) -> Self {
        label.0
    }
}

impl From<&BazelLabel> for FfiStr {
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

/// Marker used by the `proto!` macro to initialize nested submessages (`__ { ... }`) in-place
/// via the field setter method on both inline message fields and `FfiRc<T>` fields.
pub struct ProtoDefault;

const _: () = {
    assert!(size_of::<FfiStr>() == 2 * size_of::<usize>());
    assert!(align_of::<FfiStr>() == align_of::<usize>());
    assert!(size_of::<FfiVec<u8>>() == 3 * size_of::<usize>());
    assert!(align_of::<FfiVec<u8>>() == align_of::<usize>());
    assert!(size_of::<FfiRc<u8>>() == size_of::<usize>());
    assert!(align_of::<FfiRc<u8>>() == align_of::<usize>());
    assert!(size_of::<FfiMap<FfiStr, FfiStr>>() == 3 * size_of::<usize>());
    assert!(align_of::<FfiMap<FfiStr, FfiStr>>() == align_of::<usize>());
    assert!(size_of::<ItemId>() == size_of::<u64>());
    assert!(align_of::<ItemId>() == align_of::<u64>());
    assert!(size_of::<LifetimeId>() == size_of::<i32>());
    assert!(align_of::<LifetimeId>() == align_of::<i32>());
    assert!(size_of::<BazelLabel>() == size_of::<FfiStr>());
    assert!(align_of::<BazelLabel>() == align_of::<FfiStr>());
};
