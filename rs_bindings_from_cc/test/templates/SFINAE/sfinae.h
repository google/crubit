// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_SFINAE_SFINAE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_SFINAE_SFINAE_H_

#pragma clang lifetime_elision

template <typename T, typename U>
int Func(typename T::TypeA) {
  return 1;
};

template <typename T, int>
int Func(typename T::TypeB) {
  return 2;
};

struct A {
  using TypeA = int;
};

inline auto func = Func<A, int>;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_SFINAE_SFINAE_H_
