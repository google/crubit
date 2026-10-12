// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_NONNULL_STATUS_OR_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_NONNULL_STATUS_OR_LIB_H_

#include <memory>

#include "absl/base/nullability.h"
#include "absl/status/statusor.h"

// Nullability on a template argument applies to that argument, including
// through a bridged type.
inline absl::StatusOr<absl_nonnull std::unique_ptr<int>> MakeStatusOrOfNonnull(
    int value) {
  return std::make_unique<int>(value);
}

inline absl::StatusOr<absl_nullable std::unique_ptr<int>>
MakeStatusOrOfNullNullable() {
  return nullptr;
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_NONNULL_STATUS_OR_LIB_H_
