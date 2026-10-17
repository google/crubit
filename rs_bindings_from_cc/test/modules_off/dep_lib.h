// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_DEP_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_DEP_LIB_H_

// A normal library (modules enabled) that the modules-off libraries depend on.
struct DepStruct final {
  int value = 0;
};

inline DepStruct MakeDepStruct(int value) { return DepStruct{value}; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_DEP_LIB_H_
