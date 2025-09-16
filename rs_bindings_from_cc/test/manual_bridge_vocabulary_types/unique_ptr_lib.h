// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_UNIQUE_PTR_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_UNIQUE_PTR_LIB_H_

#include <memory>
#include <string>

#include "rs_bindings_from_cc/test/manual_bridge_vocabulary_types/common.h"

struct IncompleteType;

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

inline std::unique_ptr<std::string> MakeUniquePtrString() {
  return std::make_unique<std::string>("hello, world");
}

inline std::unique_ptr<OverloadedDelete> MakeUniquePtrOverloadedDelete() {
  return nullptr;
}

inline std::unique_ptr<OverloadedDestroyingDelete>
MakeUniquePtrOverloadedDestroyingDelete() {
  return nullptr;
}

inline std::unique_ptr<PolymorphicType> MakeUniquePtrPolymorphicType() {
  return nullptr;
}

inline std::unique_ptr<FinalType> MakeUniquePtrFinalType() { return nullptr; }
inline std::unique_ptr<IncompleteType> MakeUniquePtrIncompleteType() {
  return nullptr;
}
inline std::unique_ptr<DeletedDestructorType> MakeUniquePtrDeletedDestructor() {
  return nullptr;
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_UNIQUE_PTR_LIB_H_
