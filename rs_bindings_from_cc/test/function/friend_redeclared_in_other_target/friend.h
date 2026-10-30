// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_FRIEND_REDECLARED_IN_OTHER_TARGET_FRIEND_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_FRIEND_REDECLARED_IN_OTHER_TARGET_FRIEND_H_

#include <string>

class Befriending final {
 public:
  int Value() const { return value_; }

 private:
  // Declared at namespace scope (and defined) in `redeclaration.h`, which
  // this header doesn't include, so the friend declaration is the only
  // declaration that bindings for this target see.
  friend std::string Describe(const Befriending& b);

  int value_ = 42;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_FRIEND_REDECLARED_IN_OTHER_TARGET_FRIEND_H_
