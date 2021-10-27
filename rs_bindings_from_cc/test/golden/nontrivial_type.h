// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

struct Nontrivial {
  Nontrivial(Nontrivial&&);
  ~Nontrivial();

  int field;
};

struct NontrivialInline {
  NontrivialInline(NontrivialInline&&) {}
  ~NontrivialInline() {}

  int field;
};

void TakesByValue(Nontrivial nontrivial);
void TakesByValueInline(NontrivialInline nontrivial);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_
