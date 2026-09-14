// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_NONNULL_SMART_POINTER_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_NONNULL_SMART_POINTER_LIB_H_

#include <memory>

#include "absl/base/nullability.h"
#include "rs_bindings_from_cc/test/manual_bridge_vocabulary_types/common.h"

inline absl_nonnull std::unique_ptr<int> MakeNonnullUniquePtr(int value) {
  return std::make_unique<int>(value);
}

inline int UseNonnullUniquePtrByValue(absl_nonnull std::unique_ptr<int> p) {
  return *p;
}

inline absl_nonnull std::shared_ptr<int> MakeNonnullSharedPtr(int value) {
  return std::make_shared<int>(value);
}

inline int UseNonnullSharedPtrByValue(absl_nonnull std::shared_ptr<int> p) {
  return *p;
}

// A `unique_ptr` whose element type overloads `operator delete` becomes
// `virtual_unique_ptr` rather than `unique_ptr`, which is a separate branch in
// the code that spells the type.
inline absl_nonnull std::unique_ptr<OverloadedDelete>
MakeNonnullVirtualUniquePtr() {
  return std::make_unique<OverloadedDelete>();
}

// Unannotated, for contrast: this stays a plain `unique_ptr` on the Rust side.
inline std::unique_ptr<int> MakeUniquePtr(int value) {
  return std::make_unique<int>(value);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_NONNULL_SMART_POINTER_LIB_H_
