// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOT_CRUBIT_EXPOSED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOT_CRUBIT_EXPOSED_H_

struct NotCrubitExposed {
  int a;
};

typedef int NotCrubitExposedInt;

// Used in casts in macros in `macro_constants.h`.
struct ClassForCast {
  constexpr explicit ClassForCast(int) {}
};
template <typename T>
struct ClassTemplateForCast {
  constexpr explicit ClassTemplateForCast(int) {}
};
typedef ClassTemplateForCast<int> ClassTemplateForCastInt;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOT_CRUBIT_EXPOSED_H_
