// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This test checks that we generate complete bindings for a specific
// template instance.

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_ALLOWLIST_ALLOWLIST_SPECIFIC_INSTANCE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_ALLOWLIST_ALLOWLIST_SPECIFIC_INSTANCE_H_

#include "support/annotations.h"

template <typename T, typename S>
struct Ts {
  Ts(T t, S s) {}
  void Member() {}
};

CRUBIT_BIND_INSTANTIATION(Ts<int, float>);

void IntFloatCaller(const Ts<int, float> i);
void ShortDoubleCaller(const Ts<short, double> i);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_ALLOWLIST_ALLOWLIST_SPECIFIC_INSTANCE_H_
