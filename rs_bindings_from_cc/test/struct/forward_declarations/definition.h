// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DEFINITION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DEFINITION_H_

#pragma clang lifetime_elision

struct UnpinStruct final {
  int field = 0;
};

struct NonunpinStruct /* non-final */ {
  int field = 0;
};

int ReadUnpinStruct(const UnpinStruct& s);
void WriteUnpinStruct(UnpinStruct& s, int value);
int ReadNonunpinStruct(const NonunpinStruct& s);
void WriteNonunpinStruct(NonunpinStruct& s, int value);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DEFINITION_H_
