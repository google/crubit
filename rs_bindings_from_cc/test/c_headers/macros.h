// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef MACROS_H_
#define MACROS_H_

#ifdef __cplusplus
extern "C" {
#endif

#define INT_CONST 42
#define HEX_CONST 0x2A
#define NEG_CONST -42
#define FLOAT_CONST 3.14f
#define STRING_CONST "hello"
#define CHAR_CONST 'A'
#define EXPR_CONST (INT_CONST + 1)
#define CAST_CONST ((int)42)

#ifdef __cplusplus
}
#endif

#endif  // MACROS_H_
