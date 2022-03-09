// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_LIFETIMES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_LIFETIMES_H_

void AddHook(void (*)());

typedef void (*FunctionPointer)();
void AddHookWithTypedef(FunctionPointer hook);

void AddAnotherHook(void (&)());

typedef void (&FunctionReference)();
void AddAnotherHookWithTypedef(FunctionReference hook);

void ConsumeArray(int pair[2]);

typedef int Arr[2];
void ConsumeArrayWithTypedef(Arr);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_LIFETIMES_H_
