// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, expect_true, gtest};

/// `expect_eq!` requires both sides to have the same type, so this checks that typed constants
/// have the same types as in C++, and that untyped constants fit in them.
#[gtest]
fn test_values_match_cc() {
    expect_eq!(macro_constants::INT_CONSTANT!(), macro_constants::IntConstant());
    expect_eq!(macro_constants::UNSIGNED_CONSTANT!(), macro_constants::UnsignedConstant());
    expect_eq!(macro_constants::LONG_LONG_CONSTANT!(), macro_constants::LongLongConstant());
    expect_eq!(macro_constants::HEX_CONSTANT!(), macro_constants::HexConstant());
    expect_eq!(macro_constants::SHIFTED_CONSTANT!(), macro_constants::ShiftedConstant());
    expect_eq!(macro_constants::BITWISE_CONSTANT!(), macro_constants::BitwiseConstant());
    expect_eq!(macro_constants::BOOL_CONSTANT!(), macro_constants::BoolConstant());
    // `ffi_11::c_char` isn't a primitive type, so an untyped literal can't have its type.
    expect_eq!(
        ffi_11::new_c_char(macro_constants::CHAR_CONSTANT!()),
        macro_constants::CharConstant()
    );
}

#[gtest]
fn test_textual_header_constant() {
    expect_eq!(macro_constants::TEXTUAL_HEADER_CONSTANT!(), 5);
}

fn take_i64(value: i64) -> i64 {
    value
}

fn take_usize(value: usize) -> usize {
    value
}

/// Untyped constants can be used as any integer type that their value fits in.
#[gtest]
fn test_untyped_constants() {
    use macro_constants::*;

    // `INT_CONSTANT` and `HEX_CONSTANT` are an `int` and an `unsigned int` in C++.
    expect_eq!(take_i64(INT_CONSTANT!()), 4);
    expect_eq!(take_usize(INT_CONSTANT!()), 4);
    expect_eq!(take_i64(HEX_CONSTANT!()), 0xFFFFFFFF);
    let array = [0u8; INT_CONSTANT!()];
    expect_eq!(array.len(), 4);
    expect_true!(matches!(4u8, INT_CONSTANT!()));
}

/// Fails to compile unless `value` has type `T`.
fn assert_type<T>(_value: T) {}

/// Checks the value of each kind of literal, and the type of the typed ones. See
/// `macro_constants.h`.
#[gtest]
fn test_literal_types() {
    use ffi_11::{c_long, c_longlong, c_uint, c_ulong, c_ulonglong, new_c_char};
    use macro_constants::*;

    // `expect_eq!` checks that the value matches C++ on every platform, and that the type matches
    // (for typed constants) or that the value fits in the C++ type (for untyped constants).
    expect_eq!(DECIMAL_INT_MAX!(), DECIMAL_INT_MAX_value());
    expect_eq!(DECIMAL_INT_MAX_PLUS_1!(), DECIMAL_INT_MAX_PLUS_1_value());
    expect_eq!(DECIMAL_LLONG_MAX!(), DECIMAL_LLONG_MAX_value());
    expect_eq!(HEX_INT_MAX!(), HEX_INT_MAX_value());
    expect_eq!(HEX_UINT_MAX!(), HEX_UINT_MAX_value());
    expect_eq!(HEX_UINT_MAX_PLUS_1!(), HEX_UINT_MAX_PLUS_1_value());
    expect_eq!(HEX_LLONG_MAX!(), HEX_LLONG_MAX_value());
    expect_eq!(HEX_ULLONG_MAX!(), HEX_ULLONG_MAX_value());
    expect_eq!(OCTAL_INT_MAX!(), OCTAL_INT_MAX_value());
    expect_eq!(OCTAL_UINT_MAX!(), OCTAL_UINT_MAX_value());
    expect_eq!(OCTAL_ULLONG_MAX!(), OCTAL_ULLONG_MAX_value());
    expect_eq!(BINARY_INT_MAX!(), BINARY_INT_MAX_value());
    expect_eq!(BINARY_UINT_MAX!(), BINARY_UINT_MAX_value());
    expect_eq!(DECIMAL_U!(), DECIMAL_U_value());
    expect_eq!(DECIMAL_U_UINT_MAX_PLUS_1!(), DECIMAL_U_UINT_MAX_PLUS_1_value());
    expect_eq!(HEX_U!(), HEX_U_value());
    expect_eq!(DECIMAL_L!(), DECIMAL_L_value());
    expect_eq!(DECIMAL_L_INT_MAX_PLUS_1!(), DECIMAL_L_INT_MAX_PLUS_1_value());
    expect_eq!(HEX_L_UINT_MAX!(), HEX_L_UINT_MAX_value());
    expect_eq!(HEX_L_ULLONG_MAX!(), HEX_L_ULLONG_MAX_value());
    expect_eq!(DECIMAL_UL!(), DECIMAL_UL_value());
    expect_eq!(DECIMAL_LU!(), DECIMAL_LU_value());
    expect_eq!(DECIMAL_UL_UINT_MAX_PLUS_1!(), DECIMAL_UL_UINT_MAX_PLUS_1_value());
    expect_eq!(DECIMAL_LL!(), DECIMAL_LL_value());
    expect_eq!(DECIMAL_LL_LLONG_MAX!(), DECIMAL_LL_LLONG_MAX_value());
    expect_eq!(HEX_LL_ULLONG_MAX!(), HEX_LL_ULLONG_MAX_value());
    expect_eq!(DECIMAL_ULL!(), DECIMAL_ULL_value());
    expect_eq!(DECIMAL_LLU!(), DECIMAL_LLU_value());
    expect_eq!(new_c_char(CHAR_LITERAL!()), CHAR_LITERAL_value());
    expect_eq!(CHAR16_LITERAL!(), CHAR16_LITERAL_value());
    expect_eq!(CHAR32_LITERAL!(), CHAR32_LITERAL_value());
    expect_eq!(TRUE_LITERAL!(), TRUE_LITERAL_value());
    expect_eq!(FALSE_LITERAL!(), FALSE_LITERAL_value());

    // The types of the typed constants: those with a suffix, prefixed character literals, and
    // booleans.
    assert_type::<c_uint>(DECIMAL_U!());
    assert_type::<c_uint>(HEX_U!());
    assert_type::<c_long>(DECIMAL_L!());
    assert_type::<c_ulong>(DECIMAL_UL!());
    assert_type::<c_ulong>(DECIMAL_LU!());
    assert_type::<c_longlong>(DECIMAL_LL!());
    assert_type::<c_longlong>(DECIMAL_LL_LLONG_MAX!());
    // To be compatible with MSVC, Clang makes hexadecimal `ll` literals signed on
    // Windows, even if they don't fit.
    #[cfg(not(windows))]
    assert_type::<c_ulonglong>(HEX_LL_ULLONG_MAX!());
    #[cfg(windows)]
    assert_type::<c_longlong>(HEX_LL_ULLONG_MAX!());
    assert_type::<c_ulonglong>(DECIMAL_ULL!());
    assert_type::<c_ulonglong>(DECIMAL_LLU!());
    assert_type::<u16>(CHAR16_LITERAL!());
    assert_type::<u32>(CHAR32_LITERAL!());
    assert_type::<bool>(TRUE_LITERAL!());
    assert_type::<bool>(FALSE_LITERAL!());
    #[cfg(all(target_pointer_width = "64", not(windows)))]
    {
        // `long` is 64 bits.
        assert_type::<c_ulong>(DECIMAL_U_UINT_MAX_PLUS_1!());
        assert_type::<c_long>(DECIMAL_L_INT_MAX_PLUS_1!());
        assert_type::<c_long>(HEX_L_UINT_MAX!());
        assert_type::<c_ulong>(HEX_L_ULLONG_MAX!());
        assert_type::<c_ulong>(DECIMAL_UL_UINT_MAX_PLUS_1!());
    }
    #[cfg(not(all(target_pointer_width = "64", not(windows))))]
    {
        // `long` is 32 bits.
        assert_type::<c_ulonglong>(DECIMAL_U_UINT_MAX_PLUS_1!());
        assert_type::<c_longlong>(DECIMAL_L_INT_MAX_PLUS_1!());
        assert_type::<c_ulong>(HEX_L_UINT_MAX!());
        assert_type::<c_ulonglong>(HEX_L_ULLONG_MAX!());
        assert_type::<c_ulonglong>(DECIMAL_UL_UINT_MAX_PLUS_1!());
    }
}
