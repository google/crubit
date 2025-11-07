// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATE_INST_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATE_INST_H_

#include "rs_bindings_from_cc/test/golden/template_def_with_crubit.h"
#include "rs_bindings_from_cc/test/golden/template_fwd_without_crubit.h"

// This library reproduces the issue in b/458678348.

MyTemplate<int> GetMyTemplate();

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATE_INST_H_
