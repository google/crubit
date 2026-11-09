// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_ENUMS_ENUMS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_ENUMS_ENUMS_H_

enum UnscopedEnum {
  kZero = 0,
  kOne = 1,
};

enum class ScopedEnum : int {
  kZero = 0,
  kOne = 1,
};

enum class EnumWithCollide : int {
  from_raw = 0,
  into_raw = 1,
};

enum class EnumWithOneCollide : int {
  from_raw = 0,
  kOther = 1,
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_ENUMS_ENUMS_H_
