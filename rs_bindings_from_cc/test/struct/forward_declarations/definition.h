// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DEFINITION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DEFINITION_H_

#pragma clang lifetime_elision

struct UnpinStruct final {
  UnpinStruct() = default;
  int field = 0;
};

struct NonunpinStruct /* non-final */ {
  NonunpinStruct() = default;
  int field = 0;
};

inline int ReadCompleteUnpinStruct(const UnpinStruct& s) { return s.field; }
inline void WriteCompleteUnpinStruct(UnpinStruct& s, int value) {
  s.field = value;
}

inline int ReadCompleteNonunpinStruct(const NonunpinStruct& s) {
  return s.field;
}
inline void WriteCompleteNonunpinStruct(NonunpinStruct& s, int value) {
  s.field = value;
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DEFINITION_H_
