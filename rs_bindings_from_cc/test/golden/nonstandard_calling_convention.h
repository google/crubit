// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONSTANDARD_CALLING_CONVENTION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONSTANDARD_CALLING_CONVENTION_H_

float function_vectorcall(float a, float b) [[clang::vectorcall]];
int function_win64(int a) __attribute__((ms_abi));

class SomeClass {
 public:
  float function_vectorcall(float a, float b) [[clang::vectorcall]];
  int function_win64(int a) __attribute__((ms_abi));
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONSTANDARD_CALLING_CONVENTION_H_
