// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_CONSTANTS_MACRO_CONSTANTS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_CONSTANTS_MACRO_CONSTANTS_H_

#include "rs_bindings_from_cc/test/macro_constants/macro_constants_textual.inc"

#define INT_CONSTANT 4
#define UNSIGNED_CONSTANT 42u
#define HEX_CONSTANT 0xFFFFFFFF
#define CHAR_CONSTANT 'x'

// Functions returning the values of the macros, as computed by the C++
// compiler.
inline int IntConstant() { return INT_CONSTANT; }
inline unsigned int UnsignedConstant() { return UNSIGNED_CONSTANT; }
inline unsigned int HexConstant() { return HEX_CONSTANT; }
inline char CharConstant() { return CHAR_CONSTANT; }

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_CONSTANTS_MACRO_CONSTANTS_H_
