// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "qualifiers.h"
volatile int vol_var = 3;
const volatile int const_vol_var = 4;
void process_ptr_to_ptr(int** pp) {}
void process_ptr_to_const(const int* p) {}
