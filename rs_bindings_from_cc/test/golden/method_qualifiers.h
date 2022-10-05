// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_METHOD_QUALIFIERS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_METHOD_QUALIFIERS_H_

struct Noninline {
  void UnqualifiedMethod();
  void LvalueMethod() &;
  void LvalueMethodConst() const &;
  void RvalueMethod() &&;
  void RvalueMethodConst() const &&;
};

struct Inline {
  void UnqualifiedMethod() {}
  void LvalueMethod() & {}
  void LvalueMethodConst() const & {}
  void RvalueMethod() && {}
  void RvalueMethodConst() const && {}
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_METHOD_QUALIFIERS_H_
