// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::StdString;

extern crate std;
use std::slice;
use cc_std::std::string_view;
use std::string::String;
use std::vec::Vec;

impl StdString {
    pub fn as_slice(&self) -> &[u8] {
        let len = self.len();
        assert!(
            len <= isize::MAX as usize,
            "The string length does not fit in an `isize`: {}",
            len
        );
        unsafe {
            // SAFETY:
            //
            // * `StdString::data` returns the pointer of the C++ `std::string::data()`,
            //   which is guaranteed to be non-null and point to a continuous memory region.
            //   Every byte from `StdString` (i.e., [data, data + len)) is intialized.
            //   (See https://en.cppreference.com/w/cpp/string/basic_string/data)
            // * The data is guaranteed to be not mutated because we don't ever mutate
            //   data() except when accessed via &mut self, which is blocked by Rust borrow
            //   checker.
            // * `len` is guaranteed to be less than `isize::MAX` because C++
            //   implementations guarantee in practice that the object won't go past the end
            //   of the address space.
            slice::from_raw_parts(StdString::data(self) as _, len)
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        let len = self.len();
        assert!(
            len <= isize::MAX as usize,
            "The string length does not fit in an `isize`: {}",
            len
        );
        unsafe {
            // SAFETY:
            //
            // * `StdString::data` returns the pointer of the C++ `std::string::data()`,
            //   which is guaranteed to be non-null and point to a continuous memory region.
            //   And every byte from `StdString` (i.e., [data, data + len)) is intialized.
            //  (See https://en.cppreference.com/w/cpp/string/basic_string/data)
            // * `len` is guaranteed to be less than `isize::MAX` because C++
            //   implementations guarantee in practice that the object won't go past the end
            //   of the address space.
            slice::from_raw_parts_mut(StdString::data(self) as _, len)
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            // SAFETY: self is a valid reference.
            StdString::size(self)
        }
    }
}

impl From<string_view> for StdString {
    fn from(s: string_view) -> Self {
        StdString::FromStringView(s)
    }
}

impl From<&[u8]> for StdString {
    fn from(s: &[u8]) -> Self {
        string_view::from(s).into()
    }
}

impl From<&String> for StdString {
    fn from(s: &String) -> Self {
        s.as_bytes().into()
    }
}

impl From<&Vec<u8>> for StdString {
    fn from(s: &Vec<u8>) -> Self {
        s.as_slice().into()
    }
}

impl From<&str> for StdString {
    fn from(s: &str) -> Self {
        s.as_bytes().into()
    }
}

impl core::fmt::Debug for StdString {
    // TODO(jeanpierreda): format as StdString(b"...")
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StdString({:?})", self.as_slice())
    }
}

impl core::fmt::Display for StdString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.as_slice()))
    }
}

impl AsRef<[u8]> for StdString {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for StdString {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_slice_mut()
    }
}

impl Clone for StdString {
    fn clone(&self) -> Self {
        self.as_slice().into()
    }
}

impl core::ops::Deref for StdString {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl core::ops::DerefMut for StdString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}
