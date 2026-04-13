// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NODISCARD_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NODISCARD_H_

struct [[nodiscard]] NoDiscard {};
struct [[nodiscard("You really should use this")]] NoDiscardWithMessage {};

enum [[nodiscard]] NoDiscardEnum {
  kConstant,
};
enum [[nodiscard("You really should use this")]] NoDiscardEnumWithMessage {
  kConstantWithMessage,
};

[[nodiscard]] inline void* crubit_nodiscard() { return nullptr; }
[[nodiscard("You really should use this")]] inline void*
crubit_nodiscard_message() {
  return nullptr;
}

struct NodiscardCtor {
  [[nodiscard]] NodiscardCtor(int x, int y) {}
};

struct NodiscardCtorWithMessage {
  [[nodiscard("You really should use this")]] NodiscardCtorWithMessage(int x,
                                                                       int y) {}
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NODISCARD_H_
