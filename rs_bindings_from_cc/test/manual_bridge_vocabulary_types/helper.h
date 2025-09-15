// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_HELPER_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_HELPER_LIB_H_

#include <cstddef>
#include <memory>
#include <new>
#include <string>
#include <vector>

struct OverloadedDelete {
  static void operator delete(void* ptr) { ::operator delete(ptr); }
};

struct OverloadedDestroyingDelete {
  static void operator delete(OverloadedDestroyingDelete* ptr,
                              std::destroying_delete_t) {
    ptr->~OverloadedDestroyingDelete();
    ::operator delete(ptr);
  }
};

struct PolymorphicType {
  virtual ~PolymorphicType() = default;
};

struct FinalType final : PolymorphicType {};

// std::unique_ptr begins
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

// std::unique_ptr ends

// std::vector begins
inline size_t UseVectorByValue(std::vector<int> v) { return v.size(); }
inline size_t UseVectorByRef(std::vector<int>& v) { return v.size(); }
inline std::vector<int> MakeVector(int value) { return {value}; }
inline std::vector<std::string> MakeVectorString() { return {"hello, world"}; }
inline std::vector<bool> MakeVectorBool() { return {}; }

inline std::vector<OverloadedDelete> MakeVectorOverloadedDelete() { return {}; }
inline std::vector<OverloadedDestroyingDelete>
MakeVectorOverloadedDestroyingDelete() {
  return {};
}
inline std::vector<PolymorphicType> MakeVectorPolymorphicType() { return {}; }
inline std::vector<FinalType> MakeVectorFinalType() { return {}; }
// std::vector ends

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_HELPER_LIB_H_
