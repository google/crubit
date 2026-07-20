// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_CYCLE_TEST_LIBRARY_TO_BIND_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_CYCLE_TEST_LIBRARY_TO_BIND_H_

class MyType {
 public:
  int Add(int a, int b) { return a + b; }
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_CYCLE_TEST_LIBRARY_TO_BIND_H_
