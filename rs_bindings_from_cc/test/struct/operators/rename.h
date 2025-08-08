// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_RENAME_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_RENAME_H_

#include "support/annotations.h"

#pragma clang lifetime_elision

struct BitwiseNotAsRustNot {
  CRUBIT_RUST_NAME("!")
  BitwiseNotAsRustNot operator~() const { return BitwiseNotAsRustNot{~i}; }

  int i;
};

struct TwoNots {
  CRUBIT_RUST_NAME("logical_not")
  TwoNots operator!() const { return TwoNots{!i}; }

  CRUBIT_RUST_NAME("!")
  TwoNots operator~() const { return TwoNots{~i}; }

  int i;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_RENAME_H_
