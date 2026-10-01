// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_DEFAULT_NONNULL_SMART_POINTER_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_DEFAULT_NONNULL_SMART_POINTER_LIB_H_

#include <memory>

#include "absl/base/nullability.h"

ABSL_POINTERS_DEFAULT_NONNULL

// Unannotated, but non-null because of `ABSL_POINTERS_DEFAULT_NONNULL`.
inline std::unique_ptr<int> MakeDefaultNonnullUniquePtr(int value) {
  return std::make_unique<int>(value);
}

// An explicit annotation overrides the file default.
inline absl_nullable std::unique_ptr<int> MakeExplicitlyNullableUniquePtr(
    int value) {
  return std::make_unique<int>(value);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_DEFAULT_NONNULL_SMART_POINTER_LIB_H_
