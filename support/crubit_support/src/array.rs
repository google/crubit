// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Const helpers used by Crubit-generated bindings for C++ array constants.

use ffi_11::c_char;

/// Returns an array of length `N` whose first elements are `elements` and whose remaining
/// elements are `padding`.
///
/// This is used by Crubit-generated bindings for C++ array constants whose initializers don't
/// explicitly initialize every element. In C++ aggregate initialization, such elements are
/// value-initialized, so `padding` is the value-initialized value of `T` (e.g. `0` or `false`).
/// For example, `constexpr int kArray[4] = {7, 8};` is bound as
/// `pub const kArray: [c_int; 4] = padded_array(&[7, 8], 0);`.
///
/// # Panics
///
/// Panics if `elements` doesn't fit in `N` elements, i.e. if `elements.len() > N`. When evaluated
/// in a const context, this is a compile-time error.
#[must_use]
pub const fn padded_array<T: Copy, const N: usize>(elements: &[T], padding: T) -> [T; N] {
    assert!(elements.len() <= N, "elements do not fit in the array");
    let mut array = [padding; N];
    let mut i = 0;
    while i < elements.len() {
        array[i] = elements[i];
        i += 1;
    }
    array
}

/// Returns a `char` array of length `N` initialized from the bytes of a string literal, as if by
/// the C++ initialization `char array[N] = "...";`.
///
/// `bytes` excludes the NUL terminator. The elements after `bytes` are NUL, including at least
/// one NUL terminator.
///
/// This is used by Crubit-generated bindings for C++ `char` array constants which are initialized
/// by string literals. For example, `constexpr char kNoop[8] = "NOOP";` is bound as
/// `pub const kNoop: [c_char; 8] = c_char_array_from_string_literal(b"NOOP");`.
///
/// # Panics
///
/// Panics if `bytes` and the NUL terminator don't fit in `N` elements, i.e. if `bytes.len() >= N`.
/// When evaluated in a const context, this is a compile-time error.
#[must_use]
pub const fn c_char_array_from_string_literal<const N: usize>(bytes: &[u8]) -> [c_char; N] {
    assert!(bytes.len() < N, "string literal and its NUL terminator do not fit in the array");
    let mut array = [ffi_11::new_c_char(0); N];
    let mut i = 0;
    while i < bytes.len() {
        array[i] = ffi_11::new_c_char(bytes[i]);
        i += 1;
    }
    array
}

#[cfg(test)]
mod tests {
    use super::*;
    use ffi_11::{c_int, c_longlong, new_c_char, new_c_longlong};
    use googletest::{expect_eq, gtest};

    fn to_bytes<const N: usize>(array: [c_char; N]) -> [u8; N] {
        array.map(u8::from)
    }

    #[gtest]
    fn test_padded_array_pads() {
        const ARRAY: [c_int; 4] = padded_array(&[7, 8], 0);
        expect_eq!(ARRAY, [7, 8, 0, 0]);
    }

    #[gtest]
    fn test_padded_array_exact_fit() {
        const ARRAY: [c_int; 2] = padded_array(&[7, 8], 0);
        expect_eq!(ARRAY, [7, 8]);
    }

    #[gtest]
    fn test_padded_array_empty() {
        const ARRAY: [u8; 3] = padded_array(&[], 0);
        expect_eq!(ARRAY, [0; 3]);
    }

    #[gtest]
    fn test_padded_array_bool() {
        const ARRAY: [bool; 3] = padded_array(&[true], false);
        expect_eq!(ARRAY, [true, false, false]);
    }

    #[gtest]
    fn test_padded_array_newtype() {
        const ARRAY: [c_longlong; 2] = padded_array(&[new_c_longlong(24)], new_c_longlong(0));
        expect_eq!(ARRAY, [new_c_longlong(24), new_c_longlong(0)]);
    }

    #[gtest]
    fn test_padded_array_multidimensional() {
        // `int x[3][2] = {{1}, {3, 4}};`
        const ARRAY: [[c_int; 2]; 3] = padded_array(&[padded_array(&[1], 0), [3, 4]], [0; 2]);
        expect_eq!(ARRAY, [[1, 0], [3, 4], [0, 0]]);
    }

    #[gtest]
    #[should_panic(expected = "elements do not fit in the array")]
    fn test_padded_array_too_long() {
        // Not a const, so that this panics at run time instead of failing to compile.
        let _ = padded_array::<c_int, 1>(std::hint::black_box(&[1, 2]), 0);
    }

    #[gtest]
    fn test_c_char_array_from_string_literal_pads_with_nul() {
        const NOOP: [c_char; 8] = c_char_array_from_string_literal(b"NOOP");
        expect_eq!(to_bytes(NOOP), *b"NOOP\0\0\0\0");
    }

    #[gtest]
    fn test_c_char_array_from_string_literal_exact_fit() {
        const NOOP: [c_char; 5] = c_char_array_from_string_literal(b"NOOP");
        expect_eq!(to_bytes(NOOP), *b"NOOP\0");
    }

    #[gtest]
    fn test_c_char_array_from_string_literal_empty() {
        const EMPTY: [c_char; 1] = c_char_array_from_string_literal(b"");
        expect_eq!(EMPTY, [new_c_char(0)]);
    }

    #[gtest]
    fn test_c_char_array_from_string_literal_embedded_nul() {
        const EMBEDDED: [c_char; 5] = c_char_array_from_string_literal(b"a\0b");
        expect_eq!(to_bytes(EMBEDDED), *b"a\0b\0\0");
    }

    #[gtest]
    #[should_panic(expected = "string literal and its NUL terminator do not fit in the array")]
    fn test_c_char_array_from_string_literal_no_room_for_nul_terminator() {
        // Not a const, so that this panics at run time instead of failing to compile.
        let _ = c_char_array_from_string_literal::<4>(std::hint::black_box(b"NOOP"));
    }
}
