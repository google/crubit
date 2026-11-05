// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_CONSTANTS_MACRO_CONSTANTS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_CONSTANTS_MACRO_CONSTANTS_H_

#include "rs_bindings_from_cc/test/macro_constants/macro_constants_textual.inc"

#define INT_CONSTANT 4
#define UNSIGNED_CONSTANT 42u
#define LONG_LONG_CONSTANT (-(1LL << 40))
#define HEX_CONSTANT 0xFFFFFFFF
#define SHIFTED_CONSTANT (1u << 31)
#define BITWISE_CONSTANT ((0xF0 | 0x0F) & ~0x3)
#define CONDITIONAL_CONSTANT (4 > 3 ? 10 : 20)
#define BOOL_CONSTANT (4 == 4 && !false)
#define CHAR_CONSTANT 'x'

// Functions returning the values of the macros, as computed by the C++
// compiler.
inline int IntConstant() { return INT_CONSTANT; }
inline unsigned int UnsignedConstant() { return UNSIGNED_CONSTANT; }
// `long long` rather than `int64_t`, to match the type of `1LL`. `NOLINT`
// silences both ClangTidy's google-runtime-int and cpplint's runtime/int.
inline long long LongLongConstant() {  // NOLINT
  return LONG_LONG_CONSTANT;
}
inline unsigned int HexConstant() { return HEX_CONSTANT; }
inline unsigned int ShiftedConstant() { return SHIFTED_CONSTANT; }
inline int BitwiseConstant() { return BITWISE_CONSTANT; }
inline int ConditionalConstant() { return CONDITIONAL_CONSTANT; }
inline bool BoolConstant() { return BOOL_CONSTANT; }
inline char CharConstant() { return CHAR_CONSTANT; }

// The type of an integer literal is the first type in a list that can represent
// its value. The list depends on the base and suffix, see
// https://en.cppreference.com/w/cpp/language/integer_literal. Which type that
// is depends on the platform (mostly on the size of `long`), so the expected
// Rust types are in `test.rs`.
//
// Decimal, no suffix: int, long, long long.
#define DECIMAL_INT_MAX 2147483647
#define DECIMAL_INT_MAX_PLUS_1 2147483648
#define DECIMAL_LLONG_MAX 9223372036854775807
//
// Hexadecimal, octal, or binary, no suffix: int, unsigned int, long,
// unsigned long, long long, unsigned long long.
#define HEX_INT_MAX 0x7FFFFFFF
#define HEX_UINT_MAX 0xFFFFFFFF
#define HEX_UINT_MAX_PLUS_1 0x100000000
#define HEX_LLONG_MAX 0x7FFFFFFFFFFFFFFF
#define HEX_ULLONG_MAX 0xFFFFFFFFFFFFFFFF
#define OCTAL_INT_MAX 017777777777
#define OCTAL_UINT_MAX 037777777777
#define OCTAL_ULLONG_MAX 01777777777777777777777
#define BINARY_INT_MAX 0b1111111111111111111111111111111
#define BINARY_UINT_MAX 0b11111111111111111111111111111111
//
// `u`: unsigned int, unsigned long, unsigned long long.
#define DECIMAL_U 1U
#define DECIMAL_U_UINT_MAX_PLUS_1 4294967296U
#define HEX_U 0x1U
//
// `l`: long, long long (decimal); long, unsigned long, long long, unsigned
// long long (otherwise).
#define DECIMAL_L 1L
#define DECIMAL_L_INT_MAX_PLUS_1 2147483648L
#define HEX_L_UINT_MAX 0xFFFFFFFFL
#define HEX_L_ULLONG_MAX 0xFFFFFFFFFFFFFFFFL
//
// `ul`: unsigned long, unsigned long long.
#define DECIMAL_UL 1UL
#define DECIMAL_LU 1LU
#define DECIMAL_UL_UINT_MAX_PLUS_1 4294967296UL
//
// `ll`: long long (decimal); long long, unsigned long long (otherwise).
#define DECIMAL_LL 1LL
#define DECIMAL_LL_LLONG_MAX 9223372036854775807LL
#define HEX_LL_ULLONG_MAX 0xFFFFFFFFFFFFFFFFLL
//
// `ull`: unsigned long long.
#define DECIMAL_ULL 1ULL
#define DECIMAL_LLU 1LLU
//
// Character and boolean literals.
#define CHAR_LITERAL 'x'
#define CHAR16_LITERAL u'x'
#define CHAR32_LITERAL U'x'
#define TRUE_LITERAL true
#define FALSE_LITERAL false

// Defines `NAME_value()`, which returns the macro `NAME` with the type that C++
// gives it.
#define DEFINE_VALUE_FUNCTION(name) \
  inline decltype(name) name##_value() { return name; }
DEFINE_VALUE_FUNCTION(DECIMAL_INT_MAX)
DEFINE_VALUE_FUNCTION(DECIMAL_INT_MAX_PLUS_1)
DEFINE_VALUE_FUNCTION(DECIMAL_LLONG_MAX)
DEFINE_VALUE_FUNCTION(HEX_INT_MAX)
DEFINE_VALUE_FUNCTION(HEX_UINT_MAX)
DEFINE_VALUE_FUNCTION(HEX_UINT_MAX_PLUS_1)
DEFINE_VALUE_FUNCTION(HEX_LLONG_MAX)
DEFINE_VALUE_FUNCTION(HEX_ULLONG_MAX)
DEFINE_VALUE_FUNCTION(OCTAL_INT_MAX)
DEFINE_VALUE_FUNCTION(OCTAL_UINT_MAX)
DEFINE_VALUE_FUNCTION(OCTAL_ULLONG_MAX)
DEFINE_VALUE_FUNCTION(BINARY_INT_MAX)
DEFINE_VALUE_FUNCTION(BINARY_UINT_MAX)
DEFINE_VALUE_FUNCTION(DECIMAL_U)
DEFINE_VALUE_FUNCTION(DECIMAL_U_UINT_MAX_PLUS_1)
DEFINE_VALUE_FUNCTION(HEX_U)
DEFINE_VALUE_FUNCTION(DECIMAL_L)
DEFINE_VALUE_FUNCTION(DECIMAL_L_INT_MAX_PLUS_1)
DEFINE_VALUE_FUNCTION(HEX_L_UINT_MAX)
DEFINE_VALUE_FUNCTION(HEX_L_ULLONG_MAX)
DEFINE_VALUE_FUNCTION(DECIMAL_UL)
DEFINE_VALUE_FUNCTION(DECIMAL_LU)
DEFINE_VALUE_FUNCTION(DECIMAL_UL_UINT_MAX_PLUS_1)
DEFINE_VALUE_FUNCTION(DECIMAL_LL)
DEFINE_VALUE_FUNCTION(DECIMAL_LL_LLONG_MAX)
DEFINE_VALUE_FUNCTION(HEX_LL_ULLONG_MAX)
DEFINE_VALUE_FUNCTION(DECIMAL_ULL)
DEFINE_VALUE_FUNCTION(DECIMAL_LLU)
DEFINE_VALUE_FUNCTION(CHAR_LITERAL)
DEFINE_VALUE_FUNCTION(CHAR16_LITERAL)
DEFINE_VALUE_FUNCTION(CHAR32_LITERAL)
DEFINE_VALUE_FUNCTION(TRUE_LITERAL)
DEFINE_VALUE_FUNCTION(FALSE_LITERAL)

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_CONSTANTS_MACRO_CONSTANTS_H_
