// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSTANT_CONSTANT_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSTANT_CONSTANT_H_

enum Foo {
  BAR,
};

// Regression test for b/526241525. We should not see an `ItemId` in the
// resulting comment.
inline constexpr auto kEnum = Foo::BAR;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSTANT_CONSTANT_H_
