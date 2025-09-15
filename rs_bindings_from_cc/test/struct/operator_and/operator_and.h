// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATOR_AND_OPERATOR_AND_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATOR_AND_OPERATOR_AND_H_

class MyBadClass {
 public:
  // NOLINTNEXTLINE
  int operator&() const { return 42; }

  static MyBadClass& Returns() {
    static MyBadClass x;
    return x;
  }
  static void Accepts(MyBadClass&) {}
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATOR_AND_OPERATOR_AND_H_
