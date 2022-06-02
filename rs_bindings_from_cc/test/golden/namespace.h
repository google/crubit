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

inline void inline_function() {}

namespace inner {
void i();
}  // namespace inner
}  // namespace test_namespace_bindings

test_namespace_bindings::S identity(test_namespace_bindings::S s);

namespace test_namespace_bindings_reopened {
void x();
namespace inner {
struct S {};
}  // namespace inner
}  // namespace test_namespace_bindings_reopened

namespace test_namespace_bindings_reopened {
void y();
namespace inner {
void z(S s);
}  // namespace inner
}  // namespace test_namespace_bindings_reopened
#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACE_H_
