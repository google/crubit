// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_NONNULL_PAIR_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_NONNULL_PAIR_LIB_H_

#include <memory>
#include <utility>

#include "absl/base/nullability.h"

// Nullability on a template argument applies to that argument, including
// through a `std::pair`, which has more than one template argument.
inline std::pair<absl_nonnull std::unique_ptr<int>, int> MakePairOfNonnull(
    int value) {
  return {std::make_unique<int>(value), value + 1};
}

inline std::pair<int, absl_nonnull std::unique_ptr<int>>
MakePairOfNonnullSecond(int value) {
  return {value + 1, std::make_unique<int>(value)};
}

inline std::pair<absl_nullable std::unique_ptr<int>, int>
MakePairOfNullNullable() {
  return {nullptr, 0};
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_NONNULL_PAIR_LIB_H_
