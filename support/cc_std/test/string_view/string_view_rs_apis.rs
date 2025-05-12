// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::raw_string_view;
use cc_std::std::string_view;

/// Converts a raw_string_view to a &'static str.
///
/// SAFETY: Behavior is undefined if the raw_string_view has an invalid pointer,
/// or points to data with non-static lifetime or an aliasing write occurs during
/// the rest of its lifetime.
unsafe fn to_str(sv: raw_string_view) -> &'static str {
    let bytes: &'static [u8] = unsafe { &*<*const [u8]>::from(sv) };
    core::str::from_utf8(bytes).unwrap()
}

/// # Safety
/// `val` is a valid string_view for the duration of the call.
pub unsafe fn consume_raw_string_view(val: raw_string_view) {
    assert_eq!(to_str(val), "Hello World")
}

pub fn return_raw_string_view() -> raw_string_view {
    raw_string_view::from("Hello World")
}

pub fn consume_string_view<'a>(val: string_view<'a>) {
    // Safety: The string_view doesn't alias.
    unsafe { assert_eq!(val.to_str().unwrap(), "Hello World") }
}

pub fn return_string_view() -> string_view<'static> {
    string_view::from("Hello World")
}
