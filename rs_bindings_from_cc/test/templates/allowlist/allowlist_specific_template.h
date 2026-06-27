// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This file contains definitions for a very simple named implicit template
// instantiation test.

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_ALLOWLIST_ALLOWLIST_SPECIFIC_TEMPLATE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_ALLOWLIST_ALLOWLIST_SPECIFIC_TEMPLATE_H_

#include "support/annotations.h"

template <typename T, typename S>
struct CRUBIT_ALWAYS_INSTANTIATE AlwaysBoundTs {
  AlwaysBoundTs(T t, S s) {}
  void Member() {}
};

template <typename T, typename S>
struct NotBoundTs {
  NotBoundTs(T t, S s) {}
  void Member() {}
};

void IntFloatCaller(const AlwaysBoundTs<int, float> i);
void NotBoundCaller(const NotBoundTs<int, float> i);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_ALLOWLIST_ALLOWLIST_SPECIFIC_TEMPLATE_H_
