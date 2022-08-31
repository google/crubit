// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DECLARATION_1_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DECLARATION_1_H_

#pragma clang lifetime_elision

struct UnpinStruct;
struct NonunpinStruct;

int ReadUnpinStruct(const UnpinStruct& s);
void WriteUnpinStruct(UnpinStruct& s, int value);

int ReadNonunpinStruct(const NonunpinStruct& s);
void WriteNonunpinStruct(NonunpinStruct& s, int value);

// `inline` keyword forces generation of C++ thunks in the generated
// `..._rs_api_impl.cc` file.
inline int InlineFunctionTakingUnpinStruct(const UnpinStruct& s) {
  return ReadUnpinStruct(s);
}

// `inline` keyword forces generation of C++ thunks in the generated
// `..._rs_api_impl.cc` file.
inline int InlineFunctionTakingNonunpinStruct(const NonunpinStruct& s) {
  return ReadNonunpinStruct(s);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DECLARATION_1_H_
