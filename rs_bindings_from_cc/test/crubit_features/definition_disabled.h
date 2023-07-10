// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_CRUBIT_FEATURES_DEFINITION_DISABLED_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_CRUBIT_FEATURES_DEFINITION_DISABLED_H_

struct DisabledStruct final {
  int x;
};

template <typename T>
struct DisabledTemplate final {
  T x;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_CRUBIT_FEATURES_DEFINITION_H_
