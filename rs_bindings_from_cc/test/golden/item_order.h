// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ITEM_ORDER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ITEM_ORDER_H_

#pragma clang lifetime_elision

struct FirstStruct final {
  int field;
};

inline int first_func() { return 42; }

struct SecondStruct final {
  int field;
};

inline int second_func() { return 23; }

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ITEM_ORDER_H_
