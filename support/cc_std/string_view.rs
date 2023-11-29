// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::std::string_view;
use core::ptr;

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
impl From<&[u8]> for string_view {
    fn from(s: &[u8]) -> Self {
        let size = s.len();
        let ptr = if size == 0 { 0 as *const _ } else { s.as_ptr() };

        // TODO(jeanpierreda): We can't access the constructors at the moment.
        // This little maneuver's gonna cost us 51 years of annoying build breakages
        // later, so really we should try to get the constructors callable.
        unsafe {
            let mut sv = <core::mem::MaybeUninit<string_view>>::zeroed().assume_init();
            sv.__data_ = core::mem::transmute(ptr);
            sv.__size_ = core::mem::transmute(size);
            sv
        }
    }
}

impl From<&str> for string_view {
    fn from(s: &str) -> Self {
        string_view::from(s.as_bytes())
    }
}
