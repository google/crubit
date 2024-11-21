// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_HELPER_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_HELPER_LIB_H_

#include <memory>

struct NonTrivialType {
  int x;
  ~NonTrivialType() { x = 0; }
};

inline int UseUniquePtrByValue(std::unique_ptr<int> p) { return *p; }

inline int UseUniquePtrByRef(std::unique_ptr<int>& p) { return *p; }

inline std::unique_ptr<int> MakeUniquePtr(int value) {
  return std::make_unique<int>(value);
}

inline std::unique_ptr<NonTrivialType> MakeUniquePtrForNonTrivialType(
    int value) {
  return std::make_unique<NonTrivialType>(value);
}

inline int UseUniquePtrTypeByValueForNonTrivialType(
    std::unique_ptr<NonTrivialType> p) {
  return p->x;
}

inline int UseUniquePtrByRefForNonTrivialType(
    std::unique_ptr<NonTrivialType>& p) {
  return p->x;
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_HELPER_LIB_H_
