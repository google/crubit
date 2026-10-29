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

// The following macros are not supported, and are silently ignored.
#define EMPTY_MACRO
#define FUNCTION_LIKE_MACRO(x) ((x) + 1)
#define FLOAT_CONSTANT 1.5
#define STRING_CONSTANT "hello"
#define NOT_A_NUMBER 1.2.3
#define WIDE_CHAR_CONSTANT L'x'
#define UTF8_CHAR_CONSTANT u8'x'
#define __LEADING_DUNDER_CONSTANT 1
#define self 1
#define _ 1
#define CAST_CONSTANT ((unsigned char)1)

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_MACRO_CONSTANTS_H_
