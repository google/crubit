// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_EXPLICIT_SPECIALIZATION_EXPLICIT_SPECIALIZATION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_EXPLICIT_SPECIALIZATION_EXPLICIT_SPECIALIZATION_H_

template <typename T>
struct X {
  T t;
};

template <>
struct X<int> {
  int val;
  X<int>() : val(42) {}
};

inline X<int> ReturnX() { return X<int>(); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_EXPLICIT_SPECIALIZATION_EXPLICIT_SPECIALIZATION_H_
