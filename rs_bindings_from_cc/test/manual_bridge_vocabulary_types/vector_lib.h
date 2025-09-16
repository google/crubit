// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_VECTOR_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_VECTOR_LIB_H_

#include <cstddef>
#include <string>
#include <vector>

#include "rs_bindings_from_cc/test/manual_bridge_vocabulary_types/common.h"
#include "support/annotations.h"

CRUBIT_MUST_BIND
inline size_t UseVectorByValue(std::vector<int> v) { return v.size(); }
CRUBIT_MUST_BIND
inline size_t UseVectorByRef(std::vector<int>& v) { return v.size(); }
CRUBIT_MUST_BIND
inline std::vector<int> MakeVector(int value) { return {value}; }
inline std::vector<std::string> MakeVectorString() { return {"hello, world"}; }
inline std::vector<bool> MakeVectorBool() { return {}; }

inline std::vector<OverloadedDelete> MakeVectorOverloadedDelete() { return {}; }
inline std::vector<OverloadedDestroyingDelete>
MakeVectorOverloadedDestroyingDelete() {
  return {};
}
inline std::vector<PolymorphicType> MakeVectorPolymorphicType() { return {}; }
CRUBIT_MUST_BIND
inline std::vector<FinalType> MakeVectorFinalType() { return {}; }
inline std::vector<DeletedDestructorType> MakeVectorDeletedDestructor() {
  return {};
}
inline std::vector<NonTrivialType> MakeVectorNonTrivial() { return {}; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_VECTOR_LIB_H_
