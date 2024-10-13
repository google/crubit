// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NON_MEMBER_OPERATOR_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NON_MEMBER_OPERATOR_H_

namespace ns {
struct X {
  int f;
};
}  // namespace ns
inline constexpr bool operator==(ns::X a, ns::X b) { return true; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NON_MEMBER_OPERATOR_H_
