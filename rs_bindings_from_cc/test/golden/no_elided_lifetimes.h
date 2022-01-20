// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NO_ELIDED_LIFETIMES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NO_ELIDED_LIFETIMES_H_

int& free_function(int& p1);

struct S final {
  int& const_method(int& p1, int& p2) const;
  int& method(int& p1, int& p2);
};

void take_pointer(int* p);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NO_ELIDED_LIFETIMES_H_
