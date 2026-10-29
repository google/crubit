// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_MACRO_CONSTANTS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_MACRO_CONSTANTS_H_

/// A simple integer constant.
#define INT_CONSTANT 4

// The types follow the C++ rules.
#define UNSIGNED_CONSTANT 42u
#define LONG_LONG_CONSTANT 1LL
#define HEX_CONSTANT 0xFFFFFFFF
#define CHAR_CONSTANT 'x'
#define CHAR16_CONSTANT u'x'
#define CHAR32_CONSTANT U'x'
#define BOOL_CONSTANT true
#define FALSE_CONSTANT false
#define COMPARISON (4 > 3)

// Only the definition that is in effect at the end of the header is used.
#define REDEFINED_CONSTANT 1
#undef REDEFINED_CONSTANT
#define REDEFINED_CONSTANT 2
#define UNDEFINED_CONSTANT 1
#undef UNDEFINED_CONSTANT

// A macro whose name is already declared is skipped: the constant would
// conflict with the declaration.
constexpr int kAlreadyDeclared = 1;
#define kAlreadyDeclared 2

// Parentheses.
#define PARENTHESIZED_CONSTANT ((4))

// Operators.
#define NEGATIVE_CONSTANT -1
#define UNARY_PLUS_CONSTANT +1
#define COMPLEMENT_CONSTANT ~0
#define NOT_CONSTANT !true
#define SHIFTED_CONSTANT (1u << 31)
#define BITWISE_CONSTANT ((0xF0 | 0x0F) & ~0x3)
#define LOGICAL_CONSTANT (true && !false)

// Each binary operator. The operands are chosen so that each operator gives a
// different value.
#define MUL_CONSTANT (29 * 3)
#define DIV_CONSTANT (29 / 3)
#define REM_CONSTANT (29 % 3)
#define ADD_CONSTANT (29 + 3)
#define SUB_CONSTANT (29 - 3)
#define SHL_CONSTANT (29 << 3)
#define SHR_CONSTANT (29 >> 3)
#define AND_CONSTANT (29 & 3)
#define XOR_CONSTANT (29 ^ 3)
#define OR_CONSTANT (29 | 3)
// The results of comparing 1, 2, and 3 with 2, as the bits of an integer.
#define LT_CONSTANT ((1 < 2) | (2 < 2) << 1 | (3 < 2) << 2)
#define GT_CONSTANT ((1 > 2) | (2 > 2) << 1 | (3 > 2) << 2)
#define LE_CONSTANT ((1 <= 2) | (2 <= 2) << 1 | (3 <= 2) << 2)
#define GE_CONSTANT ((1 >= 2) | (2 >= 2) << 1 | (3 >= 2) << 2)
#define EQ_CONSTANT ((1 == 2) | (2 == 2) << 1 | (3 == 2) << 2)
#define NE_CONSTANT ((1 != 2) | (2 != 2) << 1 | (3 != 2) << 2)
#define LOGICAL_AND_CONSTANT (true && false)
#define LOGICAL_OR_CONSTANT (false || true)

// Precedence and associativity.
#define PRECEDENCE_CONSTANT (1 + 2 * 3)
#define LEFT_ASSOCIATIVE_CONSTANT (10 - 4 - 3)
#define SHIFT_VS_ADD_CONSTANT (1 << 2 + 1)

// The following macros are not supported, and are silently ignored.
#define EMPTY_MACRO
#define FUNCTION_LIKE_MACRO(x) ((x) + 1)
#define FLOAT_CONSTANT 1.5
#define STRING_CONSTANT "hello"
#define NOT_A_NUMBER 1.2.3
#define DIVISION_BY_ZERO (1 / 0)
#define OVERFLOW_CONSTANT (2147483647 + 1)
#define WIDE_CHAR_CONSTANT L'x'
#define UTF8_CHAR_CONSTANT u8'x'
#define __LEADING_DUNDER_CONSTANT 1
#define self 1
#define _ 1
#define CAST_CONSTANT ((unsigned char)1)

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_MACRO_CONSTANTS_H_
