// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOEXCEPT_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOEXCEPT_H_

class SomeClass final {
 public:
  static void create(int i, char c) noexcept;
  void no_except_member() noexcept;
  void no_except_true_member() noexcept(true);
  void no_except_false_member() noexcept(false);
};

void no_except() noexcept;
void no_except_true() noexcept(true);
void no_except_false() noexcept(false);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOEXCEPT_H_
