// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_USES_FNO_MODULES_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_USES_FNO_MODULES_LIB_H_

#include "rs_bindings_from_cc/test/modules_off/dep_lib.h"
#include "rs_bindings_from_cc/test/modules_off/fno_modules_lib.h"

// A normal library (modules enabled) that depends on a `-fno-modules` library.
inline int UsesFnoModulesGetValue(DepStruct s) {
  return FnoModulesGetValue(s) + 10;
}

// Same as `UsesFnoModulesGetValue(MakeDepStruct(value))`, for callers that
// don't include `dep_lib.h`.
inline int UsesFnoModulesGetValueFromInt(int value) {
  return UsesFnoModulesGetValue(MakeDepStruct(value));
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_USES_FNO_MODULES_LIB_H_
