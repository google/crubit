// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_RULE_MATH_DEPENDENCY_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_RULE_MATH_DEPENDENCY_H_

namespace math_dependency {

template <typename T>
T Add(T a, T b) {
  return a + b;
}

struct MyStruct {
  int x;
};

}  // namespace math_dependency

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_RULE_MATH_DEPENDENCY_H_
