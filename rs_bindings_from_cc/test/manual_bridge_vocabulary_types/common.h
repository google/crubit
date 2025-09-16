// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_COMMON_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_COMMON_H_

#include <new>

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

struct DeletedDestructorType {
  ~DeletedDestructorType() = delete;
};

struct NonTrivialType {
  int x;
  ~NonTrivialType() { x = 0; }
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_COMMON_H_
