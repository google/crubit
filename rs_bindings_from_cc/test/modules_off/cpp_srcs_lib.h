// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_CPP_SRCS_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_CPP_SRCS_LIB_H_

#include "rs_bindings_from_cc/test/modules_off/uses_fno_modules_lib.h"

// Only depends on `:uses_fno_modules_lib`, so `dep_lib.h` is a transitive
// header here.
inline int CppSrcsLibGetValue(int value) {
  return UsesFnoModulesGetValueFromInt(value);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_CPP_SRCS_LIB_H_
