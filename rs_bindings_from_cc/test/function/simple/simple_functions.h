// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_

#pragma clang lifetime_elision

int return_value();
int* return_pointer();
int& return_reference();
void take_pointer(int* i);
void take_reference(int& i);
const int* forward_pointer(const int* i);
const int& forward_reference(const int& i);
int multiply(int x, int y);
int multiply_with_unnamed_parameters(int, int);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
