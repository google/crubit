// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef ENUMS_H_
#define ENUMS_H_

#ifdef __cplusplus
extern "C" {
#endif

enum Negative { NEG_ONE = -1, NEG_TWO = -2 };
enum LargeU64 { LARGE = 0xFFFFFFFFFFFFFFFFull };
enum Duplicates { DUP1 = 1, DUP2 = 1 };
enum ExplicitUnderlying {
  EX1 = 1
} __attribute__((enum_extensibility(closed), mode(QI)));
enum { ANON_ENUM_CONST = 42 };

#ifdef __cplusplus
}
#endif

#endif  // ENUMS_H_
