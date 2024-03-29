// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_UNSUPPORTED_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_UNSUPPORTED_H_

#include "rs_bindings_from_cc/test/golden/unsupported.h"

#pragma clang lifetime_elision

void UseNontrivialCustomType(NontrivialCustomType non_trivial_custom_type);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_UNSUPPORTED_H_
