// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACE_H_

namespace test_namespace_bindings {
struct S {
  int i;
};

// Free comment inside namespace

int f(S s);

namespace inner {
void i();
}  // namespace inner
}  // namespace test_namespace_bindings

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACE_H_
