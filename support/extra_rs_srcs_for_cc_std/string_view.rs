// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::std::string_view;
use std::convert::TryFrom;
use std::ptr;

impl From<string_view> for *const [u8] {
    fn from(sv: string_view) -> Self {
        let mut data = unsafe { string_view::data(&sv) };
        // TODO(b/249376862): use size.
        // let size = unsafe {string_view::size(&sv)};
        let size = unsafe { string_view::end(&sv) as usize - string_view::begin(&sv) as usize };
        // Unlike C++, Rust does not allow for null data pointers in slices.
        if data.is_null() {
            data = ptr::NonNull::dangling().as_ptr();
            debug_assert_eq!(size, 0);
        }
        ptr::slice_from_raw_parts(data, size)
    }
}

/// Converts a C++ string_view to a Rust byte slice.
///
/// SAFETY: this is currently super dangerous (exactly as dangerous as C++),
/// because it assumes that all string views have `&'static` lifetime. Be
/// exactly as cautious with this as you would in C++.
// TODO(b/246425449): This should implement correct lifetimes, once string_view
// has lifetime annotations.
impl From<string_view> for &'static [u8] {
    fn from(sv: string_view) -> Self {
        let raw_slice: *const [u8] = sv.into();
        unsafe { &*raw_slice }
    }
}

/// Converts a C++ string_view to a Rust string, failing if the string_view is
/// not UTF8.
///
/// SAFETY: this is currently super dangerous (exactly as dangerous as C++),
/// because it assumes that all string views have `&'static` lifetime. Be
/// exactly as cautious with this as you would in C++.
// TODO(b/246425449): This should implement correct lifetimes, once string_view
// has lifetime annotations.
impl TryFrom<string_view> for &'static str {
    type Error = std::str::Utf8Error;
    fn try_from(sv: string_view) -> Result<Self, Self::Error> {
        std::str::from_utf8(sv.into())
    }
}

/// Currently only implementing conversion from &'static str, because
/// string_view isn't yet annotated with lifetimes, and so is unsafe to use
/// with non-static lifetimes.
// TODO(b/246425449): This should implement correct lifetimes, once string_view
// has lifetime annotations.
impl From<&'static [u8]> for string_view {
    fn from(s: &'static [u8]) -> Self {
        let ptr = s.as_ptr();
        let size = s.len();

        // TODO(jeanpierreda): We can't access the constructors at the moment.
        // This little maneuver's gonna cost us 51 years of annoying build breakages
        // later, so really we should try to get the constructors callable.
        unsafe {
            let mut sv = <std::mem::MaybeUninit<string_view>>::zeroed().assume_init();
            sv.__data_ = std::mem::transmute(ptr);
            sv.__size_ = std::mem::transmute(size);
            sv
        }
    }
}

// TODO(b/246425449): This should implement correct lifetimes, once string_view
// has lifetime annotations.
impl From<&'static str> for string_view {
    fn from(s: &'static str) -> Self {
        string_view::from(s.as_bytes())
    }
}
