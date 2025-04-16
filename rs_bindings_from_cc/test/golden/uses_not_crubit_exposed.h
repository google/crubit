// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USES_NOT_CRUBIT_EXPOSED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USES_NOT_CRUBIT_EXPOSED_H_

#include "rs_bindings_from_cc/test/golden/not_crubit_exposed.h"

void UseNotCrubitExposed(NotCrubitExposed not_crubit_exposed);

struct CannotUpcastInCrubit : public NotCrubitExposed {};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USES_NOT_CRUBIT_EXPOSED_H_
