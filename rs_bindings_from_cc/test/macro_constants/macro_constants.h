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

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_CONSTANTS_MACRO_CONSTANTS_H_
