// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_NO_HEADER_MODULES_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_NO_HEADER_MODULES_LIB_H_

#include "rs_bindings_from_cc/test/modules_off/dep_lib.h"

// Built with `features = ["-header_modules"]`.
inline int NoHeaderModulesGetValue(DepStruct s) { return s.value + 4; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_NO_HEADER_MODULES_LIB_H_
