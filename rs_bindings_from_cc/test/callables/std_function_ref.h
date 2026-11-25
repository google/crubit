// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CALLABLES_STD_FUNCTION_REF_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CALLABLES_STD_FUNCTION_REF_H_

#include <functional>

void CallStdFunctionRefVoidVoid(std::function_ref<void()> f);
int CallStdFunctionRefIntInt(std::function_ref<int(int) const> f, int n);
void CallStdFunctionRefNoexcept(std::function_ref<void() noexcept> f);
int CallStdFunctionRefMultipleTimes(std::function_ref<int(int)> f);
void CallStdFunctionRefFromTwoThreads(std::function_ref<void()> f);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CALLABLES_STD_FUNCTION_REF_H_
