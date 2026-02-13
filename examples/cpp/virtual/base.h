// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_CPP_METHOD_BASE_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_CPP_METHOD_BASE_H_

// An example base class.
class ExampleBase {
 public:
  virtual ~ExampleBase() = default;
  virtual int Method1() const = 0;
  int Method2() const { return 1; }
};

inline int GetMethod1(const ExampleBase* base) { return base->Method1(); }

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_CPP_METHOD_BASE_H_
