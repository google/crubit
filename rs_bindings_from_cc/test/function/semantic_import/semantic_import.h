// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SEMANTIC_IMPORT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SEMANTIC_IMPORT_H_

class S {
 public:
  explicit S(int x) : x_(x) {}
  int x() const { return x_; }
  void set_x(int x) { x_ = x; }

 private:
  int x_;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SEMANTIC_IMPORT_H_
