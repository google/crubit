// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DEFAULT_MEMBER_FUNCTIONS_DEFAULT_MEMBER_FUNCTIONS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DEFAULT_MEMBER_FUNCTIONS_DEFAULT_MEMBER_FUNCTIONS_H_

#include <vector>

class Uncopyable {
 public:
  Uncopyable() = default;
  Uncopyable(const Uncopyable&) = delete;
  Uncopyable& operator=(const Uncopyable&) = delete;
};

class UncopyableDespiteDecl {
 public:
  UncopyableDespiteDecl() : vector_() {}
  // Copy ctor declared, but will fail to instantiate if called.
  UncopyableDespiteDecl(const UncopyableDespiteDecl&) = default;

 private:
  std::vector<Uncopyable> vector_;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DEFAULT_MEMBER_FUNCTIONS_DEFAULT_MEMBER_FUNCTIONS_H_
