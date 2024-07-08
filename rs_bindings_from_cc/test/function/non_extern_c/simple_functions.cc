// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/function/non_extern_c/simple_functions.h"

int return_value() { return 42; }

int* return_pointer() {
  static int i = 42;
  return &i;
}

int& return_reference() {
  static int i = 42;
  return i;
}

void take_pointer(int* i) {
  if (i) {
    *i = 42;
  }
}

void take_reference(int& i) { i = 42; }

const int* forward_pointer(const int* i) { return i; }

const int& forward_reference(const int& i) { return i; }

int multiply(int x, int y) { return x * y; }

int multiply_with_unnamed_parameters(int x, int y) { return x * y; }

int multiply_with_keyword_named_parameters(int self, int crate, int super) {
  return self * crate * super;
}

int (*get_pointer_to_multiply_function())(int, int) { return multiply; }
int (&get_reference_to_multiply_function())(int, int) { return multiply; }
