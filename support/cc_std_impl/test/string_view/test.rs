// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::raw_string_view;
use cc_std::std::string_view;
use googletest::prelude::*;
use std::sync::LazyLock;
use string_view_cc_apis::crubit_string_view::GetDefault;
use string_view_cc_apis::crubit_string_view::GetHelloWorld;
use string_view_cc_apis::crubit_string_view::GetInvalidUTF;

/// Converts a raw_string_view to a &'static str.
///
/// SAFETY: Behavior is undefined if the raw_string_view has an invalid pointer,
/// or points to data with non-static lifetime.
unsafe fn to_str(sv: raw_string_view) -> &'static str {
    let bytes: &'static [u8] = unsafe { &*<*const [u8]>::from(sv) };
    core::str::from_utf8(bytes).unwrap()
}

#[gtest]
fn test_len_and_empty() {
    let original: &'static str = "";
    let sv: string_view = original.into();
    assert_eq!(sv.len(), 0);
    assert!(sv.is_empty());

    let original: &'static str = "12345";
    let sv: string_view = original.into();
    assert_eq!(sv.len(), 5);
    assert_eq!(sv.is_empty(), false);
}

#[gtest]
fn test_contains() {
    let original: &'static str = "12345";
    let sv: string_view = original.into();
    assert!(sv.contains(&b'1'));
    assert!(sv.contains(&b'5'));
    assert_eq!(sv.contains(&b'0'), false);
}

/// An empty slice round trips, but the pointer value may change.
#[gtest]
fn test_round_trip_empty_slice() {
    // we need to create an empty slice somewhere specific in memory in order to
    // test the pointer-value-discarding behavior, so let's create an array on
    // the stack.
    let stack_array: [u8; 1] = [42];
    let original = &stack_array[0..0];
    let rsv: raw_string_view = original.into();
    let raw_round_tripped = <*const [u8]>::from(rsv);
    assert_ne!(raw_round_tripped, original as *const _); // dangling -> null -> new dangling
    assert_eq!(unsafe { &*raw_round_tripped }, original);
}

#[gtest]
fn test_round_trip_str() {
    let original: &'static str = "this is a string";
    let rsv: raw_string_view = original.into();
    assert_eq!(unsafe { to_str(rsv) }, original);
}

#[gtest]
fn test_round_trip_cstr() {
    let original: &'static str = "hello, world\0";
    let cstr = core::ffi::CStr::from_bytes_with_nul(original.as_bytes()).unwrap();
    let original = &original[..original.len() - 1]; // cut off nul for the comparison.
    let rsv: raw_string_view = cstr.into();
    assert_eq!(unsafe { to_str(rsv) }, original);
}

#[gtest]
fn test_ffi() {
    assert_eq!(unsafe { to_str(GetHelloWorld()) }, "Hello, world!");
}

#[gtest]
fn test_ffi_default_string_view_livetype() {
    let rsv = GetDefault();
    let sv = unsafe { rsv.as_live() };
    assert_eq!(sv.len(), 0);
}

#[gtest]
fn test_ffi_livetype() {
    let rsv = GetHelloWorld();
    let sv = unsafe { rsv.as_live() };
    let msg = unsafe { sv.to_str() }.unwrap_or("failed");
    assert_eq!(msg, "Hello, world!");
}

#[gtest]
fn test_roundtrip_livetype() {
    let original: &'static str = "this is a string";
    let sv: string_view = original.into();
    assert_eq!(unsafe { sv.to_str() }.unwrap_or("failed"), original);
}

#[gtest]
fn test_len_livetype() {
    let original: &'static str = "this is a string";
    let sv: string_view = original.into();
    assert_eq!(sv.len(), original.len());
}

static TEST_LITERAL: &'static str = "static string";

fn get_static_string_view() -> &'static raw_string_view {
    struct SendSyncStringView(raw_string_view);
    // SAFETY: only used to share a specific string_view, which
    // can be safely shared across threads because it's immutable.
    unsafe impl Send for SendSyncStringView {}
    unsafe impl Sync for SendSyncStringView {}

    static STATIC_STRING: LazyLock<SendSyncStringView> =
        LazyLock::new(|| SendSyncStringView(TEST_LITERAL.into()));

    &STATIC_STRING.0
}

#[gtest]
fn exercise_as_static_live() {
    let static_rsv: &'static raw_string_view = get_static_string_view();

    // SAFETY: `static_rsv` is 'static (thanks to LazyLock) and points to
    // `TEST_LITERAL` which is also 'static. The safety contract is upheld.
    let sv_static: string_view<'static> = unsafe { static_rsv.as_static_live() };

    assert_eq!(sv_static.len(), TEST_LITERAL.len(), "Length should match");
    assert_eq!(
        unsafe { sv_static.as_bytes() },
        TEST_LITERAL.as_bytes(),
        "Byte content should match"
    );

    match unsafe { sv_static.to_str() } {
        Ok(s) => {
            assert_eq!(s, TEST_LITERAL, "String content should match");
            let _proof_is_static: &'static str = s; // Confirms 'static lifetime
        }
        Err(e) => panic!("Failed to convert static string_view to &str: {}", e),
    }
}

#[gtest]
fn test_invalid_utf8() {
    let rsv = GetInvalidUTF();
    let sv = unsafe { rsv.to_str() };
    assert_eq!(sv.is_err(), true);
    assert_that!(
        sv.err().unwrap().to_string(),
        contains_substring("utf-8").ignoring_unicode_case()
    );
}

#[gtest]
fn test_debug_impl() {
    // Empty string.
    let s = "";
    let sv = string_view::from(s);
    assert_eq!(format!("{sv:?}"), r#""""#);

    // Null-terminated string.
    let s = "Hello World\0";
    let sv = string_view::from(s.as_bytes());
    assert_eq!(format!("{sv:?}"), r#""Hello World\0""#);

    // String with a double quote and a backslash.
    let s: &[u8] = &[b'{', b'"', b',', b'\\', b'}'];
    let sv = string_view::from(s);
    assert_eq!(format!("{sv:?}"), r#""{\",\\}""#);

    // String with mixed valid and invalid UTF-8.
    let s: &[u8] = &[b'A', 0xf1, b'B', 0xf2, b'C', 0xf3];
    let sv = string_view::from(s);
    assert_eq!(format!("{sv:?}"), r#""A\xf1B\xf2C\xf3""#);

    // Note: the byte 0 is a special case and is printed as "\0". All other values are formatted as
    // "\x##".
    let s: &[u8] = &[0, 1, 2, 3];
    let sv = string_view::from(s);
    assert_eq!(format!("{sv:?}"), r#""\0\x01\x02\x03""#);
}
