// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_FREE_FUNCTION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_FREE_FUNCTION_H_

struct S {
  const int& int_accessor() const { return int_field; }

  int int_field;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_FREE_FUNCTION_H_
