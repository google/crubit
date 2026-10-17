// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_FNO_MODULES_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_FNO_MODULES_LIB_H_

#include "rs_bindings_from_cc/test/modules_off/dep_lib.h"

// Compiled with `-fno-modules` (and without header modules). The generated
// `_rust_api_impl.cc` inherits these copts, so it must not try to load `.pcm`
// files.
int FnoModulesGetValue(DepStruct s);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MODULES_OFF_FNO_MODULES_LIB_H_
