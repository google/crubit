// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CRUBIT_FEATURES_DEFINITION_ENABLED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CRUBIT_FEATURES_DEFINITION_ENABLED_H_

struct EnabledStruct {
  int x;
};

template <typename T>
struct EnabledTemplate {
  T x;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CRUBIT_FEATURES_DEFINITION_ENABLED_H_
